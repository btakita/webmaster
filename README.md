# webmaster

Unified CLI for search engine webmaster tools — sitemap submission, search analytics, and URL inspection across Google, Bing, and Yandex.

## Install

```bash
cargo install --path .
```

## Quick Start

```bash
# Authenticate with Google Search Console
webmaster auth google

# Submit a sitemap to all engines
webmaster submit-sitemap https://example.com/sitemap.xml

# Submit to Google only
webmaster submit-sitemap https://example.com/sitemap.xml -e google

# List sitemaps
webmaster list-sitemaps
```

## Engine Support

| Engine | Auth Method | Sitemap Submit | Search Analytics | Status |
|--------|-------------|----------------|------------------|--------|
| Google | OAuth2 | Yes | Planned | Alpha |
| Bing | API Key | Yes | Planned | Alpha |
| Yandex | OAuth | Planned | Planned | Stub |

## Configuration

### Google

Reuses OAuth2 credentials from your Google Cloud Console project. Credentials resolved in order:

1. `pass corky/gmail/client_id` / `pass corky/gmail/client_secret`
2. `GOOGLE_CLIENT_ID` / `GOOGLE_CLIENT_SECRET` env vars

Tokens stored at `~/.config/webmaster/tokens.json` (0600 permissions).

**Prerequisites:**
- Enable "Google Search Console API" in your GCP project
- OAuth2 consent screen configured

### Bing

Set `BING_WEBMASTER_API_KEY` environment variable. Get your key at https://www.bing.com/webmasters/about.

## License

MIT OR Apache-2.0
