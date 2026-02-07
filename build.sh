#!/bin/bash
# Copy markdown files to static for AI consumption
# This runs before zola build/serve

set -e

# Create agents directories
mkdir -p static/agents/posts

# Copy agents index with different name to avoid routing conflict
cp content/agents/index.md static/agents/hub.md
echo "  ✓ Copied hub.md"

# Copy all blog posts to /agents/posts/
# Exclude _index.md files, search.md, and subdirectories
count=0
for file in content/*.md; do
    filename=$(basename "$file")
    # Skip special files
    if [[ "$filename" != "_index"* && "$filename" != "search.md" ]]; then
        cp "$file" "static/agents/posts/$filename"
        ((count++))
    fi
done
echo "  ✓ Copied $count posts to /agents/posts/"
