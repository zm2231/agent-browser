/**
 * Playwright MCP Backend - Control browser via Microsoft's Playwright MCP server
 * @see https://github.com/microsoft/playwright-mcp
 */
import { spawn, type ChildProcess } from 'child_process';
import { createInterface, type Interface } from 'readline';
import type { RefMap } from './snapshot.js';

interface MCPResponse {
  result?: {
    content?: Array<{ type: string; text: string }>;
    [key: string]: unknown;
  };
  error?: { code: number; message: string };
}

/**
 * Stdio-based MCP client that spawns playwright-mcp as a subprocess
 * and communicates via stdin/stdout with JSON-RPC messages.
 */
class MCPStdioClient {
  private process: ChildProcess | null = null;
  private readline: Interface | null = null;
  private requestId = 0;
  private initialized = false;
  private pendingRequests = new Map<
    number,
    {
      resolve: (value: MCPResponse) => void;
      reject: (error: Error) => void;
    }
  >();

  constructor(
    private command: string = 'npx',
    private args: string[] = ['@playwright/mcp@latest']
  ) {}

  async connect(): Promise<void> {
    if (this.initialized && this.process && !this.process.killed) return;

    this.process = spawn(this.command, this.args, {
      stdio: ['pipe', 'pipe', 'pipe'],
      env: { ...process.env },
    });

    if (!this.process.stdout || !this.process.stdin) {
      throw new Error('Failed to spawn playwright-mcp process');
    }

    this.readline = createInterface({
      input: this.process.stdout,
      crlfDelay: Infinity,
    });

    this.readline.on('line', (line) => {
      if (!line.trim()) return;
      try {
        const msg = JSON.parse(line);
        if (msg.id !== undefined && this.pendingRequests.has(msg.id)) {
          const pending = this.pendingRequests.get(msg.id)!;
          this.pendingRequests.delete(msg.id);
          pending.resolve(msg);
        }
      } catch {
        // intentionally ignored - MCP servers emit non-JSON startup messages
      }
    });

    this.process.stderr?.on('data', (data) => {
      const msg = data.toString().trim();
      if (msg && !msg.includes('Debugger') && !msg.includes('inspector')) {
        console.error('[playwright-mcp]', msg);
      }
    });

    this.process.on('exit', (code) => {
      this.initialized = false;
      for (const [id, pending] of this.pendingRequests) {
        pending.reject(new Error(`MCP process exited with code ${code}`));
        this.pendingRequests.delete(id);
      }
    });

    const initResponse = await this.rawRequest('initialize', {
      protocolVersion: '2024-11-05',
      capabilities: {},
      clientInfo: { name: 'z-agent-browser', version: '1.0' },
    });

    if (initResponse.error) {
      throw new Error(`MCP init failed: ${initResponse.error.message}`);
    }

    this.sendNotification('notifications/initialized', {});
    this.initialized = true;
  }

  private sendNotification(method: string, params?: Record<string, unknown>): void {
    if (!this.process?.stdin) return;
    const msg = JSON.stringify({
      jsonrpc: '2.0',
      method,
      ...(params && { params }),
    });
    this.process.stdin.write(msg + '\n');
  }

  private async rawRequest(method: string, params?: Record<string, unknown>): Promise<MCPResponse> {
    if (!this.process?.stdin) {
      throw new Error('MCP process not running');
    }

    const id = ++this.requestId;
    const msg = JSON.stringify({
      jsonrpc: '2.0',
      id,
      method,
      ...(params && { params }),
    });

    return new Promise((resolve, reject) => {
      const timeoutMs = 60000;
      const timeout = setTimeout(() => {
        this.pendingRequests.delete(id);
        reject(new Error(`MCP request timed out: ${method}`));
      }, timeoutMs);

      this.pendingRequests.set(id, {
        resolve: (value) => {
          clearTimeout(timeout);
          resolve(value);
        },
        reject: (error) => {
          clearTimeout(timeout);
          reject(error);
        },
      });

      this.process!.stdin!.write(msg + '\n');
    });
  }

  async callTool(name: string, args: Record<string, unknown> = {}): Promise<MCPResponse> {
    if (!this.initialized) {
      await this.connect();
    }

    return this.rawRequest('tools/call', { name, arguments: args });
  }

