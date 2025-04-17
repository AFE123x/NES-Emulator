#!/bin/zsh

TARGET_DIR="${1:-.}"  # Defaults to current directory if no argument

find "$TARGET_DIR" -type f -name "*.json" | while read -r json_file; do
    if grep -q '"final_"' "$json_file"; then
        echo "Skipping '$json_file' (already contains 'final_')"
    else
        echo "Fixing '$json_file'..."
        sed -i '' 's/"final"/"final_"/g' "$json_file"
        echo "✔ Updated '$json_file'"
    fi
done
