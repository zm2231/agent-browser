#!/bin/bash
# Stop the persistent Chrome instance

TMUX_SESSION="z-chrome"

if tmux has-session -t $TMUX_SESSION 2>/dev/null; then
    tmux kill-session -t $TMUX_SESSION
    echo "Chrome stopped (tmux session '$TMUX_SESSION' killed)"
else
    echo "No Chrome tmux session found"
    
    # Check if Chrome is running on debug port anyway
    if lsof -i :9222 >/dev/null 2>&1; then
        echo "Chrome is running on port 9222 but not in tmux"
        echo "Kill manually with: kill \$(lsof -t -i :9222)"
    fi
fi
