use std::error::Error;

mod api;
mod browser;
mod feature;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = api::get_json_data().await?;
    let features = feature::json_to_features(data)?;
    println!("{}", serde_json::to_string(&features)?);
    Ok(())
}
