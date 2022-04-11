use std::collections::HashMap;

use crate::fetch::Payload;

pub async fn fetch() -> Result<Option<Payload>, Box<dyn std::error::Error>> {
    let response =
        reqwest::get("https://meta.rrr.flashflashrevolution.com/payloads/staging-manifest.json")
            .await?;

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<Payload>().await {
            Ok(parsed) => Ok(Some(parsed)),
            Err(err) => Err(Box::new(err)),
        },
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch() {
        let test_result = fetch().await;
        println!("{:?}", test_result);
        assert!(test_result.is_ok());
    }
}
