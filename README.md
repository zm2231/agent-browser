# z-agent-browser

Browser automation CLI for AI agents. Fast Rust CLI with Node.js fallback.

Enhanced fork of [vercel-labs/agent-browser](https://github.com/vercel-labs/agent-browser) with features AI agents actually need.

**For AI agent integration**, see [browser-skill](https://github.com/zm2231/browser-skill) - ready-to-use skill files for Claude Code, OpenCode, and other AI agents.

## Highlights

- **Gmail/Google support** via [hybrid CDP workflow](#gmailgoogle-login-hybrid-workflow) - login in real Chrome, use sessions in stealth automation
- **Stealth mode** bypasses basic bot detection (not Google - see [limitations](#stealth-mode-honest-limitations))
- **Runtime state load** - load auth into running browser, not just at launch
- **Auto-persistence** - automatic state save/restore between sessions
- **CDP flexibility** - connect via port numbers OR WebSocket URLs

## What's Different

*Compared to upstream [vercel-labs/agent-browser](https://github.com/vercel-labs/agent-browser) v0.6.0 (Jan 2026)*

| Feature | Upstream (v0.6.0) | This Fork |
|---------|-------------------|-----------|
| **Stealth Mode** | ❌ | ✅ playwright-extra integration |
| **Runtime State Load** | ❌ Returns "must load at launch" | ✅ Actually loads cookies + localStorage |
| **Auto-Persistence** | ❌ | ✅ `--persist` flag for automatic save/restore |
| **Lifecycle Control** | Implicit | ✅ Explicit `start`/`stop`/`status`/`configure` |
| **CDP Flexibility** | Port numbers only | ✅ Port + WebSocket URLs (`ws://`) |
| **Auto-detect Chrome** | ❌ | ✅ Finds system Chrome automatically |
| **Gmail Hybrid Workflow** | Not documented | ✅ Documented workflow that works |
| **Profile Mode** | ❌ | ✅ `--profile` for persistent Chrome profile |
| **Playwright MCP Backend** | ❌ | ✅ Control existing Chrome via extension |

**Note:** Upstream v0.6.0 added `connect` command, video recording, and `--proxy` flag. Both now have streaming. The key differences are in **usability** and **workflow support** - z-browser provides the tools needed for real-world AI agent tasks like authenticated automation.

## Browser Modes

```bash
z-agent-browser start                    # Headless (default)
z-agent-browser start --headed           # Visible browser
z-agent-browser start --stealth          # Bypass bot detection
z-agent-browser connect 9222             # Real Chrome with saved passwords
```

## Installation

### npm (recommended)

```bash
npm install -g z-agent-browser
z-agent-browser install  # Download Chromium
```

### From Source

```bash
git clone https://github.com/zm2231/agent-browser
cd agent-browser
pnpm install
pnpm build
pnpm build:native   # Requires Rust (https://rustup.rs)
pnpm link --global  # Makes z-agent-browser available globally
z-agent-browser install
```

### Upstream (Basic Features Only)

For the original vercel-labs version without enhanced features:
```bash
npm install -g agent-browser
```

### Linux Dependencies

On Linux, install system dependencies:

```bash
z-agent-browser install --with-deps
# or manually: npx playwright install-deps chromium
```

## Quick Start

```bash
z-agent-browser open example.com
z-agent-browser snapshot                    # Get accessibility tree with refs
z-agent-browser click @e2                   # Click by ref from snapshot
z-agent-browser fill @e3 "test@example.com" # Fill by ref
z-agent-browser get text @e1                # Get text by ref
z-agent-browser screenshot page.png
z-agent-browser close
```

### Traditional Selectors (also supported)

```bash
z-agent-browser click "#submit"
z-agent-browser fill "#email" "test@example.com"
z-agent-browser find role button click --name "Submit"
```

## Commands

### Core Commands

```bash
z-agent-browser open <url>              # Navigate to URL (aliases: goto, navigate)
z-agent-browser click <sel>             # Click element
z-agent-browser dblclick <sel>          # Double-click element
z-agent-browser focus <sel>             # Focus element
z-agent-browser type <sel> <text>       # Type into element
z-agent-browser fill <sel> <text>       # Clear and fill
z-agent-browser press <key>             # Press key (Enter, Tab, Control+a) (alias: key)
z-agent-browser keydown <key>           # Hold key down
z-agent-browser keyup <key>             # Release key
z-agent-browser hover <sel>             # Hover element
z-agent-browser select <sel> <val>      # Select dropdown option
z-agent-browser check <sel>             # Check checkbox
z-agent-browser uncheck <sel>           # Uncheck checkbox
z-agent-browser scroll <dir> [px]       # Scroll (up/down/left/right)
z-agent-browser scrollintoview <sel>    # Scroll element into view (alias: scrollinto)
z-agent-browser drag <src> <tgt>        # Drag and drop
z-agent-browser upload <sel> <files>    # Upload files
z-agent-browser screenshot [path]       # Take screenshot (--full for full page)
z-agent-browser pdf <path>              # Save as PDF
z-agent-browser snapshot                # Accessibility tree with refs (best for AI)
z-agent-browser eval <js>               # Run JavaScript
z-agent-browser connect <port>          # Connect to browser via CDP
z-agent-browser close                   # Close browser (aliases: quit, exit)
```

### Get Info

```bash
z-agent-browser get text <sel>          # Get text content
z-agent-browser get html <sel>          # Get innerHTML
z-agent-browser get value <sel>         # Get input value
z-agent-browser get attr <sel> <attr>   # Get attribute
z-agent-browser get title               # Get page title
z-agent-browser get url                 # Get current URL
z-agent-browser get count <sel>         # Count matching elements
z-agent-browser get box <sel>           # Get bounding box
```

### Check State

```bash
z-agent-browser is visible <sel>        # Check if visible
z-agent-browser is enabled <sel>        # Check if enabled
z-agent-browser is checked <sel>        # Check if checked
```

### Find Elements (Semantic Locators)

```bash
z-agent-browser find role <role> <action> [value]       # By ARIA role
z-agent-browser find text <text> <action>               # By text content
z-agent-browser find label <label> <action> [value]     # By label
z-agent-browser find placeholder <ph> <action> [value]  # By placeholder
z-agent-browser find alt <text> <action>                # By alt text
z-agent-browser find title <text> <action>              # By title attr
z-agent-browser find testid <id> <action> [value]       # By data-testid
z-agent-browser find first <sel> <action> [value]       # First match
z-agent-browser find last <sel> <action> [value]        # Last match
z-agent-browser find nth <n> <sel> <action> [value]     # Nth match
```

**Actions:** `click`, `fill`, `check`, `hover`, `text`

**Examples:**
```bash
z-agent-browser find role button click --name "Submit"
z-agent-browser find text "Sign In" click
z-agent-browser find label "Email" fill "test@test.com"
z-agent-browser find first ".item" click
z-agent-browser find nth 2 "a" text
```

### Wait

```bash
z-agent-browser wait <selector>         # Wait for element to be visible
z-agent-browser wait <ms>               # Wait for time (milliseconds)
z-agent-browser wait --text "Welcome"   # Wait for text to appear
z-agent-browser wait --url "**/dash"    # Wait for URL pattern
z-agent-browser wait --load networkidle # Wait for load state
z-agent-browser wait --fn "window.ready === true"  # Wait for JS condition
```

**Load states:** `load`, `domcontentloaded`, `networkidle`

### Mouse Control

```bash
z-agent-browser mouse move <x> <y>      # Move mouse
z-agent-browser mouse down [button]     # Press button (left/right/middle)
z-agent-browser mouse up [button]       # Release button
z-agent-browser mouse wheel <dy> [dx]   # Scroll wheel
```

### Browser Settings

```bash
z-agent-browser set viewport <w> <h>    # Set viewport size
z-agent-browser set device <name>       # Emulate device ("iPhone 14")
z-agent-browser set geo <lat> <lng>     # Set geolocation
z-agent-browser set offline [on|off]    # Toggle offline mode
z-agent-browser set headers <json>      # Extra HTTP headers
z-agent-browser set credentials <u> <p> # HTTP basic auth
z-agent-browser set media [dark|light]  # Emulate color scheme
```

### Cookies & Storage

```bash
z-agent-browser cookies                 # Get all cookies
z-agent-browser cookies set <name> <val> # Set cookie
z-agent-browser cookies clear           # Clear cookies

z-agent-browser storage local           # Get all localStorage
z-agent-browser storage local <key>     # Get specific key
z-agent-browser storage local set <k> <v>  # Set value
z-agent-browser storage local clear     # Clear all

z-agent-browser storage session         # Same for sessionStorage
```

### Network

```bash
z-agent-browser network route <url>              # Intercept requests
z-agent-browser network route <url> --abort      # Block requests
z-agent-browser network route <url> --body <json>  # Mock response
z-agent-browser network unroute [url]            # Remove routes
z-agent-browser network requests                 # View tracked requests
z-agent-browser network requests --filter api    # Filter requests
```

### Tabs & Windows

```bash
z-agent-browser tab                     # List tabs
z-agent-browser tab new [url]           # New tab (optionally with URL)
z-agent-browser tab <n>                 # Switch to tab n
z-agent-browser tab close [n]           # Close tab
z-agent-browser window new              # New window
```

### Frames

```bash
z-agent-browser frame <sel>             # Switch to iframe
z-agent-browser frame main              # Back to main frame
```

### Dialogs

```bash
z-agent-browser dialog accept [text]    # Accept (with optional prompt text)
z-agent-browser dialog dismiss          # Dismiss
```

### Debug

```bash
z-agent-browser trace start [path]      # Start recording trace
z-agent-browser trace stop [path]       # Stop and save trace
z-agent-browser console                 # View console messages
z-agent-browser console --clear         # Clear console
z-agent-browser errors                  # View page errors
z-agent-browser errors --clear          # Clear errors
z-agent-browser highlight <sel>         # Highlight element
z-agent-browser state save <path>       # Save auth state
z-agent-browser state load <path>       # Load auth state
```

### Navigation

```bash
z-agent-browser back                    # Go back
z-agent-browser forward                 # Go forward
z-agent-browser reload                  # Reload page
```

### Setup

```bash
z-agent-browser install                 # Download Chromium browser
z-agent-browser install --with-deps     # Also install system deps (Linux)
```

## Sessions

Run multiple isolated browser instances:

```bash
# Different sessions
z-agent-browser --session agent1 open site-a.com
z-agent-browser --session agent2 open site-b.com

# Or via environment variable
AGENT_BROWSER_SESSION=agent1 z-agent-browser click "#btn"

# List active sessions
z-agent-browser session list
# Output:
# Active sessions:
# -> default
#    agent1

# Show current session
z-agent-browser session
```

Each session has its own:
- Browser instance
- Cookies and storage
- Navigation history
- Authentication state

## Snapshot Options

The `snapshot` command supports filtering to reduce output size:

```bash
z-agent-browser snapshot                    # Full accessibility tree
z-agent-browser snapshot -i                 # Interactive elements only (buttons, inputs, links)
z-agent-browser snapshot -c                 # Compact (remove empty structural elements)
z-agent-browser snapshot -d 3               # Limit depth to 3 levels
z-agent-browser snapshot -s "#main"         # Scope to CSS selector
z-agent-browser snapshot -i -c -d 5         # Combine options
```

| Option | Description |
|--------|-------------|
| `-i, --interactive` | Only show interactive elements (buttons, links, inputs) |
| `-c, --compact` | Remove empty structural elements |
| `-d, --depth <n>` | Limit tree depth |
| `-s, --selector <sel>` | Scope to CSS selector |

## Token Efficiency: eval vs snapshot

For AI agents, token efficiency is critical. Use the right tool for the job:

### Use `snapshot -i` for navigation (finding what to click)
```bash
z-agent-browser snapshot -i   # Returns interactive elements with refs
# Output: ~200-500 tokens for buttons, links, inputs
```

### Use `eval` for data extraction (getting information)
```bash
# Instead of parsing a 5000-token snapshot, run JS to get exactly what you need:
z-agent-browser eval "document.querySelectorAll('.item').length"
z-agent-browser eval "[...document.querySelectorAll('a')].map(a => ({text: a.textContent, href: a.href}))"
z-agent-browser eval "document.querySelector('h1').textContent"
```

### When to use which

| Task | Best Tool | Token Cost |
|------|-----------|------------|
| Find button to click | `snapshot -i` | ~200-500 |
| Count items on page | `eval` | ~10 |
| Extract all links | `eval` | ~50-200 |
| Fill a form | `snapshot -i` + refs | ~200-500 |
| Check if logged in | `eval` | ~10 |
| Get table data | `eval` | ~100-500 |
| Navigate complex UI | `snapshot -i` | ~200-500 |

### Example: Extract data efficiently

**Bad** (snapshot approach - ~5000 tokens):
```bash
z-agent-browser snapshot    # Returns full page, AI parses it
```

**Good** (eval approach - ~100 tokens):
```bash
z-agent-browser eval "
  const rows = [...document.querySelectorAll('tr')];
  rows.slice(1, 11).map(r => ({
    title: r.cells[0]?.textContent?.trim(),
    link: r.querySelector('a')?.href
  }));
"
# Returns: [{title: "...", link: "..."}, ...]
```

**Rule of thumb**: 
- Need to CLICK/FILL something? → `snapshot -i` + refs
- Need to READ/COUNT/EXTRACT data? → `eval`

## Options

| Option | Description |
|--------|-------------|
| `--session <name>` | Use isolated session (or `AGENT_BROWSER_SESSION` env) |
| `--headers <json>` | Set HTTP headers scoped to the URL's origin |
| `--executable-path <path>` | Custom browser executable (or `AGENT_BROWSER_EXECUTABLE_PATH` env) |
| `--json` | JSON output (for agents) |
| `--full, -f` | Full page screenshot |
| `--name, -n` | Locator name filter |
| `--exact` | Exact text match |
| `--headed` | Show browser window (not headless) |
| `--cdp <port>` | Connect via Chrome DevTools Protocol |
| `--debug` | Debug output |

## Selectors

### Refs (Recommended for AI)

Refs provide deterministic element selection from snapshots:

```bash
# 1. Get snapshot with refs
z-agent-browser snapshot
# Output:
# - heading "Example Domain" [ref=e1] [level=1]
# - button "Submit" [ref=e2]
# - textbox "Email" [ref=e3]
# - link "Learn more" [ref=e4]

# 2. Use refs to interact
z-agent-browser click @e2                   # Click the button
z-agent-browser fill @e3 "test@example.com" # Fill the textbox
z-agent-browser get text @e1                # Get heading text
z-agent-browser hover @e4                   # Hover the link
```

**Why use refs?**
- **Deterministic**: Ref points to exact element from snapshot
- **Fast**: No DOM re-query needed
- **AI-friendly**: Snapshot + ref workflow is optimal for LLMs

### CSS Selectors

```bash
z-agent-browser click "#id"
z-agent-browser click ".class"
z-agent-browser click "div > button"
```

### Text & XPath

```bash
z-agent-browser click "text=Submit"
z-agent-browser click "xpath=//button"
```

### Semantic Locators

```bash
z-agent-browser find role button click --name "Submit"
z-agent-browser find label "Email" fill "test@test.com"
```

## Agent Mode

Use `--json` for machine-readable output:

```bash
z-agent-browser snapshot --json
# Returns: {"success":true,"data":{"snapshot":"...","refs":{"e1":{"role":"heading","name":"Title"},...}}}

z-agent-browser get text @e1 --json
z-agent-browser is visible @e2 --json
```

### Optimal AI Workflow

```bash
# 1. Navigate and get snapshot
z-agent-browser open example.com
z-agent-browser snapshot -i --json   # AI parses tree and refs

# 2. AI identifies target refs from snapshot
# 3. Execute actions using refs
z-agent-browser click @e2
z-agent-browser fill @e3 "input text"

# 4. Get new snapshot if page changed
z-agent-browser snapshot -i --json
```

## Headed Mode

Show the browser window for debugging:

```bash
z-agent-browser open example.com --headed
```

This opens a visible browser window instead of running headless.

## Authenticated Sessions

Use `--headers` to set HTTP headers for a specific origin, enabling authentication without login flows:

```bash
# Headers are scoped to api.example.com only
z-agent-browser open api.example.com --headers '{"Authorization": "Bearer <token>"}'

# Requests to api.example.com include the auth header
z-agent-browser snapshot -i --json
z-agent-browser click @e2

# Navigate to another domain - headers are NOT sent (safe!)
z-agent-browser open other-site.com
```

This is useful for:
- **Skipping login flows** - Authenticate via headers instead of UI
- **Switching users** - Start new sessions with different auth tokens
- **API testing** - Access protected endpoints directly
- **Security** - Headers are scoped to the origin, not leaked to other domains

To set headers for multiple origins, use `--headers` with each `open` command:

```bash
z-agent-browser open api.example.com --headers '{"Authorization": "Bearer token1"}'
z-agent-browser open api.acme.com --headers '{"Authorization": "Bearer token2"}'
```

For global headers (all domains), use `set headers`:

```bash
z-agent-browser set headers '{"X-Custom-Header": "value"}'
```

## Custom Browser Executable

Use a custom browser executable instead of the bundled Chromium. This is useful for:
- **Serverless deployment**: Use lightweight Chromium builds like `@sparticuz/chromium` (~50MB vs ~684MB)
- **System browsers**: Use an existing Chrome/Chromium installation
- **Custom builds**: Use modified browser builds

### CLI Usage

```bash
# Via flag
z-agent-browser --executable-path /path/to/chromium open example.com

# Via environment variable
AGENT_BROWSER_EXECUTABLE_PATH=/path/to/chromium z-agent-browser open example.com
```

### Serverless Example (Vercel/AWS Lambda)

```typescript
import chromium from '@sparticuz/chromium';
import { BrowserManager } from 'agent-browser';

export async function handler() {
  const browser = new BrowserManager();
  await browser.launch({
    executablePath: await chromium.executablePath(),
    headless: true,
  });
  // ... use browser
}
```

## CDP Mode

Connect to an existing browser via Chrome DevTools Protocol. Best for using real Chrome with saved passwords.

```bash
# Start Chrome with remote debugging
google-chrome --remote-debugging-port=9222

# Connect once, then run commands without --cdp
z-agent-browser connect 9222
z-agent-browser snapshot
z-agent-browser tab
z-agent-browser close

# Or pass --cdp on each command
z-agent-browser --cdp 9222 snapshot
```

This enables control of:
- Your real Chrome browser with saved passwords
- Electron apps
- Chrome/Chromium instances with remote debugging
- WebView2 applications
- Any browser exposing a CDP endpoint

**Important:** In CDP mode, headless/headed is determined by how Chrome was launched, not by z-agent-browser. The `--headed` flag has no effect when connecting via CDP.

```bash
# Headed CDP (visible browser)
google-chrome --remote-debugging-port=9222 &

# Headless CDP (no window) - use --headless=new flag when launching Chrome
google-chrome --headless=new --remote-debugging-port=9222 &
```

**For headless automation without real Chrome**, use State Save/Load instead (see above).

## Playwright MCP Mode (Experimental)

Control your existing browser session via the [Playwright MCP](https://github.com/microsoft/playwright-mcp) bridge extension. This allows AI agents to automate your actual browser instead of a separate headless instance.

### Setup

1. **Install the Chrome extension**
   - Install "Playwright MCP Bridge" from the Chrome Web Store
   - Or load unpacked from the playwright-mcp repo's `extension/` directory

2. **Set your extension token and run**
   ```bash
   # Set the token from the Chrome extension
   export PLAYWRIGHT_MCP_EXTENSION_TOKEN=your-token-here
   export AGENT_BROWSER_BACKEND=playwright-mcp
   
   # Commands work the same as native mode
   z-agent-browser open "https://example.com"
   z-agent-browser snapshot -i
   z-agent-browser click @e1
   z-agent-browser back
   z-agent-browser close
   ```

The daemon spawns `npx @playwright/mcp@latest --extension` as a subprocess and communicates via stdio. No separate server needed.

### Environment Variables

| Variable | Description |
|----------|-------------|
| `AGENT_BROWSER_BACKEND` | Set to `playwright-mcp` to use MCP mode (default: `native`) |
| `PLAYWRIGHT_MCP_EXTENSION_TOKEN` | Token from the Chrome extension (required for extension mode) |
| `PLAYWRIGHT_MCP_COMMAND` | Custom command to spawn MCP server (default: `npx`) |
| `PLAYWRIGHT_MCP_ARGS` | Space-separated args (default: `@playwright/mcp@latest --extension`) |

### Limitations

- **Feature parity**: Not all commands are supported. Streaming, state save/load, and stealth mode are not available in MCP mode.
- **Extension required**: The Chrome extension must be installed and connected for the MCP server to control your browser.

### Use Cases

- **AI-assisted browsing**: Let AI agents help you navigate complex web apps in your actual browser
- **Testing with extensions**: Test sites that require specific browser extensions
- **Debugging**: Watch AI actions in real-time in your browser

## Streaming (Browser Preview)

Stream the browser viewport via WebSocket for live preview or "pair browsing" where a human can watch and interact alongside an AI agent.

### Enable Streaming

Set the `AGENT_BROWSER_STREAM_PORT` environment variable:

```bash
AGENT_BROWSER_STREAM_PORT=9223 z-agent-browser open example.com
```

This starts a WebSocket server on the specified port that streams the browser viewport and accepts input events.

### WebSocket Protocol

Connect to `ws://localhost:9223` to receive frames and send input:

**Receive frames:**
```json
{
  "type": "frame",
  "data": "<base64-encoded-jpeg>",
  "metadata": {
    "deviceWidth": 1280,
    "deviceHeight": 720,
    "pageScaleFactor": 1,
    "offsetTop": 0,
    "scrollOffsetX": 0,
    "scrollOffsetY": 0
  }
}
```

**Send mouse events:**
```json
{
  "type": "input_mouse",
  "eventType": "mousePressed",
  "x": 100,
  "y": 200,
  "button": "left",
  "clickCount": 1
}
```

**Send keyboard events:**
```json
{
  "type": "input_keyboard",
  "eventType": "keyDown",
  "key": "Enter",
  "code": "Enter"
}
```

**Send touch events:**
```json
{
  "type": "input_touch",
  "eventType": "touchStart",
  "touchPoints": [{ "x": 100, "y": 200 }]
}
```

### Programmatic API

For advanced use, control streaming directly via the protocol:

```typescript
import { BrowserManager } from 'agent-browser';

const browser = new BrowserManager();
await browser.launch({ headless: true });
await browser.navigate('https://example.com');

// Start screencast
await browser.startScreencast((frame) => {
  // frame.data is base64-encoded image
  // frame.metadata contains viewport info
  console.log('Frame received:', frame.metadata.deviceWidth, 'x', frame.metadata.deviceHeight);
}, {
  format: 'jpeg',
  quality: 80,
  maxWidth: 1280,
  maxHeight: 720,
});

// Inject mouse events
await browser.injectMouseEvent({
  type: 'mousePressed',
  x: 100,
  y: 200,
  button: 'left',
});

// Inject keyboard events
await browser.injectKeyboardEvent({
  type: 'keyDown',
  key: 'Enter',
  code: 'Enter',
});

// Stop when done
await browser.stopScreencast();
```

## Architecture

z-agent-browser uses a client-daemon architecture:

1. **Rust CLI** (fast native binary) - Parses commands, communicates with daemon
2. **Node.js Daemon** - Manages Playwright browser instance
3. **Fallback** - If native binary unavailable, uses Node.js directly

The daemon starts automatically on first command and persists between commands for fast subsequent operations.

**Browser Engine:** Uses Chromium by default. The daemon also supports Firefox and WebKit via the Playwright protocol.

## Platforms

| Platform | Binary | Fallback |
|----------|--------|----------|
| macOS ARM64 | Native Rust | Node.js |
| macOS x64 | Native Rust | Node.js |
| Linux ARM64 | Native Rust | Node.js |
| Linux x64 | Native Rust | Node.js |
| Windows x64 | Native Rust | Node.js |

## Usage with AI Agents

### Just ask the agent

The simplest approach - just tell your agent to use it:

```
Use z-agent-browser to test the login flow. Run z-agent-browser --help to see available commands.
```

The `--help` output is comprehensive and most agents can figure it out from there.

### AGENTS.md / CLAUDE.md

For more consistent results, add to your project or global instructions file:

```markdown
## Browser Automation

Use `z-agent-browser` for web automation. Run `z-agent-browser --help` for all commands.

Core workflow:
1. `z-agent-browser open <url>` - Navigate to page
2. `z-agent-browser snapshot -i` - Get interactive elements with refs (@e1, @e2)
3. `z-agent-browser click @e1` / `fill @e2 "text"` - Interact using refs
4. Re-snapshot after page changes
```

### Claude Code Skill

For Claude Code, install the [browser-skill](https://github.com/zm2231/browser-skill):

```
/plugin marketplace add zm2231/browser-skill
/plugin install browser-skill@browser-skill-marketplace
```

Or manually:

```bash
mkdir -p ~/.claude/skills/browser-automation
curl -o ~/.claude/skills/browser-automation/skill.md \
  https://raw.githubusercontent.com/zm2231/browser-skill/main/skills/browser-automation/skill.md
```

## Enhanced Fork Features

This fork (zm2231/agent-browser) adds features for bot detection bypass, persistent auth, custom profiles, and more.

### Installation (Enhanced Fork)

```bash
git clone https://github.com/zm2231/agent-browser.git
cd agent-browser
pnpm install
pnpm build
pnpm build:native   # requires Rust: https://rustup.rs
npm link
z-agent-browser install
```

### Stealth Mode

Bypass bot detection using playwright-extra with stealth plugin:

```bash
z-agent-browser --stealth open https://bot.sannysoft.com
z-agent-browser snapshot -i
# Basic bot detection tests pass

# Via environment variable
AGENT_BROWSER_STEALTH=1 z-agent-browser open https://example.com
```

Stealth mode applies evasions for: WebDriver detection, Chrome automation flags, permissions, plugins, languages, WebGL, and more.

#### Stealth Mode: Honest Limitations

Stealth mode is **not a silver bullet**. Based on 2024-2025 research:

| Protection | Success Rate | Notes |
|------------|--------------|-------|
| Basic bot detection | ~60-80% | Simple WAFs, basic fingerprinting |
| Simple e-commerce | ~60-70% | Sites without advanced protection |
| DataDome | **~0%** | They detect stealth plugin in 4 lines of JS |
| Modern Cloudflare (Turnstile) | ~10-20% | Only basic challenges work |
| **Google/Gmail** | **<5%** | Use [hybrid workflow](#gmailgoogle-login-hybrid-workflow) instead |

**For Google/Gmail:** Stealth alone will NOT work. Use the hybrid CDP workflow (login in real Chrome → save state → use in stealth automation). This is why z-browser documents and supports that workflow.

**For enterprise anti-bot (DataDome, Cloudflare ML):** Consider commercial services (ZenRows, Bright Data) or residential proxies with CAPTCHA solving.

### Auto-Persistence

Save and restore auth state automatically between sessions:

```bash
# First session: log in with --persist
z-agent-browser --persist open "https://github.com/login" --headed
# User logs in manually
z-agent-browser close   # State saved to ~/.z-agent-browser/sessions/default.json

# Later sessions: auth restored automatically
z-agent-browser --persist open "https://github.com"   # Already logged in
```

Use explicit state file:
```bash
z-agent-browser --state ~/github-auth.json open "https://github.com"
```

### Login Persistence (State Save/Load)

Save login sessions to a JSON file and restore them later:

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

**Key points:**
- Saves cookies, localStorage, sessionStorage to JSON file
- Portable across sessions and restarts
- Works on both Mac and Linux
- Default state path: `~/.z-agent-browser/default-state.json`

**Headless Limitation:** Google, Gmail, and other strict sites detect headless Chromium and invalidate sessions. For these sites, use `--headed` or CDP Mode with real Chrome.app.

**State Save/Load vs CDP Mode:**

| Feature | State Save/Load | CDP Mode |
|---------|-----------------|----------|
| Command | `state save/load <path>` | `connect <port>` |
| Headless support | Yes | Depends on how Chrome was launched |
| Saved passwords | No (session cookies only) | Yes (real Chrome) |
| Best for | Background automation | Real Chrome, saved passwords, CAPTCHA |

### Gmail/Google Login (Hybrid Workflow)

Google detects Playwright (even stealth mode) and blocks automated login. This hybrid workflow **actually works** - tested successfully for reading Gmail:

**The key insight:** Google validates during LOGIN (detecting automation), but accepts valid cookies afterward. Get cookies from real Chrome, use them in stealth automation.

#### Option 1: AI-Assisted Login (Recommended)

Let your AI agent handle the workflow. The agent can:
1. Detect that headed mode is needed for login
2. Switch to `--headed` mode
3. Prompt you to login manually in the visible browser
4. Save state after login
5. Continue automation in stealth mode

```bash
# AI agent workflow (what actually worked):
z-agent-browser start --headed        # Agent switches to headed
z-agent-browser open "https://mail.google.com"
# User logs in manually when prompted
z-agent-browser state save ~/.z-agent-browser/gmail-state.json
z-agent-browser configure --stealth   # Switch to stealth
z-agent-browser state load ~/.z-agent-browser/gmail-state.json
# Now automation works!
```

#### Option 2: Manual CDP Workflow

```bash
# 1. Copy your Chrome profile (one-time)
cp -R "$HOME/Library/Application Support/Google/Chrome" ~/.z-agent-browser/cdp-profile

# 2. Launch real Chrome with CDP
killall "Google Chrome" 2>/dev/null || true
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
  --remote-debugging-port=9222 \
  --user-data-dir="$HOME/.z-agent-browser/cdp-profile" &

# 3. Connect and verify CDP is working
z-agent-browser connect 9222
z-agent-browser open "https://mail.google.com"
# If not logged in, login manually in the Chrome window

# 4. Save state for future use
z-agent-browser state save ~/.z-agent-browser/gmail-state.json
z-agent-browser close
killall "Google Chrome"

# 5. Now use headless stealth with saved state
z-agent-browser start --stealth
z-agent-browser state load ~/.z-agent-browser/gmail-state.json
z-agent-browser open "https://mail.google.com"  # Logged in!
```

**Why this works:** Google validates the session during login (detecting Playwright), but once you have valid cookies from real Chrome, Playwright stealth can use them for reading/navigation.

**Important:** Session cookies may expire. Re-run the login workflow periodically.

### Custom User-Agent

```bash
z-agent-browser --user-agent "MyBot/1.0 (compatible)" open https://httpbin.org/user-agent
```

### Browser Launch Arguments

Pass custom Chromium flags:

```bash
z-agent-browser --args "--disable-gpu,--no-sandbox" open https://example.com
```

Common args:
- `--disable-gpu`: disable GPU acceleration
- `--no-sandbox`: required in some Docker containers
- `--disable-dev-shm-usage`: overcome limited /dev/shm in Docker
- `--window-size=1920,1080`: set initial window size

### HTTPS Certificate Errors

Skip SSL validation for local dev servers with self-signed certs:

```bash
z-agent-browser --ignore-https-errors open "https://localhost:8443"
```

**Note**: When changing launch options (like --ignore-https-errors), kill any existing daemon first:
```bash
pkill -f "node.*daemon"; sleep 1
z-agent-browser --ignore-https-errors open "https://localhost:8443"
```

### Video Recording

Record browser sessions to WebM:

```bash
z-agent-browser open "https://example.com" --headed
z-agent-browser record start ./demo.webm
z-agent-browser fill @e1 "demo input"
z-agent-browser click @e2
z-agent-browser record stop
# Video saved to ./demo.webm

# Restart recording with new file
z-agent-browser record restart ./take2.webm
```

Recording creates a fresh context but preserves cookies and storage.

### Tab New with URL

Open new tab directly at a URL:

```bash
z-agent-browser tab new https://example.com
```

### Screenshot to Base64

Omit path to get base64-encoded PNG:

```bash
z-agent-browser screenshot --json
# Returns: {"success":true,"data":{"base64":"iVBORw0KGgo..."}}
```

### Runtime State Load

Load auth state into a running browser (not just at launch):

```bash
z-agent-browser open "https://github.com"
z-agent-browser state load ~/.browser/github-auth.json   # Loads into current session
```

### Connect Command

Establish persistent CDP connection; subsequent commands omit --cdp:

```bash
# Start Chrome with remote debugging
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
  --remote-debugging-port=9222 --user-data-dir=/tmp/chrome-debug &

# Connect once
z-agent-browser connect 9222

# All subsequent commands use CDP automatically
z-agent-browser open "https://example.com"
z-agent-browser snapshot -i
z-agent-browser close
```

### Special URL Schemes

Support for about:, data:, and file: URLs:

```bash
z-agent-browser open "about:blank"
z-agent-browser open "data:text/html,<h1>Hello</h1>"
z-agent-browser open "file:///path/to/local.html"
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `AGENT_BROWSER_SESSION` | Session name for isolation |
| `AGENT_BROWSER_HEADED` | Set to "1" for visible browser |
| `AGENT_BROWSER_STEALTH` | Set to "1" for stealth mode |
| `AGENT_BROWSER_PERSIST` | Set to "1" for auto-persistence |
| `AGENT_BROWSER_STATE` | Path to state file |
| `AGENT_BROWSER_PROFILE` | Path to Chrome profile directory |
| `AGENT_BROWSER_USER_AGENT` | Custom User-Agent string |
| `AGENT_BROWSER_ARGS` | Comma-separated browser launch args |
| `AGENT_BROWSER_IGNORE_HTTPS_ERRORS` | Set to "1" to skip SSL validation |
| `AGENT_BROWSER_EXECUTABLE_PATH` | Custom browser binary path |
| `AGENT_BROWSER_EXTENSIONS` | Path to browser extensions |
| `AGENT_BROWSER_STREAM_PORT` | WebSocket port for streaming |
| `AGENT_BROWSER_BACKEND` | Backend type: `native` (default) or `playwright-mcp` |
| `PLAYWRIGHT_MCP_COMMAND` | Command to spawn MCP server (default: `npx`) |
| `PLAYWRIGHT_MCP_ARGS` | Space-separated args for MCP server (default: `@playwright/mcp@latest`) |
| `NO_COLOR` | Disable colored output |

### Known Issues

**--ignore-https-errors with existing daemon**: If daemon already has a browser context, new launch options may not apply. Kill daemon before changing options:
```bash
pkill -f "node.*daemon"
```

## Acknowledgments

- [Playwright MCP](https://github.com/microsoft/playwright-mcp) by Microsoft - Powers the experimental MCP backend mode
- [Playwright](https://playwright.dev/) by Microsoft - Core browser automation engine
- [vercel-labs/agent-browser](https://github.com/vercel-labs/agent-browser) - Original project this fork is based on

## License

Apache-2.0
