#!/bin/bash
# Authentication with state persistence

AUTH_FILE="$HOME/.browser/auth-github.json"

# Check if we have saved auth
if [ -f "$AUTH_FILE" ]; then
    echo "Loading saved auth..."
    z-agent-browser open "https://github.com" --headed
    z-agent-browser state load "$AUTH_FILE"
    z-agent-browser reload
else
    echo "No saved auth. Please log in manually..."
    z-agent-browser open "https://github.com/login" --headed
    echo "Press Enter after you've logged in..."
    read
    z-agent-browser state save "$AUTH_FILE"
    echo "Auth saved to $AUTH_FILE"
fi

z-agent-browser snapshot -i
z-agent-browser close
