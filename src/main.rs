mod downloader;
pub mod errors;

use downloader::Downloader;
use structopt::StructOpt;
use tokio;
use num_cpus;
use std::path::PathBuf;
use std::fs;
use futures;
use log::info;

#[derive(StructOpt, Debug)]
#[structopt(name = "rustic-fetch", about = "A multi-threaded MP4 downloader.")]
struct Opt {
    /// URLs to download
    #[structopt(name = "URL", parse(try_from_str))]
    urls: Vec<String>,

    /// Directory to save the downloads
    #[structopt(short, long, parse(from_os_str), default_value = ".")]
    dir: PathBuf,

    /// Enable verbose logging
    #[structopt(short, long)]
    verbose: bool,

    /// Number of threads to use for downloading
    #[structopt(short = "t", long = "threads", default_value = "4")]
    threads: usize,

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
        eprintln!("Error: Specified path is not a writable directory.");
        std::process::exit(1);
    }

    info!("Starting download...");

    let tasks: Vec<_> = opt.urls.into_iter().map(|url| {
        // Clone the client for each task
        let dir = opt.dir.clone();
        tokio::spawn(async move {
            let task_downloader = Downloader::new(2);
            task_downloader.download(&url, &dir).await.unwrap();
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