  async close(): Promise<void> {
    if (this.readline) {
      this.readline.close();
      this.readline = null;
    }
    if (this.process) {
      this.process.kill('SIGTERM');
      this.process = null;
    }
    this.initialized = false;
  }

  isConnected(): boolean {
    return this.initialized && this.process !== null && !this.process.killed;
  }
}

export class PlaywrightMCPBackend {
  private client: MCPStdioClient;
  private refMap: RefMap = {};
  private lastSnapshot: string = '';

  constructor() {
    const command = process.env.PLAYWRIGHT_MCP_COMMAND || 'npx';
    const defaultArgs = ['@playwright/mcp@latest', '--extension'];
    const args = process.env.PLAYWRIGHT_MCP_ARGS?.split(' ') || defaultArgs;
    this.client = new MCPStdioClient(command, args);
  }

  async launch(): Promise<void> {
    await this.client.connect();
    await this.snapshot();
  }

  private extractText(response: MCPResponse): string {
    if (response.error) {
      throw new Error(`MCP error: ${response.error.message}`);
    }
    return response.result?.content?.[0]?.text || '';
  }

  async navigate(url: string): Promise<{ url: string; title: string }> {
    const result = await this.client.callTool('browser_navigate', { url });
    const text = this.extractText(result);
    const titleMatch = text.match(/Page Title: (.+)/);
    const urlMatch = text.match(/Page URL: (.+)/);
    return {
      url: urlMatch?.[1] || url,
      title: titleMatch?.[1] || '',
    };
  }

  async back(): Promise<void> {
    // Workaround: browser_navigate_back causes "Target closed" error in extension mode
    // Using history.back() via evaluate works reliably
    await this.client.callTool('browser_evaluate', { function: 'history.back()' });
  }

  async snapshot(): Promise<{ snapshot: string; refs?: RefMap }> {
    const result = await this.client.callTool('browser_snapshot', {});
    const text = this.extractText(result);
    this.lastSnapshot = text;

    const refs: RefMap = {};
    const refRegex = /\[ref=([^\]]+)\]/g;
    let match;
    while ((match = refRegex.exec(text)) !== null) {
      const ref = match[1];
      refs[ref] = { selector: ref, role: 'element' };
    }
    this.refMap = refs;

