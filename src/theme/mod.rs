pub mod theme_controller {
    use std::{fs, str};
    use serde_json;

    pub fn new(path: &str) -> Result<serde_json::Value, serde_json::Error> {
        let file = fs::read_to_string(path);
        if file.is_ok() {
            return serde_json::from_str(&file.unwrap());
        } else {
            return serde_json::from_str(get_default_json());
        }
    }

    pub fn get(path: &str, key: &str, message: &str) -> String {
        let file = new(path);
        
        if let Ok(json) = file {
            let color_code = json[key].as_str().unwrap_or_default();
            let formatted_message = format!("\x1b[{}{}\x1b[0m", color_code, message);
            return formatted_message;
        }
        message.to_string()
    }
    

    pub fn get_default_json() -> &'static str {
        r#"
        {
          "primary": "31m",
          "secondary": "34m",
          "name": "Default"
        }
        "#.trim()
    }
}
