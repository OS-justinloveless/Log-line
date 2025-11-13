# CLI Examples and Use Cases

This document provides practical examples of using the CLI arguments to override configuration.

## Basic Usage

### Using Config File Only

```bash
./log-time-analyzer --log-file app.log --config config.yaml
```

This is the simplest usage - all configuration comes from the YAML file.

---

## Overriding Patterns

### Override Message Patterns

Useful when you want to analyze different patterns without modifying the config:

```bash
# Look for errors and warnings only
./log-time-analyzer -l app.log -c config.yaml \
  -p "ERROR" -p "WARN"

# Track API endpoints
./log-time-analyzer -l api.log -c config.yaml \
  -p "POST /api" -p "Response 200"

# Monitor specific user actions
./log-time-analyzer -l user.log -c config.yaml \
  -p "User login" -p "Session created" -p "Dashboard loaded"
```

---

## Overriding Timestamp Formats

### Different Timestamp Formats

Override timestamp handling for different log formats:

```bash
# ISO 8601 timestamps
./log-time-analyzer -l app.log -c config.yaml \
  -r '(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})' \
  -t '%Y-%m-%dT%H:%M:%S'

# Syslog format
./log-time-analyzer -l syslog.log -c config.yaml \
  -r '(\w{3} \d{2} \d{2}:\d{2}:\d{2})' \
  -t '%b %d %H:%M:%S'

# Millisecond precision
./log-time-analyzer -l app.log -c config.yaml \
  -r '(\d{2}:\d{2}:\d{2}\.\d{3})' \
  -t '%H:%M:%S%.3f'

# Apache log format
./log-time-analyzer -l access.log -c config.yaml \
  -r '\[(\d{2}/\w{3}/\d{4}:\d{2}:\d{2}:\d{2})' \
  -t '%d/%b/%Y:%H:%M:%S'
```

---

## No Config File (Pure CLI)

### Complete CLI Configuration

When you don't want to create a config file:

```bash
# Quick analysis
./log-time-analyzer \
  -l app.log \
  -r '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
  -t '%Y-%m-%d %H:%M:%S' \
  -p "Request started" \
  -p "Request completed"

# With JSON output
./log-time-analyzer \
  -l app.log \
  -r '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
  -t '%Y-%m-%d %H:%M:%S' \
  -p "BEGIN" \
  -p "END" \
  -f json
```

---

## Scripting and Automation

### Dynamic Configuration in Shell Scripts

```bash
#!/bin/bash

LOG_FILE="$1"
START_PATTERN="$2"
END_PATTERN="$3"

./log-time-analyzer \
  -l "$LOG_FILE" \
  -r '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
  -t '%Y-%m-%d %H:%M:%S' \
  -p "$START_PATTERN" \
  -p "$END_PATTERN" \
  -f json

# Usage:
# ./analyze.sh app.log "Starting" "Finished"
```

### Loop Through Multiple Pattern Combinations

```bash
#!/bin/bash

for start in "Login" "Checkout" "Payment"; do
  for end in "Success" "Failure"; do
    echo "Analyzing: $start -> $end"
    ./log-time-analyzer \
      -l app.log \
      -c config.yaml \
      -p "$start" -p "$end" \
      -f simple | awk -F'|' '{sum+=$3; count++} END {print "Average:", sum/count, "ms"}'
  done
done
```

---

## CI/CD Pipeline Examples

### GitHub Actions

```yaml
- name: Analyze API Response Times
  run: |
    ./log-time-analyzer \
      -l ${{ github.workspace }}/logs/api.log \
      -r '\[(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})\]' \
      -t '%Y-%m-%dT%H:%M:%S' \
      -p "API Request" \
      -p "API Response" \
      -f json > response-times.json
    
    # Check if average is acceptable
    AVG=$(jq '[.[].duration_ms] | add / length' response-times.json)
    if (( $(echo "$AVG > 1000" | bc -l) )); then
      echo "Average response time too high: ${AVG}ms"
      exit 1
    fi
```

### GitLab CI

```yaml
analyze_logs:
  script:
    - |
      ./log-time-analyzer \
        -l logs/production.log \
        -r '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
        -t '%Y-%m-%d %H:%M:%S' \
        -p "Transaction started" \
        -p "Transaction completed" \
        -f csv > analysis.csv
    - cat analysis.csv
  artifacts:
    paths:
      - analysis.csv
```

---

## Advanced Use Cases

### Multi-Stage Analysis

Analyze different stages of a process:

