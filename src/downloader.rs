
use super::errors::DownloadError;
use reqwest;
use std::path::{Path};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use log::info;
use log::debug;

pub struct Downloader {
    pub client: reqwest::Client,
    pub fragments: usize,
}


impl Downloader {
    pub fn new(fragments: usize) -> Self {
        let client = reqwest::Client::new();
        Downloader { client, fragments }
    }

    async fn supports_range(&self, url: &str) -> Result<bool, DownloadError> {
        let resp = self.client.head(url).send().await?;
        Ok(resp.headers().get(reqwest::header::ACCEPT_RANGES).map(|v| v == "bytes").unwrap_or(false))
    }

    async fn fetch_size(&self, url: &str) -> Result<u64, DownloadError> {
        let resp = self.client.head(url).send().await?;
        let len = resp.headers().get(reqwest::header::CONTENT_LENGTH).and_then(|ct_len| ct_len.to_str().ok()).and_then(|ct_len| ct_len.parse().ok()).unwrap_or(0);
        Ok(len)
    }

    pub async fn download(&self, url: &str, dir: &Path) -> Result<(), DownloadError> {
        debug!("Fetching URL: {}", url);
        info!(
            "Downloading URL: {} on thread {:?}",
            url,
            std::thread::current().name().unwrap_or("unknown")
        );

        let file_name = Path::new(url).file_name().unwrap();
        let dest_path = dir.join(file_name);

        // Skip if file already exists
        if dest_path.exists() {
            println!("File already exists, skipping: {}", url);
            return Ok(());
        }

        if self.supports_range(url).await? {
            let total_size = self.fetch_size(url).await?;
            let fragment_size = total_size / self.fragments as u64;

            let mut handles = vec![];

            for i in 0..self.fragments {
                let start = i as u64 * fragment_size;
                let end = if i == self.fragments - 1 {
                    total_size - 1
                } else {
                    start + fragment_size - 1
                };

                let client = self.client.clone();
                let url = url.to_string();
                let dir = dir.to_path_buf();

                let handle = tokio::spawn(async move {
                    let range = format!("bytes={}-{}", start, end);
                    let resp = client.get(&url).header(reqwest::header::RANGE, range).send().await?;
                    let content = resp.bytes().await?;
                    let path = dir.join(format!("fragment_{}", i));
                    let _ = tokio::fs::write(&path, content).await.map_err(|e| DownloadError::Other(e.to_string()));
                    Ok::<_, reqwest::Error>(path)
                });

                handles.push(handle);
            }

            let mut paths = Vec::new();
            for handle in handles {
                let path = handle.await??;
                paths.push(path);
            }

            // Combine fragments
            let dest_path = dir.join(Path::new(url).file_name().unwrap());
            let mut dest = File::create(&dest_path).await?;

            for path in paths {
                let content = tokio::fs::read(&path).await?;
                dest.write_all(&content).await?;
                tokio::fs::remove_file(path).await?;
            }

            info!("Downloaded: {}", url);
            Ok(())

        } else {
            let response = self.client.get(url).send().await?;
            let bytes = response.bytes().await?;

            let path = dir.join(Path::new(url).file_name().unwrap());
            tokio::fs::write(&path, bytes).await?;

            info!("Downloaded (without fragmentation): {}", url);
            Ok(())
        }
    }
}
