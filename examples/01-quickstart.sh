#!/bin/bash
# Quickstart: Basic browser automation

# Headless mode (default, fast, no window)
z-agent-browser open "https://example.com"
z-agent-browser snapshot -i
z-agent-browser click @e1
z-agent-browser close

# Headed mode (visible window, for debugging or login)
z-agent-browser open "https://example.com" --headed
z-agent-browser snapshot -i
z-agent-browser close
