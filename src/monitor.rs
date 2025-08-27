use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub on_added: Vec<String>,
    pub on_removed: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Monitor {
    pub name: String,
}

const MONITOR_ADDED: &str = "monitoradded";
const MONITOR_REMOVED: &str = "monitorremoved";

pub enum Event {
    MonitorAdded(String),
    MonitorRemoved(String),
    Unkown,
}

pub fn get_monitors() -> Result<Vec<Monitor>, Box<dyn std::error::Error>> {
    let output = Command::new("hyprctl").args(["-j", "monitors"]).output()?;
    if !output.status.success() {
        return Err("hyprctl command failed".into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let monitors: Vec<Monitor> = serde_json::from_str(&stdout)?;

    Ok(monitors)
}

pub fn parse_event(event: &str) -> Event {
    let parts: Vec<&str> = event.split(">>").collect();
    if parts.len() < 1 {
        return Event::Unkown;
    }

    if parts[0] == MONITOR_ADDED {
        Event::MonitorAdded(parts[1].to_string())
    } else if parts[0] == MONITOR_REMOVED {
        Event::MonitorRemoved(parts[1].to_string())
    } else {
        Event::Unkown
    }
}
