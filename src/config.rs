//! Configuration for webmaster tool.
//!
//! Reads credentials from environment variables, `pass` commands, or config file.

use anyhow::{Context, Result};
use std::path::PathBuf;

/// Config directory: ~/.config/webmaster/
pub fn config_dir() -> PathBuf {
    dirs().join("webmaster")
}

fn dirs() -> PathBuf {
    std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".config")
        })
}

/// Resolve a secret from a shell command (e.g., `pass corky/gmail/client_id`).
pub fn resolve_secret_cmd(cmd: &str) -> Result<String> {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .with_context(|| format!("Failed to run: {}", cmd))?;
    if !output.status.success() {
        anyhow::bail!(
            "Command failed ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}
