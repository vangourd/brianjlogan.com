#!/usr/bin/env bash
# Hydrate the local static/ tree with binary assets from S3.
# Additive only — never deletes local files that aren't in S3.
# Run once after `git clone` so `zola serve` shows images locally.

set -euo pipefail

BUCKET="brianjlogan"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

cd "$ROOT"

mkdir -p static/images static/handwriting static/encrypted-resumes

echo "→ images/"
aws s3 sync "s3://${BUCKET}/images"            static/images            --size-only

echo "→ handwriting/"
aws s3 sync "s3://${BUCKET}/handwriting"       static/handwriting       --size-only

echo "→ encrypted-resumes/"
aws s3 sync "s3://${BUCKET}/encrypted-resumes" static/encrypted-resumes --size-only

echo "→ portraits"
for f in me.png me-shocked.png; do
    aws s3 cp "s3://${BUCKET}/$f" "static/$f"
done

echo "done."
