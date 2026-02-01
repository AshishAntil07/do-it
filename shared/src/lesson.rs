use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Lesson {
  pub id: String,
  pub lesson: String
}

impl From<Lesson> for String {
  fn from(value: Lesson) -> Self {
      format!("{} - {}", value.id, value.lesson)
  }
}

#[derive(Clone)]
pub struct PartialLesson {
  pub id: Option<String>,
  pub lesson: Option<String>
}

impl From<PartialLesson> for Lesson {
  fn from(value: PartialLesson) -> Self {
      Lesson {
        id: value.id.unwrap_or(String::new()),
        lesson: value.lesson.unwrap_or(String::new())
      }
  }
}