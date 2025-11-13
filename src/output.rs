use crate::analyzer::Interval;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Original human-readable format: "Pattern A :::: duration ::::> Pattern B"
    Human,
    /// JSON format for easy parsing
    Json,
    /// CSV format for spreadsheets
    Csv,
    /// TSV (tab-separated) format
    Tsv,
    /// Table format with aligned columns
    Table,
    /// Simple format with milliseconds: "from_pattern|to_pattern|milliseconds"
    Simple,
    /// Waterfall visualization with vertical bars
    Waterfall,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "human" => Some(OutputFormat::Human),
            "json" => Some(OutputFormat::Json),
            "csv" => Some(OutputFormat::Csv),
            "tsv" => Some(OutputFormat::Tsv),
            "table" => Some(OutputFormat::Table),
            "simple" => Some(OutputFormat::Simple),
            "waterfall" => Some(OutputFormat::Waterfall),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct IntervalJson {
    from_pattern: String,
    to_pattern: String,
    duration_ms: i64,
    duration_human: String,
}

pub struct OutputFormatter;

impl OutputFormatter {
    pub fn format_intervals(intervals: &[Interval], format: OutputFormat) -> String {
        match format {
            OutputFormat::Human => Self::format_human(intervals),
            OutputFormat::Json => Self::format_json(intervals),
            OutputFormat::Csv => Self::format_csv(intervals),
            OutputFormat::Tsv => Self::format_tsv(intervals),
            OutputFormat::Table => Self::format_table(intervals),
            OutputFormat::Simple => Self::format_simple(intervals),
            OutputFormat::Waterfall => Self::format_waterfall(intervals),
        }
    }
    
