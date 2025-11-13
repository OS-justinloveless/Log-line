# Log Time Analyzer

A CLI utility written in Rust that parses log files to find specific messages and calculates the time intervals between them based on timestamps.

## Features

- Parse log files with custom timestamp formats
- Search for multiple message patterns using regular expressions
- Calculate time intervals between consecutive pattern matches
- Configure via YAML file
- Human-readable duration output

## Installation

Build the project using Cargo:

```bash
cargo build --release
```

The binary will be available at `target/release/log-time-analyzer`.

## Configuration

Create a YAML configuration file (default: `config.yaml`) with the following structure:

```yaml
# Regular expression to extract timestamps from log lines
# Must have a capture group around the timestamp
timestamp_regex: '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})'

# Format string for parsing timestamps (chrono format)
timestamp_format: '%Y-%m-%d %H:%M:%S'

# Array of message patterns to search for in order
# Patterns are matched as regular expressions
message_patterns:
  - 'Starting request processing'
  - 'Database query completed'
  - 'Response sent to client'
```

### Configuration Parameters

- **timestamp_regex**: A regular expression with a capture group to extract the timestamp from each log line
- **timestamp_format**: A [chrono strftime format string](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) to parse the extracted timestamp
- **message_patterns**: An array of regular expression patterns to search for in the log file

## Usage

Basic usage:

```bash
./target/release/log-time-analyzer --log-file example.log --config config.yaml
```

### Command Line Options

- `-l, --log-file <PATH>`: Path to the log file to analyze (required)
- `-c, --config <PATH>`: Path to the YAML configuration file (default: `config.yaml`)
- `-h, --help`: Print help information

## Output Format

The tool outputs one line per interval found, in the following format:

```
Message pattern A :::: duration ::::> Message pattern B
```

Where `duration` is formatted in a human-readable way:
- Hours, minutes, seconds (e.g., `1h 30m 45s`)
- Minutes, seconds (e.g., `2m 30s`)
- Seconds, milliseconds (e.g., `5s 123ms`)
- Milliseconds only (e.g., `250ms`)

### Example Output

```
Starting request processing :::: 2s 0ms ::::> Database query completed
Database query completed :::: 2s 0ms ::::> Response sent to client
Response sent to client :::: 5s 0ms ::::> Starting request processing
Starting request processing :::: 5s 0ms ::::> Database query completed
Database query completed :::: 2s 0ms ::::> Response sent to client
Response sent to client :::: 43s 0ms ::::> Starting request processing
Starting request processing :::: 2s 0ms ::::> Database query completed
Database query completed :::: 1s 0ms ::::> Response sent to client
```

## Example Log File

An example log file (`example.log`) is included with timestamps in ISO format:

```
2025-11-13 10:00:00 [INFO] Application started
2025-11-13 10:00:01 [INFO] Starting request processing
2025-11-13 10:00:02 [DEBUG] Connecting to database
2025-11-13 10:00:03 [INFO] Database query completed
2025-11-13 10:00:04 [DEBUG] Processing results
2025-11-13 10:00:05 [INFO] Response sent to client
...
```

## Advanced Usage

### Different Log Formats

For logs with different timestamp formats, adjust the `timestamp_regex` and `timestamp_format`:

**Apache-style logs:**
```yaml
timestamp_regex: '\[(\d{2}/\w{3}/\d{4}:\d{2}:\d{2}:\d{2})'
timestamp_format: '%d/%b/%Y:%H:%M:%S'
```

**RFC 3339 timestamps:**
```yaml
timestamp_regex: '(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})'
timestamp_format: '%Y-%m-%dT%H:%M:%S'
```

### Complex Pattern Matching

Use regular expressions for more flexible pattern matching:

```yaml
message_patterns:
  - 'Starting request \d+ processing'
  - 'Query executed in \d+ms'
  - 'Response code: (200|201|204)'
```

## Error Handling

The tool will provide helpful error messages for:
- Missing or invalid configuration files
- Invalid regular expressions
- Timestamp parsing errors
- Missing log files
- Configurations with fewer than 2 message patterns

## Dependencies

- `clap` - Command line argument parsing
- `serde` / `serde_yaml` - YAML configuration parsing
- `regex` - Regular expression matching
- `chrono` - Timestamp parsing and duration calculations
- `anyhow` - Error handling

## License

This project is available for use under standard open source practices.
