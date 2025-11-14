mod config;
mod parser;
mod analyzer;
mod output;
mod timestamp_formats;

use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use std::path::PathBuf;
use std::io::{self, IsTerminal};

use config::Config;
use parser::LogParser;
use analyzer::Analyzer;
use output::{OutputFormat, OutputFormatter};

#[derive(ClapParser, Debug)]
#[command(name = "log-time-analyzer")]
#[command(about = "Analyze log files to find time intervals between specific message patterns", long_about = None)]
struct Args {
    /// Path to the log file to analyze (omit to read from stdin)
    #[arg(short, long)]
    log_file: Option<PathBuf>,
    
    /// Path to the YAML configuration file (optional if CLI args provided)
    #[arg(short, long)]
    config: Option<PathBuf>,
    
    /// Output format: human, json, csv, tsv, table, simple, or waterfall
    #[arg(short = 'f', long, default_value = "human")]
    format: String,
    
    /// Regular expression to extract timestamps (overrides config file)
    #[arg(short = 'r', long)]
    timestamp_regex: Option<String>,
    
    /// Timestamp format string using chrono format (overrides config file)
    #[arg(short = 't', long)]
    timestamp_format: Option<String>,
    
    /// Message patterns to search for (can be specified multiple times, overrides config file)
    #[arg(short = 'p', long = "pattern")]
    patterns: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Parse output format
    let output_format = OutputFormat::from_str(&args.format)
        .ok_or_else(|| anyhow::anyhow!(
            "Invalid output format '{}'. Valid options: human, json, csv, tsv, table, simple, waterfall",
            args.format
        ))?;
    
    // Load configuration with CLI overrides
    let patterns = if args.patterns.is_empty() {
        None
    } else {
        Some(args.patterns)
    };
    
    let config = Config::from_file_with_overrides(
        args.config.as_deref(),
        args.timestamp_regex,
        args.timestamp_format,
        patterns,
    )
    .context("Failed to load configuration")?;
    
    // Create parser
    let parser = LogParser::new(&config)
        .context("Failed to create log parser")?;
    
    // Parse log from file or stdin
    let matches = if let Some(log_file) = args.log_file {
        // Parse from file
        parser.parse_file(&log_file)
            .context("Failed to parse log file")?
    } else {
        // Check if stdin is a terminal (not piped)
        if io::stdin().is_terminal() {
            anyhow::bail!("No log file provided and stdin is not piped. Use --log-file or pipe input.");
        }
        
        // Parse from stdin
        let stdin = io::stdin();
        let reader = stdin.lock();
        parser.parse_reader(reader)
            .context("Failed to parse log from stdin")?
    };
    
    if matches.is_empty() {
        if config.is_auto_detect {
            eprintln!("Error: No matching patterns found in log file with timestamps.");
            eprintln!();
            eprintln!("The automatic timestamp detection could not find any log lines with recognizable timestamps.");
            eprintln!("This could mean:");
            eprintln!("  1. The log file doesn't contain timestamps in a supported format");
            eprintln!("  2. The timestamp format is non-standard");
            eprintln!();
            eprintln!("Supported timestamp formats include:");
            eprintln!("  - ISO 8601: 2025-11-13T10:00:00.123+00:00");
            eprintln!("  - Common log: 2025-11-13 10:00:00");
            eprintln!("  - Apache/Nginx: [13/Nov/2025:10:00:00 +0000]");
            eprintln!("  - Syslog: Nov 13 10:00:00");
            eprintln!("  - Unix timestamp: 1699876800");
            eprintln!("  - And many more...");
            eprintln!();
            eprintln!("To manually specify a timestamp format, use:");
            eprintln!("  --timestamp-regex '<regex_pattern>' --timestamp-format '<chrono_format>'");
            eprintln!();
            eprintln!("Example:");
            eprintln!("  --timestamp-regex '(\\d{{4}}-\\d{{2}}-\\d{{2}} \\d{{2}}:\\d{{2}}:\\d{{2}})' \\");
            eprintln!("  --timestamp-format '%Y-%m-%d %H:%M:%S'");
            anyhow::bail!("No timestamps could be detected automatically");
        } else {
            eprintln!("No matching patterns found in log file");
        }
        return Ok(());
    }
    
    // Analyze and find intervals
    let intervals = Analyzer::analyze(matches);
    
    if intervals.is_empty() {
        eprintln!("Not enough matches to calculate intervals");
        return Ok(());
    }
    
    // Format and output results
    let output = OutputFormatter::format_intervals(&intervals, output_format);
    println!("{}", output);
    
    Ok(())
}
