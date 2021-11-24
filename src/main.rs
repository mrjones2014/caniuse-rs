#[macro_use]
extern crate lazy_static;

use std::{env, error::Error};

use alfred_integration::{AlfredItem, AlfredItemList};

mod alfred_integration;
mod api;
mod browser;
mod constants;
mod feature;
mod skim_finder;
mod url;

fn main() -> Result<(), Box<dyn Error>> {
    let features = api::get_data()?;
    let args: Vec<String> = env::args().map(|arg| arg.to_lowercase()).collect();

    if args.contains(&String::from("--alfred")) {
        if args.len() < 2 {
            panic!("--alfred must be the only option and must immediately be followed by a query");
        }
        let query = args[2..].join(" ").to_lowercase();
        let alfred_items = AlfredItemList {
            items: features
                .iter()
                .filter(|feature| {
                    let match_str = feature.string_for_matching().to_lowercase();
                    match_str.contains(&query) || query.contains(&match_str)
                })
                .map(|feature| AlfredItem::from(feature.to_owned()))
                .collect(),
        };
        println!("{}", serde_json::to_string(&alfred_items)?);
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
