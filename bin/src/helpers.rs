// Standards ───────────────────────────────────────────────────────────────────────────
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Stdin},
    os::unix::fs::symlink,
    path::PathBuf,
    process::Command,
};

// Crates ───────────────────────────────────────────────────────────────────────────────
use rand::{rng, seq::IndexedRandom};
use serde::Deserialize;

// Structs ──────────────────────────────────────────────────────────────────────────────
use crate::structs::{App, CompiledRegex, Theme, ThemeMappingType};

// Other ───────────────────────────────────────────────────────────────────────────────
pub fn parse_config<T>(config_str: &str) -> T
where
    T: for<'a> Deserialize<'a>,
{
    match toml::from_str(config_str) {
        Err(error) => panic!("❌ Could not parse embedded config: [{}]", error),
        Ok(config) => config,
    }
}

pub fn prompt_user(themes: &HashMap<String, Theme>) -> &Theme {
    let args: Vec<String> = env::args().skip(1).collect();

    // Called with allowrgs
    if !args.is_empty() {
        let selected = args.join(" ");
        let selected = selected.trim();
        themes.get(selected).unwrap_or_else(|| {
            panic!(
                "❌ Couldn't find [{}] among the available themes {:?}",
                selected,
                themes.keys()
            )
        })
    }
    // Called without args
    else {
        let theme_names: Vec<&String> = themes.keys().collect();
        println!("✨ Choose a theme to apply by typing its number:");
        for (i, name) in theme_names.iter().enumerate() {
            println!("{}. {}", i + 1, name);
        }
        println!("\n(Press Enter without typing to pick randomly)");

        let stdin: Stdin = io::stdin();
        let mut input: String = String::new();
        let _ = stdin.read_line(&mut input);
        let input = input.trim(); // Removes the '\n' caused by the user pressing 'enter'

        let selected = if input.is_empty() {
            let mut rng = rng();
            theme_names.choose(&mut rng).unwrap()
        } else {
            let index: usize = input.parse().expect("❌ Please enter a valid number.");
            if index == 0 || index > theme_names.len() {
                panic!("❌ Number should be between 1 and {}.", theme_names.len());
            }
            theme_names[index - 1]
        };

        themes.get(selected).unwrap()
    }
}

pub fn set_apps_theme(theme_name: &str, app: &App, compiled_regexes: Option<&Vec<CompiledRegex>>) {
    let user = env::var("USER")
        .unwrap_or_else(|error| panic!("❌ USER env variable not set: [{}]", error));

    let paths = app
        .config
        .paths
        .as_ref()
        .unwrap_or_else(|| panic!("❌ Couldn't find [paths] in the app [{}]", app.name));

    let regexes = compiled_regexes.as_ref().unwrap_or_else(|| {
        panic!(
            "❌ Couldn't find compiled regexes for the app [{}]",
            app.name
        )
    });

    let targets: Vec<String> = match &app.config.theme_mapping {
        Some(theme_mapping) => match theme_mapping.get(theme_name) {
            Some(ThemeMappingType::Arr(arr)) => arr.to_owned(),
            Some(ThemeMappingType::Str(str)) => vec![str.clone(); regexes.len()],
            _ => panic!(
                "❌ [{}]'s theme_mapping needs to be in a String-to-Array format.",
                app.name
            ),
        },
        None => {
            let mut targets = Vec::with_capacity(regexes.len());
            targets.resize(regexes.len(), theme_name.to_string());
            targets
        }
    };

    for path in paths {
        let clean_path = path.replace("$USER", &user);

        let mut content = fs::read_to_string(&clean_path).unwrap_or_else(|error| {
            panic!(
                "❌ Couldn't read the file [{}] because [{}]",
                clean_path, error
            )
        });

        for (i, compiled_regex) in regexes.iter().enumerate() {
            let target = compiled_regex
                .target
                .replace(&compiled_regex.key, &targets[i]);
            content = compiled_regex.regex.replace_all(&content, target.as_str()).into_owned();
        }

        fs::write(&clean_path, content).unwrap_or_else(|error| {
            panic!(
                "❌ Couldn't write to the file [{}] because [{}]",
                clean_path, error
            )
        });
    }
}

pub fn set_folder_icon_color(theme_name: &str, app: &App) {
    let mapped_theme: String = app
        .config
        .theme_mapping
        .as_ref()
        .unwrap()
        .get(theme_name)
        .map(|res| match res {
            ThemeMappingType::Str(s) => s.to_owned(),
            _ => panic!("❌ folder_icon's theme_mapping needs to be in a String-to-String format."),
        })
        .unwrap_or_else(|| {
            panic!(
                "❌ Couldn't find [{}] among the theme_mapping in the app [{}]",
                theme_name, app.name
            )
        });

    let icon_theme = app
        .config
        .icon_theme
        .as_ref()
        .unwrap_or_else(|| panic!("❌ Couldn't find [icon_theme] in the app [{}]", app.name));

    let data_home = env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(
                env::var("HOME")
                    .unwrap_or_else(|error| panic!("❌ HOME env variable not set: [{}]", error)),
            )
            .join(".local/share")
        });

    let theme_dir = data_home.join("icons").join(icon_theme);
    if !theme_dir.is_dir() {
        panic!(
            "❌ Couldn't find the user icon theme [{}]. Copy it into [{}] first.",
            icon_theme,
            theme_dir.display()
        );
    }

    let markers = [
        ("folder", format!("folder-{}", mapped_theme)),
        ("user", format!("user-{}", mapped_theme)),
    ];

    let mut applied = 0usize;

    for size in ["22x22", "24x24", "32x32", "48x48", "64x64"] {
        let places = theme_dir.join(size).join("places");
        let Ok(entries) = fs::read_dir(&places) else {
            continue;
        };

        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let Some(name) = file_name.to_str() else {
                continue;
            };

            for (prefix, marker) in &markers {
                let Some(rest) = name.strip_prefix(marker.as_str()) else {
                    continue;
                };
                if !rest.starts_with('.') && !rest.starts_with('-') {
                    continue;
                }

                let target_path = places.join(format!("{prefix}{rest}"));
                let _ = fs::remove_file(&target_path);
                symlink(name, &target_path).unwrap_or_else(|error| {
                    panic!(
                        "❌ Couldn't link [{}] to [{}] because [{}]",
                        target_path.display(),
                        name,
                        error
                    )
                });
                applied += 1;
            }
        }
    }

    if applied == 0 {
        panic!(
            "❌ The icon theme [{}] has no files for the color [{}]",
            icon_theme, mapped_theme
        );
    }
}

pub fn set_wallpapers(wallpapers: &[String]) {
    let home = env::var("HOME")
        .unwrap_or_else(|error| panic!("❌ HOME env variable not set: [{}]", error));

    let output = Command::new("feh")
        .args(["--bg-fill", "-z"])
        .args(wallpapers.iter().map(|wp| wp.replace('~', &home)))
        .output()
        .unwrap_or_else(|error| panic!("❌ Error while executing feh: [{}]", error));

    if !output.status.success() {
        panic!("❌ feh failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}
