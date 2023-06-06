
use super::{file_to_hash, DownloadError};
use std::{path::{Path, PathBuf}, io::Write};
use md5::Md5;
use futures_util::StreamExt;

async fn download_header(url: &str) -> Result<reqwest::header::HeaderMap, DownloadError> {
    let client = reqwest::Client::new();
    let response = client.head(url).send().await.map_err(DownloadError::Reqwest)?;
    Ok(response.headers().clone())
}

pub async fn download_file<P, F>(url: &str, path: P, callback_send_delta: F) -> Result<(), DownloadError> 
where
    P: AsRef<Path>,
    F: Fn(u32) -> bool
{
    let path = path.as_ref();
    let response = reqwest::get(url).await
        .map_err(DownloadError::Reqwest)?;
    let (content_length, mut stream) = (response.content_length(), response.bytes_stream());
    
    let parent = path.parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| 
            std::env::current_dir()
            .ok()
            .unwrap_or_else(|| PathBuf::from("."))
        );
    if !parent.exists() {
        std::fs::create_dir_all(parent)
            .map_err(DownloadError::Io)?;
    }

    let mut file = std::fs::File::create(path)
        .map_err(DownloadError::Io)?;
    let mut current_bytes = 0;
    let mut current_percent = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(DownloadError::Reqwest)?;
        if let Some(length) = content_length {
            current_bytes += chunk.len();
            let next_percent = current_bytes * 100 / length as usize;
            if next_percent > current_percent {
                if callback_send_delta((next_percent - current_percent) as u32) {
                    return Err(DownloadError::DownloadStopped);
                }
                current_percent = next_percent;
            }
        }
        file
            .write_all(&chunk)
            .map_err(DownloadError::Io)?;
    }
    Ok(())
}