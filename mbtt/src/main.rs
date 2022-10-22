#[allow(unused)]
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!(
        "https://api.opendata.metlink.org.nz/v1/stop-predictions?stop_id={stop_id}", 
        stop_id = "5510",
    );

    let client = reqwest::Client::new();

    let resp = client.
        get(request_url)
        .header("x-api-key", "gyvdsui0lN1yehMHCsyIN3MejCwkIszh3NOj513P")
        .header("accept", "application/json")        
        .send()
        .await?
        .text()
        // .json::<HashMap<String, String>>()
        .await?;

    // // Remove escape characters from the response
    // let resp = resp.replace("\\", "");

    println!("{:#?}", resp);
    
    Ok(())
}