use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_binary_exists() {
    Command::cargo_bin("webmaster").unwrap();
}

#[test]
fn test_cli_help() {
    Command::cargo_bin("webmaster")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Unified CLI for search engine webmaster tools"));
}

#[test]
fn test_cli_no_args_shows_error() {
    Command::cargo_bin("webmaster")
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn test_cli_skill_install_help() {
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["skill", "install", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Install the skill definition"));
}

#[test]
fn test_cli_skill_uninstall_help() {
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["skill", "uninstall", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Uninstall the skill definition"));
}

#[test]
fn test_cli_skill_uninstall_not_installed() {
    let dir = tempfile::tempdir().unwrap();
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["skill", "uninstall"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn test_cli_skill_uninstall_after_install() {
    let dir = tempfile::tempdir().unwrap();

    // Install first
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["skill", "install"])
        .current_dir(dir.path())
        .assert()
        .success();

    let skill_path = dir.path().join(".claude/skills/webmaster/SKILL.md");
    assert!(skill_path.exists());

    // Uninstall
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["skill", "uninstall"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stderr(predicate::str::contains("Uninstalled skill"));

    assert!(!skill_path.exists());
}

#[test]
fn test_cli_skill_check_help() {
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["skill", "check", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Check if the installed skill matches"));
}

#[test]
fn test_cli_skill_install_creates_file() {
    let dir = tempfile::tempdir().unwrap();
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["skill", "install"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stderr(predicate::str::contains("Installed skill"));

    let skill_path = dir.path().join(".claude/skills/webmaster/SKILL.md");
    assert!(skill_path.exists());
    let content = std::fs::read_to_string(&skill_path).unwrap();
    assert!(content.contains("webmaster"));
    assert!(content.contains("submit-sitemap"));
    assert!(content.contains("Agent Integration"));
}

#[test]
fn test_cli_skill_install_idempotent() {
    let dir = tempfile::tempdir().unwrap();

    // First install
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["skill", "install"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stderr(predicate::str::contains("Installed skill"));

    // Second install — already up to date
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["skill", "install"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stderr(predicate::str::contains("already up to date"));
}

#[test]
fn test_cli_skill_check_not_installed() {
    let dir = tempfile::tempdir().unwrap();
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["skill", "check"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Not installed"));
}

#[test]
fn test_cli_skill_check_after_install() {
    let dir = tempfile::tempdir().unwrap();

    // Install first
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["skill", "install"])
        .current_dir(dir.path())
        .assert()
        .success();

    // Check should pass
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["skill", "check"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stderr(predicate::str::contains("Up to date"));
}

#[test]
fn test_cli_skill_check_outdated() {
    let dir = tempfile::tempdir().unwrap();
    let skill_path = dir.path().join(".claude/skills/webmaster/SKILL.md");
    std::fs::create_dir_all(skill_path.parent().unwrap()).unwrap();
    std::fs::write(&skill_path, "old content").unwrap();

    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["skill", "check"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Outdated"));
}

#[test]
fn test_cli_submit_sitemap_help() {
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["submit-sitemap", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Submit a sitemap"));
}

#[test]
fn test_cli_list_sitemaps_help() {
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["list-sitemaps", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("List sitemaps"));
}

#[test]
fn test_cli_auth_help() {
    Command::cargo_bin("webmaster")
        .unwrap()
        .args(["auth", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Authenticate"));
}
