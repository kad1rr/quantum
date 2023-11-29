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

    pub mod main {
        use crate::{theme::theme_controller, cpu};
        use std::str;
        use sysinfo::{SystemExt, System};

        pub fn run_basic(use_colors: bool) -> bool {
            let mut system = sysinfo::System::new_all();

            system.refresh_all();

            let mut cpu = cpu::cpu::get_cpu_vendor().unwrap();

            if cpu == "amd" {
                cpu = "Amd Ryzen".to_string()
            } else if cpu == "intel" {
                cpu = "Intel".to_string()
            }

            let memory_message = get_memory_usage(&system);

            if use_colors {
                print("Os", &system.name().unwrap());
                print("Host", &system.host_name().unwrap());
                print("Kernel", &system.kernel_version().unwrap());
                print("Uptime", &get_uptime());
                print("CPU", &cpu);
                print("Theme", &theme_controller::new("./quantum.theme.json").unwrap()["name"].to_string());
                print("Memory", &memory_message);
            }
            return true;
        }

        fn print(key: &str, value: &str) -> bool {
            let message = format!(
                "{}: {}",
                theme_controller::get("./quantum.theme.json", "primary", &("- ".to_owned() + key)),
                theme_controller::get("./quantum.theme.json", "secondary", value)
            );

            println!("{}", message);

            return true;
        }

        fn get_memory_usage(system: &System) -> String {
            let total_memory_gb = system.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
            let used_memory_gb = (total_memory_gb - system.free_memory() as f64 / (1024.0 * 1024.0 * 1024.0)).round();
        
            let total_memory_gb_rounded = total_memory_gb.round();
            let used_memory_gb_rounded = used_memory_gb.round();
        
            format!("{:.0}GB/{:.0}GB", used_memory_gb_rounded, total_memory_gb_rounded)
        }

        fn get_uptime() -> String {
            let mut system = sysinfo::System::new_all();

            system.refresh_all();

            let uptime_seconds = system.uptime();

            let hours = uptime_seconds / 3600;
            let minutes = (uptime_seconds % 3600) / 60;
            let seconds = uptime_seconds % 60;

            return format!("{} hours {} minutes {} seconds", hours, minutes, seconds).to_string()
        }
    }

    pub mod utils {
        use crate::theme_controller;

        #[cfg(windows)]
        pub fn clear_console() {
            std::process::Command::new("cmd").arg("/c").arg("cls").status().unwrap();
        }

        #[cfg(unix)]
        pub fn clear_console() {
            std::process::Command::new("clear").status().unwrap();
        }

        #[cfg(not(any(windows, unix)))]
        pub fn clear_console() {
            // TODO: Add support for other platforms
        }

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
