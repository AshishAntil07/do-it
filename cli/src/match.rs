use clap::ArgMatches;
use domain::{
  archive::{add::add_archive, delete::delete_archive, search::search_archive}, lessons::{add::add_lesson, delete::delete_lesson, search::search_lesson}, todo::{add::add_todo, check::check_todo, delete::delete_todo, search::search_todo}
};
use shared::{AppState, Priority};

pub fn match_it(state: AppState, matches: ArgMatches) -> Result<(), String> {
  match matches.subcommand() {
    Some(("add", matches)) => {
      let id = matches.get_one::<String>("id");
      let title = matches.get_one::<String>("title");
      let description = matches.get_one::<String>("description");
      let priority = matches.get_one::<Priority>("priority");

      add_todo(&state, id, title, description, priority)?;
    }
    Some(("search", matches)) => {
      let id = matches.get_one::<String>("id");
      let query = matches.get_one::<String>("query");
      let has_description = matches.get_flag("has_description");
      let completed = matches.get_flag("completed");
      let not_completed = matches.get_flag("not_completed");
      let priority = matches.get_one::<Priority>("priority");

      search_todo(
        &state,
        id,
        query,
        has_description,
        completed,
        not_completed,
        priority,
      );
    }
    Some(("check", matches)) => {
      let ids = matches.get_one::<String>("ids");

      check_todo(&state, ids);
    }
    Some(("delete", matches)) => {
      let ids = matches.get_one::<String>("ids");

      delete_todo(&state, ids);
    }
    Some(("lessons", matches)) => match matches.subcommand() {
      Some(("add", matches)) => {
        let id = matches.get_one::<String>("id");
        let lesson = matches.get_one::<String>("lesson");

        add_lesson(&state, id, lesson);
      }
      Some(("search", matches)) => {
        let id = matches.get_one::<String>("id");
        let query = matches.get_one::<String>("query");

        search_lesson(&state, id, query);
      }
      Some(("delete", matches)) => {
        let ids = matches.get_one::<String>("ids");

        delete_lesson(&state, ids);
      }
      _ => (),
    },
    Some(("archive", matches)) => match matches.subcommand() {
      Some(("add", matches)) => {
        let id = matches.get_one::<String>("id");
        let data = matches.get_one::<String>("data");

        add_archive(&state, id, data);
      }
      Some(("search", matches)) => {
        let id = matches.get_one::<String>("id");
        let query = matches.get_one::<String>("query");

        search_archive(&state, id, query);
      }
      Some(("delete", matches)) => {
        let ids = matches.get_one::<String>("ids");

        delete_archive(&state, ids);
      }
      _ => (),
    },
    _ => (),
  };

  Ok(())
}
