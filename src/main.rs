mod downloader;
pub mod errors;
mod helpers;

use downloader::Downloader;
use structopt::StructOpt;
use tokio;
use num_cpus;
use std::path::PathBuf;
use std::fs;
use futures;
use log::info;
use log::error;
use log::debug;

#[derive(StructOpt, Debug)]
#[structopt(name = "rustic-fetch", about = "A multi-threaded MP4 downloader.")]
struct Opt {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut opt = Opt::from_args();

    if opt.verbose {
        env_logger::Builder::new().filter_level(log::LevelFilter::Debug).init();
    } else {
        env_logger::Builder::new().filter_level(log::LevelFilter::Info).init();
    }

    // Ensure the number of threads does not exceed the number of available CPU cores
    if opt.threads > num_cpus::get() {
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
                let (_url, headers) = helpers::parse_curl_command(line)?;
                headers_parsed = headers;
                // Store the URL and headers for later use
            }
        } else {
            urls.extend(file_content.lines().map(|line| line.to_string()));
        }
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

#[cfg(test)]
mod cli_tests {
    use assert_cmd::Command;

    #[test]
    fn test_help_output() {
        let mut cmd = Command::cargo_bin("RusticFetch").unwrap();
        cmd.arg("--help");
        cmd.assert().success();
    }

    #[test]
    fn test_invalid_url() {
        let mut cmd = Command::cargo_bin("RusticFetch").unwrap();
        cmd.arg("invalid_url");
        cmd.assert().success();
    }
}
