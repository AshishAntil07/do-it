use std::collections::HashMap;

use inquire::{Editor, InquireError, Select, Text};
use shared::{PartialTodo, Priority, Todo};

pub fn take_todo_inputs(ptodo: &mut PartialTodo) -> Result<Todo, InquireError> {
  if let None = ptodo.id {
    ptodo.id = Some(Text::new("ID of todo").prompt()?);
  }
  if let None = ptodo.title {
    ptodo.title = Some(Text::new("Title").prompt()?);
  }
  if let None = ptodo.description {
    ptodo.description = Some(Editor::new("Description").prompt()?);
  }

  if let None = ptodo.priority {
    let priority_map = HashMap::from([
      (String::from("Very Low"), Priority::VeryLow),
      (String::from("Low"), Priority::Low),
      (String::from("Medium"), Priority::Medium),
      (String::from("High"), Priority::High),
      (String::from("Very High"), Priority::VeryHigh),
    ]);
    let options = priority_map.keys().collect();
    let selected = Select::new("Select Priority", options).prompt()?;
    ptodo.priority = Some(*priority_map.get(selected).unwrap_or(&Priority::VeryLow));
  }

  Ok(ptodo.clone().into())
}
