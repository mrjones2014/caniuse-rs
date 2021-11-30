#[macro_use]
extern crate lazy_static;

use std::{env, error::Error};
use structopt::StructOpt;

mod alfred_integration;
mod api;
mod browser;
mod constants;
mod feature;
mod opts;
mod skim_finder;
mod url;

fn main() -> Result<(), Box<dyn Error>> {
    let args = opts::Opts::from_args();

    if args.version {
        let version = env!("CARGO_PKG_VERSION");
        println!("caniuse {}", version);
        return Ok(());
    }

    if args.update {
        api::ensure_cached_data(true)?;
        // printing this to stderr so that we don't break JSON parsing
        // if this CLI is being used in a script or a Neovim plugin, for example
        eprintln!("Updated cache written to {}", &*constants::CACHE_PATH);
    }

    let mut features = api::get_data()?;

    if let Some(query) = &args.query {
        let lowercased_query = query.to_lowercase();
        features = features
            .into_iter()
            .filter(|feature| {
                let match_str = feature.string_for_matching().to_lowercase();
                match_str.contains(&lowercased_query) || lowercased_query.contains(&match_str)
            })
            .collect();
    }

    if args.alfred {
        let alfred_items_json = alfred_integration::get_json(&features, &args.pretty)?;
        println!("{}", alfred_items_json);
        return Ok(());
    }

    if args.dump || args.query.is_some() {
        let json = match &args.pretty {
            true => serde_json::to_string_pretty(&features)?,
            false => serde_json::to_string(&features)?,
        };
        println!("{}", json);
        return Ok(());
    }

    skim_finder::find_with_skim(&features);
    Ok(())
}
