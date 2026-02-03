use data::todo::read_todos;
use shared::AppState;



pub fn list_todo(state: &AppState) -> Result<(), String> {
  let todos = read_todos(state)?;
  
  let mut is_active = false;
  for (_, todo) in todos {
    if !todo.completed {
      println!("{}\n", String::from(todo));
      is_active = true;
    }
  }

  if !is_active {
    println!("No active todos are present. Try making some plans!");
  }

  Ok(())
}