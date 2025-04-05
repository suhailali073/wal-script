use rand::Rng;
use std::fs;
use rand::rng;
use rand::prelude::IndexedRandom;
use std::process::Command;
use crate::config::MonitorConfig;
use std::thread;
use std::sync::Arc;

pub fn directory(gif: String, dark: String, light: String) -> (String,i8) {
    let rand = rng().random_range(0..3);
    let path = match rand {
        0 => gif,
        1 => light,
        _ => dark,
    };
    (path, rand)
}

pub fn transition(transitions: Vec<String>) -> String {
    let transition_rand = rng().random_range(0..transitions.len());
    let transition = &transitions[transition_rand];
    transition.to_string()
}

pub fn get_random_image(directory: &str) -> Option<String> {

    // ✅ Read directory and collect image files
    let image_files: Vec<String> = fs::read_dir(directory)
        .expect("Failed to read directory") // ✅ Read all entries
        .filter_map(|entry| entry.ok()) // ✅ Remove errors
        .map(|entry| entry.path()) // ✅ Convert to `PathBuf`
        .filter(|path| {
            path.is_file() && matches!(path.extension().and_then(|ext| ext.to_str()), 
                Some("png") | Some("jpg") | Some("jpeg") | Some("gif")) // ✅ Keep only image files
        })
        .map(|path| path.to_string_lossy().into_owned()) // ✅ Convert PathBuf to String
        .collect();

    // ✅ Pick a random image and return as `Option<String>`
    image_files.choose(&mut rng()).cloned()
}

pub fn apply_wallpaper(
    wallpaper: String,
    filter: String,
    transition: String,
    position: Vec<String>,
    steps: String,
    duration: String,
    resize: String,
    monitor: &MonitorConfig
) {
    // Initialize height and width ratios based on the first monitor
    let rand_height = rng().random_range(0..monitor.monitors["0"].height);
    let ratio_height = 1.0 - (rand_height as f32 / monitor.monitors["0"].height as f32);
    let rand_width = rng().random_range(0..monitor.monitors["0"].width);
    let ratio_width = 1.0 - (rand_width as f32 / monitor.monitors["0"].width as f32);
    
    // Create shared data for threads
    let wallpaper = Arc::new(wallpaper);
    let filter = Arc::new(filter);
    let transition = Arc::new(transition);
    let position = Arc::new(position);
    let steps = Arc::new(steps);
    let duration = Arc::new(duration);
    let resize = Arc::new(resize);
    
    // Collect handles for all threads
    let mut handles = vec![];
    
    // Process each monitor in a separate thread
    for (_, monitors) in &monitor.monitors {
        // Clone the Arc references for this thread
        let wallpaper = Arc::clone(&wallpaper);
        let filter = Arc::clone(&filter);
        let transition = Arc::clone(&transition);
        let position = Arc::clone(&position);
        let steps = Arc::clone(&steps);
        let duration = Arc::clone(&duration);
        let resize = Arc::clone(&resize);
        
        // Clone the monitor-specific data
        let monitor_width = monitors.width;
        let monitor_height = monitors.height;
        let monitor_name = monitors.name.clone();
        let refresh_rate = monitors.refresh_rate;
        
        // Capture the ratio values
        let thread_ratio_width = ratio_width;
        let thread_ratio_height = ratio_height;
        
        // Spawn a new thread for this monitor
        let handle = thread::spawn(move || {
           
            let mut command = format!(
                "swww img \"{}\" --filter {} --resize {} --transition-type {} --transition-step {} \
                --transition-duration {} --transition-fps {} ",
                wallpaper, filter, resize, transition, steps, duration, refresh_rate as i32
            );
            
            if *transition == "grow" || *transition == "outer" {
                let pos_rand = rng().random_range(0..position.len());
                let pos = &position[pos_rand];
                let pos_str = if pos == "random" {
                    let pos_x: i32 = (monitor_width as f32 * (1.0 - thread_ratio_width)) as i32;
                    let pos_y: i32 = (monitor_height as f32 * (1.0 - thread_ratio_height)) as i32;
                    format!("{},{} -o {}", pos_x, pos_y, monitor_name)
                } else {
                    pos.clone()
                };
                
                println!("position: {}", &pos_str);
                command.push_str(&format!(" --transition-pos {}", pos_str));
            } 
            
            if *transition == "wipe" || *transition == "wave" {
                let angle = rng().random_range(0..360).to_string();
                println!("angle: {}", &angle);
                command.push_str(&format!(" --transition-angle {}", angle));
            }
            
            println!("Generated Command:\n{}", &command);
            let swww = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("Failed to execute command");
            
            if !swww.status.success() {
                eprintln!("Failed to apply wallpaper: {}", String::from_utf8_lossy(&swww.stderr));
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}
