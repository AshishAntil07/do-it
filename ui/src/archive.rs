use inquire::{InquireError, Text};
use shared::{AppState, archive::{Archive, PartialArchive}};

pub fn take_archive_inputs(state: &AppState, parchive: &mut PartialArchive) -> Result<Archive, InquireError> {
  if parchive.id.is_none() {
    parchive.id = Some(Text::new("ID of archive").prompt()?);
  }
  if parchive.data.is_none() {
    parchive.data = Some(Text::new("Data").prompt()?);
  }

  if state.debug {
    println!("took archive inputs");
  }

  Ok(parchive.to_owned().into())
}
