#!/bin/bash
# Video recording: Create demos and bug reproductions

z-agent-browser open "https://example.com" --headed
z-agent-browser snapshot -i

echo "Recording demo..."
z-agent-browser record start /tmp/demo.webm
z-agent-browser click @e1
z-agent-browser wait 1000
z-agent-browser snapshot -i
z-agent-browser record stop

echo "Video saved to /tmp/demo.webm"
z-agent-browser close
