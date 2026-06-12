"use strict";
var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  // If the importer is in node compatibility mode or this is not an ESM
  // file that has been converted to a CommonJS file using a Babel-
  // compatible transform (i.e. "__esModule" has not been set), then set
  // "default" to the CommonJS "module.exports" for node compatibility.
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// src/extension.ts
var extension_exports = {};
__export(extension_exports, {
  activate: () => activate,
  deactivate: () => deactivate
});
module.exports = __toCommonJS(extension_exports);

// src/controller.ts
var vscode3 = __toESM(require("vscode"));

// src/status.ts
var vscode = __toESM(require("vscode"));
var Statusbar = class {
  item;
  constructor() {
    this.item = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    this.item.command = "bandhu.open";
    this.setidle();
    this.item.show();
  }
  setbusy() {
    this.item.text = env("BANDHU_STATUS_BUSY_TEXT", "$(loading~spin) Bandhu");
    this.item.tooltip = env("BANDHU_STATUS_BUSY_TOOLTIP", "Working");
  }
  setidle() {
    this.item.text = env("BANDHU_STATUS_TEXT", "$(check) Bandhu");
    this.item.tooltip = env("BANDHU_STATUS_TOOLTIP", "Ready");
  }
  seterror() {
    this.item.text = env("BANDHU_STATUS_ERROR_TEXT", "$(error) Bandhu");
    this.item.tooltip = env("BANDHU_STATUS_ERROR_TOOLTIP", "Error");
  }
  dispose() {
    this.item.dispose();
  }
};
function env(name, fallback) {
  return process.env[name] || fallback;
}

