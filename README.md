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
git clone https://github.com/anupsv/rusticfetch.git
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

Certainly! Let's provide examples for each option and combination of options to give users a clear understanding of how to use RusticFetch.

---

## Examples

### Basic Download

To download a single file:

```bash
./target/release/rusticfetch https://example.com/file1.mp4
```

### Download Multiple Files

To download multiple files:

```bash
./target/release/rusticfetch https://example.com/file1.mp4 https://example.com/file2.mp4
```

### Specify Download Directory

To download files into a specific directory, such as `downloads`:

```bash
./target/release/rusticfetch -d downloads https://example.com/file1.mp4
```

### Download from a File

To download URLs listed in a file, `urls.txt`:

```bash
./target/release/rusticfetch -f urls.txt
```

### Download from a Curl-formatted File

If `urls.txt` contains curl commands:

```bash
./target/release/rusticfetch -f urls.txt --curl-format
```

### Specify Number of Threads

To download using a specific number of threads, for instance, 8 threads:

```bash
./target/release/rusticfetch -t 8 https://example.com/file1.mp4
```

### Verbose Logging

To enable verbose logging:

```bash
./target/release/rusticfetch -v https://example.com/file1.mp4
```

### Combining Options

To combine multiple options, such as downloading from a curl-formatted file with verbose logging, 8 threads, and saving to the `downloads` directory:

```bash
./target/release/rusticfetch -f urls.txt --curl-format -v -t 8 -d downloads
```

---

## Contributing

We welcome contributions! If you find a bug or have suggestions, please open an issue. If you'd like to contribute code, open a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---