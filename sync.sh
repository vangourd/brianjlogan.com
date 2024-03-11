#!/bin/bash

aws s3 sync public/ s3://brianjlogan --delete --cache-control max-age=600
aws s3 cp s3://brianjlogan s3://brianjlogan --recursive --exclude "*" --include "*.html" --metadata-directive REPLACE --cache-control max-age:no-cache --content-type text/html
