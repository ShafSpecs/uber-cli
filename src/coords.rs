use reqwest::Client;
use serde_json::{Value, from_str};

#[tokio::main]
pub async fn get_coords(address: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let geocoding_url = format!(
        "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
        address, "..."
    );

    let client = Client::new();

    let data = client.get(geocoding_url.as_str()).send().await?.text().await?;

    // the &var work just like `as_str` function (`String` to `&str`)
    let longlat: Value = from_str(&data)?;

    let lat = longlat["results"][0]["geometry"]["location"]["lat"].to_string();
    let lng = longlat["results"][0]["geometry"]["location"]["lng"].to_string();

    Ok(vec![lat, lng])
}
