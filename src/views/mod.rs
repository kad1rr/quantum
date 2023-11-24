#[warn(unused_imports)]
use std::fs;
use std::str;

pub mod view_controller {
    use std::fs;

    pub fn new(use_colors: bool) -> bool {
        
        return true;
    }

    pub fn get(name: &str, use_color: bool) -> bool {

        let is_file_exists = fs::metadata("./assets/".to_owned() + name + ".txt").is_ok();
        println!("{}", is_file_exists.to_string());
        return true;
    }
}