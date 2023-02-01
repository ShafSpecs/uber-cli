use std::error::Error;

#[tokio::main]
pub async fn view_drivers() -> Result<(), Box<dyn Error>> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );

    let uber_url = "";

    // A new client instance for making requests
    let client = reqwest::Client::new();

    let data = client
        .get(request_url)
        .header("User-Agent", "ShafSpecs")
        .send()
        .await?
        .text()
        .await?;
    println!("{:}", data);

    Ok(())
}
