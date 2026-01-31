use data::todo::read_todos;
use shared::{AppState, Priority};

pub fn search_todo(
  state: &AppState,
  id: Option<&String>,
  query: Option<&String>,
  has_description: bool,
  completed: bool,
  not_completed: bool,
  priority: Option<&Priority>,
) -> Result<(), String> {
  let todos = read_todos(state)?;

  let result = todos.into_iter().filter(|(todo_id, todo)| {
    if state.debug {
      println!(
        "{} - {} && {} && {} && {} && {}",
        todo.id,
        id.is_none() || id.is_some_and(|id| *id == *todo_id),
        query.is_none()
          || query.is_some_and(|query| {
            todo.title == *query
              || todo
                .description
                .as_ref()
                .is_some_and(|desc| *desc == *query)
          }),
        todo.description.is_some() == has_description,
        priority.is_none() || priority.is_some_and(|priority| *priority == todo.priority),
        if completed {
          todo.completed == completed
        } else if not_completed {
          todo.completed == !not_completed
        } else {
          true
        }
      );
    }

    true
      && { id.is_none() || id.is_some_and(|id| *id == *todo_id) }
      && {
        query.is_none()
          || query.is_some_and(|query| {
            todo.title == *query
              || todo
                .description
                .as_ref()
                .is_some_and(|desc| *desc == *query)
          })
      }
      && todo.description.is_some() == has_description
      && { priority.is_none() || priority.is_some_and(|priority| *priority == todo.priority) }
      && if completed {
        todo.completed == completed
      } else if not_completed {
        todo.completed == !not_completed
      } else {
        true
      }
  });

  for (_, todo) in result {
    println!("{}", String::from(todo));
  }

  Ok(())
}
