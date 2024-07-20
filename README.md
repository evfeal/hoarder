<div align="center">
  <h1 align="center">Hoarder</h1>
  <h3>The all-in-one renaming tool for people with too many files</h3>
</div>

<br/>

<div align="center">
  <a href="https://github.com/mfts/papermark/stargazers"><img alt="STAR MY REPO" src="https://img.shields.io/github/stars/mfts/papermark"></a>
  <a href="https://github.com/mfts/papermark/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/badge/license-GPLv3-purple"></a>
</div>

<br/>

The greatest command line utility for data hoarders that has ever been invented

## Features

- **Full POSIX Compliance :** Supports all the things you would expect from a unix CLI, including wildcards!
- **Prefixes, Suffixes, and Replacing:** Add prefixes, suffixes, and replace strings in your files!
- **(WIP) Indexes:** Specify an index (starting from x) i.e. file1, file2, etc.
- **(WIP) Pre-made naming conventions:** Choose from pre-existing naming conventions for your files!

## Tech Stack

- [Rust](https://www.rust-lang.org/)
  - [clap](https://crates.io/crates/clap)
  - [glob](https://crates.io/crates/glob)

## Getting Started

### Prerequisites

Here's what you need to be able to run Papermark:

- Rust compiler
- Unix-based system (Windows support coming one day)

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
