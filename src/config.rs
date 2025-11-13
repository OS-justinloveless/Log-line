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
        
        config.validate()?;
        
        Ok(config)
    }
    
    /// Create a new config with all values provided
    pub fn new(
        timestamp_regex: String,
        timestamp_format: String,
        message_patterns: Vec<String>,
    ) -> Result<Self> {
        let config = Config {
            timestamp_regex,
            timestamp_format,
            message_patterns,
        };
        
        config.validate()?;
        
        Ok(config)
    }
    
    /// Merge configuration from file with CLI overrides
    pub fn from_file_with_overrides(
        path: Option<&Path>,
        timestamp_regex: Option<String>,
        timestamp_format: Option<String>,
        message_patterns: Option<Vec<String>>,
    ) -> Result<Self> {
        // Start with config file if provided
        let mut config = if let Some(path) = path {
            Config::from_file(path)?
        } else {
            // If no config file, all values must be provided via CLI
            if timestamp_regex.is_none() || timestamp_format.is_none() || message_patterns.is_none() {
                anyhow::bail!(
                    "When no config file is provided, all options must be specified via CLI:\n\
                     --timestamp-regex, --timestamp-format, and at least 2 --pattern arguments"
                );
            }
            Config {
                timestamp_regex: String::new(),
                timestamp_format: String::new(),
                message_patterns: Vec::new(),
            }
        };
        
        // Apply CLI overrides
        if let Some(regex) = timestamp_regex {
            config.timestamp_regex = regex;
        }
        
        if let Some(format) = timestamp_format {
            config.timestamp_format = format;
        }
        
        if let Some(patterns) = message_patterns {
            if !patterns.is_empty() {
                config.message_patterns = patterns;
            }
        }
        
        config.validate()?;
        
        Ok(config)
    }
    
    /// Validate configuration
    fn validate(&self) -> Result<()> {
        if self.timestamp_regex.is_empty() {
            anyhow::bail!("timestamp_regex cannot be empty");
        }
        
        if self.timestamp_format.is_empty() {
            anyhow::bail!("timestamp_format cannot be empty");
        }
        
        if self.message_patterns.len() < 2 {
            anyhow::bail!("Configuration must have at least 2 message patterns");
        }
        
        Ok(())
    }
}
