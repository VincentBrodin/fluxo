use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub on_added: Vec<String>,
    pub on_removed: Vec<String>,
}

const MONITOR_ADDED: &str = "monitoradded";
const MONITOR_REMOVED: &str = "monitorremoved";

pub enum Event {
    MonitorAdded(String),
    MonitorRemoved(String),
    Unkown,
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
