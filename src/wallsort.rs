use std::{fs, path::Path, env, process::exit};
use image::{imageops::colorops, io::Reader as ImageReader};
use std::fs::create_dir_all;

fn compute_mean_luminance(image_path: &Path) -> f64 {
    let img = ImageReader::open(image_path).unwrap().decode().unwrap();
    let gray = colorops::grayscale(&img);
    
    let (width, height) = gray.dimensions();
    let total_pixels = (width * height) as f64;
    let sum: u64 = gray.pixels().map(|p| p.0[0] as u64).sum();
    
    sum as f64 / total_pixels / 255.0 // Normalize to [0,1]
}

fn classify_and_move_images(wallpapers_dir: &Path, light_dir: &Path, dark_dir: &Path, threshold: f64) {
    create_dir_all(light_dir).ok();
    create_dir_all(dark_dir).ok();

    for entry in fs::read_dir(wallpapers_dir).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    if ["jpg", "jpeg", "png", "bmp", "gif", "tiff"].contains(&ext.as_str()) {
                        let brightness = compute_mean_luminance(&path);
                        let dest = if brightness > threshold { light_dir } else { dark_dir };
                        let dest_path = dest.join(path.file_name().unwrap());

                        fs::rename(&path, &dest_path).unwrap();
                        println!("Moved {:?} -> {:?}", path, dest_path);
                    }
                }
            }
        }
    }
}
