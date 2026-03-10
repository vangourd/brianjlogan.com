# brianjlogan.com

Personal site built with [Zola](https://www.getzola.org/) using the `terminal` theme. Hosted on AWS S3.

## Development

Requires [Nix](https://nixos.org/) with flakes enabled.

```bash
# Local dev server (localhost only)
nix run .#dev-local

# LAN dev server (accessible from other devices)
nix run .#dev-lan
```

## Creating a New Post

1. Create a new markdown file in `content/posts/` with a slug-based filename, e.g. `content/posts/my-post-title.md`.

2. Add TOML frontmatter at the top:

```toml
+++
title = "Your Post Title"
template = "page.html"
date = 2026-03-09T12:00:00Z
[taxonomies]
tags = ["tag1", "tag2"]
[extra]
summary = "A brief summary of your post"
+++

Your markdown content goes here.
```

3. **Don't add `dates` taxonomy manually** — `build.sh` auto-populates it from the `date` field via `add-date-taxonomy.sh`.

4. Preview locally with `nix run .#dev-local` (the build script runs automatically before Zola serves).

## Deploying

```bash
./sync.sh
```

This syncs the built `public/` directory to the S3 bucket with appropriate content-type and cache headers.

## Project Structure

```
content/posts/    Blog posts (slug-based .md files)
content/          Site pages (about, contact, links, agents)
themes/terminal/  Zola theme (templates + sass)
static/           Static assets (images, agent markdown copies)
build.sh          Pre-build script (date taxonomy, agent markdown copies)
sync.sh           S3 deployment script
config.toml       Zola configuration
flake.nix         Nix dev environment
```
