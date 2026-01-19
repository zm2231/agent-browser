#!/bin/bash
# Web scraping example (headless)

z-agent-browser open "https://news.ycombinator.com"
z-agent-browser wait --load networkidle

# Get page info
echo "=== Page Title ==="
z-agent-browser get title

echo "=== Top Stories ==="
z-agent-browser snapshot -i -c

# Get specific elements
echo "=== First Link Text ==="
z-agent-browser get text @e1

# Screenshot for records
z-agent-browser screenshot /tmp/hn-screenshot.png
echo "Screenshot saved to /tmp/hn-screenshot.png"

z-agent-browser close
