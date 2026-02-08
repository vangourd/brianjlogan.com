#!/usr/bin/env bash
# Auto-populate dates taxonomy in all posts based on their publish date
# Run this once to add date tags like "2025-02" to all posts

set -e

echo "Adding date taxonomy to posts..."

for file in content/*.md; do
    # Skip _index.md and search.md
    filename=$(basename "$file")
    if [[ "$filename" == "_index"* || "$filename" == "search.md" ]]; then
        continue
    fi

    # Extract date from frontmatter (format: date = 2025-02-07 or date = 2025-02-07T00:00:00Z)
    date_line=$(grep -m 1 "^date = " "$file" || echo "")

    if [[ -z "$date_line" ]]; then
        echo "  ⚠ Skipping $filename (no date found)"
        continue
    fi

    # Extract YYYY-MM from date
    year_month=$(echo "$date_line" | sed -E 's/date = ([0-9]{4}-[0-9]{2}).*/\1/')

    if [[ -z "$year_month" ]]; then
        echo "  ⚠ Skipping $filename (couldn't parse date)"
        continue
    fi

    # Check if dates taxonomy already exists
    if grep -q "^dates = " "$file"; then
        echo "  ⚠ Skipping $filename (dates already exists)"
        continue
    fi

    # Check if there's already a [taxonomies] section
    if grep -q "^\[taxonomies\]" "$file"; then
        # Add dates to existing taxonomies section
        sed -i "/^\[taxonomies\]/a dates = [\"$year_month\"]" "$file"
        echo "  ✓ Added dates = [\"$year_month\"] to $filename"
    else
        # Add new [taxonomies] section after frontmatter header
        # Find the line with tags = and add dates on the next line
        if grep -q "^tags = " "$file"; then
            sed -i "/^tags = /a dates = [\"$year_month\"]" "$file"
            echo "  ✓ Added dates = [\"$year_month\"] to $filename"
        else
            # No tags either, add [taxonomies] section after date line
            sed -i "/^date = /a [taxonomies]\ndates = [\"$year_month\"]" "$file"
            echo "  ✓ Added [taxonomies] with dates = [\"$year_month\"] to $filename"
        fi
    fi
done

echo "Done!"
