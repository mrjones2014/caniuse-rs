#[macro_use]
extern crate lazy_static;

use std::error::Error;

mod api;
mod browser;
mod constants;
mod feature;
mod skim_finder;
mod url;

fn main() -> Result<(), Box<dyn Error>> {
    let features = api::get_data()?;
    skim_finder::find_with_skim(&features);
    Ok(())
}
