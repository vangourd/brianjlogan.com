#!/usr/bin/env bash
# Prepare site for build
# This runs before zola build/serve

set -e

# Auto-populate date taxonomy in posts
echo "Updating date taxonomy..."
./add-date-taxonomy.sh

# Create agents directories
mkdir -p static/agents/posts

# Copy agents index to root as agents.md
cp content/agents/index.md static/agents.md
echo "  ✓ Copied agents.md to root"

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

# Generate posts index
./generate-posts-index.sh
