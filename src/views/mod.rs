pub mod view_controller {
    pub mod header {
        use crate::theme::theme_controller;

        pub fn new(name: &str, use_colors: bool) -> String {
            return get(name, use_colors);
        }
    
        pub fn get(name: &str, use_color: bool) -> String {
            const AMD: &str = r#"
                                                 @@@@@@@@@@@@@@
    @@@@@@      @@@@@     @@@@   @@@@@@@@@@@       @@@@@@@@@@@@
    @@@@@@@     @@@@@@  @@@@@@   @@@     @@@@              @@@@
   @@@  @@@@    @@@@@@@@@@@@@@   @@@      @@@@    @@       @@@@
  @@@@   @@@    @@@@  @@@  @@@   @@@      @@@@   @@@       @@@@
 @@@@@@@@@@@@   @@@@       @@@   @@@     @@@@   @@@@       @@@@
 @@@       @@@  @@@@       @@@   @@@@@@@@@@@    @@@@@@@@@   @@@
                                                @@@@@@@       @

"#;

            const INTEL: &str = r#"

            ####                ----                  :@@@
            ####                %@@@                  :@@@
                                %@@@                  :@@@
            @@@+ :@@@ @@@@@@-   %@@@@@    -@@@@@@%    :@@@
            @@@+ :@@@@@%%@@@@@  %@@@@@  +@@@@%%@@@@@  :@@@
            @@@+ :@@@      @@@- %@@@   -@@@      @@@# :@@@
            @@@+ :@@@      @@@= %@@@   @@@@@@@@@@@@@@ :@@@
            @@@+ :@@@      @@@= %@@@   -@@@           :@@@
            @@@+ :@@@      @@@=  @@@@@  #@@@@@%@@@@+  :@@@
            @@@+ :@@@      @@@=   @@@@    +@@@@@@@    :@@@
            
            "#;

            
            match name {
                "amd" => {
                    if use_color {
                        return theme_controller::get("./quantum.theme.json", "primary", AMD);
                    } else {
                        return AMD.to_string()
                    }
                },
                "intel" => {
                    if use_color {
                        return  theme_controller::get("./quantum.theme.json", "primary", INTEL);
                    } else {
                        return INTEL.to_string()
                    }
                }
                _ => return "error".to_string()
            }
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
                cpu = "Amd".to_string()
            } else if cpu == "intel" {
                cpu = "Intel".to_string()
            }

            let memory_message = get_memory_usage(&system);

            if use_colors {
                print("Os", &system.name().unwrap(), true);
                print("Os Version", &system.long_os_version().unwrap(), true);
                print("Host", &system.host_name().unwrap(), true);
                print("Kernel", &system.kernel_version().unwrap(), true);
                print("Uptime", &get_uptime(), true);
                print("CPU", &cpu, true);
                print("CPU Core Count", &system.physical_core_count().unwrap().to_string(), true);
                print("Theme", &theme_controller::new("./quantum.theme.json").unwrap()["name"].to_string(), true);
                print("Memory", &memory_message, true);
            } else {
                print("Os", &system.name().unwrap(), false);
                print("Os Version", &system.long_os_version().unwrap(), false);
                print("Host", &system.host_name().unwrap(), false);
                print("Kernel", &system.kernel_version().unwrap(), false);
                print("Uptime", &get_uptime(), false);
                print("CPU", &cpu, false);
                print("CPU Core Count", &system.physical_core_count().unwrap().to_string(), false);
                print("Theme", &theme_controller::new("./quantum.theme.json").unwrap()["name"].to_string(), false);
                print("Memory", &memory_message, false);
            }
            return true;
        }

        pub fn run_advanced(use_colors: bool) -> bool {
                let mut system = sysinfo::System::new_all();
                system.refresh_all();
            
                let cpu_vendor = cpu::cpu::get_cpu_vendor().unwrap();
            
                let cpu_info = match cpu_vendor.as_str() {
                    "amd" => "AMD Ryzen",
                    "intel" => "Intel",
                    _ => "Unknown",
                };
            
                let memory_message = get_memory_usage(&system);
            
                if use_colors {
                    print("OS", &system.name().unwrap(), true);
                    print("Os Version", &system.long_os_version().unwrap(), true);
                    print("Host", &system.host_name().unwrap(), true);
                    print("Kernel", &system.kernel_version().unwrap(), true);
                    print("Uptime", &get_uptime(), true);
                    print("CPU", &cpu_info, true);
                    print("CPU Core Count", &system.physical_core_count().unwrap().to_string(), true);
                    print("Theme", &theme_controller::new("./quantum.theme.json").unwrap()["name"].to_string(), true);
                    print("Memory", &memory_message, true);
                    print("Boot Time (as ms)", &system.boot_time().to_string(), true);
                    print("Components", &system.components().len().to_string(), true);
                    print("Disks", &system.disks().len().to_string(), true);
                    print("Free Swap", &system.free_swap().to_string(), true);
                    print("Cpu Info", &format!("{:#?}", system.global_cpu_info()), true);
                    print("Networks", &format!("{:#?}", system.networks()), true);
                } else {
                    print("OS", &system.name().unwrap(), false);
                    print("Os Version", &system.long_os_version().unwrap(), false);
                    print("Host", &system.host_name().unwrap(), false);
                    print("Kernel", &system.kernel_version().unwrap(), false);
                    print("Uptime", &get_uptime(), false);
                    print("CPU", &cpu_info, false);
                    print("CPU Core Count", &system.physical_core_count().unwrap().to_string(), false);
                    print("Theme", &theme_controller::new("./quantum.theme.json").unwrap()["name"].to_string(), false);
                    print("Memory", &memory_message, false);
                    print("Boot Time (as ms)", &system.boot_time().to_string(), false);
                    print("Components", &system.components().len().to_string(), false);
                    print("Disks", &system.disks().len().to_string(), false);
                    print("Free Swap", &system.free_swap().to_string(), false);
                    print("Cpu Info", &format!("{:#?}", system.global_cpu_info()), false);
                    print("Networks", &format!("{:#?}", system.networks()), false);
                }
                return true;
        }

        fn print(key: &str, value: &str, use_colors: bool) -> bool {
            if use_colors {
                let message = format!(
                    "{}: {}",
                    theme_controller::get("./quantum.theme.json", "primary", &("- ".to_owned() + key)),
                    theme_controller::get("./quantum.theme.json", "secondary", value)
                );
    
                println!("{}", message);
    
                return true;
            } else {
                let message = format!("{}: {}", &("- ".to_owned() + key), value);
                println!("{}", message);

                return true;
            }

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
