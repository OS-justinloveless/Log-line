use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Regular expression to extract timestamps from log lines
    pub timestamp_regex: String,
    
    /// Format string for parsing timestamps (chrono format)
    pub timestamp_format: String,
    
    /// Array of message patterns to search for in order
    pub message_patterns: Vec<String>,
}

impl Config {
    /// Load configuration from a YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {:?}", path.as_ref()))?;
        
        let config: Config = serde_yaml::from_str(&contents)
            .context("Failed to parse YAML configuration")?;
        
        // Validate configuration
        if config.message_patterns.len() < 2 {
            anyhow::bail!("Configuration must have at least 2 message patterns");
        }
        
        Ok(config)
    }
}
