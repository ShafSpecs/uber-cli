use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use std::error::Error;

#[tokio::main]
pub async fn get_ride_eta(longlat: Vec<String>) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request_url = format!(
        "https://sandbox-api.uber.com/v1.2/estimates/time?start_latitude={}&start_longitude={}",
        longlat[0], longlat[1]
    );

    let mut request_headers = HeaderMap::new();
    request_headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );
    request_headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str("en_US").unwrap());
    request_headers.insert(AUTHORIZATION, HeaderValue::from_str("Bearer ...").unwrap());

    let data = client
        .get(request_url.as_str())
        .headers(request_headers)
        .send()
        .await?
        .text()
        .await?;

    println!("{:#?}", data);

    Ok(())
}

pub fn get_ride_price_estimates() {}
