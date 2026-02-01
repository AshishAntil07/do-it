use std::collections::HashSet;

use data::archive::{read_archives, write_archives};
use shared::AppState;
use ui::take_ids_input;


pub fn delete_archive(state: &AppState, ids: Option<&String>) -> Result<(), String> {

  let mut archives = read_archives(state)?;

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
    archives.remove(&id);
  }

  write_archives(state, archives)?;

  Ok(())
}
