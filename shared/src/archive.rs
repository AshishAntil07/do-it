use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Archive {
  pub id: String,
  pub data: String
}

impl From<Archive> for String {
  fn from(value: Archive) -> Self {
      format!("{} - {}", value.id, value.data)
  }
}

#[derive(Clone)]
pub struct PartialArchive {
  pub id: Option<String>,
  pub data: Option<String>
}

impl From<PartialArchive> for Archive {
  fn from(value: PartialArchive) -> Self {
      Archive {
        id: value.id.unwrap_or(String::new()),
        data: value.data.unwrap_or(String::new())
      }
  }
}