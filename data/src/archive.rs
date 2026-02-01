use std::{collections::HashMap, fs};

use crate::{get_archive_data_path, write_file};
use serde_json;
use shared::{AppState, DEFAULT_DATA_FILE_NAME, archive::{Archive}};

pub fn write_archive(state: &AppState, archive: Archive) -> Result<(), String> {
  let mut archives: HashMap<String, Archive> = read_archives(state)?;
  archives.insert(archive.id.clone(), archive);

  let result = match serde_json::to_string(&archives) {
    Ok(res) => res,
    Err(e) => return Err(e.to_string()),
  };

  match write_file(
    get_archive_data_path(Some(state))
      .join(DEFAULT_DATA_FILE_NAME)
      .as_path(),
    &result,
  ) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}

pub fn write_archives(state: &AppState, archives: HashMap<String, Archive>) -> Result<(), String> {
  let result = match serde_json::to_string(&archives) {
    Ok(res) => res,
    Err(e) => return Err(e.to_string()),
  };

  match write_file(
    get_archive_data_path(Some(state))
      .join(DEFAULT_DATA_FILE_NAME)
      .as_path(),
    &result,
  ) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}

pub fn read_archives(state: &AppState) -> Result<HashMap<String, Archive>, String> {
  let path = get_archive_data_path(Some(state)).join(DEFAULT_DATA_FILE_NAME);

  if !{
    match fs::exists(path.clone()) {
      Ok(res) => res,
      Err(_) => false,
    }
  } {
    return Ok(HashMap::new());
  }

  match fs::read_to_string(path) {
    Ok(archives) => match serde_json::from_str(&archives) {
      Ok(res) => Ok(res),
      Err(e) => Err(e.to_string()),
    },
    Err(e) => Err(e.to_string()),
  }
}
