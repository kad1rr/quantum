pub mod views;
pub mod theme;
pub mod cpu;

use views::view_controller;
use theme::theme_controller;

fn main() {
    let model = cpu::cpu::get_cpu_vendor().unwrap();
    let header = view_controller::header::new(&model, true);
    let seperator = view_controller::utils::seperator(63, true);
    println!("{}", header);
    println!("{}", seperator);
}
