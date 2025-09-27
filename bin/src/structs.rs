use std::collections::HashMap;
use serde::Deserialize;

// Defining themes.json structure ────────────────────────────
#[derive(Deserialize, Debug)]
pub struct Theme {
    pub name: String,
    pub wallpapers: Vec<String>,
}

// Defining apps.json structure ───────────────────────────────
#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum ThemeMappingType {
    Arr(Vec<String>),
    Str(String)
}

#[derive(Deserialize)]
pub struct Regex {
    pub expression: String,
    pub target: String,
    pub key: String,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    pub paths: Option<Vec<String>>,
    pub regex: Option<Vec<Regex>>,
    pub theme_mapping: Option<HashMap<String, ThemeMappingType>>,
    pub command: Option<String>,
    pub key: Option<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            paths: None,
            regex: None,
            theme_mapping: None,
            command: None,
            key: None,
        }
    }
}

#[derive(Deserialize)]
pub struct App {
    pub name: String,
    pub config: Config,
}
