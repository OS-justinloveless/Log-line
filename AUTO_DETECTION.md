# Automatic Timestamp Detection

The log-time-analyzer now includes automatic timestamp detection for common log file formats. This means you can analyze logs without manually specifying the timestamp format in most cases.

## How It Works

When you provide message patterns without specifying `--timestamp-regex` and `--timestamp-format`, the tool will automatically try to detect the timestamp format using a library of built-in patterns.

## Supported Formats

The tool automatically recognizes these timestamp formats:

1. **ISO 8601 with timezone**
   - Format: `2025-11-13T10:00:00.123+00:00`
   - Common in: Modern APIs, microservices

2. **ISO 8601 without timezone**
   - Format: `2025-11-13T10:00:00.123`
   - Common in: Application logs, JSON logs

3. **RFC 3339**
   - Format: `2025-11-13 10:00:00.123+00:00`
   - Common in: Syslog, structured logs

4. **Common log format (YYYY-MM-DD HH:MM:SS)**
   - Format: `2025-11-13 10:00:00`
   - Common in: Application logs, database logs

5. **Common log format with milliseconds**
   - Format: `2025-11-13 10:00:00.123`
   - Common in: High-precision logs

6. **Apache/Nginx common log format**
   - Format: `[13/Nov/2025:10:00:00 +0000]`
   - Common in: Web server access logs

7. **Syslog format (RFC 3164)**
   - Format: `Nov 13 10:00:00`
   - Common in: System logs, syslog

8. **Windows Event Log format**
   - Format: `11/13/2025 10:00:00 AM`
   - Common in: Windows event logs

9. **Unix timestamp (seconds)**
   - Format: `1699876800`
   - Common in: System logs, metrics

10. **Unix timestamp (milliseconds)**
    - Format: `1699876800123`
    - Common in: JavaScript logs, metrics

11. **US date format with time**
    - Format: `11/13/2025 10:00:00`
    - Common in: US-based systems

12. **European date format with time**
    - Format: `13.11.2025 10:00:00`
    - Common in: European systems

13. **Java log format**
    - Format: `2025-11-13 10:00:00,123`
    - Common in: Java applications (Log4j, etc.)

14. **Python logging format**
    - Format: `2025-11-13 10:00:00,123`
    - Common in: Python applications

15. **Compact format**
    - Format: `20251113_100000`
    - Common in: File names, compact logs

16. **Compact format with milliseconds**
    - Format: `20251113_100000.123`
    - Common in: High-precision compact logs

## Usage Examples

### Basic Auto-Detection

```bash
# Analyze a log file without specifying timestamp format
log-time-analyzer --log-file app.log \
  --pattern "Starting request" \
  --pattern "Request completed"
```

### With Piped Input

```bash
# Analyze logs from stdin
cat app.log | log-time-analyzer \
  --pattern "Starting request" \
  --pattern "Request completed"
```

### Multiple Patterns

```bash
# Track multiple steps in a process
log-time-analyzer --log-file server.log \
  --pattern "Request received" \
  --pattern "Database query started" \
  --pattern "Database query completed" \
  --pattern "Response sent"
```

## Manual Override

If your log file uses a non-standard timestamp format, you can still manually specify it:

```bash
log-time-analyzer --log-file custom.log \
  --timestamp-regex '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
  --timestamp-format '%Y-%m-%d %H:%M:%S' \
  --pattern "Event A" \
  --pattern "Event B"
```

## Error Messages

If auto-detection fails (no timestamps found), you'll see a helpful error message:

```
Error: No matching patterns found in log file with timestamps.

The automatic timestamp detection could not find any log lines with recognizable timestamps.
This could mean:
  1. The log file doesn't contain timestamps in a supported format
  2. The timestamp format is non-standard

Supported timestamp formats include:
  - ISO 8601: 2025-11-13T10:00:00.123+00:00
  - Common log: 2025-11-13 10:00:00
  - Apache/Nginx: [13/Nov/2025:10:00:00 +0000]
  - Syslog: Nov 13 10:00:00
  - Unix timestamp: 1699876800
  - And many more...

To manually specify a timestamp format, use:
  --timestamp-regex '<regex_pattern>' --timestamp-format '<chrono_format>'

Example:
  --timestamp-regex '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
  --timestamp-format '%Y-%m-%d %H:%M:%S'
```

## Examples with Different Formats

### ISO 8601 Format

```bash
# Sample log content:
# 2025-11-13T10:00:00.123Z [INFO] Starting request processing
# 2025-11-13T10:00:02.456Z [INFO] Database query completed

echo -e "2025-11-13T10:00:00.123Z [INFO] Starting request\n2025-11-13T10:00:02.456Z [INFO] Completed" | \
  log-time-analyzer --pattern "Starting" --pattern "Completed"
```

### Apache/Nginx Format

```bash
# Sample log content:
# [13/Nov/2025:10:00:00 +0000] "GET /api" 200
# [13/Nov/2025:10:00:03 +0000] "GET /api" 200

echo -e '[13/Nov/2025:10:00:00 +0000] Starting\n[13/Nov/2025:10:00:03 +0000] Completed' | \
  log-time-analyzer --pattern "Starting" --pattern "Completed"
```

### Unix Timestamp Format

```bash
# Sample log content:
# 1699876800 [INFO] Request started
# 1699876803 [INFO] Request completed

echo -e "1699876800 Starting\n1699876803 Completed" | \
  log-time-analyzer --pattern "Starting" --pattern "Completed"
```

## Benefits

- **Ease of use**: No need to figure out regex patterns and chrono format strings
- **Flexibility**: Works with most common log formats out of the box
- **Clear error messages**: Helpful guidance when auto-detection fails
- **Backward compatible**: Existing config files and manual formats still work

