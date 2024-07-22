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

The greatest command line utility for data hoarders that has ever been invented.
And yes, it even uses AI.

## Features

- **Full POSIX Compliance :** Supports all the things you would expect from a unix CLI, including wildcards!
- **Directory Mode:** Choose to add files into directories instead of simply renaming them (like YYYY-MM-DD -> YYYY/YYYY-MM-DD)
- **Prefixes, Suffixes, and Replacing:** Add prefixes, suffixes, and replace strings in your files!
- **Images:** Rename images using exif data and the YYYY-MM-DD format!
- **Media (Movies, Shows, Music, Anime):** Organize various types of media using an ollama LLM running locally on your machine, all open source!
- **Parallelization:** Blazingly fast speed achieved by using the 'rayon' crate for parallelization!

## Tech Stack

- [Rust](https://www.rust-lang.org/)
  - [once_cell](https://crates.io/crates/once_cell)
  - [chrono](https://crates.io/crates/chrono)
  - [clap](https://crates.io/crates/clap)
  - [glob](https://crates.io/crates/glob)
  - [infer](https://crates.io/crates/infer)
  - [kamadak-exif](https://crates.io/crates/kamadak-exif)
  - [regex](https://crates.io/crates/regex)
  - [walkdir](https://crates.io/crates/walkdir)
  - [anyhow](https://crates.io/crates/anyhow)
  - [rayon](https://crates.io/crates/rayon)

## Getting Started

### Prerequisites

Here's what you need to be able to run hoarder:

- Unix-based system (Windows support coming one day)
- Rust compiler
- openssl
- libexif
- ollama

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
