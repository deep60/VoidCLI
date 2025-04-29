// Configuration management module for VoidCLI
//
// This module handles loading, parsing, and validating user configurations.
//
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: String,
    pub font: FontConfig,
    pub terminal: TerminalConfig,
    pub keybindings: KeybindingsConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    pub name: String,
    pub size: f32,
    pub line_height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    pub shell: String,
    pub scrollback_lines: usize,
    pub cursor_blink: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingsConfig {
    //
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub gpu_acceleration: bool,
    pub vsync: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            font: FontConfig {
                name: "JetBrains Mono".to_string(),
                size: 14.0,
                line_height: 1.2,
            },
            terminal: TerminalConfig {
                shell: std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string()),
                scrollback_lines: 10000,
                cursor_blink: true,
            },
            keybindings: KeybindingsConfig {},
            performance: PerformanceConfig {
                gpu_acceleration: true,
                vsync: true,
            },
        }
    }
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}
