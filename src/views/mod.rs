pub mod view_controller {
    pub mod header {
        use std::fs;
        use crate::theme::theme_controller;

        pub fn new(name: &str, use_colors: bool) -> String {
            return get(name, use_colors);
        }
    
        pub fn get(name: &str, use_color: bool) -> String {
            let file_path = format!("./src/views/assets/{}.txt", name);
            let is_file_exists = fs::metadata(file_path.clone()).is_ok();
            if is_file_exists {
                if let Ok(content) = fs::read_to_string(file_path.clone()) {
                    if use_color {
                        let header = theme_controller::get("./quantum.theme.json", "primary", &content);
                        return header;
                    } else {
                        let header = content;
                        return header;
                    }
                }
            }
            return "error".to_string();
        }
    }

    pub mod utils {
        use crate::theme_controller;

        pub fn seperator(mut l: u128, use_color: bool) -> String {
            let mut message = "".to_string();

            while l > 0 {
                message = message.to_owned() + "-";
                l = l - 1;
            }

            if use_color {
                return theme_controller::get("./quantum.theme.json", "primary", &message);
            } else {
                return message;

            }
        }
    }
    
}