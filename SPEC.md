# webmaster — Specification

## 1. Overview

`webmaster` is a unified CLI for managing search engine webmaster tools. It provides a single interface for sitemap submission, search analytics, and URL inspection across multiple search engines.

## 2. Supported Engines

### 2.1 Google Search Console

- **Auth:** OAuth2 authorization code flow (localhost callback on port 8484)
- **API:** Google Search Console API v1 (`searchconsole.googleapis.com`)
- **Credentials:** Resolved from `pass corky/gmail/client_id` or `GOOGLE_CLIENT_ID` env var
- **Token storage:** `~/.config/webmaster/tokens.json` with key `google:webmaster`
- **Scopes:** `https://www.googleapis.com/auth/webmasters`

#### 2.1.1 Sitemaps

- `submit-sitemap <url>` — PUT to `/webmasters/v3/sites/{site}/sitemaps/{feedpath}`
- `list-sitemaps` — GET from `/webmasters/v3/sites/{site}/sitemaps`
- Default site: `sc-domain:esg.briantakita.me` (overridable via `--site`)

### 2.2 Bing Webmaster Tools

- **Auth:** API key (`BING_WEBMASTER_API_KEY` env var)
- **API:** Bing URL Submission API (`ssl.bing.com/webmaster/api.svc`)

#### 2.2.1 Sitemaps

- `submit-sitemap <url>` — POST to SubmitUrlbatch endpoint

### 2.3 Yandex Webmaster (Planned)

- **Auth:** OAuth2
- **API:** Yandex Webmaster API v4
- **Crate:** `yandex-webmaster-api`

## 3. CLI Interface

```
webmaster <command> [options]

Commands:
  auth <engine>           Authenticate with a search engine
  submit-sitemap <url>    Submit a sitemap
  list-sitemaps           List submitted sitemaps

Options:
  -e, --engine <engine>   Target engine: google, bing, yandex, all (default: all)
  -s, --site <site>       Site URL or property identifier
```

## 4. Token Management

- Tokens stored in `~/.config/webmaster/tokens.json`
- File permissions: 0600 (owner read/write only)
- Automatic token refresh via refresh_token grant
- 5-minute grace window before expiry triggers refresh

## 5. Credential Resolution

Order of precedence:
1. `pass` command (e.g., `pass corky/gmail/client_id`)
2. Environment variables (`GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`)

## 6. Future Features

- Search analytics queries (impressions, clicks, CTR, position)
- URL inspection (indexing status, crawl info)
- Yandex engine support
- Site management (add/remove/list verified sites)
- Batch operations across multiple sites
- agent-doc dashboard integration
