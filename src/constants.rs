use std::env;

lazy_static! {
    pub static ref CACHE_PATH: String = match env::var("HOME") {
        Ok(home) => format!("{}/.cache/caniuse-rs/data.json", home),
        Err(_) => String::from("/usr/local/share/caniuse-rs/data.json"),
    };
}
