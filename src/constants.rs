use std::env;

pub const HELP_TEXT: &str = r#"
 caniuse [options]

 OPTIONS:

 --dump           Dump the currently cached JSON data and exit
 --pretty         Pretty-print the JSON output, must be combined with --dump or --query [query]
 --query [query]  Output JSON instead of using fuzzy-finder, used for Alfred integration
 --update         Force update the currently cached JSON data and exit
 --version        Print the version and exit
 --help           Show this help text
"#;

lazy_static! {
    pub static ref CACHE_PATH: String = match env::var("HOME") {
        Ok(home) => format!("{}/.cache/caniuse-rs/data.json", home),
        Err(_) => String::from("/usr/local/share/caniuse-rs/data.json"),
    };
}
