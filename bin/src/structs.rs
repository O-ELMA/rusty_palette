use std::collections::HashMap;
use serde::Deserialize;
use regex::Regex as RegexPattern;

// Defining themes.json structure ────────────────────────────
#[derive(Deserialize, Debug)]
pub struct Theme {
    pub name: String,
    pub wallpapers: Vec<String>,
}

// Compiled regex structure ───────────────────────────────────
#[derive(Debug, Clone)]
pub struct CompiledRegex {
    pub regex: RegexPattern,
    pub target: String,
    pub key: String,
}

// Defining apps.json structure ───────────────────────────────
#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum ThemeMappingType {
    Arr(Vec<String>),
    Str(String),
}

#[derive(Deserialize)]
pub struct Regex {
    pub expression: String,
    pub target: String,
    pub key: String,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub paths: Option<Vec<String>>,
    pub regex: Option<Vec<Regex>>,
    pub theme_mapping: Option<HashMap<String, ThemeMappingType>>,
    pub command: Option<String>,
    pub key: Option<String>,
}

#[derive(Deserialize)]
pub struct App {
    pub name: String,
    pub config: Config,
}

impl App {
    pub fn compile_regexes(&self) -> Option<Vec<CompiledRegex>> {
        self.config.regex.as_ref().map(|regexes| {
            regexes
                .iter()
                .map(|r| {
                    let regex = RegexPattern::new(&r.expression)
                        .unwrap_or_else(|_| panic!("❌ Failed to compile regex: [{}]", r.expression));
                    CompiledRegex {
                        regex,
                        target: r.target.clone(),
                        key: r.key.clone(),
                    }
                })
                .collect()
        })
    }
}
