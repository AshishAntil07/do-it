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

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
  pub id: String,
  pub title: String,
  pub description: Option<String>,
  pub priority: Priority,
  pub completed: bool,
}


impl From<Todo> for String {
  fn from(todo: Todo) -> String {
    format!(
      "{} {} - {} ({}){}",
      if todo.completed { "✔" } else { "✘" },
      todo.id,
      todo.title,
      String::from(todo.priority),
      if todo.description.is_some() {
        format!("\n{}", todo.description.unwrap())
      } else {
        String::new()
      }
    )
  }
}

#[derive(Clone)]
pub struct PartialTodo {
  pub id: Option<String>,
  pub title: Option<String>,
  pub description: Option<String>,
  pub priority: Option<Priority>,
  pub completed: Option<bool>,
}

impl Default for PartialTodo {
  fn default() -> Self {
    PartialTodo {
      id: None,
      title: None,
      description: None,
      priority: None,
      completed: None,
    }
  }
}

impl From<PartialTodo> for Todo {
  fn from(ptodo: PartialTodo) -> Self {
    Todo {
      id: ptodo.id.unwrap_or(String::new()),
      title: ptodo.title.unwrap_or(String::new()),
      description: ptodo.description,
      priority: ptodo.priority.unwrap_or(Priority::VeryLow),
      completed: ptodo.completed.unwrap_or(false),
    }
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
