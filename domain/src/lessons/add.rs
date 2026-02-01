use data::lesson::write_lesson;
use shared::{AppState, lesson::{PartialLesson}};
use ui::lesson::take_lesson_inputs;

pub fn add_lesson(state: &AppState, id: Option<&String>, lesson: Option<&String>) -> Result<(), String> {
  let mut plesson = PartialLesson {
    id: if id.is_some() {Some(id.unwrap().to_owned())} else {None},
    lesson: if lesson.is_some() {Some(lesson.unwrap().to_owned())} else {None},
  };

  write_lesson(state, match take_lesson_inputs(&mut plesson) {
    Ok(res) => res,
    Err(e) => return Err(e.to_string())
  })?;

  Ok(())
}