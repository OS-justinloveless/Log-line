# Quick Start Guide

Get started with log-time-analyzer in under 5 minutes!

## Installation

```bash
cargo build --release
```

The binary will be at `target/release/log-time-analyzer`.

## Three Ways to Use It

### 1. With Config File (Recommended)

**Create `config.yaml`:**
```yaml
timestamp_regex: '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})'
timestamp_format: '%Y-%m-%d %H:%M:%S'
message_patterns:
  - 'Starting request processing'
  - 'Database query completed'
  - 'Response sent to client'
```

**Run:**
```bash
./target/release/log-time-analyzer -l example.log -c config.yaml
```

**Output:**
```
Starting request processing :::: 2s 0ms ::::> Database query completed
Database query completed :::: 2s 0ms ::::> Response sent to client
Response sent to client :::: 5s 0ms ::::> Starting request processing
```

---

### 2. Override Config with CLI Args

Use the config file but change specific values:

```bash
# Change patterns only
./log-time-analyzer -l app.log -c config.yaml \
  -p "ERROR" -p "RESOLVED"

# Change format
./log-time-analyzer -l app.log -c config.yaml \
  -f json

# Change everything
./log-time-analyzer -l app.log -c config.yaml \
  -r '(\d{2}:\d{2}:\d{2})' \
  -t '%H:%M:%S' \
  -p "BEGIN" -p "END"
```

---

### 3. Pure CLI (No Config File)

Provide everything via command line:

```bash
./log-time-analyzer \
  -l app.log \
  -r '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
  -t '%Y-%m-%d %H:%M:%S' \
  -p "Start" \
  -p "Finish"
```

---

## Output Formats

Choose from 6 output formats:

```bash
# Human-readable (default)
./log-time-analyzer -l app.log -c config.yaml

# JSON for scripts
./log-time-analyzer -l app.log -c config.yaml -f json

# CSV for Excel
./log-time-analyzer -l app.log -c config.yaml -f csv

# TSV for Unix tools
./log-time-analyzer -l app.log -c config.yaml -f tsv

# Pretty table
./log-time-analyzer -l app.log -c config.yaml -f table

# Simple pipe-separated
./log-time-analyzer -l app.log -c config.yaml -f simple
```

---

## Common Use Cases

### Find Average Duration

```bash
# Using TSV and awk
./log-time-analyzer -l app.log -c config.yaml -f tsv | \
  awk 'NR>1 {sum+=$3; count++} END {print "Average:", sum/count, "ms"}'
```

### Find Slowest Operations

```bash
# Using JSON and jq
./log-time-analyzer -l app.log -c config.yaml -f json | \
  jq 'sort_by(.duration_ms) | reverse | .[0:5]'
```

### Monitor Specific Patterns

```bash
# Quick pattern change
./log-time-analyzer -l app.log -c config.yaml \
  -p "Login" -p "Logout"
```

### Export to Spreadsheet

```bash
./log-time-analyzer -l app.log -c config.yaml -f csv > analysis.csv
```

---

## Options Reference

### Required
- `-l, --log-file` - Log file to analyze

### Configuration (choose one)
- `-c, --config` - Config file path
- **OR** provide all of:
  - `-r, --timestamp-regex` - Regex to extract timestamp
  - `-t, --timestamp-format` - Timestamp format string
  - `-p, --pattern` - Patterns to search (use multiple times)

### Optional
- `-f, --format` - Output format (default: human)

---

## Tips

1. **Start with the example:** Run `./log-time-analyzer -l example.log -c config.yaml` first
2. **Test patterns:** Use a small log sample to test your regex patterns
3. **Use variables:** Store complex regex in shell variables for reusability
4. **Chain with jq:** Combine with `jq` for powerful JSON processing
5. **Save config:** Create config files for different log types

---

## Troubleshooting

**No matches found?**
- Check your timestamp regex has a capture group: `(timestamp)`
- Verify your timestamp format string matches the captured text
- Ensure patterns match your log messages

**Timestamp parsing error?**
- Double-check the chrono format string
- Look at [chrono format docs](https://docs.rs/chrono/latest/chrono/format/strftime/)

**Need help?**
```bash
./log-time-analyzer --help
```

---

## Next Steps

- **[README.md](README.md)** - Complete documentation
- **[OUTPUT_FORMATS.md](OUTPUT_FORMATS.md)** - Detailed format comparison
- **[CLI_EXAMPLES.md](CLI_EXAMPLES.md)** - Advanced examples and recipes

---

## Example Workflow

```bash
# 1. Test with example
./log-time-analyzer -l example.log -c config.yaml

# 2. Try different patterns
./log-time-analyzer -l example.log -c config.yaml \
  -p "Starting" -p "Database"

# 3. Change output format
./log-time-analyzer -l example.log -c config.yaml -f json

# 4. Use on your own logs
./log-time-analyzer -l /var/log/myapp.log \
  -r '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
  -t '%Y-%m-%d %H:%M:%S' \
  -p "Request started" \
  -p "Request completed"

# 5. Analyze the results
./log-time-analyzer -l /var/log/myapp.log -c myconfig.yaml -f json | \
  jq '[.[].duration_ms] | add / length'
```

That's it! You're ready to analyze your logs. 🚀
