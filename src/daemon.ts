import * as net from 'net';
import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';
import { BrowserManager } from './browser.js';
import { parseCommand, serializeResponse, errorResponse, successResponse } from './protocol.js';
import { executeCommand } from './actions.js';
import { StreamServer } from './stream-server.js';
import { PlaywrightMCPBackend } from './playwright-mcp.js';

type BackendType = 'native' | 'playwright-mcp';
const backendType: BackendType =
  process.env.AGENT_BROWSER_BACKEND === 'playwright-mcp' ? 'playwright-mcp' : 'native';

// Platform detection
const isWindows = process.platform === 'win32';

// Session support - each session gets its own socket/pid
let currentSession = process.env.AGENT_BROWSER_SESSION || 'default';

// Persist support - auto-save/load state
const persistEnabled =
  process.env.AGENT_BROWSER_PERSIST === '1' || process.env.AGENT_BROWSER_PERSIST === 'true';

function getPersistPath(): string {
  if (process.env.AGENT_BROWSER_STATE) {
    return process.env.AGENT_BROWSER_STATE.replace(/^~/, os.homedir());
  }
  const dir = path.join(os.homedir(), '.z-agent-browser', 'sessions');
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
  return path.join(dir, `${currentSession}.json`);
}

function loadPersistState(): string | undefined {
  if (!persistEnabled) return undefined;
  const statePath = getPersistPath();
  if (fs.existsSync(statePath)) {
    return statePath;
  }
  return undefined;
}

async function savePersistState(browser: BrowserManager): Promise<void> {
  if (!persistEnabled) return;
  const statePath = getPersistPath();
  try {
    const state = await browser.saveState();
    fs.writeFileSync(statePath, JSON.stringify(state, null, 2));
  } catch {
    // Ignore save errors (browser may already be closed)
  }
}

// Stream server for browser preview
let streamServer: StreamServer | null = null;

// Default stream port (can be overridden with AGENT_BROWSER_STREAM_PORT)
const DEFAULT_STREAM_PORT = 9223;

/**
 * Set the current session
 */
export function setSession(session: string): void {
  currentSession = session;
}

/**
 * Get the current session
 */
export function getSession(): string {
  return currentSession;
}

/**
 * Get port number for TCP mode (Windows)
 * Uses a hash of the session name to get a consistent port
 */
function getPortForSession(session: string): number {
  let hash = 0;
  for (let i = 0; i < session.length; i++) {
    hash = (hash << 5) - hash + session.charCodeAt(i);
    hash |= 0;
  }
  // Port range 49152-65535 (dynamic/private ports)
  return 49152 + (Math.abs(hash) % 16383);
}

/**
 * Get the runtime directory for socket/pid files
 * Uses ~/.z-agent-browser/run/ for cross-platform consistency
 */
function getRuntimeDir(): string {
  const dir = path.join(os.homedir(), '.z-agent-browser', 'run');
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
  return dir;
}

/**
 * Get the socket path for the current session (Unix) or port (Windows)
 */
export function getSocketPath(session?: string): string {
  const sess = session ?? currentSession;
  if (isWindows) {
    return String(getPortForSession(sess));
  }
  return path.join(getRuntimeDir(), `${sess}.sock`);
}

/**
 * Get the port file path for Windows (stores the port number)
 */
export function getPortFile(session?: string): string {
  const sess = session ?? currentSession;
  return path.join(getRuntimeDir(), `${sess}.port`);
}

/**
 * Get the PID file path for the current session
 */
export function getPidFile(session?: string): string {
  const sess = session ?? currentSession;
  return path.join(getRuntimeDir(), `${sess}.pid`);
}

/**
 * Check if daemon is running for the current session
 */
export function isDaemonRunning(session?: string): boolean {
  const pidFile = getPidFile(session);
  if (!fs.existsSync(pidFile)) return false;

  try {
    const pid = parseInt(fs.readFileSync(pidFile, 'utf8').trim(), 10);
    // Check if process exists (works on both Unix and Windows)
    process.kill(pid, 0);
    return true;
  } catch {
    // Process doesn't exist, clean up stale files
    cleanupSocket(session);
    return false;
  }
}

