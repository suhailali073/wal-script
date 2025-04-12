mod colorscheme;
mod config;
mod get_var;
mod socket;
mod swww;
mod wallpaper;

use dirs;
use std::env;

fn help() {
    println!("Usage: wal-script [option]");
    println!(" -h, --help               Show this help messege");
    println!(" -w, --wallpaper <path>   Set a specific wallpaper");
    println!(" -d, --delete             Delete current wallpaper");
}

fn expand_tilde(path: &String) -> String {
    if path.starts_with("~") {
        if let Some(home_dir) = dirs::home_dir() {
            return path.replacen("~", home_dir.to_str().unwrap(), 1);
        }
    }
    path.to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let data: config::Config =
        config::load_config("/home/suhailali073/.config/wal-script/config.jsonc");

    let (socket, _) = socket::get_socket_path();
    let monitor_data: Vec<get_var::Monitor> = get_var::query(&socket, "j/monitors");

    let gif = data.directories.gif;
    let light = data.directories.light;
    let dark = data.directories.dark;
    let (directory, index) = wallpaper::directory(gif, light, dark);
    println!("The subkey value is: {}[{}]", directory, index);

    let transitions = data.wallpaper.transitions;
    let transition = wallpaper::transition(transitions);
    println!("The transition value is: {}", transition);

    let wallpaper;
    // Random wallpaper mode
    if args.len() < 2 {
        wallpaper = wallpaper::get_random_image(&directory).expect("No images found in directory");
        colorscheme::apply_colorscheme(&wallpaper, &data.colorscheme, index);

        swww::apply_wallpaper(
            wallpaper,
            data.wallpaper.filter.clone(),
            transition.clone(),
            data.wallpaper.positions.clone(),
            data.wallpaper.steps.clone(),
            data.wallpaper.duration.clone(),
            data.wallpaper.resize_mode.clone(),
            monitor_data.clone(),
        );

        swww::reload_env();
        return;
    }

    let subcommand = &args[1];

    // Subcommand handling
    match subcommand.as_str() {
        "-h" | "--help" => help(),
        "-w" | "--wallpaper" => {
            if args.len() < 3 || args[2].is_empty() {
                help()
            } else {
                let raw_path = args.get(2).unwrap();
                wallpaper = expand_tilde(raw_path);
                colorscheme::apply_colorscheme(&wallpaper, &data.colorscheme, index);
                swww::apply_wallpaper(
                    wallpaper,
                    data.wallpaper.filter,
                    transition,
                    data.wallpaper.positions,
                    data.wallpaper.steps,
                    data.wallpaper.duration,
                    data.wallpaper.resize_mode,
                    monitor_data,
                );
                swww::reload_env();
            }
        }
        "-d" | "--delete" => {
            // implement deletion logic here
            println!("Delete mode not implemented yet.");
        }
        _ => eprintln!("Not recognised"),
    }
}
