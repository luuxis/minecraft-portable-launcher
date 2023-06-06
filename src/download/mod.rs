mod download_unique;
pub use download_unique::*;

use sha1::Digest;
use std::path::Path;
use core::fmt;

pub enum DownloadError {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    HeaderEncoding(reqwest::header::ToStrError),
    DownloadStopped,
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DownloadError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            DownloadError::Io(e) => write!(f, "IO error: {}", e),
            DownloadError::HeaderEncoding(e) => write!(f, "Header encoding error: {}", e),
            DownloadError::DownloadStopped => write!(f, "Download stopped"),
        }
    }
}

fn file_to_hash<H, P>(path: P) -> Result<Vec<u8>, DownloadError> 
where
    H: Digest + std::io::Write,
    P: AsRef<Path>,
{
    let mut file = std::fs::File::open(path).map_err(DownloadError::Io)?;
    let mut hasher = H::new();
    std::io::copy(&mut file, &mut hasher).map_err(DownloadError::Io)?;
    Ok(hasher.finalize().to_vec())
}