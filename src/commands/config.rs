use std::io::{ErrorKind, Result};
use toml::{Value, map::Map};

use crate::config::constants::{CUSTOM_SECTION_NAME, DIFF_SIZE_LIMIT};
use crate::utils::output;

static NAME: &str = "lines_limit";

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
    output::print_normal(
        &format!("set custom config '{}' done, value: {:?}",
        NAME, lines_limit
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
