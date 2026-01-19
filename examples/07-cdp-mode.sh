#!/bin/bash
# CDP mode: Connect to existing Chrome instance
# Use this when you want Chrome to persist between sessions

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Start persistent Chrome (requires tmux)
"$SCRIPT_DIR/../scripts/start-chrome.sh"

# Connect via CDP
z-agent-browser --cdp 9222 open "https://example.com"
z-agent-browser --cdp 9222 snapshot -i
z-agent-browser --cdp 9222 get title

# Chrome stays running after this script ends
# Stop with: ./scripts/stop-chrome.sh
echo ""
echo "Chrome is still running. Stop with: $SCRIPT_DIR/../scripts/stop-chrome.sh"
