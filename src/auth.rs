//! OAuth2 authorization code flow for Google APIs.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::config;

const REDIRECT_URI: &str = "http://127.0.0.1:8484/callback";
const CALLBACK_TIMEOUT_SECS: u64 = 300;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    pub expires_at: i64, // unix timestamp
    pub scopes: Vec<String>,
}

impl Token {
    pub fn is_valid(&self) -> bool {
        chrono::Utc::now().timestamp() < self.expires_at - 300
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenStore {
    pub tokens: std::collections::HashMap<String, Token>,
}

fn token_path() -> PathBuf {
    config::config_dir().join("tokens.json")
}

impl TokenStore {
    pub fn load() -> Result<Self> {
        let path = token_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = token_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, &content)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
        }
        Ok(())
    }

    pub fn get_valid(&self, key: &str) -> Option<&Token> {
        self.tokens.get(key).filter(|t| t.is_valid())
    }

    pub fn upsert(&mut self, key: String, token: Token) {
        self.tokens.insert(key, token);
    }
}

/// Percent-encode for URL query parameters.
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

/// Run Google OAuth2 authorization code flow.
pub fn google_auth(client_id: &str, client_secret: &str, scope: &str) -> Result<Token> {
    let state = format!("{:x}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos());

    let url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth\
         ?response_type=code\
         &client_id={}\
         &redirect_uri={}\
         &state={}\
         &scope={}\
         &access_type=offline\
         &prompt=consent",
        urlencode(client_id),
        urlencode(REDIRECT_URI),
        urlencode(&state),
        urlencode(scope),
    );

    println!("Opening browser for authorization...");
    println!("If the browser doesn't open, visit:\n  {}\n", url);
    let _ = open::that(&url);

    println!("Waiting for callback on {}...", REDIRECT_URI);
    let server = tiny_http::Server::http("127.0.0.1:8484")
        .map_err(|e| anyhow::anyhow!("Failed to start callback server: {}", e))?;

    let request = server
        .recv_timeout(std::time::Duration::from_secs(CALLBACK_TIMEOUT_SECS))
        .map_err(|e| anyhow::anyhow!("Callback error: {}", e))?
        .ok_or_else(|| anyhow::anyhow!("Timed out waiting for OAuth callback"))?;

    let url_str = request.url().to_string();
    let query = url_str.split('?').nth(1).unwrap_or("");
    let params: std::collections::HashMap<&str, &str> = query
        .split('&')
        .filter_map(|p| p.split_once('='))
        .collect();

    let response = tiny_http::Response::from_string("Authorization successful! You can close this tab.");
    let _ = request.respond(response);

    let code = params.get("code").ok_or_else(|| anyhow::anyhow!("No code in callback"))?;
    let cb_state = params.get("state").ok_or_else(|| anyhow::anyhow!("No state in callback"))?;

    if *cb_state != state.as_str() {
        bail!("State mismatch (CSRF)");
    }

    println!("Exchanging authorization code...");
    let body = format!(
        "grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&client_secret={}",
        urlencode(code), urlencode(REDIRECT_URI), urlencode(client_id), urlencode(client_secret),
    );

    let resp = ureq::post("https://oauth2.googleapis.com/token")
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&body)
        .context("Token exchange failed")?;

    let json: serde_json::Value = resp.into_json()?;
    parse_google_token(&json, scope)
}

/// Refresh a Google OAuth2 token.
pub fn google_refresh(client_id: &str, client_secret: &str, refresh_token: &str, scope: &str) -> Result<Token> {
    let body = format!(
        "grant_type=refresh_token&refresh_token={}&client_id={}&client_secret={}",
        urlencode(refresh_token), urlencode(client_id), urlencode(client_secret),
    );

    let resp = ureq::post("https://oauth2.googleapis.com/token")
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&body)
        .context("Token refresh failed")?;

    let json: serde_json::Value = resp.into_json()?;
    let mut token = parse_google_token(&json, scope)?;
    token.refresh_token = Some(refresh_token.to_string());
    Ok(token)
}

fn parse_google_token(body: &serde_json::Value, scope: &str) -> Result<Token> {
    let access_token = body["access_token"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing access_token"))?
        .to_string();
    let expires_in = body["expires_in"].as_i64().unwrap_or(3600);
    let refresh_token = body["refresh_token"].as_str().map(|s| s.to_string());
    Ok(Token {
        access_token,
        refresh_token,
        expires_at: chrono::Utc::now().timestamp() + expires_in,
        scopes: vec![scope.to_string()],
    })
}
