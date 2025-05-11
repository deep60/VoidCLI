use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct CommandSuggestion {
    pub command: String,
    pub description: String,
    pub source: SuggestionSource,
}

#[derive(Debug, Clone)]
pub enum SuggestionSource {
    History,
    AI,
    Custom,
    Builtin,
}

/// Default built-in command suggestions
const DEFAULT_SUGGESTIONS: &[(&str, &str)] = &[
    ("ls", "List directory contents"),
    ("cd", "Change directory"),
    ("pwd", "Print working directory"),
    ("cp", "Copy files and directories"),
    ("mv", "Move files and directories"),
    ("rm", "Remove files or directories"),
    ("mkdir", "Make directories"),
    ("touch", "Change file timestamps"),
    ("grep", "Print lines matching a pattern"),
    ("find", "Search for files in a directory hierarchy"),
    ("cat", "Concatenate files and print on the standard output"),
    ("echo", "Display a line of text"),
];

#[derive(Debug, Clone)]
pub struct SuggestionEngine {
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
        }
    }

    pub fn set_ai_suggestions(&mut self, enabled: bool) {
        self.ai_suggestions_enabled = enabled;
    }

    pub fn add_custom_suggestions(&mut self, command: &str, description: &str) {
        self.custom_suggestions.insert(command.to_string(), description.to_string());
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
        for (cmd, desc) in &self.custom_suggestions {
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
    pub async fn get_ai_suggestions(&self, _context: &str) -> Result<Vec<CommandSuggestion>> {
        if !self.ai_suggestions_enabled {
            return Ok(Vec::new());
        }

        // This would normally call an AI service to get suggestions
        // For now, return a placeholder
        Ok(vec![CommandSuggestion {
            command: "ai_suggestion".to_string(),
            description: "AI suggested command based on context".to_string(),
            source: SuggestionSource::AI,
        }])
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggestion_engine() {
        let mut engine = SuggestionEngine::new();

        //Test built-in suggestions
        let suggestions = engine.get_suggestions("l");
        assert!(suggestions.iter().any(|s| s.command == "ls"));

        // Test custom suggestions
        engine.add_custom_suggestions("lsvirtualenv", "List all virtualenvs");
        let suggestions = engine.get_suggestions("lsv");
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].command, "lsvirtualenv");

        // Test removing custom suggestions
        assert!(engine.remove_custom_suggestions("lsvirtualenv"));
        let suggestions = engine.get_suggestions("lsv");
        assert_eq!(suggestions.len(), 0);
    }
}
