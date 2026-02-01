pub mod todo;
pub mod lesson;
pub mod archive;

use std::{
  fs,
  path::{Path, PathBuf},
};

use dirs::home_dir;
use shared::{APP_DIR_NAME, AppState, CONFIG_FILE_NAME, Config, DATA_DIR_NAME, TODO_DATA_DIR_NAME, LESSONS_DATA_DIR_NAME, ARCHIVE_DATA_DIR_NAME};

pub fn get_data_path(state: Option<&AppState>) -> PathBuf {
  if let Some(state) = state {
    PathBuf::from(&state.config.app_dir_name).join(DATA_DIR_NAME)
  } else {
    home_dir()
      .unwrap()
      .to_path_buf()
      .join(APP_DIR_NAME)
      .join(DATA_DIR_NAME)
  }
}
pub fn get_todo_data_path(state: Option<&AppState>) -> PathBuf {
  get_data_path(state).join(TODO_DATA_DIR_NAME)
}
pub fn get_lesson_data_path(state: Option<&AppState>) -> PathBuf {
  get_data_path(state).join(LESSONS_DATA_DIR_NAME)
}
pub fn get_archive_data_path(state: Option<&AppState>) -> PathBuf {
  get_data_path(state).join(ARCHIVE_DATA_DIR_NAME)
}

pub fn write_file(path: &Path, contents: &str) -> std::io::Result<()> {
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?;
  }
  fs::write(path, contents)
}

pub fn read_config() -> Result<Config, String> {
  let config_path = get_data_path(None).join(CONFIG_FILE_NAME);

  match fs::read_to_string(config_path.clone()) {
    Ok(res) => match serde_json::from_str::<Config>(&res) {
      Ok(res) => Ok(res),
      Err(e) => return Err(e.to_string()),
    },
    Err(_) => {
      let def_config = Config {
        app_dir_name: String::from(
          home_dir()
            .unwrap()
            .to_path_buf()
            .join(APP_DIR_NAME)
            .to_str()
            .unwrap(),
        ),
      };
      if let Err(e) = write_file(&config_path, &serde_json::to_string(&def_config).unwrap()) {
        Err(e.to_string())
      } else {
        Ok(def_config)
      }
    }
  }
}

pub fn get_app_state() -> Result<AppState, String> {
  Ok(AppState {
    config: read_config()?,
    debug: false
  })
}
