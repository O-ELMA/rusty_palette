// Standards ───────────────────────────────────────────────────────────────────────────
use std::collections::HashMap;

// Crates ───────────────────────────────────────────────────────────────────────────────
use rayon::prelude::*;

// Structs ──────────────────────────────────────────────────────────────────────────────
mod structs;
use structs::{App, CompiledRegex, Theme};

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

    // Pre-compile regexes for all apps
    let compiled_regexes: Vec<Option<Vec<CompiledRegex>>> =
        apps.par_iter().map(|app| app.compile_regexes()).collect();

    // 1- Set Wallpapers
    set_wallpapers(&theme.wallpapers[..]);

    apps.par_iter()
        .zip(compiled_regexes.par_iter())
        .for_each(|(app, compiled)| {
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
        });
}
