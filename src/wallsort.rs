use std::fs;
use std::path::Path;
use std::process::Command;

pub fn wallsort(
    wallpaper_dir: &str,
    light_dir: &str,
    dark_dir: &str,
    threshold: f64, // e.g., 0.5
) {
    let entries = match fs::read_dir(wallpaper_dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to read directory {}: {}", wallpaper_dir, e);
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        // Use `file --mime-type` to check if it's an image
        let output = Command::new("file")
            .arg("--mime-type")
            .arg("-b")
            .arg(&path)
            .output();

        if let Ok(out) = output {
            let mime = String::from_utf8_lossy(&out.stdout);
            if !mime.starts_with("image/") {
                continue;
            }
        } else {
            continue;
        }

        // Use `magick` to get brightness (mean gray value)
        let brightness_cmd = Command::new("magick")
            .arg(&path)
            .arg("-colorspace")
            .arg("Gray")
            .arg("-format")
            .arg("%[fx:mean]")
            .arg("info:")
            .output();

        let brightness = match brightness_cmd {
            Ok(output) if output.status.success() => {
                let val = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .parse::<f64>();
                match val {
                    Ok(b) => b,
                    Err(_) => {
                        eprintln!("Failed to parse brightness for {:?}", path);
                        continue;
                    }
                }
            }
            _ => {
                eprintln!("Failed to get brightness for {:?}", path);
                continue;
            }
        };

        // Move image based on brightness
        let dest_dir = if brightness > threshold {
            light_dir
        } else {
            dark_dir
        };

        let file_name = match path.file_name() {
            Some(name) => name,
            None => continue,
        };

        let dest_path = Path::new(dest_dir).join(file_name);
        if let Err(e) = fs::rename(&path, &dest_path) {
            eprintln!("Failed to move {:?} to {:?}: {}", path, dest_path, e);
        }
    }
}
