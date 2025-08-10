use models::BrewInfo;
use semver::Version;
use std::process::Command;

pub mod models;
pub mod reminder;

use reminder::UpdateReminder;

use crate::{constants::REPOSITORY, verbose::print_verbose};
use gim_config::config::update_config_value;
use toml::Value;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Checks whether an update reminder should be shown to the user and prints a message if a new version is available.
///
/// Loads the update reminder configuration, determines if a reminder should be shown, checks for a new version,
/// and prints a notification if an update is available. Also updates the reminder count as needed.
pub fn check_update_reminder() -> Result<(), Box<dyn std::error::Error>> {
    let mut reminder = UpdateReminder::load();
    print_verbose(&format!("Checking new version on config: {}", reminder));

    let to_reminder = reminder.should_show_reminder();
    print_verbose(&format!(
        "Should reminder update according to config: {}",
        to_reminder
    ));
    if to_reminder {
        let new_pop = new_version_available()?.0;
        print_verbose(&format!("Is a new version published remotely: {}", new_pop));
        if new_pop {
            println!("ℹ️  A new version is available. Run 'gim update' to check for updates.");

            // Increment the reminder count or reset if needed
            if let Err(e) = reminder.increment_reminder_count() {
                eprintln!("Warning: Failed to update reminder status: {}", e);
            }
        }
    }
    print_verbose("End checking new version");
    Ok(())
}

fn new_version_available() -> Result<(bool, Version, Version), Box<dyn std::error::Error>> {
    let current_version = VERSION;
    let current = semver::Version::parse(current_version)
        .map_err(|_| format!("Invalid current version format: {}", current_version))?;
    let latest = get_latest_version_by_homebrew()?;
    print_verbose(&format!(
        "Local version: {}; Remote Version: {}",
        current, latest
    ));
    Ok((latest > current, current, latest))
}

/// Gets the latest version from Homebrew
fn get_latest_version_by_homebrew() -> Result<Version, Box<dyn std::error::Error>> {
    // Get latest version from Homebrew
    let output = Command::new("brew")
        .args(["info", "--json=v2", REPOSITORY])
        .output()?;
    print_verbose(&format!("run 'brew info --json=v2 {}'", REPOSITORY));

    if !output.status.success() {
        return Err("Failed to fetch version information from Homebrew".into());
    }

    let brew_info: BrewInfo = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse Homebrew info: {}", e))?;
    let formulae = brew_info.formulae;

    let latest_version = formulae
        .first()
        .ok_or("No version information found in Homebrew response")?
        .versions
        .stable
        .trim_start_matches('v');

    // Parse versions for comparison
    let latest = semver::Version::parse(latest_version)
        .map_err(|_| format!("Invalid version format in release: {}", latest_version))?;
    Ok(latest)
}

pub async fn check_and_install_update(force: bool) -> Result<(), Box<dyn std::error::Error>> {
    print_verbose("Checking for updates via Homebrew...");
    let (new, current, latest) = new_version_available()?;

    // Only proceed if force is true or if latest is actually newer
    if !new && !force {
        println!("You're already on the latest version: {}", current);
        if force {
            println!("Forcing reinstall of version: {}", latest);
        } else {
            println!("Run with --force to reinstall anyway.");
            // Reset the reminder since the user explicitly checked for updates
            if let Err(e) = UpdateReminder::load().reset_reminder() {
                eprintln!("Warning: Failed to reset reminder: {}", e);
            }
            return Ok(());
        }
    } else if new {
        println!("New version available: {} (current: {})", latest, current);
    }

    // Use Homebrew to upgrade the package
    println!("Upgrading via Homebrew...");

    let status = Command::new("brew")
        .args(["upgrade", REPOSITORY])
        .status()?;
    print_verbose(&format!("brew upgrade {}", REPOSITORY));

    if !status.success() {
        return Err("Failed to upgrade via Homebrew".into());
    }

    println!("✅ Successfully upgraded to version: {}", latest);

    // Reset the reminder after successful update
    if let Err(e) = UpdateReminder::load().reset_reminder() {
        eprintln!("Warning: Failed to reset reminder: {}", e);
    }

    Ok(())
}

/// Sets the maximum number of update reminder attempts.
///
/// # Arguments
///
/// * `max_try` - The maximum number of times to show update reminders before stopping.
///
/// # Returns
///
/// Returns `Ok(())` if the configuration was updated successfully, or an error if the update failed.
pub fn set_max_try(max_try: u32) -> Result<(), Box<dyn std::error::Error>> {
    update_config_value("update", "max_try", Value::Integer(max_try as i64))?;
    print_verbose(&format!("Successfully set max try count to: {}", max_try));
    Ok(())
}

/// Sets the interval (in days) between update reminder checks.
///
/// # Arguments
///
/// * `interval` - The number of days to wait between checking for updates.
///
/// # Returns
///
/// Returns `Ok(())` if the configuration was updated successfully, or an error if the update failed.
pub fn set_try_interval(interval: u32) -> Result<(), Box<dyn std::error::Error>> {
    update_config_value(
        "update",
        "try_interval_days",
        Value::Integer(interval as i64),
    )?;
    print_verbose(&format!(
        "Successfully set try interval to: {} days",
        interval
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_update() {
        // This test requires Homebrew and is only relevant on macOS.
        // It will fail on other platforms where `brew` command is not available.
        // To run this test, ensure you are on macOS with Homebrew installed
        // and the package is installed via Homebrew.
        if cfg!(target_os = "macos") {
            let updated = check_and_install_update(false).await;
            assert!(updated.is_ok(), "update failed (test)");
        }
        // Test is skipped on non-macOS platforms.
    }

    #[test]
    fn test_check_update_reminder() {
        let c = check_update_reminder();
        assert!(c.is_ok(), "failed check (test)")
    }
}