/**
 * Get connection info for the current session
 * Returns { type: 'unix', path: string } or { type: 'tcp', port: number }
 */
export function getConnectionInfo(
  session?: string
): { type: 'unix'; path: string } | { type: 'tcp'; port: number } {
  const sess = session ?? currentSession;
  if (isWindows) {
    return { type: 'tcp', port: getPortForSession(sess) };
  }
  return { type: 'unix', path: path.join(getRuntimeDir(), `${sess}.sock`) };
}

/**
 * Clean up socket and PID file for the current session
 */
export function cleanupSocket(session?: string): void {
  const pidFile = getPidFile(session);
  const streamPortFile = getStreamPortFile(session);
  try {
    if (fs.existsSync(pidFile)) fs.unlinkSync(pidFile);
    if (fs.existsSync(streamPortFile)) fs.unlinkSync(streamPortFile);
    if (isWindows) {
      const portFile = getPortFile(session);
      if (fs.existsSync(portFile)) fs.unlinkSync(portFile);
    } else {
      const socketPath = getSocketPath(session);
      if (fs.existsSync(socketPath)) fs.unlinkSync(socketPath);
    }
  } catch {
    // Ignore cleanup errors
  }
}

/**
 * Get the stream port file path
 */
export function getStreamPortFile(session?: string): string {
  const sess = session ?? currentSession;
  return path.join(getRuntimeDir(), `${sess}.stream`);
}

/**
 * Start the daemon server
 * @param options.streamPort Port for WebSocket stream server (0 to disable)
 */
