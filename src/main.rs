mod config;
mod wallpaper;
use crate::config::MonitorConfig;

fn main() {
    let data: config::Config = config::load_config("/home/suhailali073/.config/wal-script/config.jsonc");
    
    // Try to load monitor data, but don't panic if it fails
    let monitor_data: config::MonitorConfig = match config::load_config::<config::MonitorConfig>("/home/suhailali073/.cache/monitors.json") {
        data => data,
    };
    
    let resize = data.wallpaper.resize_mode;
    let filter = data.wallpaper.filter;
    let transitions = data.wallpaper.transitions;
    let positions = data.wallpaper.positions;
    let duration = data.wallpaper.duration;
    let steps = data.wallpaper.steps;

    let gif = data.directories.gif;
    let light = data.directories.light;
    let dark = data.directories.dark;

    let (directory, index) = wallpaper::directory(gif, light, dark);
    println!("The subkey value is: {}[{}]", directory, index);
    
    let transition = wallpaper::transition(transitions);
    println!("The transition value is: {}", transition);

    let wallpaper = wallpaper::get_random_image(&directory).expect("No images found in directory");
    
    wallpaper::apply_wallpaper(wallpaper, filter, transition, positions, steps, duration, resize, &monitor_data);
}
