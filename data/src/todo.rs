use shared::{AppState, Todo};
use serde_json;
use crate::{get_todo_data_path, write_file};



pub fn write_todo(state: &AppState, todo: Todo) -> Result<(), String> {

  let result = match serde_json::to_string(&todo) {
    Ok(res) => res,
    Err(e) => return Err(e.to_string())
  };

  match write_file(get_todo_data_path(Some(state)).as_path(), &result) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string())
  }
}