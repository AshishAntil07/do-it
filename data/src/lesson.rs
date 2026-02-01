use std::{collections::HashMap, fs};

use crate::{get_lesson_data_path, write_file};
use serde_json;
use shared::{AppState, DEFAULT_DATA_FILE_NAME, lesson::{Lesson}};

pub fn write_lesson(state: &AppState, lesson: Lesson) -> Result<(), String> {
  let mut lessons: HashMap<String, Lesson> = read_lessons(state)?;
  lessons.insert(lesson.id.clone(), lesson);

  let result = match serde_json::to_string(&lessons) {
    Ok(res) => res,
    Err(e) => return Err(e.to_string()),
  };

  match write_file(
    get_lesson_data_path(Some(state))
      .join(DEFAULT_DATA_FILE_NAME)
      .as_path(),
    &result,
  ) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}

pub fn write_lessons(state: &AppState, lessons: HashMap<String, Lesson>) -> Result<(), String> {
  let result = match serde_json::to_string(&lessons) {
    Ok(res) => res,
    Err(e) => return Err(e.to_string()),
  };

  match write_file(
    get_lesson_data_path(Some(state))
      .join(DEFAULT_DATA_FILE_NAME)
      .as_path(),
    &result,
  ) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}

pub fn read_lessons(state: &AppState) -> Result<HashMap<String, Lesson>, String> {
  let path = get_lesson_data_path(Some(state)).join(DEFAULT_DATA_FILE_NAME);

  if !{
    match fs::exists(path.clone()) {
      Ok(res) => res,
      Err(_) => false,
    }
  } {
    return Ok(HashMap::new());
  }

  match fs::read_to_string(path) {
    Ok(lessons) => match serde_json::from_str(&lessons) {
      Ok(res) => Ok(res),
      Err(e) => Err(e.to_string()),
    },
    Err(e) => Err(e.to_string()),
  }
}
