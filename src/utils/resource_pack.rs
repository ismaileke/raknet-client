//! Downloads the resource packs the server offers and writes them to disk as .zip files.
//!
//! Bedrock flow (packs streamed over the game connection):
//!   server -> ResourcePacksInfo         (which packs exist, size, encryption key, CDN url)
//!   client -> ResourcePackClientResponse { SEND_PACKS, [uuid...] }
//!   server -> ResourcePackDataInfo      (per pack: chunk count, size, sha256)
//!   client -> ResourcePackChunkRequest  (one chunk at a time)
//!   server -> ResourcePackChunkData     (raw zip bytes)
//!   client -> ResourcePackClientResponse { HAVE_ALL_PACKS }
//!
//! Some servers do not stream the pack at all: the ResourcePacksInfo entry carries a
//! non-empty `cdn_url` (gophertunnel calls it `DownloadURL`). For those packs the real
//! client skips the chunk protocol entirely and fetches the zip over plain HTTP(S).
//! Those packs must NOT be listed in SEND_PACKS, or the server will answer with an
//! error / never send a ResourcePackDataInfo for them.
//!
//! Disabled by default. To enable, before `client::create`:
//!   bedrock_client::utils::resource_pack::set_download_dir(Some("packs".into()));

use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::Write;
use std::sync::Mutex;

static DOWNLOAD_DIR: Mutex<Option<String>> = Mutex::new(None);

/// User agent the vanilla Bedrock client uses. Some CDNs reject the default
/// reqwest agent, so send the same one.
const HTTP_USER_AGENT: &str = "libhttpclient/1.0.0.0";

/// How many ResourcePackChunkRequest packets to keep in flight per pack.
/// Requesting one chunk per round trip is far slower than the vanilla client and
/// makes proxies (WaterdogPE, Geyser) hit their resource-pack phase timeout and
/// close the connection. Requesting every chunk at once instead floods RakNet, so
/// use a sliding window.
const CHUNK_WINDOW: u32 = 8;

/// Directory the resource packs are written to. `None` = do not download (default).
pub fn set_download_dir(dir: Option<String>) {
    *DOWNLOAD_DIR.lock().unwrap() = dir;
}

pub fn download_dir() -> Option<String> {
    DOWNLOAD_DIR.lock().unwrap().clone()
}

/// A pack that is served over HTTP instead of the game connection.
#[derive(Clone, Debug)]
pub struct CdnPack {
    pub uuid: String,
    pub version: String,
    /// Encryption key announced by the server (empty means the pack is not encrypted).
    pub key: String,
    pub url: String,
    pub size: u64,
}

/// Download state of a single pack that is streamed in chunks.
struct Pending {
    uuid: String,
    version: String,
    /// Encryption key announced by the server (empty means the pack is not encrypted).
    key: String,
    buf: Vec<u8>,
    chunk_count: u32,
    /// Lowest chunk index that has not been requested yet.
    next_chunk: u32,
    /// How many chunks have actually arrived.
    received: u32,
    max_chunk_size: u32,
    expected_sha: Vec<u8>,
    expected_size: u64,
}

#[derive(Default)]
pub struct PackDownloader {
    /// Taken from ResourcePacksInfo: uuid -> (version, encryption key, size)
    announced: HashMap<String, (String, String, u64)>,
    active: HashMap<String, Pending>,
    /// Packs that have to be fetched over HTTP, drained by `take_cdn`.
    cdn: Vec<CdnPack>,
    /// How many chunk-streamed packs are still expected. The server sends one
    /// ResourcePackDataInfo at a time, so `active` being empty only means the
    /// pack in flight is done, not that every pack has arrived.
    remaining: usize,
    pub finished: usize,
    pub failed: usize,
    /// Packs that were already on disk and therefore not downloaded again.
    pub cached: usize,
}

impl PackDownloader {
    pub fn new() -> Self {
        Self::default()
    }

    /// Called when ResourcePacksInfo arrives.
    /// Each tuple is (uuid, version, encryption key, size, cdn url).
    ///
    /// Returns the pack ids to ask for over the game connection. Packs that carry a
    /// CDN url are excluded from that list and queued in `take_cdn` instead.
    pub fn on_info(&mut self, packs: &[(String, String, String, u64, String)]) -> Vec<String> {
        if download_dir().is_none() {
            return Vec::new();
        }
        self.announced.clear();
        self.cdn.clear();
        self.cached = 0;
        let mut ids = Vec::new();
        for (uuid, version, key, size, url) in packs {
            self.announced
                .insert(uuid.clone(), (version.clone(), key.clone(), *size));

            // Already on disk from an earlier session: neither request its chunks
            // nor fetch it over HTTP again. The file name carries the version, so a
            // pack update still downloads.
            if is_cached(uuid, version, key) {
                self.cached += 1;
                continue;
            }

            if !url.trim().is_empty() {
                // Served by an HTTP server: never request chunks for it.
                self.cdn.push(CdnPack {
                    uuid: uuid.clone(),
                    version: version.clone(),
                    key: key.clone(),
                    url: url.clone(),
                    size: *size,
                });
                continue;
            }

            // Some servers expect "uuid_version"; every server accepts that form.
            ids.push(format!("{}_{}", uuid, version));
        }
        self.remaining = ids.len();
        if self.cached > 0 {
            println!("[resource pack] {} pack(s) already on disk, skipped", self.cached);
        }
        ids
    }

