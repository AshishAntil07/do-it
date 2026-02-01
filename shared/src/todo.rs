use serde::{Deserialize, Serialize};

use crate::Priority;

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