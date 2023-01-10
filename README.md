# WhatYouMean

A CLI dictionary that uses [Wordnik](https://www.wordnik.com) for its info

## Usage

Run the command

```shell
wym <word>
```

in your preferred terminal, where `<word>` is the word you want to define. Use `wym --help`
for more detailed usage instructions

## Installation

### Universal Setup

Get a Wordnik API key [here](https://developer.wordnik.com/) and store it in an environment variable named
`WORDNIK_API_KEY`. Then, instal the program from either [`cargo`](#through-cargo) or a [prebuilt binary](#prebuilt-binary)

### Through Cargo

First, ensure you have the [latest stable rust version](https://www.rust-lang.org/tools/install) installed.
Then run

```shell
cargo install whatyoumean
```

and you're all set!

### Prebuilt Binary

Download the [latest release from GitHub](https://github.com/Clay-6/WhatYouMean/releases/latest) & exctract the
executable to a directory on your `PATH`.

## Building From Source

Ensure you have the [Rust toolchain](https://rustup.rs) installed.

`git clone` the repository, then run

```shell
cargo build 
```

for a debug build, or

```shell
cargo build --release
```

for a release build
