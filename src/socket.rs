use std::{env, path::PathBuf};

pub fn get_socket_path() -> (PathBuf, PathBuf) {
    let runtime_dir = env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/run/user/1000".to_string());
    let sig = env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("is Hyprland running");

    let mut socket = PathBuf::from(runtime_dir.clone());
    socket.push("hypr");
    socket.push(sig.clone());
    socket.push(".socket.sock");

    if !socket.exists() {
        eprintln!("Socket not found at {:?}", socket);
        std::process::exit(1);
    }

    let mut socket2 = PathBuf::from(runtime_dir.clone());
    socket2.push("hypr");
    socket2.push(sig.clone());
    socket2.push(".socket2.sock");

    if !socket2.exists() {
        eprintln!("Socket not found at {:?}", socket2);
        std::process::exit(1);
    }
    (socket, socket2)
}
