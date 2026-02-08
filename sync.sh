#!/bin/bash

# Sync everything except .ncl and .md files first
aws s3 sync public/ s3://brianjlogan --delete --cache-control max-age=600 --exclude "*.ncl" --exclude "*.md"

# Upload .ncl files with correct content-type and inline disposition
aws s3 sync public/ s3://brianjlogan --exclude "*" --include "*.ncl" --content-type "text/plain; charset=utf-8" --content-disposition "inline" --cache-control max-age=600

# Upload .md files with correct content-type and inline disposition
aws s3 sync public/ s3://brianjlogan --exclude "*" --include "*.md" --content-type "text/plain; charset=utf-8" --content-disposition "inline" --cache-control max-age=600

# Set HTML files to no-cache
aws s3 cp s3://brianjlogan s3://brianjlogan --recursive --exclude "*" --include "*.html" --metadata-directive REPLACE --cache-control max-age:no-cache --content-type text/html
