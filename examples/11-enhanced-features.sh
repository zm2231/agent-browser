#!/bin/bash
# Example 11: Enhanced Fork Features
# Demonstrates: --stealth, --profile, --persist, --ignore-https-errors, --user-agent, --args
# Requires: Enhanced fork (zm2231/agent-browser)

set -e

echo "=== Enhanced Fork Features Demo ==="
echo ""
echo "Prerequisites:"
echo "  - Install enhanced fork: see install.sh option 1"
echo "  - Or: git clone https://github.com/zm2231/agent-browser.git && cd agent-browser && pnpm install && pnpm build && npm link"
echo ""

# 1. Stealth mode: bypass bot detection
echo "1. Stealth mode (bypass bot detection)"
z-agent-browser --stealth open "https://bot.sannysoft.com" --headed
sleep 2
z-agent-browser screenshot /tmp/stealth-test.png
echo "   Screenshot saved to /tmp/stealth-test.png"
echo "   Most bot detection tests should pass"
z-agent-browser close
echo ""

# 2. Auto-persistence: auth survives between sessions
echo "2. Auto-persistence with --persist"
echo "   First run: login and close (state auto-saved)"
z-agent-browser --persist open "https://github.com/login" --headed
echo "   (Log in manually, then press Enter to continue)"
read -r
z-agent-browser close
echo "   State saved to ~/.z-agent-browser/sessions/default.json"
echo ""

echo "   Second run: auth auto-restored"
z-agent-browser --persist open "https://github.com"
z-agent-browser snapshot -i | head -20
z-agent-browser close
echo ""

# 3. Profile mode: persistent Chrome profile
echo "3. Profile mode with --profile"
mkdir -p ~/.browser/demo-profile
z-agent-browser --profile ~/.browser/demo-profile open "https://example.com" --headed
echo "   Profile at ~/.browser/demo-profile"
echo "   Keeps extensions, bookmarks, passwords across sessions"
z-agent-browser close
echo ""

# 4. Custom User-Agent
echo "4. Custom User-Agent"
z-agent-browser --user-agent "MyBot/1.0 (https://example.com/bot)" open "https://httpbin.org/user-agent"
z-agent-browser snapshot
z-agent-browser close
echo ""

# 5. Browser launch args
echo "5. Browser launch args"
z-agent-browser --args "--window-size=800,600" open "https://example.com" --headed
z-agent-browser screenshot /tmp/small-window.png
echo "   Screenshot saved to /tmp/small-window.png (800x600)"
z-agent-browser close
echo ""

# 6. HTTPS certificate errors (for local dev)
echo "6. HTTPS certificate errors"
echo "   For self-signed certs on localhost:"
echo "   pkill -f 'node.*daemon'  # kill daemon first"
echo "   z-agent-browser --ignore-https-errors open 'https://localhost:8443'"
echo "   (Skipped: no local server running)"
echo ""

# 7. Special URL schemes
echo "7. Special URL schemes"
z-agent-browser open "about:blank"
z-agent-browser snapshot
z-agent-browser open "data:text/html,<h1>Hello from data URL</h1>"
z-agent-browser snapshot
z-agent-browser close
echo ""

# 8. Runtime state load
echo "8. Runtime state load"
z-agent-browser open "https://example.com"
if [ -f ~/.browser/auth.json ]; then
  z-agent-browser state load ~/.browser/auth.json
  echo "   Loaded auth into running browser"
fi
z-agent-browser close
echo ""

# 9. Explicit state file
echo "9. Explicit state file with --state"
z-agent-browser --state /tmp/test-auth.json open "https://example.com"
echo "   Using /tmp/test-auth.json for state"
z-agent-browser close
echo ""

# 10. Combine multiple options
echo "10. Combine multiple options"
z-agent-browser --stealth --user-agent "StealthBot/1.0" --args "--disable-gpu" open "https://example.com"
z-agent-browser snapshot -i | head -10
z-agent-browser close
echo ""

echo "=== Demo Complete ==="
echo ""
echo "Enhanced fork features:"
echo "  --stealth              Bypass bot detection (playwright-extra)"
echo "  --persist              Auto-save/load to ~/.z-agent-browser/sessions/"
echo "  --state <path>         Explicit state file"
echo "  --profile <path>       Persistent Chrome profile directory"
echo "  --user-agent <string>  Custom User-Agent"
echo "  --args <csv>           Browser launch arguments"
echo "  --ignore-https-errors  Skip SSL validation (kill daemon first)"
echo "  connect <port>         Persistent CDP connection"
echo "  tab new <url>          Open new tab at URL"
echo "  record start/stop      Video recording"
echo "  state load <path>      Load auth at runtime"
echo ""
echo "Environment variables:"
echo "  AGENT_BROWSER_STEALTH=1"
echo "  AGENT_BROWSER_PERSIST=1"
echo "  AGENT_BROWSER_PROFILE=~/.browser/profile"
echo "  AGENT_BROWSER_USER_AGENT='Bot/1.0'"
