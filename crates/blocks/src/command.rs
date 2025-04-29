use serde::{Deserialize, Serialize};
use core::str;
use std::fmt::{self, write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub raw: String,   ///the raw command string entered by the user
    pub tokens: Vec<String>,   ///the processed and tokenized command
    pub env_vars: Vec<(String, String)>,   ///environment variables for this Command
    pub working_dir: String,     ///Working directory for this command
}

impl Command {
    pub fn new(raw: &str) -> Self {
        let tokens = tokenize(raw);
        Self {
            raw: raw.to_string(),
            tokens,
            env_vars: Vec::new(),
            working_dir: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "/".to_string()),
        }
    }

    pub fn program(&self) -> Option<&str> {
       self.tokens.first().map(|s| s.as_str()) 
    }

    pub fn args(&self) -> &[String] {
        if self.tokens.is_empty() {
            &[]
        } else {
            &self.tokens[1..]
        }
    }

    pub fn with_env_var(mut self, key: &str, value: &str) -> Self {
        self.env_vars.push((key.to_string(), value.to_string()));
        self
    }

    pub fn with_working_dir(mut self, dir: &str) -> Self {
        self.working_dir = dir.to_string();
        self
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

///Simple tokenizer for command line strings
fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';
    let mut escape_next = false;

    for c in input.chars() {
        if escape_next {
            current_token.push(c);
            escape_next = false;
            continue;
        }

        match c {
            '\\' => escape_next = true,
            '"' | '\'' => {
                if in_quotes {
                    if c == quote_char {
                        in_quotes = false;
                    } else {
                        current_token.push(c);
                    }
                } else {
                    in_quotes = true;
                    quote_char = c;
                }
            }
            ' ' | '\t' => {
                if in_quotes {
                    current_token.push(c);
                } else if !current_token.is_empty() {
                    tokens.push(current_token);
                    current_token = String::new();
                }
            }
            _ => current_token.push(c),
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "echo hello world";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec!["echo", "hello", "world"]);

        let input = "echo \"hello world\"";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec!["echo", "hello world"]);

        let input = "echo 'hello world'";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec!["echo", "hello world"]);

        let input = "echo \"hello\\\" world\"";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec!["echo", "hello\" world"]);
    }
}
