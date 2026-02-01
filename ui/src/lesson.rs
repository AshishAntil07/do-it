use inquire::{InquireError, Text};
use shared::lesson::{Lesson, PartialLesson};

pub fn take_lesson_inputs(plesson: &mut PartialLesson) -> Result<Lesson, InquireError> {
  if plesson.id.is_none() {
    plesson.id = Some(Text::new("ID of lesson").prompt()?);
  }
  if plesson.lesson.is_none() {
    plesson.lesson = Some(Text::new("The lesson").prompt()?);
  }

  Ok(plesson.to_owned().into())
}
