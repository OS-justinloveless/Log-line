use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::config::Config;
use crate::timestamp_formats::{get_builtin_formats, TimestampFormat};

#[derive(Debug, Clone)]
pub struct LogMatch {
    pub pattern: String,
    pub timestamp: NaiveDateTime,
}

pub struct LogParser {
    timestamp_regex: Option<Regex>,
    timestamp_format: Option<String>,
    pattern_regexes: Vec<(usize, String, Regex)>,
    builtin_formats: Vec<(Regex, TimestampFormat)>,
    is_auto_detect: bool,
}

impl LogParser {
    pub fn new(config: &Config) -> Result<Self> {
        let (timestamp_regex, timestamp_format, builtin_formats) = if config.is_auto_detect {
            // Prepare all built-in formats for auto-detection
            let formats = get_builtin_formats();
            let mut compiled_formats = Vec::new();
            
            for format in formats {
                let regex = Regex::new(format.regex)
                    .with_context(|| format!("Failed to compile built-in regex for format: {}", format.name))?;
                compiled_formats.push((regex, format));
            }
            
            (None, None, compiled_formats)
        } else {
            let timestamp_regex = Regex::new(&config.timestamp_regex)
                .context("Invalid timestamp regex")?;
            
            (Some(timestamp_regex), Some(config.timestamp_format.clone()), Vec::new())
        };
        
        let mut pattern_regexes = Vec::new();
        for (idx, pattern) in config.message_patterns.iter().enumerate() {
            let regex = Regex::new(pattern)
                .with_context(|| format!("Invalid message pattern regex: {}", pattern))?;
            pattern_regexes.push((idx, pattern.clone(), regex));
        }
        
        Ok(LogParser {
            timestamp_regex,
            timestamp_format,
            pattern_regexes,
            builtin_formats,
            is_auto_detect: config.is_auto_detect,
        })
    }
    
    /// Parse a log file and return all matches in order
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<LogMatch>> {
        let file = File::open(path.as_ref())
            .with_context(|| format!("Failed to open log file: {:?}", path.as_ref()))?;
        
        let reader = BufReader::new(file);
        self.parse_reader(reader)
    }
    
    /// Parse log data from any reader (file, stdin, etc.) and return all matches in order
    pub fn parse_reader<R: BufRead>(&self, reader: R) -> Result<Vec<LogMatch>> {
        let mut matches = Vec::new();
        
        for line in reader.lines() {
            let line = line.context("Failed to read line from log")?;
            
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
        if self.is_auto_detect {
            // Try each built-in format until one works
            for (regex, format) in &self.builtin_formats {
                if let Some(captures) = regex.captures(line) {
                    if let Some(ts_str) = captures.get(1) {
                        // Try to parse with this format
                        if let Ok(timestamp) = NaiveDateTime::parse_from_str(
                            ts_str.as_str(),
                            format.format,
                        ) {
                            return Ok(Some(timestamp));
                        }
                    }
                }
            }
            Ok(None)
        } else {
            // Use the configured format
            let timestamp_regex = self.timestamp_regex.as_ref().unwrap();
            let timestamp_format = self.timestamp_format.as_ref().unwrap();
            
            if let Some(captures) = timestamp_regex.captures(line) {
                if let Some(ts_str) = captures.get(1) {
                    let timestamp = NaiveDateTime::parse_from_str(
                        ts_str.as_str(),
                        timestamp_format,
                    )
                    .with_context(|| format!("Failed to parse timestamp: {}", ts_str.as_str()))?;
                    
                    return Ok(Some(timestamp));
                }
            }
            
            Ok(None)
        }
    }
}
