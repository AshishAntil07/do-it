use shared::Todo;
use serde_json;
use crate::{get_todo_data_path, write_file};



pub fn write_todo(todo: Todo) -> Result<(), String> {

  let result = match serde_json::to_string(&todo) {
    Ok(res) => res,
    Err(e) => return Err(e.to_string())
  };

  match write_file(get_todo_data_path().as_path(), &result) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string())
  }
}