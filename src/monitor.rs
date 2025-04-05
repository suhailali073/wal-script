use std::process::Command;
use serde::Deserialize;
use std::collections::HashMap;


#[derive(Deserialize, Debug)]
pub struct MonitorConfig {
    pub monitors: Option<HashMap<u32, Monitor>>,
}

#[derive(Debug, Deserialize)]
pub struct Monitor {
    pub name: String,
    pub width: usize,
    pub height: usize,
    #[serde(rename = "refreshRate")]
    pub refresh_rate: f32,
}

pub fn load_config() -> Result<Vec<Monitor>, String> {
    let output = Command::new("hyprctl")
        .arg("monitors")
        .arg("-j")  // Ensure JSON output
        .output()
        .map_err(|e| format!("Failed to execute hyprctl: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "hyprctl returned an error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let data = String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8: {}", e))?;
    let monitors: Vec<Monitor> =
        serde_json::from_str(&data).map_err(|e| format!("Error parsing JSON: {}", e))?;
    Ok(monitors)
}
