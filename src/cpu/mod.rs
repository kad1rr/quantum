pub mod cpu {
    #[cfg(target_os = "linux")]
    pub fn get_cpu_vendor() -> Option<String> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
    
        if let Ok(file) = File::open("/proc/cpuinfo") {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.starts_with("vendor_id") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if let Some(vendor) = parts.get(2) {
                            return Some(normalize_vendor(vendor));
                        }
                    }
                }
            }
        }
    
        None
    }
    
    #[cfg(target_os = "windows")]
    pub fn get_cpu_vendor() -> Option<String> {
        use std::ptr;
        use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
    
        unsafe {
            let mut system_info: SYSTEM_INFO = std::mem::zeroed();
            GetSystemInfo(&mut system_info);
    
            match system_info.wProcessorArchitecture {
                0 => Some("x86".to_string()),
                6 => Some("IA-64".to_string()),
                9 => Some("x86_64".to_string()),
                _ => None,
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    pub fn get_cpu_vendor() -> Option<String> {
        use sysctl::{Sysctl, SysctlError};
    
        fn get_sysctl_value(name: &str) -> Result<String, SysctlError> {
            sysctl::value(name)
        }
    
        if let Ok(value) = get_sysctl_value("machdep.cpu.vendor") {
            return Some(normalize_vendor(&value));
        }
    
        None
    }

    fn normalize_vendor(vendor: &str) -> String {
        let lower_vendor = vendor.to_lowercase();
        if lower_vendor.contains("amd") {
            return "amd".to_string();
        } else if lower_vendor.contains("intel") {
            return "intel".to_string();
        }

        return "quantum".to_string();
    }
}
