use std::io::{ErrorKind, Result};
use toml::{Value, map::Map};

use crate::config::constants::{CUSTOM_SECTION_NAME, DIFF_SIZE_LIMIT, MAX_DIFF_FILES};
use crate::utils::output;

static NAME: &str = "lines_limit";
static MAX_FILES_NAME: &str = "max_diff_files";

pub fn get_lines_limit() -> usize {
    let lines_limit = gim_config::config::get_config_value(CUSTOM_SECTION_NAME, NAME);
    if let Err(e) = lines_limit {
        output::print_verbose(&format!(
            "get custom config '{}' error: {:?}, return default: {}",
            NAME, e, DIFF_SIZE_LIMIT
        ));
        return DIFF_SIZE_LIMIT;
    }
    let lines_limit = lines_limit.ok();
    if let Some(limit) = lines_limit {
        output::print_verbose(&format!("get custom config '{}' value: {:?}", NAME, limit));
        return limit.as_integer().unwrap() as usize;
    }
    DIFF_SIZE_LIMIT
}

pub fn set_lines_limit(lines_limit: usize) -> Result<()> {
    let set = gim_config::config::update_config_value(
        CUSTOM_SECTION_NAME,
        NAME,
        Value::Integer(lines_limit as i64),
    );
    if let Err(e) = set {
        output::print_verbose(&format!(
            "get custom config '{}' error: {:?}, return default: {}",
            NAME, e, DIFF_SIZE_LIMIT
        ));
        if e.kind() == ErrorKind::NotFound {
            if e.to_string() == format!("Section '{}' not found", CUSTOM_SECTION_NAME) {
                let mut config = gim_config::config::get_config().unwrap();
                let map = config.as_table_mut().unwrap();

                let mut update_table = Map::new();
                update_table.insert(NAME.to_string(), Value::Integer(lines_limit as i64));
                map.insert(CUSTOM_SECTION_NAME.to_string(), Value::Table(update_table));
                return gim_config::config::save_config(&mut config);
            }
        }
        return Err(e);
    }
    output::print_normal(&format!(
        "set custom config '{}' done, value: {:?}",
        NAME, lines_limit
    ));
    Ok(())
}

pub fn get_max_diff_files() -> usize {
    let max_files = gim_config::config::get_config_value(CUSTOM_SECTION_NAME, MAX_FILES_NAME);
    if let Err(e) = max_files {
        output::print_verbose(&format!(
            "get custom config '{}' error: {:?}, return default: {}",
            MAX_FILES_NAME, e, MAX_DIFF_FILES
        ));
        return MAX_DIFF_FILES;
    }
    let max_files = max_files.ok();
    if let Some(limit) = max_files {
        output::print_verbose(&format!(
            "get custom config '{}' value: {:?}",
            MAX_FILES_NAME, limit
        ));
        return limit.as_integer().unwrap_or(MAX_DIFF_FILES as i64) as usize;
    }
    MAX_DIFF_FILES
}

pub fn set_max_diff_files(max_files: usize) -> Result<()> {
    let set = gim_config::config::update_config_value(
        CUSTOM_SECTION_NAME,
        MAX_FILES_NAME,
        Value::Integer(max_files as i64),
    );
    if let Err(e) = set {
        output::print_verbose(&format!(
            "set custom config '{}' error: {:?}",
            MAX_FILES_NAME, e
        ));
        if e.kind() == ErrorKind::NotFound {
            if e.to_string() == format!("Section '{}' not found", CUSTOM_SECTION_NAME) {
                let mut config = gim_config::config::get_config().unwrap();
                let map = config.as_table_mut().unwrap();

                let mut update_table = Map::new();
                update_table.insert(MAX_FILES_NAME.to_string(), Value::Integer(max_files as i64));
                map.insert(CUSTOM_SECTION_NAME.to_string(), Value::Table(update_table));
                return gim_config::config::save_config(&mut config);
            }
        }
        return Err(e);
    }
    output::print_normal(&format!(
        "set custom config '{}' done, value: {:?}",
        MAX_FILES_NAME, max_files
    ));
    Ok(())
}

/// Gets and prints the config file location.
///
/// # Returns
///
/// * `Ok(())` if successful, `Err` otherwise.
pub fn get_config_and_print() -> Result<()> {
    let config_dir = gim_config::directory::config_dir()?;
    output::print_normal(&format!("Config file location: {}", config_dir.display()));
    Ok(())
}
