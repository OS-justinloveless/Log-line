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

You can configure the tool either through a YAML file or via command-line arguments. CLI arguments take precedence over the config file.

### YAML Configuration

Create a YAML configuration file with the following structure:

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

### CLI Configuration Override

All configuration values can be provided or overridden via command-line arguments:

```bash
# Override just the patterns
./log-time-analyzer -l app.log -c config.yaml \
  -p "Starting" -p "Finished"

# Override timestamp handling
./log-time-analyzer -l app.log -c config.yaml \
  -r '(\d{2}:\d{2}:\d{2}\.\d{3})' \
  -t '%H:%M:%S%.3f'

# Run without config file (all values via CLI)
./log-time-analyzer -l app.log \
  -r '(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})' \
  -t '%Y-%m-%dT%H:%M:%S' \
  -p "ERROR" -p "WARN" -p "INFO"
```

This is particularly useful for:
- Quick one-off analyses without creating a config file
- Testing different patterns without modifying the config file
- Scripting and automation where configs are dynamically generated
- CI/CD pipelines where you want to parameterize the analysis

## Usage

Basic usage:

```bash
./target/release/log-time-analyzer --log-file example.log --config config.yaml
```

### Command Line Options

#### Required Arguments

- `-l, --log-file <PATH>`: Path to the log file to analyze

#### Configuration Arguments

You can either use a YAML config file or provide all settings via CLI:

- `-c, --config <PATH>`: Path to the YAML configuration file (optional if CLI options provided)
- `-r, --timestamp-regex <REGEX>`: Regular expression to extract timestamps (overrides config)
- `-t, --timestamp-format <FORMAT>`: Timestamp format string using chrono format (overrides config)
- `-p, --pattern <PATTERN>`: Message pattern to search for (can be specified multiple times, overrides config)

**Note:** When no config file is provided, you must specify `-r`, `-t`, and at least two `-p` arguments.

#### Output Options

- `-f, --format <FORMAT>`: Output format (default: `human`)
  - `human` - Human-readable format with arrows
  - `json` - JSON format for programmatic consumption
  - `csv` - CSV format for spreadsheets
  - `tsv` - Tab-separated values
  - `table` - Formatted table with aligned columns
  - `simple` - Pipe-separated format with milliseconds only

#### Other Options

- `-h, --help`: Print help information

### Usage Patterns

**Using config file:**
```bash
./log-time-analyzer --log-file app.log --config config.yaml
```

**Override specific patterns:**
```bash
./log-time-analyzer -l app.log -c config.yaml -p "Error" -p "Warning"
```

**No config file (all CLI):**
```bash
./log-time-analyzer -l app.log \
  -r '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
  -t '%Y-%m-%d %H:%M:%S' \
  -p "Starting request" \
  -p "Request completed"
```

**Override everything:**
```bash
./log-time-analyzer -l app.log -c config.yaml \
  -r '(\d{2}:\d{2}:\d{2})' \
  -t '%H:%M:%S' \
  -p "BEGIN" -p "END" \
  -f json
```

## Output Formats

The tool supports multiple output formats for both human readability and machine processing:

### 1. Human Format (Default)

Human-friendly format with arrows showing the flow:

```
Starting request processing :::: 2s 0ms ::::> Database query completed
Database query completed :::: 2s 0ms ::::> Response sent to client
Response sent to client :::: 5s 0ms ::::> Starting request processing
```

**Usage:** `--format human` (or omit the flag)

### 2. JSON Format

Structured JSON output perfect for programmatic processing:

```json
[
  {
    "from_pattern": "Starting request processing",
    "to_pattern": "Database query completed",
    "duration_ms": 2000,
    "duration_human": "2s 0ms"
  },
  {
    "from_pattern": "Database query completed",
    "to_pattern": "Response sent to client",
    "duration_ms": 2000,
    "duration_human": "2s 0ms"
  }
]
```

**Usage:** `--format json`

**Use cases:**
- Parsing with `jq` for further analysis
- Importing into monitoring tools
- Processing with Python/JavaScript scripts

### 3. CSV Format

Comma-separated values with headers, ideal for spreadsheets:

```csv
from_pattern,to_pattern,duration_ms,duration_human
"Starting request processing","Database query completed",2000,"2s 0ms"
"Database query completed","Response sent to client",2000,"2s 0ms"
```

**Usage:** `--format csv`

**Use cases:**
- Import into Excel/Google Sheets
- Load into databases
- Analysis with pandas or R

### 4. TSV Format

Tab-separated values for easy text processing:

```
from_pattern	to_pattern	duration_ms	duration_human
Starting request processing	Database query completed	2000	2s 0ms
Database query completed	Response sent to client	2000	2s 0ms
```

**Usage:** `--format tsv`

**Use cases:**
- Processing with `awk`, `cut`, or other Unix tools
- Quick inspection in terminal
- Copy-paste into documents

### 5. Table Format

Pretty-printed table with aligned columns:

```
| From Pattern                | To Pattern                  | Duration | Duration (ms) |
|-----------------------------|-----------------------------|----------|---------------|
| Starting request processing | Database query completed    | 2s 0ms   |          2000 |
| Database query completed    | Response sent to client     | 2s 0ms   |          2000 |
| Response sent to client     | Starting request processing | 5s 0ms   |          5000 |
```

**Usage:** `--format table`

**Use cases:**
- Terminal display
- Documentation and reports
- Quick visual inspection

### 6. Simple Format

Minimal pipe-separated format with durations in milliseconds only:

```
Starting request processing|Database query completed|2000
Database query completed|Response sent to client|2000
Response sent to client|Starting request processing|5000
```

**Usage:** `--format simple`

**Use cases:**
- Simplest parsing in shell scripts
- When you only need raw milliseconds
- Minimal output size

## Duration Formatting

For formats that include human-readable durations, the tool automatically formats them:
- Hours, minutes, seconds: `1h 30m 45s`
- Minutes, seconds: `2m 30s`
- Seconds, milliseconds: `5s 123ms`
- Milliseconds only: `250ms`

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

## Practical Examples

### Piping JSON to jq

Analyze which pattern transitions take the longest:

```bash
./log-time-analyzer -l app.log -f json | jq 'sort_by(.duration_ms) | reverse | .[0:5]'
```

### Import CSV into SQLite

```bash
./log-time-analyzer -l app.log -f csv > intervals.csv
sqlite3 analysis.db ".import intervals.csv intervals"
sqlite3 analysis.db "SELECT from_pattern, AVG(duration_ms) FROM intervals GROUP BY from_pattern;"
```

### Quick stats with awk (TSV format)

Calculate average duration between patterns:

```bash
./log-time-analyzer -l app.log -f tsv | awk 'NR>1 {sum+=$3; count++} END {print "Average:", sum/count, "ms"}'
```

### Filter specific patterns (Simple format)

```bash
./log-time-analyzer -l app.log -f simple | grep "Database query" | cut -d'|' -f3
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

## More Examples

For extensive CLI examples including scripting, automation, and CI/CD integration, see:
- **[CLI_EXAMPLES.md](CLI_EXAMPLES.md)** - Comprehensive CLI usage examples
- **[OUTPUT_FORMATS.md](OUTPUT_FORMATS.md)** - Detailed output format comparison

## Dependencies

- `clap` - Command line argument parsing
- `serde` / `serde_yaml` - YAML configuration parsing
- `serde_json` - JSON output formatting
- `regex` - Regular expression matching
- `chrono` - Timestamp parsing and duration calculations
- `anyhow` - Error handling

## License

This project is available for use under standard open source practices.
