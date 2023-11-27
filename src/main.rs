#[cfg(windows)]
fn clear_console() {
    std::process::Command::new("cmd").arg("/c").arg("cls").status().unwrap();
}


#[cfg(unix)]
fn clear_console() {
    std::process::Command::new("clear").status().unwrap();
}

#[cfg(not(any(windows, unix)))]
fn clear_console() {
    // TODO: Add support for other platforms
}

pub mod views;
pub mod theme;
pub mod cpu;

use views::view_controller;
use theme::theme_controller;

fn main() {
    clear_console();
    let model = cpu::cpu::get_cpu_vendor().unwrap();
    let header = view_controller::header::new(&model, true);
    let separator = view_controller::utils::seperator(63, true);
    println!("{}", header);
    println!("{}", separator);
    view_controller::main::run_basic(true);
}
