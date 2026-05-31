#!/usr/bin/env bash
# Upload a single binary asset to S3 with safety guards.
#
# Usage: upload-asset.sh <local-file> <s3-key> [--force] [--no-cache]
#
# Defaults:
#   - Refuses to overwrite an existing key (bucket versioning still keeps prior
#     versions on overwrite, but we want explicit consent).
#   - Detects Content-Type from extension.
#   - Cache-Control: max-age=600 (use --no-cache for resume .bin uploads).
#   - On success, also copies the file into the local static/ mirror so
#     `zola serve` reflects what's deployed.
#
# Examples:
#   upload-asset.sh ~/Pictures/coffee.jpg images/coffee.jpg
#   upload-asset.sh tmp-aws/abc123.bin encrypted-resumes/abc123.bin --no-cache
#   upload-asset.sh ~/Downloads/new-portrait.png me.png --force

set -euo pipefail

BUCKET="brianjlogan"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

if [ $# -lt 2 ]; then
    sed -n '2,18p' "$0" | sed 's/^# \{0,1\}//'
    exit 2
fi

LOCAL="$1"; shift
KEY="$1";   shift

FORCE=0
NO_CACHE=0
while [ $# -gt 0 ]; do
    case "$1" in
        --force)    FORCE=1 ;;
        --no-cache) NO_CACHE=1 ;;
        *) echo "unknown flag: $1" >&2; exit 2 ;;
    esac
    shift
done

if [ ! -f "$LOCAL" ]; then
    echo "error: $LOCAL not found" >&2
    exit 1
fi

# Existence check (overridden by --force; versioning keeps the old object either way).
if [ "$FORCE" -eq 0 ] && aws s3api head-object --bucket "$BUCKET" --key "$KEY" >/dev/null 2>&1; then
    echo "error: s3://${BUCKET}/${KEY} already exists. Pass --force to overwrite (prior version preserved by bucket versioning)." >&2
    exit 1
fi

# Content-Type from extension.
case "${LOCAL##*.}" in
    png)  CT="image/png" ;;
    jpg|jpeg) CT="image/jpeg" ;;
    gif)  CT="image/gif" ;;
    webp) CT="image/webp" ;;
    svg)  CT="image/svg+xml" ;;
    pdf)  CT="application/pdf" ;;
    bin)  CT="application/octet-stream" ;;
    *)    CT="application/octet-stream" ;;
esac

if [ "$NO_CACHE" -eq 1 ]; then
    CACHE="no-store"
else
    CACHE="max-age=600"
fi

echo "→ s3://${BUCKET}/${KEY}  (${CT}, ${CACHE})"
aws s3 cp "$LOCAL" "s3://${BUCKET}/${KEY}" \
    --content-type "$CT" \
    --cache-control "$CACHE"

# Mirror to local static/ so `zola serve` reflects what's deployed.
MIRROR="${ROOT}/static/${KEY}"
mkdir -p "$(dirname "$MIRROR")"
cp "$LOCAL" "$MIRROR"
echo "→ mirrored: static/${KEY}"
