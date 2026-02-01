use std::collections::HashSet;

use data::lesson::{read_lessons, write_lessons};
use shared::AppState;
use ui::take_ids_input;



pub fn delete_lesson(state: &AppState, ids: Option<&String>) -> Result<(), String> {
  let mut lessons = read_lessons(state)?;

  let map_ids: HashSet<String>;

  map_ids = if let Some(ids) = ids {
    ids.split(',').map(|string| String::from(string)).collect()
  } else {
    match take_ids_input() {
      Ok(res) => res,
      Err(e) => return Err(e.to_string())
    }
  };

  for id in map_ids {
    lessons.remove(&id);
  }

  write_lessons(state, lessons)?;

  Ok(())
}