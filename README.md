<div align="center">
  <h1 align="center">Hoarder</h1>
  <h3>The all-in-one renaming tool for people with too many files</h3>
</div>

<br/>

<div align="center">
  <a href="https://github.com/evfeal/hoarder/stargazers"><img alt="STAR MY REPO" src="https://img.shields.io/github/stars/evfeal/hoarder"></a>
  <a href="https://github.com/evfeal/hoarder/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/badge/license-GPLv3-purple"></a>
</div>

<br/>

The greatest command line utility for data hoarders that has ever been invented

## Features

- **Full POSIX Compliance :** Supports all the things you would expect from a unix CLI, including wildcards!
- **Prefixes, Suffixes, and Replacing:** Add prefixes, suffixes, and replace strings in your files!
- **Images:** Rename images using exif data and the IMG_YYYYMMDD format!
- **(WIP) Movies and TV Shows:** im working on it alright, its gonna be great trust me.

## Tech Stack

- [Rust](https://www.rust-lang.org/)
  - [lazy_static](https://crates.io/crates/lazy_static)
  - [clap](https://crates.io/crates/clap)
  - [glob](https://crates.io/crates/glob)
  - [infer](https://crates.io/crates/infer)
  - [kamadak-exif](https://crates.io/crates/kamadak-exif)
  - [regex](https://crates.io/crates/regex)

## Getting Started

### Prerequisites

Here's what you need to be able to run hoarder:

- Rust compiler
- Unix-based system (Windows support coming one day)
- libexif

### 1. Clone the repository

```shell
git clone https://github.com/evfeal/hoarder.git
cd hoarder
```

### 2. Compile the program
```shell
cargo build --release
```

### 3. Run the program

```shell
./target/release/hoarder
```

### Contributors

<a href="https://github.com/mfts/papermark/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=evfeal/hoarder" />
</a>
