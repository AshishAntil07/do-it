pub mod todo;
pub mod lesson;
pub mod archive;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Priority {
  VeryLow = 1,
  Low,
  Medium,
  High,
  VeryHigh,
}

impl From<Priority> for String {
  fn from(value: Priority) -> String {
    String::from(if value == Priority::VeryLow {
      "Very Low"
    } else if value == Priority::Low {
      "Low"
    } else if value == Priority::Medium {
      "Medium"
    } else if value == Priority::High {
      "High"
    } else {
      "Very High"
    })
  }
}

pub const APP_DIR_NAME: &str = ".do-it";
pub const DATA_DIR_NAME: &str = "data";
pub const TODO_DATA_DIR_NAME: &str = "todo";
pub const LESSONS_DATA_DIR_NAME: &str = "lessons";
pub const ARCHIVE_DATA_DIR_NAME: &str = "archive";
pub const DEFAULT_DATA_FILE_NAME: &str = "data.json";
pub const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
  #[serde(default = "get_app_dir_name")]
  pub app_dir_name: String,
}

fn get_app_dir_name() -> String {
  APP_DIR_NAME.to_string()
}

pub struct AppState {
  pub debug: bool,
  pub config: Config,
}