    /// Takes the packs that must be fetched over HTTP. Call this right after `on_info`.
    pub fn take_cdn(&mut self) -> Vec<CdnPack> {
        std::mem::take(&mut self.cdn)
    }

    /// Called when ResourcePackDataInfo arrives.
    /// Returns the chunk indices to request immediately (a full window, not just one).
    pub fn on_data_info(
        &mut self,
        pack_id: &str,
        chunk_count: u32,
        max_chunk_size: u32,
        compressed_size: u64,
        sha256: &[u8],
    ) -> Option<Vec<u32>> {
        download_dir()?;
        // pack_id may be in the "uuid_version" form.
        let uuid = pack_id.split('_').next().unwrap_or(pack_id).to_string();
        let (version, key, _) = self
            .announced
            .get(&uuid)
            .cloned()
            .unwrap_or_else(|| (String::new(), String::new(), 0));

        let calculated_chunk_count = (compressed_size + max_chunk_size as u64 - 1) / max_chunk_size as u64;
        let real_chunk_count = chunk_count.max(calculated_chunk_count as u32);

        self.active.insert(
            pack_id.to_string(),
            Pending {
                uuid,
                version,
                key,
                buf: Vec::with_capacity(compressed_size as usize),
                chunk_count: real_chunk_count,
                next_chunk: 0,
                received: 0,
                max_chunk_size,
                expected_sha: sha256.to_vec(),
                expected_size: compressed_size,
            },
        );

        let window = CHUNK_WINDOW.min(real_chunk_count);
        if let Some(p) = self.active.get_mut(pack_id) {
            p.next_chunk = window;
        }
        Some((0..window).collect())
    }

    /// Called when ResourcePackChunkData arrives.
    /// Returns the next chunk index to request; `None` means this pack is complete.
    pub fn on_chunk(&mut self, pack_id: &str, index: u32, offset: u64, data: &[u8]) -> Option<u32> {
        let _ = index;
        let done_uuid;
        {
            let Some(p) = self.active.get_mut(pack_id) else {
                return None;
            };
            // Chunks can arrive out of order because several requests are in flight,
            // so always write at `offset` rather than appending.
            let start = offset as usize;
            if p.buf.len() < start + data.len() {
                p.buf.resize(start + data.len(), 0);
            }
            p.buf[start..start + data.len()].copy_from_slice(data);

            p.received += 1;

            // Slide the window: one chunk arrived, so ask for one more.
            if p.next_chunk < p.chunk_count {
                let i = p.next_chunk;
                p.next_chunk += 1;
                return Some(i);
            }
            // Chunks may arrive out of order, so wait for the count, not the index.
            if p.received < p.chunk_count {
                return None;
            }
            done_uuid = pack_id.to_string();
        }
        self.finish(&done_uuid);
        None
    }

    /// True once EVERY pack streamed over the connection has been written, not just
    /// the one currently in flight. CDN packs run in the background and are not counted.
    pub fn all_done(&self) -> bool {
        self.active.is_empty() && self.remaining == 0
    }

    fn finish(&mut self, pack_id: &str) {
        let Some(p) = self.active.remove(pack_id) else { return };

        // Size check against what ResourcePackDataInfo announced.
        if p.expected_size > 0 && p.buf.len() as u64 != p.expected_size {
            println!(
                "[resource pack] {} size mismatch: got {} bytes, expected {}",
                p.uuid,
                p.buf.len(),
                p.expected_size
            );
        }
        if !p.expected_sha.is_empty() {
            let got = Sha256::digest(&p.buf);
            if got.as_slice() != p.expected_sha.as_slice() {
                println!(
                    "[resource pack] {} sha256 MISMATCH (saving anyway): got {}, expected {}",
                    p.uuid,
                    hex::encode(got),
                    hex::encode(&p.expected_sha)
                );
            }
        }

        match save_pack(&p.uuid, &p.version, &p.key, &p.buf) {
            Ok(_) => self.finished += 1,
            Err(_) => self.failed += 1,
        }
        self.remaining = self.remaining.saturating_sub(1);
        // Announced by the server but not needed for sequential requests.
        let _ = p.max_chunk_size;
    }
}

