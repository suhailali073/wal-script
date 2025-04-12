use crate::socket;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::PathBuf,
};

#[derive(Clone, Deserialize, Debug)]
pub struct Monitor {
    pub id: i8,
    pub name: String,
    pub width: i32,
    pub height: i32,
    #[serde(rename = "refreshRate")]
    pub refresh_rate: f32,
}

pub fn query<T: DeserializeOwned>(socket: &PathBuf, endpoint: &str) -> T {
    let mut stream = UnixStream::connect(socket).expect("Failed to connect to Hyprland socket");

    stream
        .write_all(endpoint.as_bytes())
        .expect("Failed to write to Hyprland socket");

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("Failed to read from Hyprland socket");
    serde_json::from_str(&response).expect("Failed to parse json")
}

pub fn dispatch(command: &str) {
    let (socket, _) = socket::get_socket_path();
    let mut stream = UnixStream::connect(&socket).expect("Failed to connect to Hyprland socket");

    let dispatch_command = format!("dispatch {}", command);
    stream
        .write_all(dispatch_command.as_bytes())
        .expect("Failed to write dispatch command");

    // Optionally read and discard the response (if any)
    let mut _dummy = String::new();
    let _ = stream.read_to_string(&mut _dummy);
}
