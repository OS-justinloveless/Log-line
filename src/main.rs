mod config;
mod parser;
mod analyzer;

use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use std::path::PathBuf;

use config::Config;
use parser::LogParser;
use analyzer::Analyzer;

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
}

fn main() -> Result<()> {
    let args = Args::parse();
    
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
    
    // Output results
    for interval in intervals {
        println!("{}", interval.format());
    }
    
    Ok(())
}
