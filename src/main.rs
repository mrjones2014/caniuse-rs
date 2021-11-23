use std::env;
use std::error::Error;

mod api;
mod browser;
mod feature;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = api::get_json_data().await?;
    let _features = feature::json_to_features(data)?;
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 2 || args[1].is_empty() {
        panic!("No query passed as argument.");
    }

    let query = &args[1];
    println!("{}", query);
    Ok(())
}
