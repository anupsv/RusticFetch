# RusticFetch

RusticFetch is a multi-threaded MP4 downloader written in Rust. It's designed to efficiently download multiple files concurrently, with support for fragmented downloads. Whether you're looking to batch download a collection of videos or ensure that you're downloading files in the most efficient manner, RusticFetch has got you covered.

## Features

- **Multi-threaded Downloads**: Download multiple files concurrently to maximize bandwidth usage.
- **Fragmented Downloads**: Supports downloading files in fragments, ensuring faster and more reliable downloads.
- **Command Line Interface**: Easy-to-use CLI for quick downloads.
- **Verbose Logging**: Detailed logs to understand what's happening behind the scenes.

## Installation

Before running RusticFetch, ensure you have Rust and Cargo installed. If not, [install them from here](https://rustup.rs/).

Clone the repository:

```bash
git clone https://github.com/yourusername/rusticfetch.git
cd rusticfetch
```

Build the project:

```bash
cargo build --release
```

## Usage

After building, you can run RusticFetch using:

```bash
./target/release/rusticfetch [OPTIONS] <URL>...
```

### Arguments

- **URL**: The URLs of the MP4 files you want to download. You can specify multiple URLs separated by spaces.

### Options

- **-d, --dir <DIRECTORY>**: Specify the directory where the downloaded files will be saved. Defaults to the current directory.
- **-v, --verbose**: Enable verbose logging to get detailed logs of the download process.

## Example

To download two files and save them in the `downloads` directory with verbose logging:

```bash
./target/release/rusticfetch -v -d downloads https://example.com/file1.mp4 https://example.com/file2.mp4
```

## Contributing

We welcome contributions! If you find a bug or have suggestions, please open an issue. If you'd like to contribute code, open a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

Remember to adjust the repository URL and any other specific details to match your project's actual details.