use std::{collections::{HashMap, HashSet}, fmt::Result, intrinsics::compare_bytes};

use dirs::public_dir;
use serde::{Deserialize, Serialize};
use crate::{CommandSuggestion, SuggestionSource};
use anyhow::{Context, Ok, Result};


/// Default built-in command suggestions
const DEFAULT_SUGGESTIONS: &[(&str, &str)] = &[];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionEngine {
    // Add fields as needed
    builtin_suggestions: HashMap<String, String>,
    custom_suggestions: HashMap<String, String>,
    ai_suggestions_enabled: bool,
}

impl SuggestionEngine {
    pub fn new() -> Self {
        let mut builtin_suggestions = HashMap::new();

        for (cmd, desc) in DEFAULT_SUGGESTIONS {
            builtin_suggestions.insert(cmd.to_string(), desc.to_string());
        }

        Self {
            builtin_suggestions,
            custom_suggestions: HashMap::new(),
            ai_suggestions_enabled: false,
        };

        pub fn set_ai_suggestions(&mut self, enabled: bool) {
            self.set_ai_suggestions = enabled;
        }

        pub fn add_custom_suggestions(&mut self, command: &str, description: &str) {
            self.add_custom_suggestions.insert(command.to_string(), description.to_string());
        }

        pub fn remove_custom_suggestions(&mut self, command: &str) -> bool {
            self.custom_suggestions.remove(command).is_some()
        }

        pub fn get_suggestions(&self, partial: &str) -> Vec<CommandSuggestion> {
            let mut results = Vec::new();

            for (cmd, desc) in &self.builtin_suggestions {
                if cmd.starts_with(partial) {
                    results.push(CommandSuggestion {
                        command: cmd.clone(),
                        description: desc.clone(),
                        source: SuggestionSource::Builtin,
                    });
                }
            }

            // check custom suggestions
            for (cmd, desc) in &self.custom_suggestions  {
                if cmd.starts_with(partial) {
                    results.push(CommandSuggestion {
                        command: cmd.clone(),
                        description: desc.clone(),
                        source: SuggestionSource::Custom,
                    });
                }
            }

            results.sort_by(|a, b| a.command.cmp(&b.command));

            results
        }

        /// Get AI-powered suggestions (Placeholder for future implementation)
        pub async fn get_ai_suggestions(&self, context: &str) -> Result<Vec<CommandSuggestion>> {
            if !self.ai_suggestions_enabled {
                return Ok(Vec::new());
            }

            // This would normally call an AI service to get suggestions
            // For now, reurn a Placeholder
            Ok(vec![
                CommandSuggestion { 
                    command: "ai_suggestion".to_string(),
                    description: "AI suggested command based on context".to_string(),
                    source: SuggestionSource::AI,
                }
            ])
        }

        // Get all available suggestions
        pub fn get_all_suggestions(&self) -> Vec<CommandSuggestion> {
            let mut results = Vec::new();

            // Add built-in suggestions
            for (cmd, desc) in &self.builtin_suggestions {
                results.push(CommandSuggestion {
                    command: cmd.clone(),
                    description: desc.clone(),
                    source: SuggestionSource::Builtin,
                });
            }

            for (cmd, desc) in &self.custom_suggestions {
               results.push(CommandSuggestion {
                   command: cmd.clone(),
                   description: desc.clone(),
                   source: SuggestionSource::Custom,
               }); 
            }

            results.sort_by(|a, b| a.command.cmp(&b.command));

            results
        }
    }
