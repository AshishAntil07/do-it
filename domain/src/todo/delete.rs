use std::collections::{HashMap, HashSet};

use data::todo::{read_todos, write_todos};
use shared::{AppState, Todo};
use ui::take_ids_input;



pub fn delete_todo(state: &AppState, ids: Option<&String>) -> Result<(), String> {
  let todos = read_todos(state)?;
  let ids_set: HashSet<String>;
  
  if let Some(ids) = ids {
    ids_set = ids.split(',').map(|slice| String::from(slice)).collect();
  } else {
    ids_set = match take_ids_input() {
      Ok(res) => res,
      Err(e) => return Err(e.to_string())
    }
  }

  let todos: HashMap<String, Todo> = todos.into_iter().filter(|(id, _)| !ids_set.contains(id.as_str())).collect();
  write_todos(state, todos)?;
  
  Ok(())
}