```bash
# Stage 1: Request to Auth
./log-time-analyzer -l app.log -c config.yaml \
  -p "Request received" -p "Auth completed" \
  -f simple > stage1.txt

# Stage 2: Auth to Database
./log-time-analyzer -l app.log -c config.yaml \
  -p "Auth completed" -p "Database query done" \
  -f simple > stage2.txt

# Stage 3: Database to Response
./log-time-analyzer -l app.log -c config.yaml \
  -p "Database query done" -p "Response sent" \
  -f simple > stage3.txt

# Combine and analyze
cat stage1.txt stage2.txt stage3.txt | \
  awk -F'|' '{sum+=$3; count++} END {print "Total stages:", count, "Average:", sum/count, "ms"}'
```

### Comparing Different Log Files

```bash
#!/bin/bash

echo "Production vs Staging Response Times"
echo "====================================="

PROD_AVG=$(./log-time-analyzer \
  -l prod.log \
  -r '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
  -t '%Y-%m-%d %H:%M:%S' \
  -p "Request" -p "Response" \
  -f simple | awk -F'|' '{sum+=$3; n++} END {print sum/n}')

STAGING_AVG=$(./log-time-analyzer \
  -l staging.log \
  -r '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
  -t '%Y-%m-%d %H:%M:%S' \
  -p "Request" -p "Response" \
  -f simple | awk -F'|' '{sum+=$3; n++} END {print sum/n}')

echo "Production average: ${PROD_AVG}ms"
echo "Staging average: ${STAGING_AVG}ms"
```

### Filter by Time Period

```bash
# Extract logs for specific hour, then analyze
grep "2024-11-13 10:" app.log > hour10.log

./log-time-analyzer \
  -l hour10.log \
  -r '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
  -t '%Y-%m-%d %H:%M:%S' \
  -p "Query" -p "Result" \
  -f table
```

---

## Testing Different Regex Patterns

### Finding the Right Pattern

When you're not sure what pattern to use:

```bash
# Test different patterns quickly
for pattern in "ERROR" "CRITICAL" "FATAL"; do
  echo "Testing pattern: $pattern"
  ./log-time-analyzer \
    -l app.log \
    -c config.yaml \
    -p "$pattern" -p "Recovery" \
    -f simple | wc -l
done
```

### Debug Timestamp Extraction

```bash
# Try different timestamp formats to see which works
./log-time-analyzer \
  -l app.log \
  -r '(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})' \
  -t '%Y-%m-%d %H:%M:%S' \
  -p "Start" -p "End" 2>&1

# If it fails, try another format
./log-time-analyzer \
  -l app.log \
  -r '\[(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})\]' \
  -t '%Y-%m-%dT%H:%M:%S' \
  -p "Start" -p "End" 2>&1
```

---

## Performance Monitoring

### Real-time Monitoring (with watch)

```bash
# Monitor log file every 5 seconds
watch -n 5 './log-time-analyzer \
  -l /var/log/app.log \
  -c config.yaml \
  -p "Request" -p "Response" \
  -f table'
```

### Generate Reports

```bash
#!/bin/bash
DATE=$(date +%Y-%m-%d)

./log-time-analyzer \
  -l /var/log/app-${DATE}.log \
  -c config.yaml \
  -p "Transaction start" -p "Transaction end" \
  -f csv > reports/transactions-${DATE}.csv

echo "Report generated: reports/transactions-${DATE}.csv"

# Email the report
mail -s "Daily Transaction Report" team@example.com < reports/transactions-${DATE}.csv
```

---

## Tips and Best Practices

### 1. Start Simple

Begin with the config file, then override specific values:
```bash
./log-time-analyzer -l app.log -c config.yaml -p "new_pattern"
```

### 2. Test Patterns First

Use a small log sample to test your patterns:
```bash
head -1000 large.log > sample.log
./log-time-analyzer -l sample.log -c config.yaml
```

### 3. Use Shell Variables

Make your commands reusable:
```bash
TIMESTAMP_REGEX='(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})'
TIMESTAMP_FORMAT='%Y-%m-%d %H:%M:%S'

./log-time-analyzer \
  -l app.log \
  -r "$TIMESTAMP_REGEX" \
  -t "$TIMESTAMP_FORMAT" \
  -p "Start" -p "End"
```

### 4. Combine with jq for Complex Analysis

```bash
# Find outliers (durations > 2 standard deviations)
./log-time-analyzer -l app.log -c config.yaml -f json | \
  jq '[.[].duration_ms] | add / length as $avg | 
      map(. - $avg | . * .) | add / length | sqrt as $std |
      $avg, $std' | \
  read avg std && \
  ./log-time-analyzer -l app.log -c config.yaml -f json | \
  jq --arg avg "$avg" --arg std "$std" \
    '.[] | select(.duration_ms > ($avg|tonumber) + 2 * ($std|tonumber))'
```
