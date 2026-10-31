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
        // 2- Change each app's theme
        if app.config.command.is_none() {
            set_apps_theme(&theme.name, app, compiled.as_ref());
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
    }
}
