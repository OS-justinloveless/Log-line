# Output Format Comparison

This document shows all available output formats with the same example data.

## Quick Reference

| Format | Best For | Flag |
|--------|----------|------|
| **human** | Human reading, logs | `--format human` |
| **json** | APIs, jq, scripts | `--format json` |
| **csv** | Spreadsheets, databases | `--format csv` |
| **tsv** | Unix tools (awk, cut) | `--format tsv` |
| **table** | Terminal display | `--format table` |
| **simple** | Shell scripts, minimal | `--format simple` |
| **waterfall** | Visual duration comparison | `--format waterfall` |

---

## 1. Human Format (Default)

```
Starting request processing :::: 2s 0ms ::::> Database query completed
Database query completed :::: 2s 0ms ::::> Response sent to client
Response sent to client :::: 5s 0ms ::::> Starting request processing
```

**Pros:**
- Very readable
- Shows flow direction with arrows
- Clear start/end patterns

**Cons:**
- Harder to parse programmatically
- Takes more horizontal space

---

## 2. JSON Format

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

**Pros:**
- Standard format, widely supported
- Easy to parse in any language
- Includes both raw and formatted durations
- Works with jq for powerful queries

**Cons:**
- Verbose
- Not human-friendly for quick viewing

**Example Usage:**
```bash
# Find slowest intervals
log-time-analyzer -l app.log -f json | jq 'max_by(.duration_ms)'

# Get average duration
log-time-analyzer -l app.log -f json | jq '[.[].duration_ms] | add / length'

# Filter by pattern
log-time-analyzer -l app.log -f json | jq '.[] | select(.from_pattern | contains("Database"))'
```

---

## 3. CSV Format

```csv
from_pattern,to_pattern,duration_ms,duration_human
"Starting request processing","Database query completed",2000,"2s 0ms"
"Database query completed","Response sent to client",2000,"2s 0ms"
"Response sent to client","Starting request processing",5000,"5s 0ms"
```

**Pros:**
- Standard format for data exchange
- Opens directly in Excel/Google Sheets
- Easy database import
- Handles commas in patterns correctly

**Cons:**
- Quotes can make manual editing tricky
- Slightly verbose

**Example Usage:**
```bash
# Save to file
log-time-analyzer -l app.log -f csv > analysis.csv

# Import to PostgreSQL
psql -c "COPY intervals FROM '/path/to/analysis.csv' CSV HEADER"

# Load in Python
import pandas as pd
df = pd.read_csv('analysis.csv')
print(df.groupby('from_pattern')['duration_ms'].mean())
```

---

## 4. TSV Format

```
from_pattern	to_pattern	duration_ms	duration_human
Starting request processing	Database query completed	2000	2s 0ms
Database query completed	Response sent to client	2000	2s 0ms
Response sent to client	Starting request processing	5000	5s 0ms
```

**Pros:**
- Works perfectly with Unix tools (awk, cut, sort)
- Clean visual separation
- No quoting issues
- Easy to paste into documents

**Cons:**
- Less standard than CSV
- Issues if patterns contain tabs (though we handle this)

**Example Usage:**
```bash
# Get just the durations
log-time-analyzer -l app.log -f tsv | cut -f3

# Calculate average
log-time-analyzer -l app.log -f tsv | awk 'NR>1 {sum+=$3; n++} END {print sum/n}'

# Sort by duration
log-time-analyzer -l app.log -f tsv | sort -t$'\t' -k3 -n

# Count occurrences
log-time-analyzer -l app.log -f tsv | cut -f1 | sort | uniq -c
```

---

## 5. Table Format

```
| From Pattern                | To Pattern                  | Duration | Duration (ms) |
|-----------------------------|-----------------------------|----------|---------------|
| Starting request processing | Database query completed    | 2s 0ms   |          2000 |
| Database query completed    | Response sent to client     | 2s 0ms   |          2000 |
| Response sent to client     | Starting request processing | 5s 0ms   |          5000 |
```

