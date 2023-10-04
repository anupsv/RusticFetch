mod downloader;
mod errors;
mod helpers;
use crate::downloader::Downloader;
use structopt::StructOpt;
use tokio;
use num_cpus;
use log::info;
use log::error;
use std::path::PathBuf;
use std::fs;
use futures;
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, mock, predicate::*};

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "rustic-fetch", about = "A multi-threaded MP4 downloader.")]
pub struct Opt {
    /// URLs to download
    #[structopt(name = "URL", parse(try_from_str))]
    urls: Vec<String>,

    /// File containing URLs to download (one URL per line)
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    file: Option<PathBuf>,

    /// Directory to save the downloads
    #[structopt(short, long, parse(from_os_str), default_value = ".")]
    dir: PathBuf,

    /// Enable verbose logging
    #[structopt(short, long)]
    verbose: bool,

    /// Number of threads to use for downloading
    #[structopt(short = "t", long = "threads", default_value = "4")]
    threads: usize,

    /// Treat file input as curl commands
    #[structopt(long = "curl-format")]
    curl_format: bool,

}

struct App {
    opt: Opt,
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait AppRunner {
    fn init(opt: Opt) -> Self;
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl AppRunner for App {
    fn init(opt: Opt) -> Self {
        Self { opt }
    }

    // pub fn setup_logger(&self) {
    //     if self.opt.verbose {
    //         env_logger::Builder::new().filter_level(log::LevelFilter::Debug).init();
    //     } else {
    //         env_logger::Builder::new().filter_level(log::LevelFilter::Info).init();
    //     }
    // }
    //
    // pub fn validate_threads(&mut self) {
    //     if self.opt.threads > num_cpus::get() {
    //         self.opt.threads = num_cpus::get();
    //     }
    // }
    //
    // pub fn ensure_directory(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     if !self.opt.dir.exists() {
    //         fs::create_dir_all(&self.opt.dir)?;
    //     }
    //     if !self.opt.dir.is_dir() || fs::metadata(&self.opt.dir)?.permissions().readonly() {
    //         return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Specified path is not a writable directory.")));
    //     }
    //     Ok(())
    // }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut opt = self.opt.clone();

        if opt.verbose {
            env_logger::Builder::new().filter_level(log::LevelFilter::Debug).init();
        } else {
            env_logger::Builder::new().filter_level(log::LevelFilter::Info).init();
        }

        // Ensure the number of threads does not exceed the number of available CPU cores
        if opt.threads > num_cpus::get() {
            error!("Error: {}", opt.threads);
            opt.threads = num_cpus::get();
        }

        // Ensure directory exists and is writable
        if !opt.dir.exists() {
            fs::create_dir_all(&opt.dir)?;
        }
        if !opt.dir.is_dir() || fs::metadata(&opt.dir)?.permissions().readonly() {
            error!("Error: Specified path is not a writable directory.");
            std::process::exit(1);
        }

        let mut urls = Vec::new();
        let mut headers_parsed = Vec::new();

        if let Some(file_path) = opt.file {
            let file_content = std::fs::read_to_string(file_path)?;
            if opt.curl_format {
                for line in file_content.lines() {
                    let (url, headers) = helpers::parse_curl_command(line)?;
                    headers_parsed = headers;
                    urls.push(url.clone());
                }
            } else {
                urls.extend(file_content.lines().map(|line| line.to_string()));
            }
        } else {
            urls.extend(opt.urls);
        }

        if urls.is_empty() {
            error!("Error: No URLs provided. Please specify URLs via the command line or provide a file with URLs.");
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No URLs provided"))?;
        }

        info!("Starting download...");

        let tasks: Vec<_> = urls.into_iter().map(|url| {
            // Clone the client for each task
            let dir = opt.dir.clone();
            let new_headers_parsed = headers_parsed.clone();
            tokio::spawn(async move {
                let task_downloader = Downloader::new(2);
                task_downloader.download(&url, &new_headers_parsed, &dir).await.unwrap();
            })
        }).collect();
        futures::future::join_all(tasks).await;
        Ok(())
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let app = App::init(opt);
    app.run().await
}

#[cfg(test)]
mod cli_tests {
    use super::*;

    // #[tokio::test]
    // async fn test_run() {
    //     let args = vec!["rustic-fetch", "http://example.com", "-t", "1125", "-d", "/tmp"];
    //     let opt = Opt::from_iter(args);
    //     let mut mock_app = MockAppRunner::new();
    //     mock_app.init(opt);
    //     // mock_app.expect_run().times(1).returning(|| Box::pin(async { Ok(()) }));
    //     //
    //     // let result = mock_app.run().await;
    //     // assert!(result.is_ok());
    // }

    #[test]
    fn test_argument_parsing() {
        let args = vec!["rustic-fetch", "http://example.com", "-t", "5", "-d", "/tmp"];
        let opt = Opt::from_iter(args);
        assert_eq!(opt.urls, vec!["http://example.com"]);
        assert_eq!(opt.threads, 5);
        assert_eq!(opt.dir, PathBuf::from("/tmp"));
    }

    #[test]
    fn test_directory_creation() {
        let dir = PathBuf::from("test_dir");
        if dir.exists() {
            fs::remove_dir_all(&dir).unwrap();
        }
        assert!(!dir.exists());
        fs::create_dir_all(&dir).unwrap();
        assert!(dir.exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_url_collection_from_command_line() {
        let args = vec!["rustic-fetch", "http://example.com"];
        let opt = Opt::from_iter(args);
        assert_eq!(opt.urls, vec!["http://example.com"]);
    }
}
