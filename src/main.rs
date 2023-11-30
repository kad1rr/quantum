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

fn run_basic_view(use_colors: bool) {
    clear_console();
    let model = cpu::cpu::get_cpu_vendor().unwrap();
    let header = view_controller::header::new(&model, use_colors);
    let separator = view_controller::utils::seperator(63, use_colors);
    println!("{}", header);
    println!("{}", separator);
    view_controller::main::run_basic(use_colors);
}

fn run_advanced_view(use_colors: bool) {
    clear_console();
    let model = cpu::cpu::get_cpu_vendor().unwrap();
    let header = view_controller::header::new(&model, use_colors);
    let separator = view_controller::utils::seperator(63, use_colors);
    println!("{}", header);
    println!("{}", separator);
    view_controller::main::run_advanced(use_colors);
}

fn main() {
    run_basic_view(true);
    loop {
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("err main.rs:47 read_line failed with expect");

        let input = input.trim();

        if input.to_lowercase() == "exit" {
            break;
        } else if input.to_lowercase() == "no_color" {
            run_basic_view(false)
        } else if input.to_lowercase() == "refresh" {
            run_basic_view(true)
        } else if input.to_lowercase() == "advanced" {
            run_advanced_view(true)
        } else if input.to_lowercase() == "no_color_advanced" {
            run_advanced_view(false);
        }
    }
}
