pub mod theme_controller {
  use std::{fs, str};
  use serde_json;

  pub fn new(path: &str) -> Result<serde_json::Value, serde_json::Error> {
      let file = fs::read_to_string(path);
      serde_json::from_str(&file.unwrap())
  }

  pub fn get(path: &str, key: &str, message: &str) -> String {
      if let Ok(theme) = new(path) {
          if let Some(color_value) = theme[key].as_str() {
              let color_code = format!("\x1b[{}", color_value);
              return format!("{}{}{}", color_code, message, "\x1b[0m");
          }
      }

      message.to_string()
  }
}
