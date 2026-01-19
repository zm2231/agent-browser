#!/bin/bash
# Browser streaming: Watch browser in real-time via WebSocket
# Useful for "pair browsing" where human watches AI navigate

# Start browser with streaming enabled
AGENT_BROWSER_STREAM_PORT=9223 z-agent-browser open "https://example.com" --headed &
BROWSER_PID=$!

echo "Browser streaming on ws://localhost:9223"
echo "Connect a WebSocket client to watch the browser viewport"
echo ""
echo "Press Enter to stop..."
read

kill $BROWSER_PID 2>/dev/null
z-agent-browser close
