use std::collections::HashSet;

use data::todo::{read_todos, write_todos};
use shared::AppState;
use ui::take_ids_input;



pub fn check_todo(state: &AppState, ids: Option<&String>) -> Result<(), String> {

  let some_ids: HashSet<String>;

  some_ids = if let Some(ids) = ids {
    ids.split(',').map(|slice| String::from(slice)).collect::<HashSet<String>>()
  } else {
    match take_ids_input() {
      Ok(res) => res,
      Err(e) => return Err(e.to_string())
    }
  };

  let mut todos = read_todos(state)?;

  some_ids.into_iter().for_each(|id| {
    if let Some(todo) = todos.get_mut(&id) {
      todo.completed = true;
    }
  });

  write_todos(state, todos)?;

  println!("Marked todos as completed");

  Ok(())
}