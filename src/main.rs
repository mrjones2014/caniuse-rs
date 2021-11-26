#[macro_use]
extern crate lazy_static;

use std::{env, error::Error};

mod alfred_integration;
mod api;
mod browser;
mod constants;
mod feature;
mod skim_finder;
mod url;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().map(|arg| arg.to_lowercase()).collect();

    let version = env!("CARGO_PKG_VERSION");
    if args.contains(&String::from("--version")) {
        println!("caniuse v{}", version);
        return Ok(());
    }

    let update_cache = args.contains(&String::from("--update"));
    api::ensure_cached_data(update_cache)?;
    if update_cache {
        println!("Updated cache written to {}", &*constants::CACHE_PATH);
        return Ok(());
    }

    if args.contains(&String::from("--help")) {
        println!("{}", constants::HELP_TEXT);
        return Ok(());
    }

    let features = api::get_data()?;

    if args.contains(&String::from("--query")) {
        if args.len() < 2 {
            panic!("--query must must immediately be followed by a query");
        }
        let query = &args[2];
        println!(
            "{}",
            alfred_integration::get_json(
                &features,
                &query,
                &args.contains(&String::from("--pretty"))
            )?
        );
        return Ok(());
    }

    if args.contains(&String::from("--dump")) {
        let printer = if args.contains(&String::from("--pretty")) {
            serde_json::to_string_pretty
        } else {
            serde_json::to_string
        };
        println!("{}", printer(&features)?);
        return Ok(());
    }

    skim_finder::find_with_skim(&features);
    Ok(())
}
