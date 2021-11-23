pub async fn get_json_data() -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://github.com/Fyrd/caniuse/raw/main/data.json")
        .await?
        .text()
        .await?;
    Ok(body)
}
