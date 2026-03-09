//! Bing Webmaster Tools integration.
//!
//! Uses API key authentication (no OAuth needed).
//! Get your API key at: https://www.bing.com/webmasters/about

use anyhow::{Context, Result};

fn get_api_key() -> Result<String> {
    std::env::var("BING_WEBMASTER_API_KEY")
        .context("BING_WEBMASTER_API_KEY not set. Get your key at https://www.bing.com/webmasters/about")
}

/// Submit a sitemap to Bing.
pub fn submit_sitemap(sitemap_url: &str, _site: Option<&str>) -> Result<()> {
    let api_key = get_api_key()?;

    println!("Submitting sitemap to Bing: {}", sitemap_url);

    let url = format!(
        "https://ssl.bing.com/webmaster/api.svc/json/SubmitUrlbatch?apikey={}",
        api_key
    );

    let body = serde_json::json!({
        "siteUrl": sitemap_url,
        "urlList": [sitemap_url]
    });

    let resp = ureq::post(&url)
        .set("Content-Type", "application/json")
        .send_json(body);

    match resp {
        Ok(_) => {
            println!("Sitemap submitted to Bing successfully!");
            Ok(())
        }
        Err(ureq::Error::Status(status, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            anyhow::bail!("Bing HTTP {}: {}", status, body);
        }
        Err(e) => Err(e.into()),
    }
}
