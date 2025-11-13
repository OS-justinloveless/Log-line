use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct LogMatch {
    pub pattern: String,
    pub timestamp: NaiveDateTime,
}

pub struct LogParser {
    timestamp_regex: Regex,
    timestamp_format: String,
    pattern_regexes: Vec<(usize, String, Regex)>,
}

impl LogParser {
    pub fn new(config: &Config) -> Result<Self> {
        let timestamp_regex = Regex::new(&config.timestamp_regex)
            .context("Invalid timestamp regex")?;
        
        let mut pattern_regexes = Vec::new();
        for (idx, pattern) in config.message_patterns.iter().enumerate() {
            let regex = Regex::new(pattern)
                .with_context(|| format!("Invalid message pattern regex: {}", pattern))?;
            pattern_regexes.push((idx, pattern.clone(), regex));
        }
        
        Ok(LogParser {
            timestamp_regex,
            timestamp_format: config.timestamp_format.clone(),
            pattern_regexes,
        })
    }
    
    /// Parse a log file and return all matches in order
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<LogMatch>> {
        let file = File::open(path.as_ref())
            .with_context(|| format!("Failed to open log file: {:?}", path.as_ref()))?;
        
        let reader = BufReader::new(file);
        let mut matches = Vec::new();
        
        for line in reader.lines() {
            let line = line.context("Failed to read line from log file")?;
            
            if let Some(log_match) = self.parse_line(&line)? {
                matches.push(log_match);
            }
        }
        
        Ok(matches)
    }
    
    /// Parse a single log line and return a match if found
    fn parse_line(&self, line: &str) -> Result<Option<LogMatch>> {
        // First, extract the timestamp
        let timestamp = match self.extract_timestamp(line)? {
            Some(ts) => ts,
            None => return Ok(None),
        };
        
        // Check each pattern to see if it matches
        for (_idx, pattern, regex) in &self.pattern_regexes {
            if regex.is_match(line) {
                return Ok(Some(LogMatch {
                    pattern: pattern.clone(),
                    timestamp,
                }));
            }
        }
        
        Ok(None)
    }
    
    /// Extract timestamp from a log line
    fn extract_timestamp(&self, line: &str) -> Result<Option<NaiveDateTime>> {
        if let Some(captures) = self.timestamp_regex.captures(line) {
            if let Some(ts_str) = captures.get(1) {
                let timestamp = NaiveDateTime::parse_from_str(
                    ts_str.as_str(),
                    &self.timestamp_format,
                )
                .with_context(|| format!("Failed to parse timestamp: {}", ts_str.as_str()))?;
                
                return Ok(Some(timestamp));
            }
        }
        
        Ok(None)
    }
}
