use clap::{Command, arg, command};

fn main() {
    let matches = command!()
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds a todo to the list.")
                .aliases(["new", "todo"])
        ).subcommand(
            Command::new("search")
                .about("Searches for your query and filters out matching results.")
                .alias("find")
        ).subcommand(
            Command::new("check")
                .about("Marks the todo as 'completed'.")
                .aliases(["complete", "done"])
        ).subcommand(
            Command::new("delete")
                .about("Deletes a todo from the list.")
                .alias("remove")
        ).get_matches();

    
}
