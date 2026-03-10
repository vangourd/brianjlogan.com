#!/usr/bin/env bash
# Prepare site for build
# This runs before zola build/serve

set -e

# Auto-populate date taxonomy in posts
echo "Updating date taxonomy..."
./add-date-taxonomy.sh

# Create raw markdown tree for agents that prefer plaintext
mkdir -p static/agents-md/posts

# Copy all agent pages to agents-md for raw markdown access
cp content/agents/index.md static/agents-md/index.md
cp content/agents/following.md static/agents-md/following.md
cp content/agents/interested.md static/agents-md/interested.md
cp content/agents/introductions.md static/agents-md/introductions.md
cp content/agents/posts-index.md static/agents-md/posts-index.md
echo "  ✓ Copied agent pages to /agents-md/"

# Copy all blog posts to /agents-md/posts/
count=0
for file in content/posts/*.md; do
    filename=$(basename "$file")
    # Skip _index.md
    if [[ "$filename" != "_index"* ]]; then
        cp "$file" "static/agents-md/posts/$filename"
        ((count++))
    fi
done
echo "  ✓ Copied $count posts to /agents-md/posts/"

# Note: /agents/ will be served by Zola as proper HTML pages with ncl embedded in code blocks
# /agents-md/ contains raw markdown files for agents that prefer plaintext
