mod api;
mod browser;
mod feature;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let data = api::get_json_data().await?;
    println!("{}", data);
    Ok(())
}