/// Writes the finished archive (and its key, if encrypted) to the download directory.
/// Where a pack is stored. `None` when downloading is disabled.
///
/// The version is part of the name, so a pack the server bumped is treated as a
/// different file and downloaded again.
pub fn pack_path(uuid: &str, version: &str) -> Option<String> {
    let dir = download_dir()?;

    // Keep the file name safe for every filesystem.
    let safe: String = uuid
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '_' })
        .collect();
    let name = if version.is_empty() {
        format!("{}.zip", safe)
    } else {
        format!("{}_{}.zip", safe, version.replace('.', "-"))
    };
    Some(format!("{}/{}", dir.trim_end_matches('/'), name))
}

/// Is this pack already downloaded? A pack is only written once every chunk has
/// arrived and the sha256 matched, so an existing non-empty file is complete.
pub fn is_cached(uuid: &str, version: &str, key: &str) -> bool {
    let Some(path) = pack_path(uuid, version) else { return false };
    let Ok(meta) = std::fs::metadata(&path) else { return false };
    if !meta.is_file() || meta.len() == 0 {
        return false;
    }
    // Encrypted packs are useless without the key file next to them.
    if !key.is_empty() && !std::path::Path::new(&format!("{}.key", path)).is_file() {
        return false;
    }
    true
}

fn save_pack(uuid: &str, version: &str, key: &str, buf: &[u8]) -> Result<String, ()> {
    let Some(dir) = download_dir() else { return Err(()) };

    if std::fs::create_dir_all(&dir).is_err() {
        println!("[resource pack] could not create directory: {}", dir);
        return Err(());
    }

    let Some(path) = pack_path(uuid, version) else { return Err(()) };

    match std::fs::File::create(&path).and_then(|mut f| f.write_all(buf)) {
        Ok(_) => {
            println!(
                "[resource pack] saved: {} ({} bytes){}",
                path,
                buf.len(),
                if key.is_empty() { "" } else { "  [ENCRYPTED]" }
            );
            if !key.is_empty() {
                // Marketplace packs arrive AES-encrypted. Store the key next to the
                // archive so whoever wants to decrypt the contents can use it.
                let _ = std::fs::write(format!("{}.key", path), key.as_bytes());
                println!("[resource pack] encryption key: {}.key", path);
            }
            Ok(path)
        }
        Err(e) => {
            println!("[resource pack] could not write {}: {}", path, e);
            Err(())
        }
    }
}

/// Fetches one CDN-hosted pack over HTTP and writes it out.
///
/// Spawn this on the tokio runtime; it must not block the RakNet loop, otherwise no
/// ACKs are sent while a large pack downloads and the server drops the connection.
pub async fn download_cdn(pack: CdnPack) {
    if download_dir().is_none() {
        return;
    }
    if is_cached(&pack.uuid, &pack.version, &pack.key) {
        println!("[resource pack] {} already on disk, skipping CDN download", pack.uuid);
        return;
    }
    println!("[resource pack] {} via CDN: {}", pack.uuid, pack.url);

    let client = match reqwest::Client::builder()
        .user_agent(HTTP_USER_AGENT)
        // Some CDNs answer with a 30x to a signed URL on another host.
        .redirect(reqwest::redirect::Policy::limited(10))
        .timeout(std::time::Duration::from_secs(120))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            println!("[resource pack] http client error: {}", e);
            return;
        }
    };

    let resp = match client.get(&pack.url).send().await {
        Ok(r) => r,
        Err(e) => {
            println!("[resource pack] {} download failed: {}", pack.uuid, e);
            return;
        }
    };
    if !resp.status().is_success() {
        println!("[resource pack] {} download failed: HTTP {}", pack.uuid, resp.status());
        return;
    }

    let body = match resp.bytes().await {
        Ok(b) => b,
        Err(e) => {
            println!("[resource pack] {} body error: {}", pack.uuid, e);
            return;
        }
    };

    // The size in ResourcePacksInfo is advisory for CDN packs, so only warn.
    if pack.size > 0 && body.len() as u64 != pack.size {
        println!(
            "[resource pack] {} size mismatch: got {} bytes, announced {}",
            pack.uuid,
            body.len(),
            pack.size
        );
    }
    // A CDN that 404s through a friendly HTML page would otherwise be saved as a zip.
    if body.len() < 4 || &body[..2] != b"PK" {
        println!("[resource pack] {} is not a zip archive, skipping", pack.uuid);
        return;
    }

    let _ = save_pack(&pack.uuid, &pack.version, &pack.key, &body);
}