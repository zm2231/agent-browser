---
name: browser-automation
description: Automates browser interactions for web testing, form filling, screenshots, and data extraction. Use when user needs to navigate websites, interact with web pages, fill forms, take screenshots, or extract information.
requires:
  bins: [z-agent-browser]
---

# Browser Automation with z-agent-browser

## BEFORE YOU START - Mode Selection

**STOP. Ask the user which mode to use before executing any browser commands.**

| Mode | Best For | Requires |
|------|----------|----------|
| **Native** (default) | General automation, scraping, testing | Nothing extra |
| **Native + Stealth** | Sites with bot detection (Cloudflare, etc.) | Nothing extra |
| **CDP (Real Chrome)** | Saved passwords, CAPTCHA, 2FA, Google/Gmail | Chrome running with `--remote-debugging-port=9222` |
| **Playwright MCP** | Control user's existing Chrome tabs | [Extension from GitHub](https://github.com/microsoft/playwright-mcp/releases) + token |

**Ask the user:**
> "I can automate browsers in a few ways:
> 1. **Headless** - Fast, invisible automation (default)
> 2. **Headed** - Visible browser window (for debugging or manual steps)
> 3. **Stealth** - Bypasses bot detection (for protected sites)
> 4. **Real Chrome (CDP)** - Uses your actual Chrome with saved passwords
> 5. **Playwright MCP** - Controls your existing Chrome tabs (experimental)
>
> Which mode should I use? (or describe what you're trying to do and I'll recommend one)"

### Quick Decision Guide

```
Need saved passwords or 2FA?     → CDP Mode (Real Chrome)
Site blocks automation?          → Native + Stealth  
Google/Gmail?                    → CDP for login, then Stealth
Just testing/scraping?           → Native (default)
Want to watch it run?            → Native + Headed
Control existing Chrome tabs?    → Playwright MCP
```

### Mode Setup Commands

```bash
# Native (default)
z-agent-browser start

# Native + Headed (visible)
z-agent-browser start --headed

# Native + Stealth (anti-detection)
z-agent-browser start --stealth

# CDP (Real Chrome) - requires custom profile directory (Chrome 136+ blocks CDP on default profile)
# First-time: cp -R "$HOME/Library/Application Support/Google/Chrome" ~/.z-agent-browser/chrome-profile
# Then launch: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --remote-debugging-port=9222 --user-data-dir="$HOME/.z-agent-browser/chrome-profile" &
z-agent-browser connect 9222

# Playwright MCP - requires extension from https://github.com/microsoft/playwright-mcp/releases
# Load unpacked in chrome://extensions, get token from extension icon, then:
# export PLAYWRIGHT_MCP_EXTENSION_TOKEN=<token>
# export AGENT_BROWSER_BACKEND=playwright-mcp
z-agent-browser open <url>
```

---

## Core Workflow

1. **Navigate**: `z-agent-browser open <url>`
2. **Snapshot**: `z-agent-browser snapshot -i` (returns refs like `@e1`, `@e2`)
3. **Interact** using refs from snapshot
4. **Re-snapshot** after navigation or DOM changes

## Token Efficiency: eval vs snapshot

**Use `snapshot -i` for navigation** (finding what to click):
```bash
z-agent-browser snapshot -i   # ~200-500 tokens
```

**Use `eval` for data extraction** (getting information):
```bash
z-agent-browser eval "document.querySelectorAll('.item').length"
z-agent-browser eval "[...document.querySelectorAll('a')].map(a => a.href)"
```

| Task | Best Tool | Tokens |
|------|-----------|--------|
| Find button to click | `snapshot -i` | ~200-500 |
| Count items on page | `eval` | ~10 |
| Extract all links/data | `eval` | ~50-100 |
| Fill a form | `snapshot -i` + refs | ~200-500 |

**Rule**: Need to CLICK? → snapshot. Need to READ/EXTRACT? → eval.

## Commands

### Browser Lifecycle
```bash
z-agent-browser start                 # Start headless browser
z-agent-browser start --headed        # Start visible browser
z-agent-browser start --stealth       # Start with anti-detection
z-agent-browser status                # Check current mode
z-agent-browser stop                  # Stop browser
z-agent-browser connect 9222          # Connect to Chrome via CDP
```

### Navigation
```bash
z-agent-browser open <url>            # Navigate to URL
z-agent-browser back                  # Go back
z-agent-browser forward               # Go forward
z-agent-browser reload                # Reload page
z-agent-browser close                 # Close browser
```

### Snapshot (page analysis)
```bash
z-agent-browser snapshot              # Full accessibility tree
z-agent-browser snapshot -i           # Interactive elements only (recommended)
z-agent-browser snapshot -c           # Compact output
z-agent-browser snapshot -d 3         # Limit depth to 3
z-agent-browser snapshot -s "#main"   # Scope to CSS selector
```

### Interactions (use @refs from snapshot)
```bash
z-agent-browser click @e1             # Click
z-agent-browser dblclick @e1          # Double-click
z-agent-browser fill @e2 "text"       # Clear and type
z-agent-browser type @e2 "text"       # Type without clearing
z-agent-browser press Enter           # Press key
z-agent-browser press Control+a       # Key combination
z-agent-browser hover @e1             # Hover
z-agent-browser check @e1             # Check checkbox
z-agent-browser uncheck @e1           # Uncheck checkbox
z-agent-browser select @e1 "value"    # Select dropdown
z-agent-browser scroll down 500       # Scroll page
z-agent-browser scrollintoview @e1    # Scroll element into view
z-agent-browser drag @e1 @e2          # Drag and drop
z-agent-browser upload @e1 file.pdf   # Upload files
```

