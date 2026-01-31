pub mod todo;

use std::collections::HashSet;
use inquire::{Editor, InquireError};

pub fn take_ids_input() -> Result<HashSet<String>, InquireError> {
  match Editor::new("IDs of the elements to delete (separated by a comma)").prompt() {
    Ok(res) => {
      let set_res: HashSet<String> = res.split(',').map(|slice| slice.to_string()).collect();

      Ok(set_res)
    },
    Err(e) => Err(e)
  }
}