    return { snapshot: text, refs };
  }

  async click(
    selector: string,
    options?: { button?: 'left' | 'right' | 'middle'; clickCount?: number }
  ): Promise<void> {
    const ref = this.extractRef(selector);
    await this.client.callTool('browser_click', {
      element: selector,
      ref: ref || selector,
      button: options?.button,
      doubleClick: options?.clickCount === 2,
    });
  }

  async type(selector: string, text: string, options?: { delay?: number }): Promise<void> {
    const ref = this.extractRef(selector);
    await this.client.callTool('browser_type', {
      element: selector,
      ref: ref || selector,
      text,
      slowly: options?.delay ? true : false,
    });
  }

  async fill(selector: string, value: string): Promise<void> {
    const ref = this.extractRef(selector);
    await this.client.callTool('browser_type', {
      element: selector,
      ref: ref || selector,
      text: value,
    });
  }

  async hover(selector: string): Promise<void> {
    const ref = this.extractRef(selector);
    await this.client.callTool('browser_hover', {
      element: selector,
      ref: ref || selector,
    });
  }

  async press(key: string): Promise<void> {
    await this.client.callTool('browser_press_key', { key });
  }

  async screenshot(options?: {
    path?: string;
    fullPage?: boolean;
    format?: 'png' | 'jpeg';
  }): Promise<{ path?: string; base64?: string }> {
    await this.client.callTool('browser_take_screenshot', {
      filename: options?.path,
      fullPage: options?.fullPage,
      type: options?.format,
    });
    return { path: options?.path };
  }

  async evaluate(script: string): Promise<unknown> {
    const result = await this.client.callTool('browser_evaluate', { function: script });
    const text = this.extractText(result);
    try {
      return JSON.parse(text);
    } catch {
      return text;
    }
  }

  async runCode(code: string): Promise<unknown> {
    const result = await this.client.callTool('browser_run_code', { code });
    const text = this.extractText(result);
    try {
      return JSON.parse(text);
    } catch {
      return text;
    }
  }

  async wait(options: { time?: number; text?: string; textGone?: string }): Promise<void> {
    await this.client.callTool('browser_wait_for', options);
  }

  async getConsoleMessages(level?: 'error' | 'warning' | 'info' | 'debug'): Promise<string[]> {
    const result = await this.client.callTool('browser_console_messages', { level });
    const text = this.extractText(result);
    return text.split('\n').filter((line: string) => line.trim());
  }

  async getNetworkRequests(includeStatic?: boolean): Promise<string[]> {
    const result = await this.client.callTool('browser_network_requests', { includeStatic });
    const text = this.extractText(result);
    return text.split('\n').filter((line: string) => line.trim());
  }

  async listTabs(): Promise<{
    tabs: Array<{ index: number; url: string; title: string }>;
    active: number;
  }> {
    const result = await this.client.callTool('browser_tabs', { action: 'list' });
    const text = this.extractText(result);
    const tabs: Array<{ index: number; url: string; title: string; active: boolean }> = [];
    let activeIndex = 0;

    const lines = text.split('\n');
    for (const line of lines) {
      const match = line.match(/\[(\d+)\]\s+(.+)/);
      if (match) {
        const isActive = line.includes('*') || line.includes('(active)');
        const index = parseInt(match[1], 10);
        if (isActive) activeIndex = index;
        tabs.push({ index, url: match[2], title: '', active: isActive });
      }
    }

    return { tabs, active: activeIndex };
  }

  async newTab(url?: string): Promise<{ index: number }> {
    await this.client.callTool('browser_tabs', { action: 'new' });
    if (url) {
      await this.navigate(url);
    }
    const { tabs } = await this.listTabs();
    return { index: tabs.length - 1 };
  }

  async switchTab(index: number): Promise<void> {
    await this.client.callTool('browser_tabs', { action: 'select', index });
  }

  async closeTab(index?: number): Promise<void> {
    await this.client.callTool('browser_tabs', { action: 'close', index });
  }

  async handleDialog(accept: boolean, promptText?: string): Promise<void> {
    await this.client.callTool('browser_handle_dialog', { accept, promptText });
  }

  async close(): Promise<void> {
    try {
      await this.client.callTool('browser_close', {});
    } finally {
      await this.client.close();
    }
  }

  async resize(width: number, height: number): Promise<void> {
    await this.client.callTool('browser_resize', { width, height });
  }

  async select(selector: string, values: string[]): Promise<void> {
    const ref = this.extractRef(selector);
    await this.client.callTool('browser_select_option', {
      element: selector,
      ref: ref || selector,
      values,
    });
  }

  async drag(sourceSelector: string, targetSelector: string): Promise<void> {
    const sourceRef = this.extractRef(sourceSelector);
    const targetRef = this.extractRef(targetSelector);
    await this.client.callTool('browser_drag', {
      startElement: sourceSelector,
      startRef: sourceRef || sourceSelector,
      endElement: targetSelector,
      endRef: targetRef || targetSelector,
    });
  }

  async upload(paths: string[]): Promise<void> {
    await this.client.callTool('browser_file_upload', { paths });
  }

  async fillForm(
    fields: Array<{
      name: string;
      type: 'textbox' | 'checkbox' | 'radio' | 'combobox' | 'slider';
      ref: string;
      value: string;
    }>
  ): Promise<void> {
    await this.client.callTool('browser_fill_form', { fields });
  }

  private extractRef(selector: string): string | null {
    if (/^e\d+$/.test(selector)) {
      return selector;
    }
    const match = selector.match(/\[ref=([^\]]+)\]/);
    return match?.[1] || null;
  }

  async isConnected(): Promise<boolean> {
    try {
      await this.snapshot();
      return true;
    } catch {
      return false;
    }
  }

  async saveState(): Promise<unknown> {
    return {};
  }

  async loadState(): Promise<void> {}

  getActivePage(): null {
    return null;
  }

  getRefMap(): RefMap {
    return this.refMap;
  }

  getLastSnapshot(): string {
    return this.lastSnapshot;
  }

  async startScreencast(): Promise<void> {
    throw new Error('Screencast not supported in playwright-mcp mode');
  }

  async stopScreencast(): Promise<void> {}

  setFrameCallback(): void {}
}
