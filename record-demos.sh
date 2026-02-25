#!/usr/bin/env bash
set -euo pipefail

# Record all ratatui-interact demos as GIFs using VHS
# Prerequisites: cargo, vhs (brew install vhs)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DEMOS_DIR="$SCRIPT_DIR/demos"
GIFS_DIR="$DEMOS_DIR/gifs"

# Check dependencies
command -v vhs >/dev/null 2>&1 || { echo "Error: VHS not installed. Run: brew install vhs"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "Error: cargo not installed."; exit 1; }

# Parse arguments
FILTER="${1:-}"

# Pre-build all examples to avoid compilation delays during recording
echo "Pre-building all examples..."
cargo build --examples --all-features 2>&1
echo "Build complete."

mkdir -p "$GIFS_DIR"

# Collect tape files
FAILED=()
SUCCEEDED=()
SKIPPED=()

for tape in "$DEMOS_DIR"/*.tape; do
    name=$(basename "$tape" .tape)

    # Skip settings file
    [[ "$name" == "settings" ]] && continue

    # Apply filter if provided
    if [[ -n "$FILTER" ]] && [[ "$name" != *"$FILTER"* ]]; then
        SKIPPED+=("$name")
        continue
    fi

    echo "Recording $name..."
    if vhs "$tape" 2>&1; then
        SUCCEEDED+=("$name")
        echo "  -> demos/gifs/${name}.gif"
    else
        FAILED+=("$name")
        echo "  WARNING: $name failed"
    fi
done

# Summary
echo ""
echo "=== Recording Summary ==="
echo "Succeeded: ${#SUCCEEDED[@]}"
[[ ${#SKIPPED[@]} -gt 0 ]] && echo "Skipped:   ${#SKIPPED[@]}"
if [[ ${#FAILED[@]} -gt 0 ]]; then
    echo "Failed:    ${#FAILED[@]}"
    for f in "${FAILED[@]}"; do
        echo "  - $f"
    done
    exit 1
fi
echo ""
echo "GIFs saved to $GIFS_DIR/"
