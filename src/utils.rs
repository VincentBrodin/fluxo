use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

const APP_NAME: &str = "fluxo";
const CONFIG_NAME: &str = "config.json";

fn get_config_path() -> PathBuf {
    let mut home = env::home_dir().expect("Could not find your home directory");
    home = home.join(".config");
    home.join(APP_NAME)
}

pub fn open_or_create_config() -> io::Result<File> {
    let config_path = get_config_path();
    if !get_config_path().is_dir() {
        fs::create_dir(&config_path)?;
    }

    let config_file = config_path.join(CONFIG_NAME);

    match Path::new(&config_file).is_file() {
        true => File::open(config_file),
        false => {
            {
                // Create in a new scope to close before opening
                let mut file = File::create(&config_file)?;
                file.write_all(b"{}")?;
            }
            File::open(config_file)
        }
    }
}

pub fn run_hyprctl(cmd: &str) -> io::Result<ExitStatus> {
    println!("> hyprctl keyword {}", cmd);
    Command::new("hyprctl")
        .args(["keyword", "monitor"])
        .arg(cmd)
        .status()
}
