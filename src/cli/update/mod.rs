use models::BrewInfo;
use semver::Version;
use std::process::Command;

pub mod models;
pub mod reminder;

use reminder::UpdateReminder;

use crate::{
    cli::output::{self, print_verbose},
    constants::REPOSITORY,
};
use gim_config::config::update_config_value;
use toml::Value;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Checks whether an update reminder should be shown to the user and prints a message if a new version is available.
///
/// Loads the update reminder configuration, determines if a reminder should be shown, checks for a new version,
/// and prints a notification if an update is available. Also updates the reminder count as needed.
/// Supports both Homebrew (macOS/Linux) and Scoop (Windows) package managers.
pub fn check_update_reminder() -> Result<(), Box<dyn std::error::Error>> {
    let mut reminder = UpdateReminder::load();
    print_verbose(&format!("Checking new version on config: {}", reminder));

    let to_reminder = reminder.should_show_reminder();
    print_verbose(&format!(
        "Should reminder update according to config: {}",
        to_reminder
    ));
    if to_reminder {
        let check_result = new_version_available()?;
        if check_result.0 {
            output::print_normal(&format!(
                "ℹ️  A new version is available: {}. Run 'gim update' to update.",
                check_result.2
            ));

            // Increment the reminder count or reset if needed
            if let Err(e) = reminder.increment_reminder_count() {
                eprintln!("Warning: Failed to update reminder status: {}", e);
            }
        }
    }
    output::print_verbose(&format!("[background] End checking new version"));
    Ok(())
}

/// Asynchronous version of check_update_reminder that can be run in background
/// without blocking the main program execution.
pub async fn check_update_reminder_async() {
    tokio::task::spawn_blocking(|| {
        if let Err(e) = check_update_reminder() {
            output::print_warning(&format!("Warning: {}", e));
        }
    })
    .await
    .unwrap_or_else(|e| {
        output::print_warning(&format!("Warning: Failed to check for updates: {}", e));
    });
}

fn new_version_available() -> Result<(bool, Version, Version), Box<dyn std::error::Error>> {
    let current_version = VERSION;
    let current = semver::Version::parse(current_version)
        .map_err(|_| format!("Invalid current version format: {}", current_version))?;

    let latest = if cfg!(target_os = "windows") {
        get_latest_version_by_scoop()?
    } else {
        get_latest_version_by_homebrew()?
    };

    output::print_verbose(&format!(
        "[background] Local version: {}; Remote Version: {}",
        current, latest
    ));
    Ok((&latest > &current, current, latest))
}

/// Gets the latest version from Homebrew
fn get_latest_version_by_homebrew() -> Result<Version, Box<dyn std::error::Error>> {
    // Get latest version from Homebrew
    let output = Command::new("brew")
        .args(["info", "--json=v2", REPOSITORY])
        .output()?;
    output::print_verbose(&format!("[background] run 'brew info --json=v2 {}'", REPOSITORY));

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

fn get_scoop_exe() -> Result<String, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Home directory not found")?;
    let scoop_exe = home.join("scoop\\shims\\scoop.cmd");
    Ok(scoop_exe.to_string_lossy().to_string())
}

