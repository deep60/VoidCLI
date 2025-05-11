use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Application name
    pub app_name: String,
    /// Application theme
    pub theme: Theme,
    /// API Config
    pub connection: ConnectionConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Theme {
    pub primary_color: String,
    pub secondary_color: String,
    pub background_color: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionConfig {
    pub endpoint: String,
    pub api_key: String,
    pub timeout_sec: u64,
}

impl Config {
    /// Create a new Config from a file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("Loading config from: {}", path.as_ref().display());
        let config_str = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;

        let config: Config = toml::from_str(&config_str).with_context(|| "Failed to parse file")?;

        Ok(config)
    }

    /// Create a default configuration
    pub fn default() -> Self {
        warn!("Using default configuration");
        Self {
            app_name: "Void_CLI".to_string(),
            theme: Theme {
                primary_color: "#5E81AC".to_string(),
                secondary_color: "#88C0D0".to_string(),
                background_color: "#2E3440".to_string(),
            },
            connection: ConnectionConfig {
                endpoint: "".to_string(),
                api_key: None,
                timeout_sec: 30,
            },
        }
    }
}
