mod downloader;
pub mod errors;

use downloader::Downloader;
use structopt::StructOpt;
use tokio;
use num_cpus;
use std::path::PathBuf;
use std::fs;
use futures;
use std::fmt;
use std::error::Error;


#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long, default_value = "4", parse(try_from_str = parse_threads))]
    threads: usize,

    #[structopt(short, long, parse(from_os_str))]
    dir: PathBuf,

    #[structopt(name = "URL")]
    urls: Vec<String>,
}

#[derive(Debug)]
enum ThreadParseError {
    ParseError(std::num::ParseIntError),
    TooManyThreads,
}

impl fmt::Display for ThreadParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThreadParseError::ParseError(e) => write!(f, "Parse error: {}", e),
            ThreadParseError::TooManyThreads => write!(f, "Specified number of threads exceeds available CPUs"),
        }
    }
}

impl Error for ThreadParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ThreadParseError::ParseError(e) => Some(e),
            ThreadParseError::TooManyThreads => None,
        }
    }
}

fn parse_threads(src: &str) -> Result<usize, ThreadParseError> {
    let parsed_value = src.parse::<usize>().map_err(ThreadParseError::ParseError)?;
    if parsed_value > num_cpus::get() {
        Err(ThreadParseError::TooManyThreads)
    } else {
        Ok(parsed_value)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut opt = Opt::from_args();

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
