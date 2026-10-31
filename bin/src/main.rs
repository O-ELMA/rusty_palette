// Standards ───────────────────────────────────────────────────────────────────────────
use std::collections::HashMap;

// Structs ──────────────────────────────────────────────────────────────────────────────
mod structs;
use structs::{AppsConfig, CompiledRegex, Theme};

// Helpers ──────────────────────────────────────────────────────────────────────────────
mod helpers;
use helpers::{parse_config, prompt_user, set_apps_theme, set_folder_icon_color, set_wallpapers};

// Constants ────────────────────────────────────────────────────────────────────────────
const APPS_TOML: &str = include_str!("../config/apps.toml");
const THEMES_TOML: &str = include_str!("../config/themes.toml");

// Main ────────────────────────────────────────────────────────────────────────────
fn main() {
    let themes: HashMap<String, Theme> = parse_config(THEMES_TOML);
    let theme = prompt_user(&themes);

    let apps: AppsConfig = parse_config(APPS_TOML);

    // Pre-compile regexes for all apps
    let compiled_regexes: Vec<Option<Vec<CompiledRegex>>> =
        apps.apps.iter().map(|app| app.compile_regexes()).collect();

    // 1- Set Wallpapers
    set_wallpapers(&theme.wallpapers[..]);

    for (app, compiled) in apps.apps.iter().zip(compiled_regexes.iter()) {
        if app.name == "folder_icon" {
            // 3- Change Papirus Folders icons' color
            set_folder_icon_color(&theme.name, app);
        } else {
            // 2- Change each app's theme
            set_apps_theme(&theme.name, app, compiled.as_ref());
        }
    }
}
