#!/usr/bin/env bash
# Copy into an app's tools/ folder. Runs one of acorn's tools from the app's repository root:
#   tools/acorn.sh macos/bundle.sh
# Finds acorn through cargo (the checkout of the acorn-app dependency, path or git).
set -euo pipefail
cd "$(dirname "$0")/.."
ACORN=$(cargo metadata --format-version 1 | python3 -c 'import json,sys; m=json.load(sys.stdin); print(next(p["manifest_path"] for p in m["packages"] if p["name"]=="acorn-app").rsplit("/crates/",1)[0])')
tool=$1; shift
exec "$ACORN/tools/$tool" "$@"
