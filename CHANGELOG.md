# Changelog - Automatic Timestamp Detection

## Summary

Added automatic timestamp detection feature that allows the tool to recognize and parse common timestamp formats without requiring manual configuration.

## Changes Made

### 1. New Module: `timestamp_formats.rs`
- Created a new module with 16+ built-in timestamp format definitions
- Each format includes:
  - Name (descriptive)
  - Regex pattern for extraction
  - Chrono format string for parsing
  - Example timestamp
- Comprehensive unit tests to validate all formats

### 2. Modified: `config.rs`
- Added `is_auto_detect` field to `Config` struct
- Created `for_auto_detection()` method for creating auto-detect configs
- Updated `from_file_with_overrides()` to support auto-detection mode
- Modified validation to skip timestamp validation when in auto-detect mode
- Made Config struct derive Clone for easier handling

### 3. Modified: `parser.rs`
- Updated `LogParser` struct to hold optional timestamp regex/format
- Added `builtin_formats` field to store compiled built-in formats
- Added `is_auto_detect` flag
- Modified `new()` to compile all built-in formats when in auto-detect mode
- Updated `extract_timestamp()` to try each built-in format sequentially
- Falls back through formats until one successfully parses

### 4. Modified: `main.rs`
- Added import for new `timestamp_formats` module
- Enhanced error handling for failed auto-detection
- Added comprehensive error message when no timestamps are detected
- Error message includes:
  - List of supported formats with examples
  - Instructions for manual override
  - Example command showing how to specify custom format

### 5. Documentation
- Created `AUTO_DETECTION.md` with comprehensive guide
- Updated `README.md` to highlight auto-detection as primary feature
- Added examples for all supported timestamp formats
- Updated usage instructions to show auto-detection first

## Supported Timestamp Formats

1. ISO 8601 with timezone: `2025-11-13T10:00:00.123+00:00`
2. ISO 8601 without timezone: `2025-11-13T10:00:00.123`
3. RFC 3339: `2025-11-13 10:00:00.123+00:00`
4. Common log format: `2025-11-13 10:00:00`
5. Common log with milliseconds: `2025-11-13 10:00:00.123`
6. Apache/Nginx: `[13/Nov/2025:10:00:00 +0000]`
7. Syslog RFC 3164: `Nov 13 10:00:00`
8. Syslog RFC 5424: `2025-11-13T10:00:00.123+00:00`
9. Windows Event Log: `11/13/2025 10:00:00 AM`
10. Unix timestamp (seconds): `1699876800`
11. Unix timestamp (milliseconds): `1699876800123`
12. US date format: `11/13/2025 10:00:00`
13. European date format: `13.11.2025 10:00:00`
14. Java log format: `2025-11-13 10:00:00,123`
15. Python logging: `2025-11-13 10:00:00,123`
16. Compact format: `20251113_100000`
17. Compact with milliseconds: `20251113_100000.123`

## Backward Compatibility

✅ All existing functionality preserved:
- YAML config files still work as before
- Manual timestamp specification via CLI still works
- All output formats unchanged
- All existing command-line flags work the same way

## Usage Examples

### Before (required manual configuration)
```bash
./log-time-analyzer -l app.log \
  -r '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
  -t '%Y-%m-%d %H:%M:%S' \
  -p "Starting" -p "Finished"
```

### After (auto-detection)
```bash
./log-time-analyzer -l app.log \
  -p "Starting" -p "Finished"
```

## Testing

All tests pass successfully:
- ✅ Unit tests for all built-in formats
- ✅ Integration test with example.log
- ✅ Error message test with no timestamps
- ✅ Multiple format tests (ISO 8601, Apache, Unix timestamp)
- ✅ Stdin input test
- ✅ JSON output format test
- ✅ Backward compatibility with config files

## Performance Impact

Minimal performance impact:
- Formats are compiled once at startup
- Sequential testing stops at first match
- Most logs match within first 2-3 formats
- No noticeable delay in typical usage

## Benefits

1. **Easier to use**: No need to figure out regex patterns and format strings
2. **Faster setup**: Works immediately with most log files
3. **Less error-prone**: No manual regex writing reduces mistakes
4. **Better UX**: Clear error messages when detection fails
5. **Still flexible**: Manual override always available for edge cases