### Get Information
```bash
z-agent-browser get text @e1          # Get element text
z-agent-browser get html @e1          # Get innerHTML
z-agent-browser get value @e1         # Get input value
z-agent-browser get attr @e1 href     # Get attribute
z-agent-browser get title             # Get page title
z-agent-browser get url               # Get current URL
z-agent-browser get count ".item"     # Count matching elements
```

### Check State
```bash
z-agent-browser is visible @e1        # Check if visible
z-agent-browser is enabled @e1        # Check if enabled
z-agent-browser is checked @e1        # Check if checked
```

### Screenshots & PDF
```bash
z-agent-browser screenshot            # Screenshot to stdout (base64)
z-agent-browser screenshot path.png   # Save to file
z-agent-browser screenshot --full     # Full page
z-agent-browser pdf output.pdf        # Save as PDF
```

### State Persistence
```bash
z-agent-browser state save auth.json  # Save cookies/localStorage
z-agent-browser state load auth.json  # Restore session
```

### Video Recording
```bash
z-agent-browser record start demo.webm    # Start recording
z-agent-browser record stop               # Stop and save
z-agent-browser record restart take2.webm # Restart with new file
```

### Wait
```bash
z-agent-browser wait @e1                  # Wait for element
z-agent-browser wait 2000                 # Wait milliseconds
z-agent-browser wait --text "Success"     # Wait for text
z-agent-browser wait --url "**/dashboard" # Wait for URL pattern
z-agent-browser wait --load networkidle   # Wait for network idle
```

### JavaScript
```bash
z-agent-browser eval "document.title"
z-agent-browser eval "[...document.querySelectorAll('a')].map(a => a.href)"
```

### Browser Settings
```bash
z-agent-browser set viewport 1920 1080
z-agent-browser set device "iPhone 14"
z-agent-browser set headers '{"X-Key":"v"}'
```

### Tabs
```bash
z-agent-browser tab                   # List tabs
z-agent-browser tab new [url]         # New tab
z-agent-browser tab 2                 # Switch to tab
z-agent-browser tab close             # Close tab
```

### Debug
```bash
z-agent-browser console               # View console messages
z-agent-browser errors                # View page errors
z-agent-browser highlight @e1         # Highlight element
```

## Login Persistence

Use `state save/load` to persist login sessions:

```bash
# First time: Login manually in headed mode
z-agent-browser start --headed
z-agent-browser open "https://github.com"
# [User logs in manually]
z-agent-browser state save ~/.z-agent-browser/github.json
z-agent-browser stop

# Later: Restore session headlessly
z-agent-browser start
z-agent-browser state load ~/.z-agent-browser/github.json
z-agent-browser open "https://github.com"  # Already logged in!
```

## CDP Mode (Real Chrome)

For CAPTCHA, 2FA, or using saved passwords.

> **Note**: Chrome 136+ blocks CDP on default profile. You must use `--user-data-dir`.

**First-time setup** (copy your Chrome profile):
```bash
# macOS
mkdir -p ~/.z-agent-browser
cp -R "$HOME/Library/Application Support/Google/Chrome" ~/.z-agent-browser/chrome-profile

# Linux  
cp -R ~/.config/google-chrome ~/.z-agent-browser/chrome-profile
```

**Launch and connect**:
```bash
# Quit existing Chrome
pkill -9 "Google Chrome"

# Launch with CDP + custom profile
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
  --remote-debugging-port=9222 \
  --user-data-dir="$HOME/.z-agent-browser/chrome-profile" &

z-agent-browser connect 9222
z-agent-browser open "https://github.com"  # Uses real Chrome with your logins
```

## Gmail/Google (Hybrid Workflow)

Google blocks Chromium automation. Use real Chrome for login, then stealth for automation.

> **Prerequisite**: Complete "CDP Mode" first-time setup above (copy Chrome profile).

```bash
# 1. Launch real Chrome with CDP (using copied profile)
pkill -9 "Google Chrome"
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
  --remote-debugging-port=9222 \
  --user-data-dir="$HOME/.z-agent-browser/chrome-profile" &

# 2. Connect and login if needed
z-agent-browser connect 9222
z-agent-browser open "https://mail.google.com"
# User completes login in Chrome window

# 3. Save state
z-agent-browser state save ~/.z-agent-browser/gmail-state.json
z-agent-browser close && pkill -9 "Google Chrome"

# 4. Use stealth with saved state
z-agent-browser start --stealth
z-agent-browser state load ~/.z-agent-browser/gmail-state.json
z-agent-browser open "https://mail.google.com"  # Logged in!
```

## Important Notes

1. **Always use `snapshot -i`** to reduce output size
2. **Use refs (@e1, @e2)** from snapshot, not CSS selectors
3. **Re-snapshot after navigation** - refs change when page changes
4. **Save state after login** - `state save` persists sessions
5. **For Google/Gmail** - use hybrid CDP workflow, not direct login
6. **Use `eval` for extraction** - much more token efficient than parsing snapshots
