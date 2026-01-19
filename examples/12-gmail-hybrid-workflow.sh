#!/bin/bash
# Gmail/Google Login - Hybrid Workflow
# This workflow ACTUALLY WORKS - tested successfully for reading Gmail
#
# Why this is needed:
#   - Google detects Playwright (even stealth) and blocks automated login
#   - Solution: Login via real Chrome CDP, save state, use in stealth automation
#
# Why this only works in z-agent-browser (not upstream Vercel):
#   - Vercel's `state load` just returns "must load at launch" - doesn't actually load
#   - Vercel has no stealth mode
#   - z-browser has runtime state load + stealth = working Gmail automation

set -e

PROFILE="$HOME/.z-agent-browser/cdp-profile"
STATE_FILE="$HOME/.z-agent-browser/gmail-state.json"

echo "=== Gmail Hybrid Workflow ==="
echo ""
echo "This workflow:"
echo "  1. Uses real Chrome (bypasses Google's bot detection for login)"
echo "  2. Saves session cookies"
echo "  3. Uses stealth Playwright with saved cookies for automation"
echo ""

# Step 1: Copy Chrome profile (one-time)
if [ ! -d "$PROFILE" ]; then
    echo "Step 1: Copying Chrome profile (first-time setup)..."
    if pgrep -x "Google Chrome" > /dev/null; then
        echo "  Chrome is running. Close it first: killall 'Google Chrome'"
        exit 1
    fi
    mkdir -p "$HOME/.z-agent-browser"
    cp -R "$HOME/Library/Application Support/Google/Chrome" "$PROFILE"
    echo "  [ok] Profile copied to $PROFILE"
else
    echo "Step 1: Profile already exists at $PROFILE"
fi
echo ""

# Step 2: Kill existing Chrome & launch with CDP
echo "Step 2: Launching Chrome with CDP..."
killall "Google Chrome" 2>/dev/null || true
sleep 1

"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
  --remote-debugging-port=9222 \
  --user-data-dir="$PROFILE" &

# Wait for Chrome to start
echo -n "  Waiting for Chrome"
for i in {1..20}; do
    if lsof -i :9222 >/dev/null 2>&1; then
        echo ""
        echo "  [ok] Chrome running on port 9222"
        break
    fi
    echo -n "."
    sleep 0.5
done
echo ""

# Step 3: Connect via CDP
echo "Step 3: Connecting via CDP..."
z-agent-browser connect 9222
echo "  [ok] Connected"
echo ""

# Step 4: Open Gmail - user completes login if needed
echo "Step 4: Opening Gmail..."
z-agent-browser open "https://mail.google.com"
echo ""
echo "  >>> Look at the Chrome window <<<"
echo "  If not logged in, complete login now (including 2FA if needed)"
echo ""
read -p "  Press Enter when logged into Gmail..."
echo ""

# Step 5: Save state
echo "Step 5: Saving state..."
z-agent-browser state save "$STATE_FILE"
echo "  [ok] State saved to $STATE_FILE"
echo ""

# Step 6: Quit CDP Chrome
echo "Step 6: Closing CDP Chrome..."
z-agent-browser close
killall "Google Chrome" 2>/dev/null || true
sleep 1
echo "  [ok] Chrome closed"
echo ""

# Step 7: Test stealth mode with saved cookies
echo "Step 7: Testing stealth mode with saved cookies..."
z-agent-browser start --stealth
z-agent-browser state load "$STATE_FILE"
z-agent-browser open "https://mail.google.com"
echo ""

# Verify we're logged in
echo "  Checking if logged in..."
TITLE=$(z-agent-browser get title)
echo "  Page title: $TITLE"

if echo "$TITLE" | grep -qi "inbox\|gmail"; then
    echo ""
    echo "  ✅ SUCCESS! Logged into Gmail in stealth mode!"
else
    echo ""
    echo "  ⚠️  May not be fully logged in. Check the title above."
fi

echo ""
echo "  Taking snapshot of inbox..."
z-agent-browser snapshot -i | head -30

z-agent-browser close
echo ""
echo "=== Workflow Complete ==="
echo ""
echo "To use this session in the future:"
echo "  z-agent-browser start --stealth"
echo "  z-agent-browser state load $STATE_FILE"
echo "  z-agent-browser open 'https://mail.google.com'"
echo ""
echo "Note: Session cookies may expire. Re-run this workflow to refresh."
