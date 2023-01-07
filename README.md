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
`WORDNIK_API_KEY`. You can, however, use a builtin API key which is rate limited to 100 requests per hour
for all users.

### Through Cargo

First, ensure you have the [latest stable rust version](https://www.rust-lang.org/tools/install) installed.
Then run

```shell
cargo install whatyoumean
```

and you're all set!

### Manual

Download the [latest release from GitHub](https://github.com/Clay-6/WhatYouMean) & move the
executable to a directory on your `PATH`. Currently, only Windows binaries are distributed.

### Windows Installer

Download & run the installer from the [latest GitHub release](https://github.com/Clay-6/WhatYouMean).
All necessary changes to your `PATH` will be made by the installer.
