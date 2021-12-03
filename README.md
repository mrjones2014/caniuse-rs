# caniuse-rs

A Rust client for [caniuse.com](https://caniuse.com).

![demo](https://github.com/mrjones2014/caniuse-rs/raw/master/images/demo.gif)

It pulls data from caniuse.com and caches it locally, in a transformed JSON structure
that is easier to reason about. It will update data on next run after 24 hours since
last update. Fuzzy finder is built using [skim](https://github.com/lotabout/skim).

## Install

### Via Cargo

If you have a Rust toolchain installed, you can install by running `cargo install caniuse-rs`.
The installed binary is named `caniuse`.

### Pre-built Binaries

Pre-bulit binaries are available from the [latest GitHub release](https://github.com/mrjones2014/caniuse-rs/releases).
Download the binary for your platform, rename it to `caniuse`, and put it somewhere on your `$PATH`. Then, you will
need to make it executable by running `chmod +x path/to/caniuse`. When installing this way on MacOS, after attempting
to run for the first time, you will need to open MacOS System Preferences -> Security & Privacy -> General tab, then
click "Allow Anyway" to allow the `caniuse` executable to run.

### Alfred Workflow

You can use this as an [Alfred](https://www.alfredapp.com) workflow on MacOS by downloading the
`*.alfredworkflow` file for your platform (`caniuse-macos-x86.alfredworkflow` for Intel Macs,
`caniuse-macos-arm.alfredworkflow` for M1 macs) from the [latest GitHub release](https://github.com/mrjones2014/caniuse-rs/releases)
and double-clicking the file from Finder once downloaded. After attempting to run
for the first time, you will need to open MacOS System Preferences -> Security & Privacy -> General tab,
then click "Allow Anyway" to allow the `caniuse` executable to run.

## Usage

Run `caniuse` by itself to open the fuzzy finder, then enter a search query to fuzzy find
what you're looking for. Pressing enter will open the selected item. If no items match,
pressing enter will search for the query you've typed by opening `https://caniuse.com/?search={query}`.

## Options

```
USAGE:
    caniuse [FLAGS] [OPTIONS]

FLAGS:
        --alfred     Transform JSON structure for use in Alfred workflow. See:
                     https://www.alfredapp.com/help/workflows/inputs/script-filter/json/
    -d, --dump       Print currently cached data and exit instead of using fuzzy-finder
    -h, --help       Prints help information
    -p, --pretty     Pretty-print JSON output, must be combined with --dump or --query option
    -u, --update     Force-update cached data before querying
    -v, --version    Print the version and exit

OPTIONS:
    -q, --query <query>    Run a query and output the JSON results instead of using the fuzzy-finder
```
