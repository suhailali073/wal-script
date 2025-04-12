use rand::Rng;
use std::fs;
use rand::rng;
use rand::prelude::IndexedRandom;

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

