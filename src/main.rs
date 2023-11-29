use std::io::{self, Write};

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
    println!("Unsupported platform");
}

#[cfg(target_os = "macos")]
fn clear_console() {
    std::process::Command::new("clear").status().unwrap();
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

    loop {
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("err main.rs:45 read_line failed with expect");

        let input = input.trim();

        if input.to_lowercase() == "exit" {
            break;
        }
    }
}
