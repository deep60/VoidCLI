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
    pub colors: ThemeColors,
    pub styles: ThemeStyles,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub error: String,
    pub success: String,
    pub warning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeStyles {
    pub font_family: String,
    pub font_size: u32,
    pub line_height: f32,
    pub padding: u32,
    pub border_radius: u32,
}

lazy_static! {
    static ref THEMES: HashMap<String, Theme> = {
        let mut themes = HashMap::new();
        
        // Default dark theme
        themes.insert("dark".to_string(), Theme {
            name: "dark".to_string(),
            colors: ThemeColors {
                background: "#1a1a1a".to_string(),
                foreground: "#ffffff".to_string(),
                accent: "#007acc".to_string(),
                error: "#ff5555".to_string(),
                success: "#50fa7b".to_string(),
                warning: "#ffb86c".to_string(),
            },
            styles: ThemeStyles {
                font_family: "monospace".to_string(),
                font_size: 14,
                line_height: 1.5,
                padding: 8,
                border_radius: 4,
            },
        });

        // Default light theme
        themes.insert("light".to_string(), Theme {
            name: "light".to_string(),
            colors: ThemeColors {
                background: "#ffffff".to_string(),
                foreground: "#000000".to_string(),
                accent: "#007acc".to_string(),
                error: "#ff0000".to_string(),
                success: "#00ff00".to_string(),
                warning: "#ffa500".to_string(),
            },
            styles: ThemeStyles {
                font_family: "monospace".to_string(),
                font_size: 14,
                line_height: 1.5,
                padding: 8,
                border_radius: 4,
            },
        });

        themes
    };
}

pub struct ThemeManager {
    current_theme: Theme,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            current_theme: THEMES.get("dark").unwrap().clone(),
        }
    }

    pub fn get_theme(&self, name: &str) -> Option<Theme> {
        THEMES.get(name).cloned()
    }

    pub fn load_theme_from_file(&mut self, path: &str) -> Result<()> {
        let contents = std::fs::read_to_string(path)?;
        let theme: Theme = serde_yaml::from_str(&contents)?;
        self.current_theme = theme;
        Ok(())
    }

    pub fn get_current_theme(&self) -> Theme {
        self.current_theme.clone()
    }

    pub fn set_theme(&mut self, name: &str) -> Result<()> {
        if let Some(theme) = THEMES.get(name) {
            self.current_theme = theme.clone();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Theme not found: {}", name))
        }
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self {
            current_theme: THEMES.get("dark").unwrap().clone(),
        }
    }
}
