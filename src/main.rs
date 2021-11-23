use std::env;
use std::error::Error;

mod api;
mod browser;
mod feature;

fn main() -> Result<(), Box<dyn Error>> {
    api::ensure_cached_data()?;
    let features = api::get_data()?;
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 2 || args[1].is_empty() {
        panic!("No query passed as argument.");
    }

    let query = &args[1];
    let filtered_features = api::fuzzy_find(&query, &features);
    println!("{}", serde_json::to_string(&filtered_features)?);
    Ok(())
}
