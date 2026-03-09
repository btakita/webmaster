use anyhow::Result;
use clap::{Parser, Subcommand};

mod auth;
mod config;
mod engines;
mod skill;

#[derive(Parser)]
#[command(name = "webmaster", about = "Unified CLI for search engine webmaster tools")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with a search engine
    Auth {
        /// Search engine: google, bing, yandex
        engine: String,
    },
    /// Submit a sitemap to one or all search engines
    #[command(name = "submit-sitemap")]
    SubmitSitemap {
        /// Sitemap URL (e.g., https://example.com/sitemap.xml)
        sitemap_url: String,
        /// Site URL or property (e.g., sc-domain:example.com)
        #[arg(short, long)]
        site: Option<String>,
        /// Target engine: google, bing, yandex, all (default: all)
        #[arg(short, long, default_value = "all")]
        engine: String,
    },
    /// List sitemaps for a site
    #[command(name = "list-sitemaps")]
    ListSitemaps {
        /// Site URL or property
        #[arg(short, long)]
        site: Option<String>,
        /// Target engine: google, bing, yandex, all (default: all)
        #[arg(short, long, default_value = "all")]
        engine: String,
    },
    /// Manage the agent skill definition
    Skill {
        #[command(subcommand)]
        command: SkillCommands,
    },
}

#[derive(Subcommand)]
enum SkillCommands {
    /// Install the skill definition to .claude/skills/webmaster/SKILL.md
    Install,
    /// Check if the installed skill matches the binary version
    Check,
    /// Uninstall the skill definition
    Uninstall,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Auth { engine } => {
            engines::auth(&engine)?;
        }
        Commands::SubmitSitemap {
            sitemap_url,
            site,
            engine,
        } => {
            engines::submit_sitemap(&engine, &sitemap_url, site.as_deref())?;
        }
        Commands::ListSitemaps { site, engine } => {
            engines::list_sitemaps(&engine, site.as_deref())?;
        }
        Commands::Skill { command } => match command {
            SkillCommands::Install => skill::install()?,
            SkillCommands::Check => skill::check()?,
            SkillCommands::Uninstall => skill::uninstall()?,
        },
    }
    Ok(())
}
