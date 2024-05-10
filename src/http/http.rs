use reqwest::Client;
use anyhow::{Error, Result, anyhow};
use crate::tiktok::constants::BLACKLIST_RESPONSE;


pub async fn make_request(url: &str, client: Client ) -> Result<String, Error> {
    // Abstraction adds some headroom for evasion measures...
    let response = client.get(url)
        .send()
        .await?;
    let response_text = response.text().await?;
    
    // Check for bad signatures
    // TODO: Change this... 10221 is an error code for an/a invalid/deleted account
    for blackword in BLACKLIST_RESPONSE.iter() {
        if response_text.contains(blackword) {
            println!("{}", url);
            return Err(anyhow!("Blacklisted word detected"))
        }
    }

    Ok(response_text)
}