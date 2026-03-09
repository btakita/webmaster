//! Google Search Console integration.

use anyhow::{Context, Result};

use crate::auth::{self, TokenStore};
use crate::config;

const SCOPE: &str = "https://www.googleapis.com/auth/webmasters";
const TOKEN_KEY: &str = "google:webmaster";

fn get_credentials() -> Result<(String, String)> {
    // Try pass commands first (corky-compatible), then env vars
    let client_id = config::resolve_secret_cmd("pass corky/gmail/client_id")
        .or_else(|_| std::env::var("GOOGLE_CLIENT_ID"))
        .context("Google client_id not found. Set via `pass corky/gmail/client_id` or GOOGLE_CLIENT_ID env var.")?;
    let client_secret = config::resolve_secret_cmd("pass corky/gmail/client_secret")
        .or_else(|_| std::env::var("GOOGLE_CLIENT_SECRET"))
        .context("Google client_secret not found. Set via `pass corky/gmail/client_secret` or GOOGLE_CLIENT_SECRET env var.")?;
    Ok((client_id, client_secret))
}

fn get_access_token() -> Result<String> {
    let mut store = TokenStore::load()?;

    // Check for valid cached token
    if let Some(token) = store.get_valid(TOKEN_KEY) {
        return Ok(token.access_token.clone());
    }

    // Try refresh
    if let Some(token) = store.tokens.get(TOKEN_KEY).cloned() {
        if let Some(ref refresh) = token.refresh_token {
            let (client_id, client_secret) = get_credentials()?;
            match auth::google_refresh(&client_id, &client_secret, refresh, SCOPE) {
                Ok(new_token) => {
                    let access = new_token.access_token.clone();
                    store.upsert(TOKEN_KEY.to_string(), new_token);
                    store.save()?;
                    return Ok(access);
                }
                Err(e) => eprintln!("Token refresh failed: {}. Re-authenticating...", e),
            }
        }
    }

    // Full auth flow
    let (client_id, client_secret) = get_credentials()?;
    let token = auth::google_auth(&client_id, &client_secret, SCOPE)?;
    let access = token.access_token.clone();
    store.upsert(TOKEN_KEY.to_string(), token);
    store.save()?;
    Ok(access)
}

/// Run Google OAuth2 authentication.
pub fn auth() -> Result<()> {
    let (client_id, client_secret) = get_credentials()?;
    let token = auth::google_auth(&client_id, &client_secret, SCOPE)?;
    let mut store = TokenStore::load()?;
    store.upsert(TOKEN_KEY.to_string(), token);
    store.save()?;
    println!("Google Search Console token stored.");
    Ok(())
}

fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

/// Submit a sitemap to Google Search Console.
pub fn submit_sitemap(sitemap_url: &str, site: Option<&str>) -> Result<()> {
    let site = site.unwrap_or("sc-domain:esg.briantakita.me");
    let token = get_access_token()?;

    let url = format!(
        "https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps/{}",
        urlencode(site),
        urlencode(sitemap_url),
    );

    println!("Submitting sitemap: {}", sitemap_url);
    println!("Site: {}", site);

    let resp = ureq::put(&url)
        .set("Authorization", &format!("Bearer {}", token))
        .set("Content-Length", "0")
        .call();

    match resp {
        Ok(_) => {
            println!("Sitemap submitted successfully!");
            Ok(())
        }
        Err(ureq::Error::Status(status, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            anyhow::bail!("HTTP {}: {}", status, body);
        }
        Err(e) => Err(e.into()),
    }
}

/// List sitemaps for a site.
pub fn list_sitemaps(site: Option<&str>) -> Result<()> {
    let site = site.unwrap_or("sc-domain:esg.briantakita.me");
    let token = get_access_token()?;

    let url = format!(
        "https://www.googleapis.com/webmasters/v3/sites/{}/sitemaps",
        urlencode(site),
    );

    let resp = ureq::get(&url)
        .set("Authorization", &format!("Bearer {}", token))
        .call()
        .context("Failed to list sitemaps")?;

    let body: serde_json::Value = resp.into_json()?;
    println!("{}", serde_json::to_string_pretty(&body)?);
    Ok(())
}
