use serde::Deserialize;
use std::fs;
use std::collections::HashMap;
use serde::de::DeserializeOwned;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub directories: Directories,
    pub wallpaper: Wallpaper,
    pub colorscheme: Vec<String>,
}
#[derive(Deserialize, Debug)]
pub struct Directories {
    pub gif: String,
    pub dark: String,
    pub light: String,
}

#[derive(Deserialize, Debug)]
pub struct Wallpaper {
    pub fps: String,
    #[serde(rename = "resize-mode")]
    pub resize_mode: String,
    pub filter: String,
    pub transitions: Vec<String>,
    pub positions: Vec<String>,
    pub duration: String,
    pub steps: String,
}

#[derive(Deserialize, Debug)]
pub struct MonitorConfig {
    pub monitors: HashMap<String, Monitor>,
}

#[derive(Deserialize, Debug)]
pub struct Monitor {
    pub name: String,
    pub id: u32,
    pub height: u32,
    pub width: u32,
    #[serde(rename = "refresh-rate")]
    pub refresh_rate: f32,
}

pub fn load_config<T: DeserializeOwned>(file_path: &str) -> T {
    let data = fs::read_to_string(file_path).expect(&format!("File not found: {}", file_path));
serde_json::from_str(&data).expect(&format!("Error parsing JSON in: {}", file_path))
}
