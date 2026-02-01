use shared::AppState;
use data::{archive::read_archives};

pub fn search_archive(state: &AppState, id: Option<&String>, query: Option<&String>) -> Result<(), String> {
  let archives = read_archives(state)?;

  let filtered = archives.into_iter().filter(|(archive_id, archive)| {
    if state.debug {
      println!(
        "{} - {}, {}",
        archive_id,
        id.is_none() || *archive_id == *id.unwrap(),
        query.is_none() || archive.data.contains(query.unwrap())
      );
    }

    return { id.is_none() || *archive_id == *id.unwrap() } || {
      query.is_none() || archive.data.contains(query.unwrap())
    };
  });

  for (_, archive) in filtered {
    println!("{}", String::from(archive));
  }

  Ok(())
}