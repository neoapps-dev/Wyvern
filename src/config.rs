use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub terminal: String,
    pub keyboard_shortcuts: HashMap<String, String>,
    pub startup_apps: Vec<String>,
    pub display: Display,
}

#[derive(Debug, Deserialize)]
pub struct Display {
    pub scale: u32
}

impl Config {
    pub fn load() -> Self {
        let path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("wyvern/config.toml");

        let content = fs::read_to_string(&path).unwrap_or_else(|_| {
            eprintln!("Using default config; failed to read {}", path.display());
            String::new()
        });

        toml::from_str(&content).unwrap_or_else(|_| {
            eprintln!("Failed to parse config.toml, using defaults");
            Config::default()
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut keyboard_shortcuts = HashMap::new();
        keyboard_shortcuts.insert("super+return".into(), "weston-terminal".into());
        keyboard_shortcuts.insert("super+f4".into(), "killall weston-terminal".into()); // TODO: make it work
        keyboard_shortcuts.insert("super+q".into(), "quit".into());

        Self {
            terminal: "weston-terminal".into(),
            keyboard_shortcuts,
            startup_apps: vec!["waybar".into()],
            display: Display{ scale: 1 }
        }
    }
}
