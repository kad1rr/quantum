pub mod views;
pub mod theme;

use views::view_controller;

fn main() {
    view_controller::get("amd", true);
    println!("Hello, world!");
}
