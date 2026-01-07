use std::io::Error;

use data::todo::write_todo;
use shared::{PartialTodo, Priority};
use ui::take_todo_inputs;

pub fn add_todo(
  id: Option<&String>,
  title: Option<&String>,
  description: Option<&String>,
  priority: Option<&Priority>,
) -> Result<(), String> {
  let id = if let Some(id) = id {
    Some(id.clone())
  } else { None };
  let title = if let Some(title) = title {
    Some(title.clone())
  } else { None };
  let description = if let Some(description) = description {
    Some(description.clone())
  } else { None };
  let priority = if let Some(priority) = priority {
    Some(*priority)
  } else { None };

  let mut ptodo: PartialTodo = PartialTodo {
    id,
    title,
    description,
    priority
  };

  let todo_res = take_todo_inputs(&mut ptodo);



  match todo_res {
    Ok(todo) => {
      write_todo(todo)
    },
    Err(e) => return Err(e.to_string())
  }
}