**Pros:**
- Beautiful terminal output
- Aligned columns for easy reading
- Professional-looking for reports
- Includes both human and raw durations

**Cons:**
- Not meant for parsing
- Takes more vertical space

**Example Usage:**
```bash
# Display in terminal
log-time-analyzer -l app.log -f table

# Save to markdown file
log-time-analyzer -l app.log -f table > report.md

# Pipe to less for large outputs
log-time-analyzer -l app.log -f table | less -S
```

---

## 6. Simple Format

```
Starting request processing|Database query completed|2000
Database query completed|Response sent to client|2000
Response sent to client|Starting request processing|5000
```

**Pros:**
- Minimal output
- Easy to parse in shell scripts
- Only milliseconds (consistent unit)
- No headers (useful for appending)

**Cons:**
- No human-readable duration
- Less descriptive

**Example Usage:**
```bash
# Get specific pattern durations
log-time-analyzer -l app.log -f simple | grep "Database" | cut -d'|' -f3

# Calculate sum
log-time-analyzer -l app.log -f simple | cut -d'|' -f3 | paste -sd+ | bc

# Find max
log-time-analyzer -l app.log -f simple | cut -d'|' -f3 | sort -n | tail -1

# Quick pattern matching
log-time-analyzer -l app.log -f simple | while IFS='|' read from to ms; do
  echo "$from -> $to took ${ms}ms"
done
```

---

## 7. Waterfall Format

```
                                                            |                        
                                                            |                        
                                                            |                        
                                                            |                        
                        |           |                       |                        
                        |           |                       |                        
                        |           |                       |                        
|           |           |           |           |           |           |            
|           |           |           |           |           |           |           |
=====================================================================================

1: Start...→Datab... (2s 0ms)
2: Datab...→Respo... (2s 0ms)
3: Respo...→Start... (5s 0ms)
4: Start...→Datab... (5s 0ms)
5: Datab...→Respo... (2s 0ms)
6: Respo...→Start... (43s 0ms)
7: Start...→Datab... (2s 0ms)
8: Datab...→Respo... (1s 0ms)
```

**Pros:**
- Visual comparison of relative durations
- Easy to spot outliers at a glance
- Shows temporal progression horizontally
- Height proportional to duration (max 40 rows)
- Works well in terminals

**Cons:**
- Not meant for parsing
- Requires adequate terminal width
- Less detailed than table format

**Example Usage:**
```bash
# Quick visual overview of performance
log-time-analyzer -l app.log -f waterfall

# Identify bottlenecks visually
log-time-analyzer -l app.log -f waterfall | less -S
```

**How It Works:**
- Each interval is represented by a vertical bar (`|`)
- Bar height is proportional to the duration (normalized to max 40 rows)
- Intervals are spread evenly across 100 columns
- Minimum height is 1 row (even for very short durations)
- Labels show pattern transitions below the waterfall

---

## Choosing the Right Format

### For Humans
- **Quick view in terminal**: `table` or `human`
- **Visual duration comparison**: `waterfall`
- **Documentation/reports**: `table`
- **Debugging logs**: `human`

### For Machines
- **APIs and web services**: `json`
- **Spreadsheet analysis**: `csv`
- **Shell script processing**: `tsv` or `simple`
- **Database import**: `csv` or `tsv`
- **Data science/ML**: `json` or `csv`

### For Pipelines
- **With jq**: `json`
- **With awk/cut/sort**: `tsv`
- **With Python pandas**: `csv`
- **With minimal tools**: `simple`

---

## Performance Notes

All formats have similar performance characteristics. The primary difference is the output formatting, not the parsing speed.

- **Fastest to generate**: `simple` (minimal formatting)
- **Most compact**: `simple` (no headers or formatting)
- **Most compatible**: `csv` or `json` (widely supported)
