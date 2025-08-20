// Standards ───────────────────────────────────────────────────────────────────────────
use std::{
    collections::HashMap,
    io::{self, Stdin},
};

// Crates ───────────────────────────────────────────────────────────────────────────────
use rayon::prelude::*;

// Structs ──────────────────────────────────────────────────────────────────────────────
mod structs;
use structs::{App, Theme};

// Helpers ──────────────────────────────────────────────────────────────────────────────
mod helpers;
use helpers::{
    load_config,
    set_apps_theme,
    set_folder_icon_color,
    set_wallpapers,
};

// Constants ────────────────────────────────────────────────────────────────────────────
const APPS_PATH: &str = "./config/apps.json";
const THEMES_PATH: &str = "./config/themes.json";

// Main ────────────────────────────────────────────────────────────────────────────
fn main() {
    let themes: HashMap<String, Theme> = load_config(THEMES_PATH);

    println!("✨ Write one of the themes below to apply:\n🎨 {:?}", &themes.keys());

    let stdin: Stdin = io::stdin();
    let mut input: String = String::new();
    let _ = stdin.read_line(&mut input);
    input = input.trim().to_owned(); // Removes the '\n' caused by the user pressing 'enter'

    let apps: Vec<App> = load_config(APPS_PATH);

    let theme: &Theme = themes.get(&input)
        .unwrap_or_else(|| panic!(
            "❌ Couldn't find [{}] among the available themes {:?}",
            input, themes.keys()
        ));

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
