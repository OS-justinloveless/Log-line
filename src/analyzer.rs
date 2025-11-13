use chrono::Duration;
use crate::parser::LogMatch;

#[derive(Debug)]
pub struct Interval {
    pub from_pattern: String,
    pub to_pattern: String,
    pub duration: Duration,
}

impl Interval {
    pub fn format(&self) -> String {
        let duration_str = self.format_duration();
        format!("{} :::: {} ::::> {}", 
            self.from_pattern, 
            duration_str, 
            self.to_pattern)
    }
    
    pub fn format_duration(&self) -> String {
        format_duration(&self.duration)
    }
}

pub struct Analyzer;

impl Analyzer {
    /// Analyze log matches and find intervals between consecutive pattern matches
    pub fn analyze(matches: Vec<LogMatch>) -> Vec<Interval> {
        let mut intervals = Vec::new();
        
        if matches.is_empty() {
            return intervals;
        }
        
        // Find intervals between consecutive matches
        for i in 0..matches.len() - 1 {
            let from = &matches[i];
            let to = &matches[i + 1];
            
            // Calculate duration
            let duration = to.timestamp.signed_duration_since(from.timestamp);
            
            intervals.push(Interval {
                from_pattern: from.pattern.clone(),
                to_pattern: to.pattern.clone(),
                duration,
            });
        }
        
        intervals
    }
}

/// Format duration in a human-readable way
fn format_duration(duration: &Duration) -> String {
    let total_seconds = duration.num_seconds();
    let is_negative = total_seconds < 0;
    let abs_seconds = total_seconds.abs();
    
    let hours = abs_seconds / 3600;
    let minutes = (abs_seconds % 3600) / 60;
    let seconds = abs_seconds % 60;
    let millis = duration.num_milliseconds().abs() % 1000;
    
    let sign = if is_negative { "-" } else { "" };
    
    if hours > 0 {
        format!("{}{}h {}m {}s", sign, hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}{}m {}s", sign, minutes, seconds)
    } else if seconds > 0 {
        format!("{}{}s {}ms", sign, seconds, millis)
    } else {
        format!("{}{}ms", sign, millis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        let duration = Duration::seconds(3661);
        assert_eq!(format_duration(&duration), "1h 1m 1s");
        
        let duration = Duration::seconds(125);
        assert_eq!(format_duration(&duration), "2m 5s");
        
        let duration = Duration::milliseconds(1500);
        assert_eq!(format_duration(&duration), "1s 500ms");
        
        let duration = Duration::milliseconds(500);
        assert_eq!(format_duration(&duration), "500ms");
    }
}