/// Gets the latest version from Scoop
fn get_latest_version_by_scoop() -> Result<Version, Box<dyn std::error::Error>> {
    // First update the scoop bucket to get latest information
    let scoop_exe = &get_scoop_exe()?;
    let update_output = Command::new(scoop_exe).args(["update"]).output();
    output::print_verbose(&format!("[background] run '{} update'", scoop_exe));

    if let Err(e) = update_output {
        output::print_normal(&format!("Warning: Failed to update Scoop bucket: {}", e));
        return Err("Skip new version checking. You may have to add 'scoop' to your PATH.".into());
    }
    let update_output = update_output?;

    if !update_output.status.success() {
        return Err("Error when running 'scoop update'".into());
    }

    // Check if there's an update available for the package
    let status_output = Command::new(scoop_exe)
        .args(["status", REPOSITORY])
        .output()?;
    output::print_normal(&format!("run '{} status {}'", scoop_exe, REPOSITORY));

    if !status_output.status.success() {
        // If status command fails, try to get info about the package
        let info_output = Command::new(scoop_exe)
            .args(["info", REPOSITORY])
            .output()?;
        output::print_normal(&format!(
            "[I'm TRYing] run '{} info {}'",
            scoop_exe, REPOSITORY
        ));

        if !info_output.status.success() {
            return Err("Failed to fetch version information from Scoop".into());
        }

        let output_str = String::from_utf8_lossy(&info_output.stdout);

        // Parse the scoop info output to extract version
        let version_line = output_str
            .lines()
            .find(|line| line.trim().starts_with("Version:"))
            .ok_or("No version information found in Scoop response")?;

        let latest_version = version_line
            .split(':')
            .nth(1)
            .ok_or("Invalid version format in Scoop response")?
            .trim()
            .trim_start_matches('v');

        let latest = semver::Version::parse(latest_version)
            .map_err(|_| format!("Invalid version format in release: {}", latest_version))?;
        return Ok(latest);
    }

    let status_str = String::from_utf8_lossy(&status_output.stdout);

    // Parse scoop status output to find available updates
    // Look for lines that show package updates available
    for line in status_str.lines() {
        if line.contains(REPOSITORY) {
            // Extract version from status output
            // Typical format: "package_name: 1.0.0 -> 1.1.0"
            let latest_version = if let Some(arrow_pos) = line.find(" -> ") {
                line[arrow_pos + 4..].trim().trim_start_matches('v')
            } else {
                // Typical format: "package_name 1.0.0  1.1.0"
                line.split_whitespace()
                    .nth(2)
                    .ok_or(format!("Unknown version format in status: '{}'", line))?
            };
            let latest = semver::Version::parse(latest_version)
                .map_err(|_| format!("Invalid version format in status: {}", latest_version))?;
            return Ok(latest);
        }
    }

    // If no update found in status, get current version from info
    let info_output = Command::new(scoop_exe)
        .args(["info", REPOSITORY])
        .output()?;
    output::print_normal(&format!(
        "[CHECK DONE] run '{} info {}'",
        scoop_exe, REPOSITORY
    ));

    if !info_output.status.success() {
        return Err("Failed to fetch version information from Scoop".into());
    }

    let output_str = String::from_utf8_lossy(&info_output.stdout);

    let version_line = output_str
        .lines()
        .find(|line| line.trim().starts_with("Version:"))
        .ok_or("No version information found in Scoop response")?;

    let latest_version = version_line
        .split(':')
        .nth(1)
        .ok_or("Invalid version format in Scoop response")?
        .trim()
        .trim_start_matches('v');

    let latest = semver::Version::parse(latest_version)
        .map_err(|_| format!("Invalid version format in release: {}", latest_version))?;
    Ok(latest)
}

pub async fn check_and_install_update(force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let scoop_exe = &get_scoop_exe()?;
    let package_manager = if cfg!(target_os = "windows") {
        scoop_exe
    } else {
        "Homebrew"
    };

    output::print_normal(&format!("Checking for updates via {}...", package_manager));
    let (new, current, latest) = new_version_available()?;

    // Only proceed if force is true or if latest is actually newer
    if !new && !force {
        output::print_normal(&format!(
            "You're already on the latest version: {}. Run with '--force' to update me anyway.",
            current
        ));
        // Reset the reminder since the user explicitly checked for updates
        if let Err(e) = UpdateReminder::load().reset_reminder() {
            eprintln!("Failed to reset update reminder: {}", e);
        }
        return Ok(());
    } else if new {
        output::print_normal(&format!(
            "New version available: {} (current: {})",
            latest, current
        ));
    }

    // Use the appropriate package manager to upgrade
    output::print_normal(&format!("Upgrading via {}...", package_manager));

    let status = if cfg!(target_os = "windows") {
        // Use Scoop to upgrade the package
        let status = Command::new(scoop_exe)
            .args(["update", REPOSITORY])
            .status()?;
        print_verbose(&format!("{} update {}", package_manager, REPOSITORY));
        status
    } else {
        // Use Homebrew to upgrade the package
        let status = Command::new("brew")
            .args(["upgrade", REPOSITORY])
            .status()?;
        print_verbose(&format!("brew upgrade {}", REPOSITORY));
        status
    };

    if !status.success() {
        return Err(format!("Failed to upgrade via {}", package_manager).into());
    }

    output::print_normal(&format!("✅ Successfully upgraded to version: {}", latest));

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
        let updated = check_and_install_update(false).await;
        assert!(updated.is_ok(), "update failed (test)");
    }

    #[test]
    fn test_check_update_reminder() {
        let c = check_update_reminder();
        assert!(c.is_ok(), "failed check (test)")
    }
}
