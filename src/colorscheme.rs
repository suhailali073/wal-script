use std::process::Command;

pub fn apply_colorscheme(wallpaper: &String, colorschemes: &Vec<String>, index: i8) {
    let colorscheme_flag = if colorschemes.len() == 1 {
        match colorschemes[0].as_str() {
            "dark" => "-d",
            "light" => "-l",
            _ => "-d",
        }
    } else {
        match index {
            1 => "-d",
            2 => "-l",
            _ => "-d",
        }
    };

    let command = format!(
        "hellwal -i \"{}\" {} && \
         cp ~/.cache/hellwal/colors.json ~/.cache/wal/ && \
         pywalfox update",
        wallpaper, colorscheme_flag
    );

    println!("Generated command: {}", command);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!(
            "Command failed: {:?}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
