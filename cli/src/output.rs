use crate::color;
use crate::connection::Response;

pub fn print_response(resp: &Response, json_mode: bool) {
    if json_mode {
        println!("{}", serde_json::to_string(resp).unwrap_or_default());
        return;
    }

    if !resp.success {
        eprintln!(
            "{} {}",
            color::error_indicator(),
            resp.error.as_deref().unwrap_or("Unknown error")
        );
        return;
    }

    if let Some(data) = &resp.data {
        // Navigation response
        if let Some(url) = data.get("url").and_then(|v| v.as_str()) {
            if let Some(title) = data.get("title").and_then(|v| v.as_str()) {
                println!("{} {}", color::success_indicator(), color::bold(title));
                println!("  {}", color::dim(url));
                return;
            }
            println!("{}", url);
            return;
        }
        // Snapshot
        if let Some(snapshot) = data.get("snapshot").and_then(|v| v.as_str()) {
            println!("{}", snapshot);
            return;
        }
        // Title
        if let Some(title) = data.get("title").and_then(|v| v.as_str()) {
            println!("{}", title);
            return;
        }
        // Text
        if let Some(text) = data.get("text").and_then(|v| v.as_str()) {
            println!("{}", text);
            return;
        }
        // HTML
        if let Some(html) = data.get("html").and_then(|v| v.as_str()) {
            println!("{}", html);
            return;
        }
        // Value
        if let Some(value) = data.get("value").and_then(|v| v.as_str()) {
            println!("{}", value);
            return;
        }
        // Count
        if let Some(count) = data.get("count").and_then(|v| v.as_i64()) {
            println!("{}", count);
            return;
        }
        // Boolean results
        if let Some(visible) = data.get("visible").and_then(|v| v.as_bool()) {
            println!("{}", visible);
            return;
        }
        if let Some(enabled) = data.get("enabled").and_then(|v| v.as_bool()) {
            println!("{}", enabled);
            return;
        }
        if let Some(checked) = data.get("checked").and_then(|v| v.as_bool()) {
            println!("{}", checked);
            return;
        }
        // Eval result
        if let Some(result) = data.get("result") {
            println!(
                "{}",
                serde_json::to_string_pretty(result).unwrap_or_default()
            );
            return;
        }
        // Tabs
        if let Some(tabs) = data.get("tabs").and_then(|v| v.as_array()) {
            for (i, tab) in tabs.iter().enumerate() {
                let title = tab
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Untitled");
                let url = tab.get("url").and_then(|v| v.as_str()).unwrap_or("");
                let active = tab.get("active").and_then(|v| v.as_bool()).unwrap_or(false);
                let marker = if active { "→" } else { " " };
                println!("{} [{}] {} - {}", marker, i, title, url);
            }
            return;
        }
        // Console logs
        if let Some(logs) = data.get("messages").and_then(|v| v.as_array()) {
            for log in logs {
                let level = log.get("type").and_then(|v| v.as_str()).unwrap_or("log");
                let text = log.get("text").and_then(|v| v.as_str()).unwrap_or("");
                println!("{} {}", color::console_level_prefix(level), text);
            }
            return;
        }
        // Errors
        if let Some(errors) = data.get("errors").and_then(|v| v.as_array()) {
            for err in errors {
                let msg = err.get("message").and_then(|v| v.as_str()).unwrap_or("");
                println!("{} {}", color::error_indicator(), msg);
            }
            return;
        }
        // Cookies
        if let Some(cookies) = data.get("cookies").and_then(|v| v.as_array()) {
            for cookie in cookies {
                let name = cookie.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let value = cookie.get("value").and_then(|v| v.as_str()).unwrap_or("");
                println!("{}={}", name, value);
            }
            return;
        }
        // Bounding box
        if let Some(box_data) = data.get("box") {
            println!(
                "{}",
                serde_json::to_string_pretty(box_data).unwrap_or_default()
            );
            return;
        }
        // Closed
        if data.get("closed").is_some() {
            println!("{} Browser closed", color::success_indicator());
            return;
        }
        // Recording start (has "started" field)
        if let Some(started) = data.get("started").and_then(|v| v.as_bool()) {
            if started {
                if let Some(path) = data.get("path").and_then(|v| v.as_str()) {
                    println!("{} Recording started: {}", color::success_indicator(), path);
                } else {
                    println!("{} Recording started", color::success_indicator());
                }
                return;
            }
        }
        // Recording restart (has "stopped" field - from recording_restart action)
        if data.get("stopped").is_some() {
            let path = data.get("path").and_then(|v| v.as_str()).unwrap_or("unknown");
            if let Some(prev_path) = data.get("previousPath").and_then(|v| v.as_str()) {
                println!("{} Recording restarted: {} (previous saved to {})", color::success_indicator(), path, prev_path);
            } else {
                println!("{} Recording started: {}", color::success_indicator(), path);
            }
            return;
        }
        // Recording stop (has "frames" field - from recording_stop action)
        if data.get("frames").is_some() {
            if let Some(path) = data.get("path").and_then(|v| v.as_str()) {
                if let Some(error) = data.get("error").and_then(|v| v.as_str()) {
                    println!("{} Recording saved to {} - {}", color::warning_indicator(), path, error);
                } else {
                    println!("{} Recording saved to {}", color::success_indicator(), path);
                }
            } else {
                println!("{} Recording stopped", color::success_indicator());
            }
            return;
        }
        // Screenshot path (no "started" or "frames" field)
        if let Some(path) = data.get("path").and_then(|v| v.as_str()) {
            println!("{} Screenshot saved to {}", color::success_indicator(), path);
            return;
        }
        // Default success
        println!("{} Done", color::success_indicator());
    }
}

