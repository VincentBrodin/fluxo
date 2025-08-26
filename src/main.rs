use std::{collections::HashMap, env, io::{BufRead, BufReader}, os::unix::net::UnixStream};

mod monitor;
mod utils;

fn main() {
    let config_file = utils::open_or_create_config().expect("Could not get config file");
    let monitors: HashMap<String, monitor::Config> =
        serde_json::from_reader(config_file).expect("Config file is not valid");

    let runtime_dir = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR is not set");
    let instance =env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("HYPRLAND_INSTANCE_SIGNATURE is not set");
    let socket_path = format!("{}/hypr/{}/.socket2.sock", runtime_dir, instance);

    let stream = UnixStream::connect(socket_path).expect("Could not connect to stream");
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
            monitor::Event::MonitorAdded(name) => match monitors.get(&name) {
                Some(config) => &config.on_added,
                None => continue,
            },
            monitor::Event::MonitorRemoved(name) => match monitors.get(&name) {
                Some(config) => &config.on_removed,
                None => continue,
            },
            monitor::Event::Unkown => continue,
        };

        for cmd in cmds {
            match utils::run_hyprctl(cmd) {
                Ok(exit_status) => println!("{}", exit_status.to_string()),
                Err(err) => eprintln!("Failed: {}", err),
            };
        }
    }
}
