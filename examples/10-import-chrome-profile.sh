#!/bin/bash
# Import existing Chrome logins by copying your profile

set -e

PROFILE="$HOME/.z-agent-browser/chrome-profile"

echo "This script copies your Chrome profile to use existing logins."
echo "Your main Chrome must be closed."
echo ""

if pgrep -x "Google Chrome" > /dev/null; then
    echo "Chrome is running. Close it first:"
    echo "  pkill -9 'Google Chrome'"
    exit 1
fi

echo "Copying Chrome profile (this may take a minute)..."
mkdir -p "$HOME/.z-agent-browser"
rm -rf "$PROFILE"
cp -R "$HOME/Library/Application Support/Google/Chrome" "$PROFILE"
echo "  [ok] Copied to $PROFILE"

echo ""
echo "Launching Chrome with remote debugging..."
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
  --remote-debugging-port=9222 \
  --user-data-dir="$PROFILE" &

sleep 3

if lsof -i :9222 > /dev/null 2>&1; then
    echo "  [ok] Chrome running on port 9222"
else
    echo "  [error] Chrome failed to start"
    exit 1
fi

echo ""
echo "Testing connection (z-agent-browser auto-detects CDP)..."
z-agent-browser open "https://example.com"
z-agent-browser snapshot -i
z-agent-browser get title

echo ""
echo "Success! Your existing logins are available."
echo ""
echo "Usage (no --cdp needed, auto-detected):"
echo "  z-agent-browser open 'https://github.com'"
echo "  z-agent-browser snapshot -i"
echo ""
echo "Profile location: $PROFILE"
