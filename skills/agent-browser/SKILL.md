---
name: z-agent-browser
description: Automates browser interactions for web testing, form filling, screenshots, video recording, and data extraction. Use when the user needs to navigate websites, interact with web pages, fill forms, take screenshots, test web applications, or extract information from web pages. Enhanced fork with stealth mode, auto-persistence, profile mode, and MCP backend.
---

# Browser Automation with z-agent-browser

## Quick start

```bash
z-agent-browser open <url>        # Navigate to page
z-agent-browser snapshot -i       # Get interactive elements with refs
z-agent-browser click @e1         # Click element by ref
z-agent-browser fill @e2 "text"   # Fill input by ref
z-agent-browser close             # Close browser
```

## Core workflow

1. Navigate: `z-agent-browser open <url>`
2. Snapshot: `z-agent-browser snapshot -i` (returns elements with refs like `@e1`, `@e2`)
3. Interact using refs from the snapshot
4. Re-snapshot after navigation or significant DOM changes

## Commands

### Navigation
```bash
z-agent-browser open <url>      # Navigate to URL
z-agent-browser back            # Go back
z-agent-browser forward         # Go forward
z-agent-browser reload          # Reload page
z-agent-browser close           # Close browser
```

### Snapshot (page analysis)
```bash
z-agent-browser snapshot            # Full accessibility tree
z-agent-browser snapshot -i         # Interactive elements only (recommended)
z-agent-browser snapshot -c         # Compact output
z-agent-browser snapshot -d 3       # Limit depth to 3
z-agent-browser snapshot -s "#main" # Scope to CSS selector
```

### Interactions (use @refs from snapshot)
```bash
z-agent-browser click @e1           # Click
z-agent-browser dblclick @e1        # Double-click
z-agent-browser focus @e1           # Focus element
z-agent-browser fill @e2 "text"     # Clear and type
z-agent-browser type @e2 "text"     # Type without clearing
z-agent-browser press Enter         # Press key
z-agent-browser press Control+a     # Key combination
z-agent-browser keydown Shift       # Hold key down
z-agent-browser keyup Shift         # Release key
z-agent-browser hover @e1           # Hover
z-agent-browser check @e1           # Check checkbox
z-agent-browser uncheck @e1         # Uncheck checkbox
z-agent-browser select @e1 "value"  # Select dropdown
z-agent-browser scroll down 500     # Scroll page
z-agent-browser scrollintoview @e1  # Scroll element into view
z-agent-browser drag @e1 @e2        # Drag and drop
z-agent-browser upload @e1 file.pdf # Upload files
```

### Get information
```bash
z-agent-browser get text @e1        # Get element text
z-agent-browser get html @e1        # Get innerHTML
z-agent-browser get value @e1       # Get input value
z-agent-browser get attr @e1 href   # Get attribute
z-agent-browser get title           # Get page title
z-agent-browser get url             # Get current URL
z-agent-browser get count ".item"   # Count matching elements
z-agent-browser get box @e1         # Get bounding box
```

### Check state
```bash
z-agent-browser is visible @e1      # Check if visible
z-agent-browser is enabled @e1      # Check if enabled
z-agent-browser is checked @e1      # Check if checked
```

### Screenshots & PDF
```bash
z-agent-browser screenshot          # Screenshot to stdout
z-agent-browser screenshot path.png # Save to file
z-agent-browser screenshot --full   # Full page
z-agent-browser pdf output.pdf      # Save as PDF
```

### Video recording
```bash
z-agent-browser record start ./demo.webm    # Start recording (uses current URL + state)
z-agent-browser click @e1                   # Perform actions
z-agent-browser record stop                 # Stop and save video
z-agent-browser record restart ./take2.webm # Stop current + start new recording
```
Recording creates a fresh context but preserves cookies/storage from your session. If no URL is provided, it automatically returns to your current page. For smooth demos, explore first, then start recording.

### Wait
```bash
z-agent-browser wait @e1                     # Wait for element
z-agent-browser wait 2000                    # Wait milliseconds
z-agent-browser wait --text "Success"        # Wait for text
z-agent-browser wait --url "**/dashboard"    # Wait for URL pattern
z-agent-browser wait --load networkidle      # Wait for network idle
z-agent-browser wait --fn "window.ready"     # Wait for JS condition
```

### Mouse control
```bash
z-agent-browser mouse move 100 200      # Move mouse
z-agent-browser mouse down left         # Press button
z-agent-browser mouse up left           # Release button
z-agent-browser mouse wheel 100         # Scroll wheel
```

### Semantic locators (alternative to refs)
```bash
z-agent-browser find role button click --name "Submit"
z-agent-browser find text "Sign In" click
z-agent-browser find label "Email" fill "user@test.com"
z-agent-browser find first ".item" click
z-agent-browser find nth 2 "a" text
```

### Browser settings
```bash
z-agent-browser set viewport 1920 1080      # Set viewport size
z-agent-browser set device "iPhone 14"      # Emulate device
z-agent-browser set geo 37.7749 -122.4194   # Set geolocation
z-agent-browser set offline on              # Toggle offline mode
z-agent-browser set headers '{"X-Key":"v"}' # Extra HTTP headers
z-agent-browser set credentials user pass   # HTTP basic auth
z-agent-browser set media dark              # Emulate color scheme
```

### Cookies & Storage
```bash
z-agent-browser cookies                     # Get all cookies
z-agent-browser cookies set name value      # Set cookie
z-agent-browser cookies clear               # Clear cookies
z-agent-browser storage local               # Get all localStorage
z-agent-browser storage local key           # Get specific key
z-agent-browser storage local set k v       # Set value
z-agent-browser storage local clear         # Clear all
```

