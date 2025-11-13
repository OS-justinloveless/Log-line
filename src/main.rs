mod config;
mod parser;
mod analyzer;
mod output;

use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use std::path::PathBuf;

use config::Config;
use parser::LogParser;
use analyzer::Analyzer;
use output::{OutputFormat, OutputFormatter};

#[derive(ClapParser, Debug)]
#[command(name = "log-time-analyzer")]
#[command(about = "Analyze log files to find time intervals between specific message patterns", long_about = None)]
struct Args {
    /// Path to the log file to analyze
    #[arg(short, long)]
    log_file: PathBuf,
    
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
    
    // Parse log file
    let matches = parser.parse_file(&args.log_file)
        .context("Failed to parse log file")?;
    
    if matches.is_empty() {
        eprintln!("No matching patterns found in log file");
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
