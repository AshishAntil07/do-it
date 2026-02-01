use data::archive::write_archive;
use shared::{AppState, archive::{PartialArchive}};
use ui::archive::take_archive_inputs;



pub fn add_archive(state: &AppState, id: Option<&String>, data: Option<&String>) -> Result<(), String> {
  let mut parchive = PartialArchive {
    id: if id.is_some() {Some(id.unwrap().to_owned())} else {None},
    data: if data.is_some() {Some(data.unwrap().to_owned())} else {None},
  };

  write_archive(state, match take_archive_inputs(state, &mut parchive) {
    Ok(res) => res,
    Err(e) => return Err(e.to_string())
  })?;

  Ok(())
}