#!/bin/sh
# Simple helper to dump final CI results

if [ -z "$1" ]; then
    echo "Usage: $0 <message>" >&2
    exit 1
fi

echo "$1" > work_results.txt
