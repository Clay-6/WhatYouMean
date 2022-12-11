# WhatYouMean

**Currently not ready for use, but will replace the main branch once the switch to Wordnik is complete**

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

While a default API key can be used, it's rate limited to 100 requests per hour accross all users. To use
your own API key, get one [here](https://developer.wordnik.com/) and store it in an environment variable named
`WORDNIK_API_KEY`

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
