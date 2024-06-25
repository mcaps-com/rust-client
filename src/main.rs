use reqwest::Error;
use serde_json::Value;

const BASE_URL: &str = "http://www.mcaps.com/api/v0";

async fn get_price(token: &str) -> Result<(), Error> {
    let url = format!("{}/price/pump/{}", BASE_URL, token);

    let client = reqwest::Client::new();
    let response = client.get(&url)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if response.status().is_success() {
        let response_data: Value = response.json().await?;
        println!("{:#?}", response_data);
    } else {
        eprintln!("HTTP error! Status: {}", response.status());
    }

    Ok(())
}

// Example usage
#[tokio::main]
async fn main() {
    match get_price("2dRJvQpBQK4PhVDsRosqsLtxWKKerhrVkukPkCDepump").await {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
