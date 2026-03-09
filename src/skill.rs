//! `webmaster skill` — Manage the Claude Code skill definition.
//!
//! Delegates to `agent_kit::skill::SkillConfig` for the actual install/check logic.

use anyhow::Result;
use agent_kit::skill::SkillConfig;

const BUNDLED_SKILL: &str = include_str!("../SKILL.md");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn config() -> SkillConfig {
    SkillConfig::new("webmaster", BUNDLED_SKILL, VERSION)
}

pub fn install() -> Result<()> {
    config().install(None)
}

pub fn uninstall() -> Result<()> {
    config().uninstall(None)
}

pub fn check() -> Result<()> {
    let up_to_date = config().check(None)?;
    if !up_to_date {
        std::process::exit(1);
    }
    Ok(())
}
