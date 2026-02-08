#!/usr/bin/env python3
"""Generate an index of all posts in /agents/posts/"""

import os
import re
from pathlib import Path

def extract_frontmatter(content):
    """Extract TOML frontmatter from between +++ markers"""
    match = re.search(r'^\+\+\+\s*\n(.*?)\n\+\+\+', content, re.DOTALL)
    if match:
        return match.group(1)
    return ""

def extract_field(frontmatter, field):
    """Extract a field from frontmatter"""
    pattern = rf'^{field} = "(.*?)"'
    match = re.search(pattern, frontmatter, re.MULTILINE)
    if match:
        return match.group(1)

    # Try without quotes for dates
    pattern = rf'^{field} = (.*)$'
    match = re.search(pattern, frontmatter, re.MULTILINE)
    if match:
        return match.group(1).strip()

    return ""

def main():
    output_file = Path("static/agents/posts.ncl")
    content_dir = Path("content")

    # Start building the output
    lines = [
        "# Posts Index",
        "# All available blog posts",
        "",
        "{",
        '  version = "1.0",',
        "  posts = ["
    ]

    count = 0
    for md_file in sorted(content_dir.glob("*.md")):
        filename = md_file.name
        # Skip special files
        if filename.startswith("_index") or filename == "search.md":
            continue

        content = md_file.read_text()
        frontmatter = extract_frontmatter(content)

        if not frontmatter:
            continue

        title = extract_field(frontmatter, "title")
        if not title:
            continue

        date = extract_field(frontmatter, "date")
        summary = extract_field(frontmatter, "summary")

        # Build entry
        lines.append("    {")
        lines.append(f'      url = "/agents/posts/{filename}",')
        lines.append(f'      title = "{title}",')
        if date:
            lines.append(f'      date = "{date}",')
        if summary:
            lines.append(f'      summary = "{summary}",')
        lines.append("    },")
        count += 1

    lines.append("  ]")
    lines.append("}")

    output_file.write_text("\n".join(lines) + "\n")
    print(f"  ✓ Generated index with {count} posts at {output_file}")

if __name__ == "__main__":
    main()
