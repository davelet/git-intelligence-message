use std::io::Result;
use toml;

use crate::utils::output;

/// Updates the AI configuration in the provided TOML value with the specified model, API key, URL, and language.
///
/// # Arguments
///
/// * `config` - Mutable reference to the TOML configuration value.
/// * `model` - Optional model name to set.
/// * `apikey` - Optional API key to set.
/// * `url` - Optional API URL to set.
/// * `language` - Optional language to set.
pub fn update_ai_config(
    config: &mut toml::Value,
    model: &Option<String>,
    apikey: &Option<String>,
    url: &Option<String>,
    language: &Option<String>,
) {
    let ai_table = config
        .get_mut("ai")
        .expect("Missing ai section")
        .as_table_mut()
        .expect("ai section is not a table");

    if let Some(model_value) = model {
        ai_table.insert(
            "model".to_string(),
            toml::Value::String(model_value.clone()),
        );
    }
    if let Some(apikey_value) = apikey {
        ai_table.insert(
            "apikey".to_string(),
            toml::Value::String(apikey_value.clone()),
        );
    }
    if let Some(url_value) = url {
        ai_table.insert("url".to_string(), toml::Value::String(url_value.clone()));
    }
    if let Some(language_value) = language {
        ai_table.insert(
            "language".to_string(),
            toml::Value::String(language_value.clone()),
        );
    }

    if let Err(e) = gim_config::config::save_config(config) {
        eprintln!("Failed to save AI info to file: {}", e)
    }
}

/// Retrieves the AI configuration section from the TOML configuration file.
///
/// # Returns
///
/// * `Ok(toml::Value)` containing the AI configuration if successful.
/// * `Err(std::io::Error)` if the AI section is missing or invalid.
pub fn get_ai_config() -> Result<toml::Value> {
    let toml = gim_config::config::get_config();
    if toml.is_err() {
        toml
    } else if let Ok(toml) = toml {
        let ai = toml.get("ai");
        if let Some(ai) = ai {
            Ok(ai.clone())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to get ai section",
            ))
        }
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to get ai config",
        ))
    }
}

/// Masks the API key for display.
///
/// # Arguments
///
/// * `api_key` - The API key to mask.
///
/// # Returns
///
/// * A masked version of the API key.
pub fn mask_api_key(api_key: &str) -> String {
    if api_key.len() <= 8 {
        "***".to_string()
    } else {
        format!("{}***", &api_key[..8])
    }
}

/// Validates and retrieves AI configuration.
///
/// # Arguments
///
/// * `auto_add` - Whether auto-add is enabled.
/// * `changed` - Whether there are changes.
///
/// # Returns
///
/// * `Some((url, model_name, api_key, language))` if valid, `None` otherwise.
pub fn get_validated_ai_config(
    auto_add: bool,
    changed: bool,
) -> Option<(String, String, String, String)> {
    let ai_config = get_ai_config();
    if ai_config.is_err() {
        ai_generating_error(
            "Error: ai section is not configured, abort",
            auto_add && changed,
        );
        return None;
    }
    let ai_config = match ai_config {
        Ok(config) => config,
        Err(e) => {
            ai_generating_error(
                &format!("Error: Failed to get AI config - {}", e),
                auto_add && changed,
            );
            return None;
        }
    };

    let url = match ai_config.get("url").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => {
            ai_generating_error("Error: Missing 'url' in AI config", auto_add && changed);
            return None;
        }
    };
    let model_name = match ai_config.get("model").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => {
            ai_generating_error("Error: Missing 'model' in AI config", auto_add && changed);
            return None;
        }
    };
    let api_key = match ai_config.get("apikey").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => {
            ai_generating_error("Error: Missing 'apikey' in AI config", auto_add && changed);
            return None;
        }
    };
    let language = match ai_config.get("language").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => {
            ai_generating_error(
                "Error: Missing 'language' in AI config",
                auto_add && changed,
            );
            return None;
        }
    };

    Some((
        url.to_string(),
        model_name.to_string(),
        api_key.to_string(),
        language.to_string(),
    ))
}

/// Prints AI generation error message.
///
/// # Arguments
///
/// * `abort` - The error message.
/// * `auto_add` - Whether auto-add is enabled.
pub fn ai_generating_error(abort: &str, auto_add: bool) {
    eprintln!("{}", abort);
    if auto_add {
        output::print_verbose("No staged changes to commit.");
    }
}
