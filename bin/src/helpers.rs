// Standards ───────────────────────────────────────────────────────────────────────────
use std::{
    borrow::Cow, collections::HashMap, env, fs::{self}, io::{self, Stdin}, process::Command
};

// Crates ───────────────────────────────────────────────────────────────────────────────
use serde::Deserialize;
use serde_json;
use regex::Regex;
use rand::{rng, seq::IndexedRandom};

// Structs ──────────────────────────────────────────────────────────────────────────────
use crate::structs::{App, Theme, ThemeMappingType};

// Other ───────────────────────────────────────────────────────────────────────────────
pub fn parse_config<T>(json_str: &str) -> T
where 
    T: for<'a> Deserialize<'a>
{
    match serde_json::from_str(json_str) {
        Err(error) => panic!("❌ Could not parse embedded config: [{}]", error),
        Ok(json) => json,
    }
}

pub fn prompt_user<'a>(themes: &'a HashMap<String, Theme>) -> &'a Theme {
    let theme_names: Vec<&String> = themes.keys().collect();

    println!("✨ Choose a theme to apply by typing its number:");
    for (i, name) in theme_names.iter().enumerate() {
        println!("{}. {}", i + 1, name);
    }
    println!("\n(Press Enter without typing to pick randomly)");

    let stdin: Stdin = io::stdin();
    let mut input: String = String::new();
    let _ = stdin.read_line(&mut input);
    input = input.trim().to_owned(); // Removes the '\n' caused by the user pressing 'enter'

    let selected = if input.is_empty() {
        let mut rng = rng();
        theme_names.choose(&mut rng).unwrap().to_string()
    } else {
        let index: usize = input.parse().expect("❌ Please enter a valid number.");
        if index == 0 || index > theme_names.len() {
            panic!("❌ Number should be between 1 and {}.", theme_names.len());
        }
        theme_names[index - 1].to_string()
    };

    let theme: &Theme = themes.get(&selected)
        .unwrap_or_else(|| panic!(
            "❌ Couldn't find [{}] among the available themes {:?}",
            selected, themes.keys()
        ));

    theme
}

pub fn set_apps_theme(theme_name: String, app: &App) {
    let user = env::var("USER").unwrap_or_else(|error| {
        panic!(
            "❌ USER env variable not set: [{}]",
            error
        )
    });

    for path in app.config.paths.clone().unwrap() {
        let clean_path = path.replace("$USER", &user);

        let file_content: String = fs::read_to_string(&clean_path).unwrap_or_else(|error| {
            panic!(
                "❌ Couldn't read the file [{}] because [{}]",
                clean_path, error
            )
        });

        let regexes = app.config.regex
            .as_ref()
            .unwrap_or_else(|| {
                panic!("❌ Couldn't find [regex] in the app [{}]", app.name)
            });

        let targets: Vec<String> = if let Some(theme_mapping) = &app.config.theme_mapping {
            match theme_mapping.get(&theme_name) {
                Some(ThemeMappingType::Arr(arr)) => arr.to_owned(),
                Some(ThemeMappingType::Str(str)) => vec![str.clone(); regexes.len()],
                _ => panic!(
                    "❌ [{}]'s theme_mapping needs to be in a String-to-Array format.",
                    app.name
                ),
            }
        } else {
            // repeat theme_name for each regex
            vec![theme_name.clone(); regexes.len()]
        };

        let mut content = Cow::from(file_content.clone());

        for (i, regex) in regexes.iter().enumerate() {
            let target = regex.target.replace(&regex.key, &targets[i]);

            let re = Regex::new(&regex.expression).unwrap_or_else(|error| {
                panic!(
                    "❌ The regex [{}] failed because [{}]", 
                    regex.expression, error
                )
            });

            content = Cow::from(re.replace(&content, &target).into_owned());
        }

        fs::write(&clean_path, content.into_owned()).unwrap_or_else(|error| {
            panic!(
                "❌ Couldn't write to the file [{}] because [{}]",
                clean_path, error
            )
        }); 
    }
}

pub fn set_folder_icon_color(theme_name: &str, app: &App) {
    let mapped_theme: String = app.config.theme_mapping
        .clone()
        .unwrap()
        .get(theme_name)
        .map(|res| match res {
            ThemeMappingType::Str(s) => s.to_owned(),
            _ => panic!("❌ folder_icon's theme_mapping needs to be in a String-to-String format."),
        })
        .unwrap_or_else(|| panic!(
            "Couldn't find [{}] among the theme_mapping in the app [{}]",
            theme_name, app.name
        ));

    let clean_command: String = app.config.command
        .clone()
        .unwrap()
        .replace(&app.config.key.clone().unwrap(), &mapped_theme);

    // command = papirus-folders -C {color} --theme Papirus-Dark
    let output = Command::new("sh")
        .arg("-c")
        .arg(clean_command)
        .output();
        
    match output {
        Err(error) => panic!(
            "❌ folder_icon command executed but failed: {}",
            error
        ),
        Ok(output) => if !output.status.success() {
            panic!(
                "❌ folder_icon command executed but failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        },
    };
}

pub fn set_wallpapers(wallpapers: &Vec<String>) {
    let _output = Command::new("sh")
        .arg("-c")
        .arg(format!("feh --bg-fill -z {}*", wallpapers.join(" ")))
        .output()
        .unwrap_or_else(|error| panic!("❌ Error while executing feh: [{}]", error));
}
