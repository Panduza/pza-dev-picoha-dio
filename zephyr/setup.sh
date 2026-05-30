#!/usr/bin/env bash
set -euo pipefail

# --- usage ---
if [ $# -lt 1 ]; then
    echo "Usage: $0 <project>" >&2
    echo "  e.g. $0 fw-blink-1" >&2
    exit 1
fi

PROJECT="$1"

# --- validate project directory ---
if [ ! -d "$PROJECT" ]; then
    echo "Error: project directory '$PROJECT' not found." >&2
    exit 1
fi

# --- venv: create if absent ---
if [ ! -d ".venv" ]; then
    echo "Creating Python virtual environment..."
    python3 -m venv .venv
fi

# --- activate venv ---
source .venv/bin/activate

# --- install west if absent ---
if ! command -v west &>/dev/null; then
    echo "Installing west..."
    pip install west
fi

# --- west init: only if not already initialised ---
if [ ! -d ".west" ]; then
    echo "Initialising west workspace with manifest from '$PROJECT'..."
    west init -l "$PROJECT"
fi

# --- west update: always run to sync dependencies ---
echo "Updating west dependencies..."
west update

# --- 
pip install -r zephyr/scripts/requirements.txt
# west sdk install
west sdk install -t arm-zephyr-eabi