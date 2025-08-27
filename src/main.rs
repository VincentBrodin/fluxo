use std::{
    collections::HashMap,
    env,
    io::{BufRead, BufReader},
    os::unix::net::UnixStream,
};

mod monitor;
mod utils;

fn main() {
    let config_file = utils::open_or_create_config()
        .unwrap_or_else(|err| panic!("Could create or open the config file: {}", err));
    let configs: HashMap<String, monitor::Config> = serde_json::from_reader(config_file)
        .unwrap_or_else(|err| panic!("Config file is not valid: {}", err));

    let monitors = monitor::get_monitors()
        .unwrap_or_else(|err| panic!("Could not get active monitors: {}", err));
    for (monitor, config) in configs.iter() {
        let cmds = match monitors.iter().any(|m| &m.name == monitor) {
            true => &config.on_added,
            false => &config.on_removed,
        };
        utils::run_cmds(cmds);
    }

    let runtime_dir = env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|err| panic!("XDG_RUNTIME_DIR is not set: {}", err));
    let instance = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .unwrap_or_else(|err| panic!("HYPRLAND_INSTANCE_SIGNATURE is not set: {}", err));

    let socket_path = format!("{}/hypr/{}/.socket2.sock", runtime_dir, instance);
    let stream = UnixStream::connect(socket_path)
        .unwrap_or_else(|err| panic!("Could not connect to stream: {}", err));

    let reader = BufReader::new(stream);

    // This reads all inputs from hyprland
    for line in reader.lines() {
        let line = match line {
            Ok(v) => v,
            Err(err) => {
                eprintln!("Error reading stream: {}", err);
                continue;
            }
        };
        let cmds = match monitor::parse_event(&line) {
            monitor::Event::MonitorAdded(name) => match configs.get(&name) {
                Some(config) => &config.on_added,
                None => {
                    println!("No config for: {}", name);
                    continue;
                }
            },
            monitor::Event::MonitorRemoved(name) => match configs.get(&name) {
                Some(config) => &config.on_removed,
                None => {
                    println!("No config for: {}", name);
                    continue;
                }
            },
            monitor::Event::Unkown => continue,
        };

        utils::run_cmds(cmds);
    }
}