export async function startDaemon(options?: { streamPort?: number }): Promise<void> {
  cleanupSocket();

  const browser = backendType === 'playwright-mcp' ? null : new BrowserManager();
  const mcpBackend = backendType === 'playwright-mcp' ? new PlaywrightMCPBackend() : null;
  let shuttingDown = false;
  let mcpInitialized = false;

  // Start stream server if port is specified (native backend only)
  const streamPort =
    options?.streamPort ??
    (process.env.AGENT_BROWSER_STREAM_PORT
      ? parseInt(process.env.AGENT_BROWSER_STREAM_PORT, 10)
      : 0);

  if (streamPort > 0 && browser) {
    streamServer = new StreamServer(browser, streamPort);
    await streamServer.start();
    const streamPortFile = getStreamPortFile();
    fs.writeFileSync(streamPortFile, streamPort.toString());
  }

  const server = net.createServer((socket) => {
    let buffer = '';

    socket.on('data', async (data) => {
      buffer += data.toString();

      // Process complete lines
      while (buffer.includes('\n')) {
        const newlineIdx = buffer.indexOf('\n');
        const line = buffer.substring(0, newlineIdx);
        buffer = buffer.substring(newlineIdx + 1);

        if (!line.trim()) continue;

        try {
          const parseResult = parseCommand(line);

          if (!parseResult.success) {
            const resp = errorResponse(parseResult.id ?? 'unknown', parseResult.error);
            socket.write(serializeResponse(resp) + '\n');
            continue;
          }

          if (mcpBackend) {
            // playwright-mcp backend
            if (!mcpInitialized && parseResult.command.action !== 'close') {
              await mcpBackend.launch();
              mcpInitialized = true;
            }

            if (parseResult.command.action === 'close') {
              await mcpBackend.close();
              socket.write(serializeResponse(successResponse(parseResult.command.id, {})) + '\n');
              if (!shuttingDown) {
                shuttingDown = true;
                setTimeout(() => {
                  server.close();
                  cleanupSocket();
                  process.exit(0);
                }, 100);
              }
              return;
            }

            const response = await executeMCPCommand(parseResult.command, mcpBackend);
            socket.write(serializeResponse(response) + '\n');
          } else if (browser) {
            // Native Playwright backend
            if (
              !browser.isLaunched() &&
              parseResult.command.action !== 'launch' &&
              parseResult.command.action !== 'close'
            ) {
              const extensions = process.env.AGENT_BROWSER_EXTENSIONS
                ? process.env.AGENT_BROWSER_EXTENSIONS.split(',')
                    .map((p) => p.trim())
                    .filter(Boolean)
                : undefined;
              const persistState = loadPersistState();
              const headedEnv =
                process.env.AGENT_BROWSER_HEADED === '1' ||
                process.env.AGENT_BROWSER_HEADED === 'true';
              const ignoreHttpsErrors =
                process.env.AGENT_BROWSER_IGNORE_HTTPS_ERRORS === '1' ||
                process.env.AGENT_BROWSER_IGNORE_HTTPS_ERRORS === 'true';
              const userAgent = process.env.AGENT_BROWSER_USER_AGENT;
              const argsEnv = process.env.AGENT_BROWSER_ARGS
                ? process.env.AGENT_BROWSER_ARGS.split(',')
                    .map((a) => a.trim())
                    .filter(Boolean)
                : undefined;
              await browser.launch({
                id: 'auto',
                action: 'launch',
                headless: !headedEnv,
                executablePath: process.env.AGENT_BROWSER_EXECUTABLE_PATH,
                extensions: extensions,
                storageState: persistState,
                profile: process.env.AGENT_BROWSER_PROFILE,
                ignoreHTTPSErrors: ignoreHttpsErrors,
                userAgent: userAgent,
                args: argsEnv,
              });
            }

            if (parseResult.command.action === 'close') {
              await savePersistState(browser);
              const response = await executeCommand(parseResult.command, browser);
              socket.write(serializeResponse(response) + '\n');
              if (!shuttingDown) {
                shuttingDown = true;
                setTimeout(() => {
                  server.close();
                  cleanupSocket();
                  process.exit(0);
                }, 100);
              }
              return;
            }

            const response = await executeCommand(parseResult.command, browser);
            socket.write(serializeResponse(response) + '\n');
          }
        } catch (err) {
          const message = err instanceof Error ? err.message : String(err);
          socket.write(serializeResponse(errorResponse('error', message)) + '\n');
        }
      }
    });

    socket.on('error', () => {
      // Client disconnected, ignore
    });
  });

  const pidFile = getPidFile();

  // Write PID file before listening
  fs.writeFileSync(pidFile, process.pid.toString());

  if (isWindows) {
    // Windows: use TCP socket on localhost
    const port = getPortForSession(currentSession);
    const portFile = getPortFile();
    fs.writeFileSync(portFile, port.toString());
    server.listen(port, '127.0.0.1', () => {
      // Daemon is ready on TCP port
    });
  } else {
    // Unix: use Unix domain socket
    const socketPath = getSocketPath();
    server.listen(socketPath, () => {
      // Daemon is ready
    });
  }

  server.on('error', (err) => {
    console.error('Server error:', err);
    cleanupSocket();
    process.exit(1);
  });

  const shutdown = async () => {
    if (shuttingDown) return;
    shuttingDown = true;

    if (streamServer) {
      await streamServer.stop();
      streamServer = null;
      const streamPortFile = getStreamPortFile();
      try {
        if (fs.existsSync(streamPortFile)) fs.unlinkSync(streamPortFile);
      } catch {
        // Ignore
      }
    }

    if (browser) await browser.close();
    if (mcpBackend) await mcpBackend.close();
    server.close();
    cleanupSocket();
    process.exit(0);
  };

  process.on('SIGINT', shutdown);
  process.on('SIGTERM', shutdown);
  process.on('SIGHUP', shutdown);

  // Handle unexpected errors - always cleanup
  process.on('uncaughtException', (err) => {
    console.error('Uncaught exception:', err);
    cleanupSocket();
    process.exit(1);
  });

  process.on('unhandledRejection', (reason) => {
    console.error('Unhandled rejection:', reason);
    cleanupSocket();
    process.exit(1);
  });

  // Cleanup on normal exit
  process.on('exit', () => {
    cleanupSocket();
  });

  // Keep process alive
  process.stdin.resume();
}

import type { Command, Response } from './types.js';

