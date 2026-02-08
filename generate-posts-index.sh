#!/usr/bin/env bash
# Generate an index of all posts in /agents/posts/

OUTPUT="static/agents/posts.ncl"

cat > "$OUTPUT" << 'HEADER'
# Posts Index
# All available blog posts

{
  version = "1.0",
  posts = [
HEADER

count=0
for file in content/*.md; do
    filename=$(basename "$file")
    # Skip special files
    if [[ "$filename" == "_index"* ]] || [[ "$filename" == "search.md" ]]; then
        continue
    fi

    # Extract frontmatter and fields
    # This creates a temp file with just the frontmatter
    tmpfile=$(mktemp)
    sed -n '/^+++$/,/^+++$/p' "$file" > "$tmpfile"

    title=$(grep '^title = ' "$tmpfile" | head -1 | cut -d'"' -f2)
    date=$(grep '^date = ' "$tmpfile" | head -1 | cut -d'=' -f2 | tr -d ' ')
    summary=$(grep '^summary = ' "$tmpfile" | head -1 | cut -d'"' -f2)

    rm "$tmpfile"

    # Skip if no title
    if [ -z "$title" ]; then
        continue
    fi

    # Write entry
    cat >> "$OUTPUT" << ENTRY
    {
      url = "/agents/posts/$filename",
      title = "$title",
ENTRY

    if [ -n "$date" ]; then
        echo "      date = \"$date\"," >> "$OUTPUT"
    fi

    if [ -n "$summary" ]; then
        echo "      summary = \"$summary\"," >> "$OUTPUT"
    fi

    echo "    }," >> "$OUTPUT"
    ((count++))
done

cat >> "$OUTPUT" << 'FOOTER'
  ]
}
FOOTER

echo "  ✓ Generated index with $count posts at $OUTPUT"
