pub mod google;
pub mod bing;

use anyhow::{bail, Result};

/// Authenticate with a search engine.
pub fn auth(engine: &str) -> Result<()> {
    match engine {
        "google" => google::auth(),
        "bing" => {
            println!("Bing uses API key authentication — no OAuth flow needed.");
            println!("Set BING_WEBMASTER_API_KEY or pass it via --api-key.");
            println!("Get your key at: https://www.bing.com/webmasters/about");
            Ok(())
        }
        "yandex" => {
            println!("Yandex OAuth not yet implemented. Use yandex-webmaster-api crate directly.");
            Ok(())
        }
        _ => bail!("Unknown engine: {}. Supported: google, bing, yandex", engine),
    }
}

/// Submit a sitemap.
pub fn submit_sitemap(engine: &str, sitemap_url: &str, site: Option<&str>) -> Result<()> {
    match engine {
        "google" => google::submit_sitemap(sitemap_url, site),
        "bing" => bing::submit_sitemap(sitemap_url, site),
        "all" => {
            println!("=== Google ===");
            if let Err(e) = google::submit_sitemap(sitemap_url, site) {
                eprintln!("Google: {}", e);
            }
            println!("\n=== Bing ===");
            if let Err(e) = bing::submit_sitemap(sitemap_url, site) {
                eprintln!("Bing: {}", e);
            }
            Ok(())
        }
        _ => bail!("Unknown engine: {}. Supported: google, bing, all", engine),
    }
}

/// List sitemaps.
pub fn list_sitemaps(engine: &str, site: Option<&str>) -> Result<()> {
    match engine {
        "google" => google::list_sitemaps(site),
        "bing" => {
            println!("Bing sitemap listing not yet implemented.");
            Ok(())
        }
        "all" => {
            println!("=== Google ===");
            if let Err(e) = google::list_sitemaps(site) {
                eprintln!("Google: {}", e);
            }
            Ok(())
        }
        _ => bail!("Unknown engine: {}", engine),
    }
}
