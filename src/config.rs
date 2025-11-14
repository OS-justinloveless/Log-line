use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// Regular expression to extract timestamps from log lines
    pub timestamp_regex: String,
    
    /// Format string for parsing timestamps (chrono format)
    pub timestamp_format: String,
    
    /// Array of message patterns to search for in order
    pub message_patterns: Vec<String>,
    
    /// Whether this config is for auto-detection mode
    #[serde(skip)]
    pub is_auto_detect: bool,
}

impl Config {
    /// Load configuration from a YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {:?}", path.as_ref()))?;
        
        let mut config: Config = serde_yaml::from_str(&contents)
            .context("Failed to parse YAML configuration")?;
        
        config.is_auto_detect = false;
        config.validate()?;
        
        Ok(config)
    }
    
    /// Create a config for auto-detection mode
    pub fn for_auto_detection(message_patterns: Vec<String>) -> Result<Self> {
        let config = Config {
            timestamp_regex: String::new(),
            timestamp_format: String::new(),
            message_patterns,
            is_auto_detect: true,
        };
        
        // Only validate message patterns for auto-detection
        if config.message_patterns.len() < 2 {
            anyhow::bail!("Configuration must have at least 2 message patterns");
        }
        
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
            // If no config file, check if we can use auto-detection
            // Auto-detection requires message patterns but not timestamp config
            if let Some(patterns) = message_patterns.clone() {
                if timestamp_regex.is_none() && timestamp_format.is_none() {
                    // Use auto-detection mode
                    return Config::for_auto_detection(patterns);
                }
                // User provided some timestamp config but not all
                if timestamp_regex.is_none() || timestamp_format.is_none() {
                    anyhow::bail!(
                        "When providing timestamp configuration, both --timestamp-regex and --timestamp-format are required"
                    );
                }
                Config {
                    timestamp_regex: String::new(),
                    timestamp_format: String::new(),
                    message_patterns: Vec::new(),
                    is_auto_detect: false,
                }
            } else {
                anyhow::bail!(
                    "When no config file is provided, at least 2 --pattern arguments must be specified.\n\
                     Timestamp format will be auto-detected, or you can manually specify:\n\
                     --timestamp-regex and --timestamp-format"
                );
            }
        };
        
        // Apply CLI overrides
        if let Some(regex) = timestamp_regex {
            config.timestamp_regex = regex;
            config.is_auto_detect = false;
        }
        
        if let Some(format) = timestamp_format {
            config.timestamp_format = format;
            config.is_auto_detect = false;
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
        // Skip timestamp validation for auto-detection mode
        if !self.is_auto_detect {
            if self.timestamp_regex.is_empty() {
                anyhow::bail!("timestamp_regex cannot be empty");
            }
            
            if self.timestamp_format.is_empty() {
                anyhow::bail!("timestamp_format cannot be empty");
            }
        }
        
        if self.message_patterns.len() < 2 {
            anyhow::bail!("Configuration must have at least 2 message patterns");
        }
        
        Ok(())
    }
}
