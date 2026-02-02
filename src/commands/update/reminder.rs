use std::{fmt::Display, str::FromStr};

use chrono::{Local, NaiveDate};
use gim_config::config::{get_config, update_config_value};
use serde::{Deserialize, Serialize};
use toml::Value;

const UPDATE_SECTION: &str = "update";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateReminder {
    last_update_check: NaiveDate,
    reminder_count: u32,
    max_try_count: u32,
    max_check_interval: u32,
}

impl Display for UpdateReminder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "last_update_check: {}, reminder_count: {}/{}, check_interval: {} days",
            self.last_update_check,
            self.reminder_count,
            self.max_try_count,
            self.max_check_interval
        )
    }
}

impl Default for UpdateReminder {
    fn default() -> Self {
        let last_update_check = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        Self {
            last_update_check,
            reminder_count: 0,
            max_try_count: 5,
            max_check_interval: 30,
        }
    }
}

impl UpdateReminder {
    pub fn load() -> Self {
        let config = get_config().unwrap();
        let mut remider = UpdateReminder::default();
        if let Some(update) = config.get(UPDATE_SECTION) {
            if let Some(tried) = update.get("tried") {
                remider.reminder_count = tried.as_integer().unwrap() as u32;
            }
            if let Some(max_try) = update.get("max_try") {
                remider.max_try_count = max_try.as_integer().unwrap() as u32;
            }
            if let Some(interval) = update.get("try_interval_days") {
                remider.max_check_interval = interval.as_integer().unwrap() as u32;
            }
            if let Some(last_check) = update.get("last_try_day") {
                remider.last_update_check =
                    NaiveDate::from_str(last_check.as_str().unwrap()).unwrap();
            }
        }
        remider
    }

    pub fn save(&self) -> std::io::Result<()> {
        update_config_value(
            UPDATE_SECTION,
            "tried",
            Value::Integer(self.reminder_count as i64),
        )?;
        update_config_value(
            UPDATE_SECTION,
            "last_try_day",
            Value::String(self.last_update_check.to_string()),
        )?;
        Ok(())
    }

    pub fn should_show_reminder(&self) -> bool {
        let today = Local::now().date_naive();
        let days_since_last_check = today
            .signed_duration_since(self.last_update_check)
            .num_days();

        days_since_last_check >= self.max_check_interval as i64
            && self.reminder_count < self.max_try_count
    }

    pub fn increment_reminder_count(&mut self) -> std::io::Result<()> {
        let should_reset = self.reminder_count >= self.max_try_count - 1;

        if should_reset {
            self.reset_reminder()
        } else {
            self.reminder_count += 1;
            self.save()
        }
    }

    pub fn reset_reminder(&mut self) -> std::io::Result<()> {
        self.last_update_check = Local::now().date_naive();
        self.reminder_count = 0;
        self.save()
    }
}