async function executeMCPCommand(
  command: Command,
  backend: PlaywrightMCPBackend
): Promise<Response> {
  const { id, action } = command;

  try {
    switch (action) {
      case 'launch':
        await backend.launch();
        return successResponse(id, { launched: true });

      case 'navigate': {
        const cmd = command as { url: string };
        const result = await backend.navigate(cmd.url);
        return successResponse(id, result);
      }

      case 'snapshot': {
        const result = await backend.snapshot();
        return successResponse(id, result);
      }

      case 'click': {
        const cmd = command as {
          selector: string;
          button?: 'left' | 'right' | 'middle';
          clickCount?: number;
        };
        await backend.click(cmd.selector, { button: cmd.button, clickCount: cmd.clickCount });
        return successResponse(id, { clicked: cmd.selector });
      }

      case 'type': {
        const cmd = command as { selector: string; text: string; delay?: number };
        await backend.type(cmd.selector, cmd.text, { delay: cmd.delay });
        return successResponse(id, { typed: cmd.text });
      }

      case 'fill': {
        const cmd = command as { selector: string; value: string };
        await backend.fill(cmd.selector, cmd.value);
        return successResponse(id, { filled: cmd.value });
      }

      case 'hover': {
        const cmd = command as { selector: string };
        await backend.hover(cmd.selector);
        return successResponse(id, { hovered: cmd.selector });
      }

      case 'press': {
        const cmd = command as { key: string };
        await backend.press(cmd.key);
        return successResponse(id, { pressed: cmd.key });
      }

      case 'screenshot': {
        const cmd = command as { path?: string; fullPage?: boolean; format?: 'png' | 'jpeg' };
        const result = await backend.screenshot({
          path: cmd.path,
          fullPage: cmd.fullPage,
          format: cmd.format,
        });
        return successResponse(id, result);
      }

      case 'evaluate': {
        const cmd = command as { script: string };
        const result = await backend.evaluate(cmd.script);
        return successResponse(id, { result });
      }

      case 'wait': {
        const cmd = command as { timeout?: number; selector?: string };
        if (cmd.timeout) {
          await backend.wait({ time: cmd.timeout / 1000 });
        }
        return successResponse(id, { waited: true });
      }

      case 'back':
        await backend.back();
        return successResponse(id, { navigated: 'back' });

      case 'tab_list': {
        const result = await backend.listTabs();
        return successResponse(id, result);
      }

      case 'tab_new': {
        const cmd = command as { url?: string };
        const result = await backend.newTab(cmd.url);
        return successResponse(id, result);
      }

      case 'tab_switch': {
        const cmd = command as { index: number };
        await backend.switchTab(cmd.index);
        return successResponse(id, { switched: cmd.index });
      }

      case 'tab_close': {
        const cmd = command as { index?: number };
        await backend.closeTab(cmd.index);
        return successResponse(id, { closed: true });
      }

      case 'select': {
        const cmd = command as { selector: string; values: string | string[] };
        const values = Array.isArray(cmd.values) ? cmd.values : [cmd.values];
        await backend.select(cmd.selector, values);
        return successResponse(id, { selected: values });
      }

      case 'drag': {
        const cmd = command as { source: string; target: string };
        await backend.drag(cmd.source, cmd.target);
        return successResponse(id, { dragged: true });
      }

      case 'upload': {
        const cmd = command as { files: string | string[] };
        const files = Array.isArray(cmd.files) ? cmd.files : [cmd.files];
        await backend.upload(files);
        return successResponse(id, { uploaded: files });
      }

      case 'dialog': {
        const cmd = command as { response: 'accept' | 'dismiss'; promptText?: string };
        await backend.handleDialog(cmd.response === 'accept', cmd.promptText);
        return successResponse(id, { handled: cmd.response });
      }

      case 'viewport': {
        const cmd = command as { width: number; height: number };
        await backend.resize(cmd.width, cmd.height);
        return successResponse(id, { resized: true });
      }

      case 'console': {
        const messages = await backend.getConsoleMessages();
        return successResponse(id, { messages });
      }

      default:
        return errorResponse(id, `Action "${action}" not supported in playwright-mcp mode`);
    }
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    return errorResponse(id, message);
  }
}

if (process.argv[1]?.endsWith('daemon.js') || process.env.AGENT_BROWSER_DAEMON === '1') {
  startDaemon().catch((err) => {
    console.error('Daemon error:', err);
    cleanupSocket();
    process.exit(1);
  });
}