    fn format_human(intervals: &[Interval]) -> String {
        intervals
            .iter()
            .map(|interval| interval.format())
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn format_json(intervals: &[Interval]) -> String {
        let json_intervals: Vec<IntervalJson> = intervals
            .iter()
            .map(|interval| IntervalJson {
                from_pattern: interval.from_pattern.clone(),
                to_pattern: interval.to_pattern.clone(),
                duration_ms: interval.duration.num_milliseconds(),
                duration_human: interval.format_duration(),
            })
            .collect();
        
        serde_json::to_string_pretty(&json_intervals)
            .unwrap_or_else(|_| "[]".to_string())
    }
    
    fn format_csv(intervals: &[Interval]) -> String {
        let mut output = String::from("from_pattern,to_pattern,duration_ms,duration_human\n");
        
        for interval in intervals {
            output.push_str(&format!(
                "\"{}\",\"{}\",{},\"{}\"\n",
                Self::escape_csv(&interval.from_pattern),
                Self::escape_csv(&interval.to_pattern),
                interval.duration.num_milliseconds(),
                interval.format_duration()
            ));
        }
        
        output.trim_end().to_string()
    }
    
    fn format_tsv(intervals: &[Interval]) -> String {
        let mut output = String::from("from_pattern\tto_pattern\tduration_ms\tduration_human\n");
        
        for interval in intervals {
            output.push_str(&format!(
                "{}\t{}\t{}\t{}\n",
                Self::escape_tsv(&interval.from_pattern),
                Self::escape_tsv(&interval.to_pattern),
                interval.duration.num_milliseconds(),
                interval.format_duration()
            ));
        }
        
        output.trim_end().to_string()
    }
    
    fn format_table(intervals: &[Interval]) -> String {
        if intervals.is_empty() {
            return String::new();
        }
        
        // Calculate column widths
        let max_from = intervals
            .iter()
            .map(|i| i.from_pattern.len())
            .max()
            .unwrap_or(0)
            .max(12); // "From Pattern" header length
        
        let max_to = intervals
            .iter()
            .map(|i| i.to_pattern.len())
            .max()
            .unwrap_or(0)
            .max(10); // "To Pattern" header length
        
        let max_duration = intervals
            .iter()
            .map(|i| i.format_duration().len())
            .max()
            .unwrap_or(0)
            .max(8); // "Duration" header length
        
        let max_ms = intervals
            .iter()
            .map(|i| i.duration.num_milliseconds().to_string().len())
            .max()
            .unwrap_or(0)
            .max(13); // "Duration (ms)" header length
        
        let mut output = String::new();
        
        // Header
        output.push_str(&format!(
            "| {:<width_from$} | {:<width_to$} | {:<width_duration$} | {:>width_ms$} |\n",
            "From Pattern",
            "To Pattern",
            "Duration",
            "Duration (ms)",
            width_from = max_from,
            width_to = max_to,
            width_duration = max_duration,
            width_ms = max_ms
        ));
        
        // Separator
        output.push_str(&format!(
            "|{:-<width_from$}|{:-<width_to$}|{:-<width_duration$}|{:-<width_ms$}|\n",
            "-",
            "-",
            "-",
            "-",
            width_from = max_from + 2,
            width_to = max_to + 2,
            width_duration = max_duration + 2,
            width_ms = max_ms + 2
        ));
        
        // Rows
        for interval in intervals {
            output.push_str(&format!(
                "| {:<width_from$} | {:<width_to$} | {:<width_duration$} | {:>width_ms$} |\n",
                interval.from_pattern,
                interval.to_pattern,
                interval.format_duration(),
                interval.duration.num_milliseconds(),
                width_from = max_from,
                width_to = max_to,
                width_duration = max_duration,
                width_ms = max_ms
            ));
        }
        
        output.trim_end().to_string()
    }
    
    fn format_simple(intervals: &[Interval]) -> String {
        intervals
            .iter()
            .map(|interval| {
                format!(
                    "{}|{}|{}",
                    interval.from_pattern,
                    interval.to_pattern,
                    interval.duration.num_milliseconds()
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn escape_csv(s: &str) -> String {
        s.replace('"', "\"\"")
    }
    
    fn escape_tsv(s: &str) -> String {
        s.replace('\t', "    ").replace('\n', " ")
    }
    
    fn format_waterfall(intervals: &[Interval]) -> String {
        if intervals.is_empty() {
            return String::new();
        }
        
        const MAX_HEIGHT: usize = 40;
        const MIN_HEIGHT: usize = 1;
        const SCREEN_WIDTH: usize = 100;
        
        // Calculate the maximum duration in milliseconds for normalization
        let max_duration_ms = intervals
            .iter()
            .map(|i| i.duration.num_milliseconds())
            .max()
            .unwrap_or(1) // Avoid division by zero
            .max(1); // Ensure at least 1ms
        
        // Calculate height for each interval (proportional to duration)
        let heights: Vec<usize> = intervals
            .iter()
            .map(|interval| {
                let duration_ms = interval.duration.num_milliseconds();
                let normalized = (duration_ms as f64 / max_duration_ms as f64) * (MAX_HEIGHT as f64);
                normalized.ceil().max(MIN_HEIGHT as f64) as usize
            })
            .collect();
        
        let num_intervals = intervals.len();
        
        // Calculate width per interval (spread evenly)
        let width_per_interval = if num_intervals > 0 {
            (SCREEN_WIDTH / num_intervals).max(1)
        } else {
            1
        };
        
        // Find the maximum height we'll actually use
        let actual_max_height = *heights.iter().max().unwrap_or(&MIN_HEIGHT);
        
        let mut output = String::new();
        
        // Draw from top to bottom
        for row in (1..=actual_max_height).rev() {
            for (i, &height) in heights.iter().enumerate() {
                // Draw the bar if we're within its height
                if row <= height {
                    output.push('|');
                } else {
                    output.push(' ');
                }
                
                // Add spacing between bars (except for last one)
                if i < num_intervals - 1 {
                    for _ in 1..width_per_interval {
                        output.push(' ');
                    }
                }
            }
            output.push('\n');
        }
        
        // Add a baseline
        for i in 0..num_intervals {
            output.push('=');
            if i < num_intervals - 1 {
                for _ in 1..width_per_interval {
                    output.push('=');
                }
            }
        }
        output.push('\n');
        
        // Add labels for each interval (showing pattern transitions)
        for (i, interval) in intervals.iter().enumerate() {
            let label = format!("{}→{}", 
                Self::truncate_label(&interval.from_pattern, 8),
                Self::truncate_label(&interval.to_pattern, 8));
            
            output.push_str(&format!("\n{}: {} ({})", 
                i + 1, 
                label,
                interval.format_duration()));
        }
        
        output
    }
    
    fn truncate_label(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            format!("{}...", &s[..max_len.saturating_sub(3)])
        }
    }
}
