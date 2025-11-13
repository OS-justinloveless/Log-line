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
    
    /// Path to the YAML configuration file
    #[arg(short, long, default_value = "config.yaml")]
    config: PathBuf,
    
    /// Output format: human, json, csv, tsv, table, or simple
    #[arg(short = 'f', long, default_value = "human")]
    format: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Parse output format
    let output_format = OutputFormat::from_str(&args.format)
        .ok_or_else(|| anyhow::anyhow!(
            "Invalid output format '{}'. Valid options: human, json, csv, tsv, table, simple",
            args.format
        ))?;
    
    // Load configuration
    let config = Config::from_file(&args.config)
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