/// Print command-specific help. Returns true if help was printed, false if command unknown.
pub fn print_command_help(command: &str) -> bool {
    let help = match command {
        // === Navigation ===
        "open" | "goto" | "navigate" => r##"
z-agent-browser open - Navigate to a URL

Usage: z-agent-browser open <url>

Navigates the browser to the specified URL. If no protocol is provided,
https:// is automatically prepended.

Aliases: goto, navigate

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session
  --headers <json>     Set HTTP headers (scoped to this origin)
  --headed             Show browser window

Examples:
  z-agent-browser open example.com
  z-agent-browser open https://github.com
  z-agent-browser open localhost:3000
  z-agent-browser open api.example.com --headers '{"Authorization": "Bearer token"}'
    # ^ Headers only sent to api.example.com, not other domains
"##,
        "back" => r##"
z-agent-browser back - Navigate back in history

Usage: z-agent-browser back

Goes back one page in the browser history, equivalent to clicking
the browser's back button.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser back
"##,
        "forward" => r##"
z-agent-browser forward - Navigate forward in history

Usage: z-agent-browser forward

Goes forward one page in the browser history, equivalent to clicking
the browser's forward button.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser forward
"##,
        "reload" => r##"
z-agent-browser reload - Reload the current page

Usage: z-agent-browser reload

Reloads the current page, equivalent to pressing F5 or clicking
the browser's reload button.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser reload
"##,

        // === Core Actions ===
        "click" => r##"
z-agent-browser click - Click an element

Usage: z-agent-browser click <selector>

Clicks on the specified element. The selector can be a CSS selector,
XPath, or an element reference from snapshot (e.g., @e1).

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser click "#submit-button"
  z-agent-browser click @e1
  z-agent-browser click "button.primary"
  z-agent-browser click "//button[@type='submit']"
"##,
        "dblclick" => r##"
z-agent-browser dblclick - Double-click an element

Usage: z-agent-browser dblclick <selector>

Double-clicks on the specified element. Useful for text selection
or triggering double-click handlers.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser dblclick "#editable-text"
  z-agent-browser dblclick @e5
"##,
        "fill" => r##"
z-agent-browser fill - Clear and fill an input field

Usage: z-agent-browser fill <selector> <text>

Clears the input field and fills it with the specified text.
This replaces any existing content in the field.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser fill "#email" "user@example.com"
  z-agent-browser fill @e3 "Hello World"
  z-agent-browser fill "input[name='search']" "query"
"##,
        "type" => r##"
z-agent-browser type - Type text into an element

Usage: z-agent-browser type <selector> <text>

Types text into the specified element character by character.
Unlike fill, this does not clear existing content first.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser type "#search" "hello"
  z-agent-browser type @e2 "additional text"
"##,
        "hover" => r##"
z-agent-browser hover - Hover over an element

Usage: z-agent-browser hover <selector>

Moves the mouse to hover over the specified element. Useful for
triggering hover states or dropdown menus.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser hover "#dropdown-trigger"
  z-agent-browser hover @e4
"##,
        "focus" => r##"
z-agent-browser focus - Focus an element

Usage: z-agent-browser focus <selector>

Sets keyboard focus to the specified element.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser focus "#input-field"
  z-agent-browser focus @e2
"##,
        "check" => r##"
z-agent-browser check - Check a checkbox

Usage: z-agent-browser check <selector>

Checks a checkbox element. If already checked, no action is taken.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser check "#terms-checkbox"
  z-agent-browser check @e7
"##,
        "uncheck" => r##"
z-agent-browser uncheck - Uncheck a checkbox

Usage: z-agent-browser uncheck <selector>

Unchecks a checkbox element. If already unchecked, no action is taken.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser uncheck "#newsletter-opt-in"
  z-agent-browser uncheck @e8
"##,
        "select" => r##"
z-agent-browser select - Select a dropdown option

Usage: z-agent-browser select <selector> <value>

Selects an option in a <select> dropdown by its value attribute.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser select "#country" "US"
  z-agent-browser select @e5 "option2"
"##,
        "drag" => r##"
z-agent-browser drag - Drag and drop

Usage: z-agent-browser drag <source> <target>

Drags an element from source to target location.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser drag "#draggable" "#drop-zone"
  z-agent-browser drag @e1 @e2
"##,
        "upload" => r##"
z-agent-browser upload - Upload files

Usage: z-agent-browser upload <selector> <files...>

Uploads one or more files to a file input element.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser upload "#file-input" ./document.pdf
  z-agent-browser upload @e3 ./image1.png ./image2.png
"##,

        // === Keyboard ===
        "press" | "key" => r##"
z-agent-browser press - Press a key or key combination

Usage: z-agent-browser press <key>

Presses a key or key combination. Supports special keys and modifiers.

Aliases: key

Special Keys:
  Enter, Tab, Escape, Backspace, Delete, Space
  ArrowUp, ArrowDown, ArrowLeft, ArrowRight
  Home, End, PageUp, PageDown
  F1-F12

Modifiers (combine with +):
  Control, Alt, Shift, Meta

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser press Enter
  z-agent-browser press Tab
  z-agent-browser press Control+a
  z-agent-browser press Control+Shift+s
  z-agent-browser press Escape
"##,
        "keydown" => r##"
z-agent-browser keydown - Press a key down (without release)

Usage: z-agent-browser keydown <key>

Presses a key down without releasing it. Use keyup to release.
Useful for holding modifier keys.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser keydown Shift
  z-agent-browser keydown Control
"##,
        "keyup" => r##"
z-agent-browser keyup - Release a key

Usage: z-agent-browser keyup <key>

Releases a key that was pressed with keydown.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser keyup Shift
  z-agent-browser keyup Control
"##,

        // === Scroll ===
        "scroll" => r##"
z-agent-browser scroll - Scroll the page

Usage: z-agent-browser scroll [direction] [amount]

Scrolls the page in the specified direction.

Arguments:
  direction            up, down, left, right (default: down)
  amount               Pixels to scroll (default: 300)

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser scroll
  z-agent-browser scroll down 500
  z-agent-browser scroll up 200
  z-agent-browser scroll left 100
"##,
        "scrollintoview" | "scrollinto" => r##"
z-agent-browser scrollintoview - Scroll element into view

Usage: z-agent-browser scrollintoview <selector>

Scrolls the page until the specified element is visible in the viewport.

Aliases: scrollinto

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser scrollintoview "#footer"
  z-agent-browser scrollintoview @e15
"##,

        // === Wait ===
        "wait" => r##"
z-agent-browser wait - Wait for condition

Usage: z-agent-browser wait <selector|ms|option>

Waits for an element to appear, a timeout, or other conditions.

Modes:
  <selector>           Wait for element to appear
  <ms>                 Wait for specified milliseconds
  --url <pattern>      Wait for URL to match pattern
  --load <state>       Wait for load state (load, domcontentloaded, networkidle)
  --fn <expression>    Wait for JavaScript expression to be truthy
  --text <text>        Wait for text to appear on page

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser wait "#loading-spinner"
  z-agent-browser wait 2000
  z-agent-browser wait --url "**/dashboard"
  z-agent-browser wait --load networkidle
  z-agent-browser wait --fn "window.appReady === true"
  z-agent-browser wait --text "Welcome back"
"##,

        // === Screenshot/PDF ===
        "screenshot" => r##"
z-agent-browser screenshot - Take a screenshot

Usage: z-agent-browser screenshot [path]

Captures a screenshot of the current page. If no path is provided,
outputs base64-encoded image data.

Options:
  --full, -f           Capture full page (not just viewport)

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser screenshot
  z-agent-browser screenshot ./screenshot.png
  z-agent-browser screenshot --full ./full-page.png
"##,
        "pdf" => r##"
z-agent-browser pdf - Save page as PDF

Usage: z-agent-browser pdf <path>

Saves the current page as a PDF file.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser pdf ./page.pdf
  z-agent-browser pdf ~/Documents/report.pdf
"##,

        // === Snapshot ===
        "snapshot" => r##"
z-agent-browser snapshot - Get accessibility tree snapshot

Usage: z-agent-browser snapshot [options]

Returns an accessibility tree representation of the page with element
references (like @e1, @e2) that can be used in subsequent commands.
Designed for AI agents to understand page structure.

Options:
  -i, --interactive    Only include interactive elements
  -c, --compact        Remove empty structural elements
  -d, --depth <n>      Limit tree depth
  -s, --selector <sel> Scope snapshot to CSS selector

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser snapshot
  z-agent-browser snapshot -i
  z-agent-browser snapshot --compact --depth 5
  z-agent-browser snapshot -s "#main-content"
"##,

        // === Eval ===
        "eval" => r##"
z-agent-browser eval - Execute JavaScript

Usage: z-agent-browser eval <script>

Executes JavaScript code in the browser context and returns the result.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser eval "document.title"
  z-agent-browser eval "window.location.href"
  z-agent-browser eval "document.querySelectorAll('a').length"
"##,

        // === Close ===
        "close" | "quit" | "exit" => r##"
z-agent-browser close - Close the browser

Usage: z-agent-browser close

Closes the browser instance for the current session.

Aliases: quit, exit

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser close
  z-agent-browser close --session mysession
"##,

        // === Get ===
        "get" => r##"
z-agent-browser get - Retrieve information from elements or page

Usage: z-agent-browser get <subcommand> [args]

Retrieves various types of information from elements or the page.

Subcommands:
  text <selector>            Get text content of element
  html <selector>            Get inner HTML of element
  value <selector>           Get value of input element
  attr <selector> <name>     Get attribute value
  title                      Get page title
  url                        Get current URL
  count <selector>           Count matching elements
  box <selector>             Get bounding box (x, y, width, height)

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser get text @e1
  z-agent-browser get html "#content"
  z-agent-browser get value "#email-input"
  z-agent-browser get attr "#link" href
  z-agent-browser get title
  z-agent-browser get url
  z-agent-browser get count "li.item"
  z-agent-browser get box "#header"
"##,

        // === Is ===
        "is" => r##"
z-agent-browser is - Check element state

Usage: z-agent-browser is <subcommand> <selector>

Checks the state of an element and returns true/false.

Subcommands:
  visible <selector>   Check if element is visible
  enabled <selector>   Check if element is enabled (not disabled)
  checked <selector>   Check if checkbox/radio is checked

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser is visible "#modal"
  z-agent-browser is enabled "#submit-btn"
  z-agent-browser is checked "#agree-checkbox"
"##,

        // === Find ===
        "find" => r##"
z-agent-browser find - Find and interact with elements by locator

Usage: z-agent-browser find <locator> <value> [action] [text]

Finds elements using semantic locators and optionally performs an action.

Locators:
  role <role>              Find by ARIA role (--name <n>, --exact)
  text <text>              Find by text content (--exact)
  label <label>            Find by associated label (--exact)
  placeholder <text>       Find by placeholder text (--exact)
  alt <text>               Find by alt text (--exact)
  title <text>             Find by title attribute (--exact)
  testid <id>              Find by data-testid attribute
  first <selector>         First matching element
  last <selector>          Last matching element
  nth <index> <selector>   Nth matching element (0-based)

Actions (default: click):
  click, fill, type, hover, focus, check, uncheck

Options:
  --name <name>        Filter role by accessible name
  --exact              Require exact text match

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser find role button click --name Submit
  z-agent-browser find text "Sign In" click
  z-agent-browser find label "Email" fill "user@example.com"
  z-agent-browser find placeholder "Search..." type "query"
  z-agent-browser find testid "login-form" click
  z-agent-browser find first "li.item" click
  z-agent-browser find nth 2 ".card" hover
"##,

        // === Mouse ===
        "mouse" => r##"
z-agent-browser mouse - Low-level mouse operations

Usage: z-agent-browser mouse <subcommand> [args]

Performs low-level mouse operations for precise control.

Subcommands:
  move <x> <y>         Move mouse to coordinates
  down [button]        Press mouse button (left, right, middle)
  up [button]          Release mouse button
  wheel <dy> [dx]      Scroll mouse wheel

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser mouse move 100 200
  z-agent-browser mouse down
  z-agent-browser mouse up
  z-agent-browser mouse down right
  z-agent-browser mouse wheel 100
  z-agent-browser mouse wheel -50 0
"##,

        // === Set ===
        "set" => r##"
z-agent-browser set - Configure browser settings

Usage: z-agent-browser set <setting> [args]

Configures various browser settings and emulation options.

Settings:
  viewport <w> <h>           Set viewport size
  device <name>              Emulate device (e.g., "iPhone 12")
  geo <lat> <lng>            Set geolocation
  offline [on|off]           Toggle offline mode
  headers <json>             Set extra HTTP headers
  credentials <user> <pass>  Set HTTP authentication
  media [dark|light]         Set color scheme preference
        [reduced-motion]     Enable reduced motion

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser set viewport 1920 1080
  z-agent-browser set device "iPhone 12"
  z-agent-browser set geo 37.7749 -122.4194
  z-agent-browser set offline on
  z-agent-browser set headers '{"X-Custom": "value"}'
  z-agent-browser set credentials admin secret123
  z-agent-browser set media dark
  z-agent-browser set media light reduced-motion
"##,

        // === Network ===
        "network" => r##"
z-agent-browser network - Network interception and monitoring

Usage: z-agent-browser network <subcommand> [args]

Intercept, mock, or monitor network requests.

Subcommands:
  route <url> [options]      Intercept requests matching URL pattern
    --abort                  Abort matching requests
    --body <json>            Respond with custom body
  unroute [url]              Remove route (all if no URL)
  requests [options]         List captured requests
    --clear                  Clear request log
    --filter <pattern>       Filter by URL pattern

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser network route "**/api/*" --abort
  z-agent-browser network route "**/data.json" --body '{"mock": true}'
  z-agent-browser network unroute
  z-agent-browser network requests
  z-agent-browser network requests --filter "api"
  z-agent-browser network requests --clear
"##,

        // === Storage ===
        "storage" => r##"
z-agent-browser storage - Manage web storage

Usage: z-agent-browser storage <type> [operation] [key] [value]

Manage localStorage and sessionStorage.

Types:
  local                localStorage
  session              sessionStorage

Operations:
  get [key]            Get all storage or specific key
  set <key> <value>    Set a key-value pair
  clear                Clear all storage

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser storage local
  z-agent-browser storage local get authToken
  z-agent-browser storage local set theme "dark"
  z-agent-browser storage local clear
  z-agent-browser storage session get userId
"##,

        // === Cookies ===
        "cookies" => r##"
z-agent-browser cookies - Manage browser cookies

Usage: z-agent-browser cookies [operation] [args]

Manage browser cookies for the current context.

Operations:
  get                  Get all cookies (default)
  set <name> <value>   Set a cookie
  clear                Clear all cookies

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser cookies
  z-agent-browser cookies get
  z-agent-browser cookies set session_id "abc123"
  z-agent-browser cookies clear
"##,

        // === Tabs ===
        "tab" => r##"
z-agent-browser tab - Manage browser tabs

Usage: z-agent-browser tab [operation] [args]

Manage browser tabs in the current window.

Operations:
  list                 List all tabs (default)
  new [url]            Open new tab
  close [index]        Close tab (current if no index)
  <index>              Switch to tab by index

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser tab
  z-agent-browser tab list
  z-agent-browser tab new
  z-agent-browser tab new https://example.com
  z-agent-browser tab 2
  z-agent-browser tab close
  z-agent-browser tab close 1
"##,

        // === Window ===
        "window" => r##"
z-agent-browser window - Manage browser windows

Usage: z-agent-browser window <operation>

Manage browser windows.

Operations:
  new                  Open new browser window

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser window new
"##,

        // === Frame ===
        "frame" => r##"
z-agent-browser frame - Switch frame context

Usage: z-agent-browser frame <selector|main>

Switch to an iframe or back to the main frame.

Arguments:
  <selector>           CSS selector for iframe
  main                 Switch back to main frame

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser frame "#embed-iframe"
  z-agent-browser frame "iframe[name='content']"
  z-agent-browser frame main
"##,

        // === Dialog ===
        "dialog" => r##"
z-agent-browser dialog - Handle browser dialogs

Usage: z-agent-browser dialog <response> [text]

Respond to browser dialogs (alert, confirm, prompt).

Operations:
  accept [text]        Accept dialog, optionally with prompt text
  dismiss              Dismiss/cancel dialog

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser dialog accept
  z-agent-browser dialog accept "my input"
  z-agent-browser dialog dismiss
"##,

        // === Trace ===
        "trace" => r##"
z-agent-browser trace - Record execution trace

Usage: z-agent-browser trace <operation> [path]

Record a trace for debugging with Playwright Trace Viewer.

Operations:
  start [path]         Start recording trace
  stop [path]          Stop recording and save trace

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser trace start
  z-agent-browser trace start ./my-trace
  z-agent-browser trace stop
  z-agent-browser trace stop ./debug-trace.zip
"##,

        // === Record (video) ===
        "record" => r##"
z-agent-browser record - Record browser session to video

Usage: z-agent-browser record start <path.webm> [url]
       z-agent-browser record stop
       z-agent-browser record restart <path.webm> [url]

Record the browser to a WebM video file using Playwright's native recording.
Creates a fresh browser context but preserves cookies and localStorage.
If no URL is provided, automatically navigates to your current page.

Operations:
  start <path> [url]     Start recording (defaults to current URL if omitted)
  stop                   Stop recording and save video
  restart <path> [url]   Stop current recording (if any) and start a new one

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  # Record from current page (preserves login state)
  z-agent-browser open https://app.example.com/dashboard
  z-agent-browser snapshot -i            # Explore and plan
  z-agent-browser record start ./demo.webm
  z-agent-browser click @e3              # Execute planned actions
  z-agent-browser record stop

  # Or specify a different URL
  z-agent-browser record start ./demo.webm https://example.com

  # Restart recording with a new file (stops previous, starts new)
  z-agent-browser record restart ./take2.webm
"##,

        // === Console/Errors ===
        "console" => r##"
z-agent-browser console - View console logs

Usage: z-agent-browser console [--clear]

View browser console output (log, warn, error, info).

Options:
  --clear              Clear console log buffer

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser console
  z-agent-browser console --clear
"##,
        "errors" => r##"
z-agent-browser errors - View page errors

Usage: z-agent-browser errors [--clear]

View JavaScript errors and uncaught exceptions.

Options:
  --clear              Clear error buffer

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser errors
  z-agent-browser errors --clear
"##,

        // === Highlight ===
        "highlight" => r##"
z-agent-browser highlight - Highlight an element

Usage: z-agent-browser highlight <selector>

Visually highlights an element on the page for debugging.

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser highlight "#target-element"
  z-agent-browser highlight @e5
"##,

        // === State ===
        "state" => r##"
z-agent-browser state - Save/load browser state

Usage: z-agent-browser state <operation> <path>

Save or restore browser state (cookies, localStorage, sessionStorage).

Operations:
  save <path>          Save current state to file
  load <path>          Load state from file

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser state save ./auth-state.json
  z-agent-browser state load ./auth-state.json
"##,

        // === Session ===
        "session" => r##"
z-agent-browser session - Manage sessions

Usage: z-agent-browser session [operation]

Manage isolated browser sessions. Each session has its own browser
instance with separate cookies, storage, and state.

Operations:
  (none)               Show current session name
  list                 List all active sessions

Environment:
  AGENT_BROWSER_SESSION    Default session name

Global Options:
  --json               Output as JSON
  --session <name>     Use specific session

Examples:
  z-agent-browser session
  z-agent-browser session list
  z-agent-browser --session test open example.com
"##,

        // === Install ===
        "install" => r##"
z-agent-browser install - Install browser binaries

Usage: z-agent-browser install [--with-deps]

Downloads and installs browser binaries required for automation.

Options:
  -d, --with-deps      Also install system dependencies (Linux only)

Examples:
  z-agent-browser install
  z-agent-browser install --with-deps
"##,

        _ => return false,
    };
    println!("{}", help.trim());
    true
}

