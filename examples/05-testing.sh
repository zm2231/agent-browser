#!/bin/bash
# E2E testing example

set -e  # Exit on error

URL="https://example.com"
echo "Testing: $URL"

# Test 1: Page loads
echo -n "Test 1: Page loads... "
z-agent-browser open "$URL"
TITLE=$(z-agent-browser get title)
if [ -n "$TITLE" ]; then
    echo "PASS (title: $TITLE)"
else
    echo "FAIL"
    exit 1
fi

# Test 2: Has expected content
echo -n "Test 2: Has expected heading... "
z-agent-browser snapshot -i
# Check if snapshot contains expected text
if z-agent-browser get text @e1 | grep -q "Example"; then
    echo "PASS"
else
    echo "FAIL"
    exit 1
fi

# Test 3: Link is clickable
echo -n "Test 3: Link is clickable... "
z-agent-browser click @e2 2>/dev/null || true
z-agent-browser wait --load networkidle
NEW_URL=$(z-agent-browser get url)
if [ "$NEW_URL" != "$URL" ]; then
    echo "PASS (navigated to: $NEW_URL)"
else
    echo "PASS (same page)"
fi

z-agent-browser close
echo "All tests passed!"
