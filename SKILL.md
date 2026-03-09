# webmaster

Unified CLI for search engine webmaster tools — sitemap submission, search analytics, URL inspection across Google, Bing, and Yandex.

## Commands

```bash
webmaster auth <engine>                          # Authenticate (google, bing)
webmaster submit-sitemap <URL>                   # Submit to all engines
webmaster submit-sitemap <URL> -e google         # Submit to specific engine
webmaster submit-sitemap <URL> -s <site>         # Specify site property
webmaster list-sitemaps                          # List from all engines
webmaster list-sitemaps -e google                # List from specific engine
webmaster skill install                          # Install/update this skill
webmaster skill check                            # Check skill version
```

## Authentication

- **Google:** OAuth2 via `webmaster auth google`. Reuses GCP client credentials (`pass corky/gmail/client_id`). Tokens stored at `~/.config/webmaster/tokens.json`.
- **Bing:** API key via `BING_WEBMASTER_API_KEY` environment variable.
- **Yandex:** Not yet implemented.

## Site Property Format

- Google uses `sc-domain:example.com` format for domain properties
- Bing uses `https://example.com` format
- When `--site` is omitted, the engine may auto-detect from the sitemap URL

## Common Workflows

1. **Submit sitemaps for a site:**
   ```bash
   webmaster submit-sitemap https://example.com/sitemap.xml
   ```

2. **Check submitted sitemaps:**
   ```bash
   webmaster list-sitemaps -s sc-domain:example.com
   ```

## Agent Integration

This skill document is compatible with any AI agent or coding assistant:
- **Claude Code** — installed to `.claude/skills/webmaster/SKILL.md`
- **Codex / OpenCode / Pi / Grok** — read as plain markdown; all commands are standard CLI
- **MCP-aware agents** — commands can be exposed as tool descriptions

Install via: `webmaster skill install`
