# Webmaster

Unified CLI for search engine webmaster tools — sitemap submission, search analytics, URL inspection across Google, Bing, and Yandex.

## Architecture

- `src/main.rs` — CLI entrypoint (clap)
- `src/auth.rs` — Google OAuth2 flow (reuses corky's GCP credentials)
- `src/config.rs` — Credential resolution (pass, env vars)
- `src/engines/` — Per-engine modules
  - `google.rs` — Google Search Console API v1
  - `bing.rs` — Bing Webmaster Tools API (API key auth)

## Credentials

- **Google:** Reuses corky's OAuth client (`pass corky/gmail/client_id`). Tokens stored separately at `~/.config/webmaster/tokens.json`.
- **Bing:** API key via `BING_WEBMASTER_API_KEY` env var.
- **Yandex:** Not yet implemented.

## Usage

```bash
webmaster auth google                    # OAuth2 flow
webmaster submit-sitemap URL             # Submit to all engines
webmaster submit-sitemap URL -e google   # Google only
webmaster list-sitemaps                  # List from Google
```

## Conventions

- Use `make check` (clippy + test) before committing
- Use `anyhow` for errors
- Keep engine modules independent
- Tokens stored with 0600 permissions
