use std::{collections::HashMap, fs};

use crate::{get_todo_data_path, write_file};
use serde_json;
use shared::{AppState, DEFAULT_DATA_FILE_NAME};
use shared::todo::Todo;

pub fn write_todo(state: &AppState, todo: Todo) -> Result<(), String> {
  let mut todos: HashMap<String, Todo> = read_todos(state)?;
  todos.insert(todo.id.clone(), todo);

  let result = match serde_json::to_string(&todos) {
    Ok(res) => res,
    Err(e) => return Err(e.to_string()),
  };

  match write_file(
    get_todo_data_path(Some(state))
      .join(DEFAULT_DATA_FILE_NAME)
      .as_path(),
    &result,
  ) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}

pub fn write_todos(state: &AppState, todos: HashMap<String, Todo>) -> Result<(), String> {
  let result = match serde_json::to_string(&todos) {
    Ok(res) => res,
    Err(e) => return Err(e.to_string()),
  };

  match write_file(
    get_todo_data_path(Some(state))
      .join(DEFAULT_DATA_FILE_NAME)
      .as_path(),
    &result,
  ) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}

pub fn read_todos(state: &AppState) -> Result<HashMap<String, Todo>, String> {
  let path = get_todo_data_path(Some(state)).join(DEFAULT_DATA_FILE_NAME);

  if !{
    match fs::exists(path.clone()) {
      Ok(res) => res,
      Err(_) => false,
    }
  } {
    return Ok(HashMap::new());
  }

  match fs::read_to_string(path) {
    Ok(todos) => match serde_json::from_str(&todos) {
      Ok(res) => Ok(res),
      Err(e) => Err(e.to_string()),
    },
    Err(e) => Err(e.to_string()),
  }
}
