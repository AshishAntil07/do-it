use data::todo::write_todo;
use shared::{AppState, todo::PartialTodo, Priority};
use ui::todo::take_todo_inputs;

pub fn add_todo(
  state: &AppState,
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
    priority,
    ..Default::default()
  };

  let todo_res = take_todo_inputs(&mut ptodo);

  match todo_res {
    Ok(mut todo) => {
      if todo.description.clone().is_some_and(|desc| desc == "") {
        todo.description = None;
      }
      write_todo(state, todo)
    },
    Err(e) => return Err(e.to_string())
  }
}
