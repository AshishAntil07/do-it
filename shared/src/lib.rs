use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Priority {
  VeryLow = 1,
  Low,
  Medium,
  High,
  VeryHigh,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
  pub id: String,
  pub title: String,
  pub description: Option<String>,
  pub priority: Priority,
}

#[derive(Clone)]
pub struct PartialTodo {
  pub id: Option<String>,
  pub title: Option<String>,
  pub description: Option<String>,
  pub priority: Option<Priority>,
}

impl From<PartialTodo> for Todo {
  fn from(ptodo: PartialTodo) -> Self {
    Todo {
      id: ptodo.id.unwrap_or(String::new()),
      title: ptodo.title.unwrap_or(String::new()),
      description: ptodo.description,
      priority: ptodo.priority.unwrap_or(Priority::VeryLow)
    }
  }
}

pub const APP_DIR_NAME: &str = ".do-it";
pub const DATA_DIR_NAME: &str = "data";
pub const TODO_DATA_DIR_NAME: &str = "todo";
pub const LESSONS_DATA_DIR_NAME: &str = "lessons";
pub const ARCHIVE_DATA_DIR_NAME: &str = "archive";
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
  pub config: Config,

}

