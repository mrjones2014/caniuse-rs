# caniuse-rs

A Rust client for [caniuse.com](https://caniuse.com).

![demo](https://github.com/mrjones2014/caniuse-rs/raw/master/images/demo.gif)

It pulls data from caniuse.com and caches it locally, in a transformed JSON structure
that is easier to reason about. It will update data on next run after 24 hours since
last update. Fuzzy finder is built using [skim](https://github.com/lotabout/skim).

## Options

To dump the data instead of fuzzy finding, you can run `caniuse --dump` which will simply output the
currently cached JSON data that is being used. To pretty-print it, you can run `caniuse --dump --pretty`.

To force update the cached data, you can run `caniuse --update`.
