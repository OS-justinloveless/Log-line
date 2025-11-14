/// Built-in timestamp format definitions for automatic detection
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampFormat {
    /// Name of the format
    pub name: &'static str,
    /// Regular expression to extract the timestamp (with a capture group)
    pub regex: &'static str,
    /// Chrono format string for parsing the timestamp
    pub format: &'static str,
    /// Example timestamp for reference
    pub example: &'static str,
}

/// Get all built-in timestamp formats
pub fn get_builtin_formats() -> Vec<TimestampFormat> {
    vec![
        // ISO 8601 with timezone
        TimestampFormat {
            name: "ISO 8601 with timezone",
            regex: r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2}))",
            format: "%Y-%m-%dT%H:%M:%S%.f%:z",
            example: "2025-11-13T10:00:00.123+00:00",
        },
        // ISO 8601 without timezone
        TimestampFormat {
            name: "ISO 8601 without timezone",
            regex: r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?)",
            format: "%Y-%m-%dT%H:%M:%S%.f",
            example: "2025-11-13T10:00:00.123",
        },
        // RFC 3339 (similar to ISO 8601)
        TimestampFormat {
            name: "RFC 3339",
            regex: r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})?)",
            format: "%Y-%m-%d %H:%M:%S%.f%:z",
            example: "2025-11-13 10:00:00.123+00:00",
        },
        // Common log format (YYYY-MM-DD HH:MM:SS)
        TimestampFormat {
            name: "Common log format (YYYY-MM-DD HH:MM:SS)",
            regex: r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})",
            format: "%Y-%m-%d %H:%M:%S",
            example: "2025-11-13 10:00:00",
        },
        // Common log format with milliseconds
        TimestampFormat {
            name: "Common log format with milliseconds",
            regex: r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3})",
            format: "%Y-%m-%d %H:%M:%S%.3f",
            example: "2025-11-13 10:00:00.123",
        },
        // Apache/Nginx common log format
        TimestampFormat {
            name: "Apache/Nginx common log format",
            regex: r"\[(\d{2}/[A-Za-z]{3}/\d{4}:\d{2}:\d{2}:\d{2} [+-]\d{4})\]",
            format: "%d/%b/%Y:%H:%M:%S %z",
            example: "[13/Nov/2025:10:00:00 +0000]",
        },
        // Syslog format (RFC 3164)
        TimestampFormat {
            name: "Syslog format (RFC 3164)",
            regex: r"([A-Za-z]{3}\s+\d{1,2} \d{2}:\d{2}:\d{2})",
            format: "%b %d %H:%M:%S",
            example: "Nov 13 10:00:00",
        },
        // Syslog format (RFC 5424)
        TimestampFormat {
            name: "Syslog format (RFC 5424)",
            regex: r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2}))",
            format: "%Y-%m-%dT%H:%M:%S%.f%:z",
            example: "2025-11-13T10:00:00.123+00:00",
        },
        // Windows Event Log format
        TimestampFormat {
            name: "Windows Event Log format",
            regex: r"(\d{1,2}/\d{1,2}/\d{4} \d{1,2}:\d{2}:\d{2} (?:AM|PM))",
            format: "%m/%d/%Y %I:%M:%S %p",
            example: "11/13/2025 10:00:00 AM",
        },
        // Unix timestamp (seconds since epoch)
        TimestampFormat {
            name: "Unix timestamp (seconds)",
            regex: r"\b(\d{10})\b",
            format: "%s",
            example: "1699876800",
        },
        // Unix timestamp with milliseconds
        TimestampFormat {
            name: "Unix timestamp (milliseconds)",
            regex: r"\b(\d{13})\b",
            format: "%s%.3f",
            example: "1699876800123",
        },
        // Date with slashes and time (US format)
        TimestampFormat {
            name: "US date format with time",
            regex: r"(\d{1,2}/\d{1,2}/\d{4} \d{2}:\d{2}:\d{2})",
            format: "%m/%d/%Y %H:%M:%S",
            example: "11/13/2025 10:00:00",
        },
        // European date format with time
        TimestampFormat {
            name: "European date format with time",
            regex: r"(\d{1,2}\.\d{1,2}\.\d{4} \d{2}:\d{2}:\d{2})",
            format: "%d.%m.%Y %H:%M:%S",
            example: "13.11.2025 10:00:00",
        },
        // Java log format
        TimestampFormat {
            name: "Java log format",
            regex: r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2},\d{3})",
            format: "%Y-%m-%d %H:%M:%S,%3f",
            example: "2025-11-13 10:00:00,123",
        },
        // Python logging default format
        TimestampFormat {
            name: "Python logging format",
            regex: r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2},\d{3})",
            format: "%Y-%m-%d %H:%M:%S,%3f",
            example: "2025-11-13 10:00:00,123",
        },
        // Compact format (YYYYMMDD_HHMMSS)
        TimestampFormat {
            name: "Compact format (YYYYMMDD_HHMMSS)",
            regex: r"(\d{8}_\d{6})",
            format: "%Y%m%d_%H%M%S",
            example: "20251113_100000",
        },
        // Compact format with milliseconds
        TimestampFormat {
            name: "Compact format with milliseconds",
            regex: r"(\d{8}_\d{6}\.\d{3})",
            format: "%Y%m%d_%H%M%S%.3f",
            example: "20251113_100000.123",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use regex::Regex;

    #[test]
    fn test_all_formats_compile() {
        for format in get_builtin_formats() {
            // Test that regex compiles
            let regex = Regex::new(format.regex);
            assert!(regex.is_ok(), "Failed to compile regex for format: {}", format.name);
            
            // Test that the example matches the regex
            let regex = regex.unwrap();
            let captures = regex.captures(format.example);
            assert!(captures.is_some(), "Example doesn't match regex for format: {}", format.name);
        }
    }

    #[test]
    fn test_common_log_format() {
        let format = get_builtin_formats()
            .into_iter()
            .find(|f| f.name == "Common log format (YYYY-MM-DD HH:MM:SS)")
            .unwrap();
        
        let regex = Regex::new(format.regex).unwrap();
        let test_line = "2025-11-13 10:00:00 [INFO] Application started";
        
        let captures = regex.captures(test_line);
        assert!(captures.is_some());
        
        let ts_str = captures.unwrap().get(1).unwrap().as_str();
        let parsed = NaiveDateTime::parse_from_str(ts_str, format.format);
        assert!(parsed.is_ok());
    }
}

