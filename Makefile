.PHONY: all macos linux windows clean

all: macos linux windows

macos:
	cross build --release --target=aarch64-apple-darwin
	cp target/aarch64-apple-darwin/release/RusticFetch dist/RusticFetch_macos

linux:
	cross build --release --target=x86_64-unknown-linux-gnu
	cp target/x86_64-unknown-linux-gnu/release/RusticFetch dist/RusticFetch_linux

windows:
	cross build --release --target=x86_64-pc-windows-gnu
	cp target/x86_64-pc-windows-gnu/release/RusticFetch.exe dist/RusticFetch_windows.exe

clean:
	rm -rf dist/*
