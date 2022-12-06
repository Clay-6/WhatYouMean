# WhatYouMean

**Currently not ready for use, but will replace the main branch once the switch to Wordnik is complete**

A CLI dictionary that uses [WordsAPI](https://www.wordsapi.com) for its info

## Usage

Run the command

```shell
wym <word>
```

in your preferred terminal, where `<word>` is the word you want to define. Use `wym --help`
for more detailed usage instructions

## Installation

### Universal Setup

Get an [API key](https://rapidapi.com/dpventures/api/wordsapi/pricing) & save it in
an environment variable called `WORDSAPI_KEY`

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
