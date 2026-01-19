#!/bin/bash
# Start Chrome with remote debugging for persistent browser sessions
# Chrome survives between Claude sessions when run via this script

set -e

PORT=9222
PROFILE="$HOME/.z-agent-browser/chrome-profile"
TMUX_SESSION="z-chrome"

# Create directories
mkdir -p "$HOME/.z-agent-browser"

# Check if Chrome is already running on the port
if lsof -i :$PORT >/dev/null 2>&1; then
    echo "Chrome already running on port $PORT"
    echo "z-agent-browser will auto-connect"
    exit 0
fi

# Check if tmux is installed
if ! command -v tmux &>/dev/null; then
    echo "tmux not found. Install with: brew install tmux"
    echo ""
    echo "Alternatively, run Chrome manually in a separate terminal:"
    echo "/Applications/Google\\ Chrome.app/Contents/MacOS/Google\\ Chrome \\"
    echo "  --remote-debugging-port=$PORT \\"
    echo "  --user-data-dir=\"$PROFILE\" \\"
    echo "  --no-first-run"
    exit 1
fi

# Kill existing tmux session if it exists but Chrome isn't responding
if tmux has-session -t $TMUX_SESSION 2>/dev/null; then
    echo "Cleaning up stale tmux session..."
    tmux kill-session -t $TMUX_SESSION 2>/dev/null || true
fi

# Copy Chrome profile if it doesn't exist
if [ ! -d "$PROFILE" ]; then
    echo "Copying Chrome profile (first-time setup)..."
    cp -R "$HOME/Library/Application Support/Google/Chrome" "$PROFILE"
fi

# Launch Chrome in tmux
echo "Starting Chrome in tmux session '$TMUX_SESSION'..."
tmux new-session -d -s $TMUX_SESSION \
    "/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
    --remote-debugging-port=$PORT \
    --user-data-dir=\"$PROFILE\" \
    --no-first-run \
    --no-default-browser-check"

# Wait for Chrome to start
echo -n "Waiting for Chrome"
for i in {1..20}; do
    if lsof -i :$PORT >/dev/null 2>&1; then
        echo ""
        echo "Chrome started on port $PORT"
        echo ""
        echo "z-agent-browser will auto-connect. Just run:"
        echo "  z-agent-browser open \"https://example.com\""
        echo "  z-agent-browser snapshot -i"
        echo ""
        echo "To stop Chrome:"
        echo "  tmux kill-session -t $TMUX_SESSION"
        exit 0
    fi
    echo -n "."
    sleep 0.5
done

echo ""
echo "Failed to start Chrome within 10 seconds"
echo "Check tmux session: tmux attach -t $TMUX_SESSION"
exit 1
