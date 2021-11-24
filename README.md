# caniuse-rs

**This project is a WIP and should not be considered stable.**

A Rust client for [caniuse.com](https://caniuse.com). It pulls data from caniuse.com and caches it locally,
in a transformed JSON structure that is easier to reason about. It will update data on next run after 24 hours
since last update.

## Options

To dump the data instead of fuzzy finding, you can run `caniuse --dump` which will simply output the
currently cached JSON data that is being used. To pretty-print it, you can run `caniuse --dump --pretty`.