pub fn print_help() {
    println!(
        r#"
z-agent-browser - fast browser automation CLI for AI agents

Usage: z-agent-browser <command> [args] [options]

Core Commands:
  open <url>                 Navigate to URL
  click <sel>                Click element (or @ref)
  dblclick <sel>             Double-click element
  type <sel> <text>          Type into element
  fill <sel> <text>          Clear and fill
  press <key>                Press key (Enter, Tab, Control+a)
  hover <sel>                Hover element
  focus <sel>                Focus element
  check <sel>                Check checkbox
  uncheck <sel>              Uncheck checkbox
  select <sel> <val>         Select dropdown option
  drag <src> <dst>           Drag and drop
  upload <sel> <files...>    Upload files
  scroll <dir> [px]          Scroll (up/down/left/right)
  scrollintoview <sel>       Scroll element into view
  wait <sel|ms>              Wait for element or time
  screenshot [path]          Take screenshot
  pdf <path>                 Save as PDF
  snapshot                   Accessibility tree with refs (for AI)
  eval <js>                  Run JavaScript
  connect <port>             Connect to browser via CDP (e.g., connect 9222)
  close                      Close browser

Navigation:
  back                       Go back
  forward                    Go forward
  reload                     Reload page

Get Info:  z-agent-browser get <what> [selector]
  text, html, value, attr <name>, title, url, count, box

Check State:  z-agent-browser is <what> <selector>
  visible, enabled, checked

Find Elements:  z-agent-browser find <locator> <value> <action> [text]
  role, text, label, placeholder, alt, title, testid, first, last, nth

Mouse:  z-agent-browser mouse <action> [args]
  move <x> <y>, down [btn], up [btn], wheel <dy> [dx]

Browser Settings:  z-agent-browser set <setting> [value]
  viewport <w> <h>, device <name>, geo <lat> <lng>
  offline [on|off], headers <json>, credentials <user> <pass>
  media [dark|light] [reduced-motion]

Network:  z-agent-browser network <action>
  route <url> [--abort|--body <json>]
  unroute [url]
  requests [--clear] [--filter <pattern>]

Storage:
  cookies [get|set|clear]    Manage cookies
  storage <local|session>    Manage web storage

Tabs:
  tab [new|list|close|<n>]   Manage tabs

Debug:
  trace start|stop [path]    Record trace
  record start <path> [url]  Start video recording (WebM)
  record stop                Stop and save video
  console [--clear]          View console logs
  errors [--clear]           View page errors
  highlight <sel>            Highlight element

Sessions:
  session                    Show current session name
  session list               List active sessions

Setup:
  install                    Install browser binaries
  install --with-deps        Also install system dependencies (Linux)

Snapshot Options:
  -i, --interactive          Only interactive elements
  -c, --compact              Remove empty structural elements
  -d, --depth <n>            Limit tree depth
  -s, --selector <sel>       Scope to CSS selector

Options:
  --session <name>           Isolated session (or AGENT_BROWSER_SESSION env)
  --headers <json>           HTTP headers scoped to URL's origin (for auth)
  --executable-path <path>   Custom browser executable (or AGENT_BROWSER_EXECUTABLE_PATH)
  --extension <path>         Load browser extensions (repeatable).
  --proxy <url>              Proxy server (http://[user:pass@]host:port)
  --json                     JSON output
  --full, -f                 Full page screenshot
  --headed                   Show browser window (not headless)
  --cdp <port>               Connect via CDP (Chrome DevTools Protocol)
  --debug                    Debug output
  --version, -V              Show version

Environment:
  AGENT_BROWSER_SESSION          Session name (default: "default")
  AGENT_BROWSER_EXECUTABLE_PATH  Custom browser executable path
  AGENT_BROWSER_STREAM_PORT      Enable WebSocket streaming on port (e.g., 9223)

Examples:
  z-agent-browser open example.com
  z-agent-browser snapshot -i              # Interactive elements only
  z-agent-browser click @e2                # Click by ref from snapshot
  z-agent-browser fill @e3 "test@example.com"
  z-agent-browser find role button click --name Submit
  z-agent-browser get text @e1
  z-agent-browser screenshot --full
  z-agent-browser --cdp 9222 snapshot      # Connect via CDP port
"#
    );
}

pub fn print_version() {
    println!("z-agent-browser {}", env!("CARGO_PKG_VERSION"));
}