### Network
```bash
z-agent-browser network route <url>              # Intercept requests
z-agent-browser network route <url> --abort      # Block requests
z-agent-browser network route <url> --body '{}'  # Mock response
z-agent-browser network unroute [url]            # Remove routes
z-agent-browser network requests                 # View tracked requests
z-agent-browser network requests --filter api    # Filter requests
```

### Tabs & Windows
```bash
z-agent-browser tab                 # List tabs
z-agent-browser tab new [url]       # New tab
z-agent-browser tab 2               # Switch to tab
z-agent-browser tab close           # Close tab
z-agent-browser window new          # New window
```

### Frames
```bash
z-agent-browser frame "#iframe"     # Switch to iframe
z-agent-browser frame main          # Back to main frame
```

### Dialogs
```bash
z-agent-browser dialog accept [text]  # Accept dialog
z-agent-browser dialog dismiss        # Dismiss dialog
```

### JavaScript
```bash
z-agent-browser eval "document.title"   # Run JavaScript
```

## Example: Form submission

```bash
z-agent-browser open https://example.com/form
z-agent-browser snapshot -i
# Output shows: textbox "Email" [ref=e1], textbox "Password" [ref=e2], button "Submit" [ref=e3]

z-agent-browser fill @e1 "user@example.com"
z-agent-browser fill @e2 "password123"
z-agent-browser click @e3
z-agent-browser wait --load networkidle
z-agent-browser snapshot -i  # Check result
```

## Example: Authentication with saved state

```bash
# Login once
z-agent-browser open https://app.example.com/login
z-agent-browser snapshot -i
z-agent-browser fill @e1 "username"
z-agent-browser fill @e2 "password"
z-agent-browser click @e3
z-agent-browser wait --url "**/dashboard"
z-agent-browser state save auth.json

# Later sessions: load saved state
z-agent-browser state load auth.json
z-agent-browser open https://app.example.com/dashboard
```

## Sessions (parallel browsers)

```bash
z-agent-browser --session test1 open site-a.com
z-agent-browser --session test2 open site-b.com
z-agent-browser session list
```

## JSON output (for parsing)

Add `--json` for machine-readable output:
```bash
z-agent-browser snapshot -i --json
z-agent-browser get text @e1 --json
```

## Debugging

```bash
z-agent-browser open example.com --headed              # Show browser window
z-agent-browser console                                # View console messages
z-agent-browser errors                                 # View page errors
z-agent-browser record start ./debug.webm   # Record from current page
z-agent-browser record stop                            # Save recording
z-agent-browser open example.com --headed  # Show browser window
z-agent-browser --cdp 9222 snapshot        # Connect via CDP
z-agent-browser console                    # View console messages
z-agent-browser console --clear            # Clear console
z-agent-browser errors                     # View page errors
z-agent-browser errors --clear             # Clear errors
z-agent-browser highlight @e1              # Highlight element
z-agent-browser trace start                # Start recording trace
z-agent-browser trace stop trace.zip       # Stop and save trace
```

## Enhanced Features

### Stealth Mode
Bypass bot detection:
```bash
z-agent-browser --stealth open "https://example.com"
# Or via env: AGENT_BROWSER_STEALTH=1
```

### Auto-Persistence
Auto-save/restore auth state:
```bash
z-agent-browser --persist open "https://github.com/login" --headed
# State saved to ~/.z-agent-browser/sessions/default.json on close
```

### Profile Mode
Use persistent Chrome profile (keeps extensions, passwords):
```bash
z-agent-browser --profile ~/.browser/my-profile open "https://example.com"
```

### CDP Mode
Connect to existing Chrome:
```bash
z-agent-browser connect 9222   # Connect once
z-agent-browser snapshot       # No --cdp needed after
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `AGENT_BROWSER_SESSION` | Session name for isolation |
| `AGENT_BROWSER_HEADED` | "1" for visible browser |
| `AGENT_BROWSER_STEALTH` | "1" for stealth mode |
| `AGENT_BROWSER_PERSIST` | "1" for auto-persistence |
| `AGENT_BROWSER_STATE` | Path to state file |
| `AGENT_BROWSER_PROFILE` | Chrome profile directory |
| `AGENT_BROWSER_BACKEND` | `native` (default) or `playwright-mcp` |
| `PLAYWRIGHT_MCP_EXTENSION_TOKEN` | Token for MCP mode |

## Playwright MCP Mode (Experimental)

Control your existing Chrome browser via the Playwright MCP bridge extension.

### Setup
1. Install "Playwright MCP Bridge" Chrome extension
2. Click extension icon to get your token
3. Set environment variables:

```bash
export PLAYWRIGHT_MCP_EXTENSION_TOKEN=your-token-from-extension
export AGENT_BROWSER_BACKEND=playwright-mcp

# Commands work the same
z-agent-browser open "https://example.com"
z-agent-browser snapshot -i
z-agent-browser click @e1
z-agent-browser back
z-agent-browser close
```

### MCP Mode Compatibility

| Works in MCP Mode | Not Available in MCP Mode |
|-------------------|---------------------------|
| open, back, close | forward, reload |
| snapshot | get text/html/value |
| click, fill, type, hover | scroll, scrollintoview |
| press, select, drag | state save/load |
| screenshot | pdf, video recording |
| tabs (list/new/switch/close) | cookies, storage |
| eval, console | network routes, frames |
| dialog accept/dismiss | stealth, profile, CDP |

Use native mode (default) for full feature support. MCP mode is for controlling your actual browser with extensions.
