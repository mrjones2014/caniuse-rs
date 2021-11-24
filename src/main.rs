#[macro_use]
extern crate lazy_static;

use std::{env, error::Error};

mod api;
mod browser;
mod constants;
mod feature;
mod skim_finder;
mod url;

fn main() -> Result<(), Box<dyn Error>> {
    let features = api::get_data()?;
    let args: Vec<String> = env::args().map(|arg| arg.to_lowercase()).collect();
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
