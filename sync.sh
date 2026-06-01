#!/bin/bash

# Binary asset prefixes are managed out-of-band (see tools/upload-asset.sh).
# We must exclude them from --delete sweeps or CI would wipe them on every run.
BIN_EXCLUDES=(
    --exclude "images/*"
    --exclude "handwriting/*"
    --exclude "encrypted-resumes/*"
    --exclude "me.png"
    --exclude "me-shocked.png"
)

# Sync everything except .ncl and .md files first
aws s3 sync public/ s3://brianjlogan --delete --cache-control max-age=600 \
    --exclude "*.ncl" --exclude "*.md" "${BIN_EXCLUDES[@]}"

# Upload .ncl files with correct content-type and inline disposition
aws s3 sync public/ s3://brianjlogan --exclude "*" --include "*.ncl" --content-type "text/plain; charset=utf-8" --content-disposition "inline" --cache-control max-age=600

# Upload .md files with correct content-type and inline disposition
aws s3 sync public/ s3://brianjlogan --exclude "*" --include "*.md" --content-type "text/plain; charset=utf-8" --content-disposition "inline" --cache-control max-age=600

# Set HTML files to no-cache
aws s3 cp s3://brianjlogan s3://brianjlogan --recursive --exclude "*" --include "*.html" --metadata-directive REPLACE --cache-control max-age:no-cache --content-type text/html
