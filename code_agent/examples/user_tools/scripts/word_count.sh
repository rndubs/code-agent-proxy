#!/bin/bash
# Example user tool: word_count
# Counts words, lines, and characters in a file or text

# Read JSON params from stdin
params=$(cat)

# Extract file_path or text from JSON
file_path=$(echo "$params" | jq -r '.file_path // empty')
text=$(echo "$params" | jq -r '.text // empty')

# Determine input source
if [ -n "$file_path" ]; then
    if [ ! -f "$file_path" ]; then
        echo '{"success": false, "output": "", "error": "File not found: '"$file_path"'"}'
        exit 0
    fi
    input="$file_path"
    wc_result=$(wc "$input")
else
    # Use text from params
    input=$(echo "$text")
    wc_result=$(echo "$input" | wc)
fi

# Parse wc output (lines, words, chars)
lines=$(echo "$wc_result" | awk '{print $1}')
words=$(echo "$wc_result" | awk '{print $2}')
chars=$(echo "$wc_result" | awk '{print $3}')

# Return result as JSON
cat <<EOF
{
  "success": true,
  "output": "Lines: $lines\nWords: $words\nCharacters: $chars"
}
EOF
