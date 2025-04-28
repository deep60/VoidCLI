// Theming system for VoidCLI
//
// This module provides functionality for managing terminal color schemes and styling

use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub author: String,
    pub colors: ColorScheme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub selection: String,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    pub bright_black: String,
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_magenta: String,
    pub bright_cyan: String,
    pub bright_white: String,
}

lazy_static! {
    static ref THEMES: HashMap<String, Theme> = {
        let mut m = HashMap::new();

        //Add default themes
        m.insert("dark".to_string(), Theme {
            name: "Dark".to_string(),
            author: "VYZE".to_string(),
            colors: ColorScheme {
                background: "#282a36".to_string(),
                foreground: "#f8f8f2".to_string(),
                cursor: "#f8f8f2".to_string(),
                selection: "#44475a".to_string(),
                black: "#21222c".to_string(),
                red: "#ff5555".to_string(),
                green: " #50fa7b".to_string(),
                yellow: "#f1f8ac".to_string(),
                blue: "#bd93f9".to_string(),
                magenta: "#ff79c6".to_string(),
                cyan: "#8be9fd".to_string(),
                white: "#f8f8f2".to_string(),
                bright_black: "#6272a4".to_string(),
                bright_red: "#ff6e6e".to_string(),
                bright_green: "#69ff94".to_string(),
                bright_yellow: "ffffa5".to_string(),
                bright_blue: "#d6acff".to_string(),
                bright_magenta: "#ff92df".to_string(),
                bright_cyan: "#a4ffff".to_string(),
                bright_white: "#ffffff".to_string(),
            },
        });
        //Add more items
        m
    };
}

impl Theme {
    pub fn from_name(name: &str) -> Option<Self> {
        THEMES.get(name).cloned()
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let theme: Theme = serde_yaml::from_str(&contents)?;
        Ok(theme)
    }
}

impl Default for Theme {
    fn default() -> Self {
        THEMES.get("dark").unwrap().clone()
    }
}
