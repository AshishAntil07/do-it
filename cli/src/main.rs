mod r#match;

use clap::{ArgAction, Command, arg, command};
use data::{get_app_state};
// use dotenvy::dotenv;
use shared::AppState;

use crate::r#match::match_it;

fn main() -> Result<(), String> {
  // if let Err(err) = dotenv() {
  //   eprintln!("Warning: Couldn't load .env variables: {err}");
  // }

  let mut app_state: AppState = get_app_state()?;

  let quotes_command = Command::new("quotes")
    .alias("lessons")
    .about("Lessons learnt.")
    .subcommands([
      Command::new("add")
        .alias("new")
        .about("Learns a lesson.")
        .args([
          arg!(id: "A teeny tiny lesson ID."),
          arg!(-l --lesson <LESSON> "Lesson string"),
        ]),
      Command::new("search")
        .alias("find")
        .about("Searches for a lesson.")
        .args([
          arg!(-i --id <ID> "ID of the todo"),
          arg!(query: "Query to search for."),
        ]),
      Command::new("delete")
        .alias("remove")
        .about("Delete a lesson.")
        .args([arg!(ids: "IDs of the lessons, separated by comma.")]),
    ]);

  let cold_command = Command::new("archive").about("Cold data.").subcommands([
    Command::new("add")
      .alias("new")
      .about("Push to archive.")
      .args([
        arg!(id: "A teeny tiny archive ID."),
        arg!(-d --data <DATA> "Archive data."),
      ]),
    Command::new("search")
      .alias("find")
      .about("Searches for an archive.")
      .args([
        arg!(-i --id <ID> "ID of the todo"),
        arg!(query: "Query to search for."),
      ]),
    Command::new("delete")
      .alias("remove")
      .about("Delete an archive.")
      .args([arg!(ids: "IDs of the archives, separated by comma.")]),
  ]);

  let matches = command!()
    .propagate_version(true)
    .arg(
      arg!(--debug "Switch on debugging mode.").action(ArgAction::SetTrue)
    )
    .subcommands([
      Command::new("add")
        .about("Adds a todo to the list.")
        .aliases(["new", "todo"])
        .args([
          arg!(id: "A small teeny tiny ID for the todo."),
          arg!(-t --title <TITLE> "Title of the todo."),
          arg!(-d --description <DESC> "Description of the todo."),
          arg!(-p --priority <PRIORITY> "Priority of the todo."),
        ]),
      Command::new("search")
        .about("Searches for your query and filters out matching results.")
        .alias("find")
        .args([
          arg!(query: "Query to search for"),
          arg!(-i --id <ID> "ID of the todo"),
          arg!(-d --"has-description" "Todos that have description").action(ArgAction::SetTrue),
          arg!(-c --completed "Completed todos.").action(ArgAction::SetTrue),
          arg!(-b --"not-completed" "Incomplete todos.").action(ArgAction::SetTrue),
          arg!(-p --priority <PRIORITY> "Priority of the todo."),
        ]),
      Command::new("check")
        .about("Marks the todo as 'completed'.")
        .aliases(["complete", "done"])
        .args([arg!(ids: "IDs of the todos, separated by a comma.")]),
      Command::new("delete")
        .about("Deletes a todo from the list.")
        .alias("remove")
        .args([arg!(ids: "IDs of the tods, separated by a comma.")]),
      quotes_command,
      cold_command,
    ])
    .get_matches();

  if matches.get_flag("debug") {
    app_state.debug = true;
  }

  match_it(app_state, matches)?;

  Ok(())
}
