// Standards ───────────────────────────────────────────────────────────────────────────
use std::collections::HashMap;

// Crates ───────────────────────────────────────────────────────────────────────────────
use rayon::prelude::*;

// Structs ──────────────────────────────────────────────────────────────────────────────
mod structs;
use structs::{App, Theme};

// Helpers ──────────────────────────────────────────────────────────────────────────────
mod helpers;
use helpers::{parse_config, prompt_user, set_apps_theme, set_folder_icon_color, set_wallpapers};

// Constants ────────────────────────────────────────────────────────────────────────────
const APPS_JSON: &str = include_str!("../config/apps.json");
const THEMES_JSON: &str = include_str!("../config/themes.json");

// Main ────────────────────────────────────────────────────────────────────────────
fn main() {
    let themes: HashMap<String, Theme> = parse_config(THEMES_JSON);
    let theme = prompt_user(&themes);

    let apps: Vec<App> = parse_config(APPS_JSON);

    // 1- Set Wallpapers
    set_wallpapers(&theme.wallpapers);

    apps.par_iter().for_each(|app| {
        // 2- Change each app's theme
        if app.config.command.is_none() {
            set_apps_theme(theme.name.clone(), app);
        }

        // 3- Change Papirus Folders icons' color
        else if app.name == "folder_icon" {
            set_folder_icon_color(&theme.name, app);
        }

        else {
            panic!(
                "❌ The App [{}] must be a new app that has the 'command' key but not named 'folder_icon'. You should handle it.",
                app.name
            );
        }
    });
}