// src/chatui.ts
var vscode2 = __toESM(require("vscode"));
var ChatPanel = class {
  constructor(placeholder = "Ask Bandhu...") {
    this.placeholder = placeholder;
  }
  placeholder;
  panel;
  _onDidReceiveMessage = new vscode2.EventEmitter();
  onDidReceiveMessage = this._onDidReceiveMessage.event;
  create(column = vscode2.ViewColumn.One) {
    if (this.panel) {
      this.panel.reveal(column);
      return;
    }
    this.panel = vscode2.window.createWebviewPanel(
      "bandhuChat",
      "Bandhu Chat",
      column,
      { enableScripts: true }
    );
    this.panel.webview.html = this.getHtml();
    this.panel.webview.onDidReceiveMessage((msg) => {
      this._onDidReceiveMessage.fire(msg);
    });
    this.panel.onDidDispose(() => {
      this.panel = void 0;
    });
  }
  focus() {
    this.create();
    if (!this.panel) {
      return;
    }
    this.panel.webview.postMessage({ type: "focus" });
  }
  append(msg) {
    if (!this.panel) {
      return;
    }
    this.panel.webview.postMessage({ type: "message", data: msg });
  }
  clear() {
    if (!this.panel) {
      return;
    }
    this.panel.webview.html = this.getHtml();
  }
  dispose() {
    this._onDidReceiveMessage.dispose();
    if (this.panel) {
      this.panel.dispose();
    }
  }
  escapeHtml(text) {
    return text.replace(/[&<>"']/g, (m) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[m] || m);
  }
  getHtml() {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Bandhu Chat</title>
    <style>
        body { font-family: var(--vscode-font-family); padding: 10px; color: var(--vscode-foreground); background: var(--vscode-editor-background); margin: 0; }
        #messages { min-height: 200px; max-height: 600px; overflow-y: auto; padding: 8px; }
        .msg { padding: 8px 12px; margin: 4px 0; border-radius: 4px; word-wrap: break-word; }
        .response { background: var(--vscode-editor-inactiveSelectionBackground); }
        .tool_result { background: var(--vscode-textBlockQuote-background); }
        .tool_error { background: var(--vscode-inputValidation-errorBackground); color: var(--vscode-errorForeground); }
        .tool_approval { background: var(--vscode-inputValidation-infoBackground); border: 1px solid var(--vscode-inputValidation-infoBorder); padding: 12px; }
        .error { background: var(--vscode-inputValidation-errorBackground); color: var(--vscode-errorForeground); }
        .complete { background: var(--vscode-editor-snippetFinalTabstop-foreground); padding: 4px; }
        .approval-buttons { margin-top: 8px; }
        .approval-buttons button { margin-right: 8px; padding: 4px 12px; cursor: pointer; }
        .input-box { margin-top: 12px; padding: 8px; display: flex; gap: 8px; }
        .input-box input { flex: 1; padding: 6px; font-family: var(--vscode-font-family); font-size: 13px; }
        .input-box button { padding: 6px 12px; cursor: pointer; }
        .path-display { font-family: var(--vscode-editor-font-family); font-size: 12px; color: var(--vscode-textLink-foreground); margin: 4px 0; }
    </style>
</head>
<body>
    <div id="messages"></div>
    <div class="input-box">
        <input id="input" type="text" placeholder="${this.escapeHtml(this.placeholder)}" autocomplete="off" />
        <button id="send">Send</button>
    </div>
    <script>
        const vscode = acquireVsCodeApi();
        const messages = document.getElementById('messages');
        const input = document.getElementById('input');
        const sendBtn = document.getElementById('send');

        function addMessage(type, content) {
            const div = document.createElement('div');
            div.className = 'msg ' + type;
            div.textContent = content;
            messages.appendChild(div);
            messages.scrollTop = messages.scrollHeight;
        }

        function addApproval(id, tool, inputVal, diffVal) {
            const div = document.createElement('div');
            div.className = 'msg tool_approval';
            
            const pathDisplay = document.createElement('div');
            pathDisplay.className = 'path-display';
            pathDisplay.textContent = tool + ': ' + (typeof inputVal === 'object' && inputVal !== null ? inputVal.path || inputVal.command : '');
            div.appendChild(pathDisplay);
            
            if (tool === 'writefile' && diffVal) {
                const diffPre = document.createElement('pre');
                diffPre.style.fontSize = '12px';
                diffPre.style.whiteSpace = 'pre-wrap';
                diffPre.style.margin = '4px 0';
                diffPre.style.backgroundColor = 'var(--vscode-editor-lineHighlightBackground)';
                diffPre.style.padding = '8px';
                diffPre.style.borderRadius = '4px';
                diffPre.textContent = diffVal;
                div.appendChild(diffPre);
            } else {
                const contentPre = document.createElement('pre');
                contentPre.style.fontSize = '12px';
                contentPre.style.whiteSpace = 'pre-wrap';
                contentPre.style.margin = '4px 0';
                contentPre.textContent = JSON.stringify(inputVal, null, 2);
                div.appendChild(contentPre);
            }
            
            const buttonsDiv = document.createElement('div');
            buttonsDiv.className = 'approval-buttons';
            
            const approveBtn = document.createElement('button');
            approveBtn.textContent = 'Approve';
            approveBtn.onclick = () => vscode.postMessage({ type: 'approve', id: id });
            buttonsDiv.appendChild(approveBtn);
            
            const rejectBtn = document.createElement('button');
            rejectBtn.textContent = 'Reject';
            rejectBtn.onclick = () => vscode.postMessage({ type: 'reject', id: id });
            buttonsDiv.appendChild(rejectBtn);
            
            div.appendChild(buttonsDiv);
            messages.appendChild(div);
            messages.scrollTop = messages.scrollHeight;
        }

        window.addEventListener('message', event => {
            const msg = event.data;
            if (msg.type === 'message') {
                const data = msg.data;
                if (data.type === 'tool_approval') {
                    addApproval(data.id, data.tool, data.input, data.diff);
                } else if (data.type === 'response' || data.type === 'tool_result' || data.type === 'tool_error' || data.type === 'error') {
                    addMessage(data.type, data.content || data.error || '');
                }
            } else if (msg.type === 'focus') {
                input.focus();
            }
        });

        function sendMessage() {
            const text = input.value.trim();
            if (!text) return;
            vscode.postMessage({ type: 'send', text: text });
            input.value = '';
        }

        sendBtn.onclick = sendMessage;
        input.onkeydown = e => { if (e.key === 'Enter') sendMessage(); };
    </script>
</body>
</html>`;
  }
};

// src/api.ts
var chatms = intenv("BANDHU_CHAT_TIMEOUT_MS", 12e4);
var chatretries = intenv("BANDHU_CHAT_RETRIES", 2);
var chatdelay = intenv("BANDHU_CHAT_RETRY_DELAY_MS", 500);
var commandms = intenv("BANDHU_COMMAND_TIMEOUT_MS", 3e4);
var commandretries = intenv("BANDHU_COMMAND_RETRIES", 1);
var commanddelay = intenv("BANDHU_COMMAND_RETRY_DELAY_MS", 500);
var backend = process.env.BANDHU_BACKEND_URL || "http://127.0.0.1:3000";
async function sendchat(prompt) {
  const res = await postjson(
    `${backend}/chat`,
    { prompt },
    chatms,
    chatretries,
    chatdelay
  );
  const data = await res.json();
  if (!res.ok) {
    throw new Error(`chat failed: ${res.status}`);
  }
  return data;
}
async function approve(req) {
  const res = await postjson(
    `${backend}/approve`,
    { request_id: req.id, approved: true },
    commandms,
    commandretries,
    commanddelay
  );
  return res.ok;
}
async function reject(req) {
  const res = await postjson(
    `${backend}/approve`,
    { request_id: req.id, approved: false },
    commandms,
    commandretries,
    commanddelay
  );
  return res.ok;
}
async function postjson(url, body, timeout, retries, delay) {
  let attempt = 0;
  let last;
  while (attempt <= retries) {
    const controller = new AbortController();
    const timer = setTimeout(() => controller.abort(), timeout);
    try {
      const res = await fetch(url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
        signal: controller.signal
      });
      clearTimeout(timer);
      if (res.ok || attempt >= retries) {
        return res;
      }
    } catch (err) {
      last = err;
      clearTimeout(timer);
    } finally {
      clearTimeout(timer);
    }
    attempt += 1;
    if (attempt <= retries) {
      await wait(delay);
    }
  }
  throw last instanceof Error ? last : new Error("request failed");
}
function intenv(name, fallback) {
  const raw = process.env[name];
  const value = Number.parseInt(raw || "", 10);
  return Number.isFinite(value) ? value : fallback;
}
async function wait(ms) {
  await new Promise((resolve) => setTimeout(resolve, ms));
}

// src/config.ts
function fromEnv() {
  return {
    backendUrl: process.env.BANDHU_BACKEND_URL || "http://127.0.0.1:3000",
    defaultApproval: process.env.BANDHU_DEFAULT_APPROVAL === "true",
    approvalTimeoutSecs: parseInt(process.env.BANDHU_APPROVAL_TIMEOUT_SECS || "300", 10),
    forbiddenCommands: (process.env.BANDHU_FORBIDDEN_CMDS || "").split(",").map((s) => s.trim().toLowerCase()).filter(Boolean),
    forbiddenPaths: (process.env.BANDHU_FORBIDDEN_PATHS || "").split(",").map((s) => s.trim()).filter(Boolean),
    placeholder: process.env.BANDHU_CHAT_PLACEHOLDER || "Ask Bandhu..."
  };
}

// src/controller.ts
var Controller = class {
  constructor(ctx) {
    this.ctx = ctx;
    ctx.subscriptions.push(this);
  }
  ctx;
  status = new Statusbar();
  config = fromEnv();
  chat = new ChatPanel(this.config.placeholder);
  async activate() {
    this.chat.create();
    const disposables = [];
    disposables.push(vscode3.commands.registerCommand("bandhu.open", () => this.chat.focus()));
    disposables.push(this.chat.onDidReceiveMessage((msg) => this.handleWebviewMsg(msg)));
    for (const d of disposables) {
      this.ctx.subscriptions.push(d);
    }
  }
  async handleWebviewMsg(msg) {
    if (msg.type === "send" && msg.text) {
      this.status.setbusy();
      try {
        const res = await sendchat(msg.text);
        this.status.setidle();
        this.show(res);
      } catch (e) {
        this.status.seterror();
        this.chat.append({ type: "error", error: String(e) });
      }
    }
    if (msg.type === "approve" && msg.id) {
      await approve({ id: msg.id, tool: "", input: {} });
    }
    if (msg.type === "reject" && msg.id) {
      await reject({ id: msg.id, tool: "", input: {} });
    }
  }
  show(res) {
    const list = res.messages && res.messages.length > 0 ? res.messages : [{ type: "response", content: res.response }];
    for (const msg of list) {
      this.chat.append(msg);
    }
  }
  dispose() {
    this.status.dispose();
    this.chat.dispose();
  }
};

// src/extension.ts
function activate(ctx) {
  const controller = new Controller(ctx);
  controller.activate();
}
function deactivate() {
}
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  activate,
  deactivate
});
//# sourceMappingURL=extension.js.map
