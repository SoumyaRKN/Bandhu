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
var vscode4 = __toESM(require("vscode"));

// src/status.ts
var vscode = __toESM(require("vscode"));
var StatusBar = class {
  item;
  constructor() {
    this.item = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    this.item.command = "bandhu.helloWorld";
    this.item.show();
  }
  setBusy() {
    this.item.text = "$(loading~spin) Bandhu";
    this.item.tooltip = "Working";
  }
  setIdle() {
    this.item.text = "$(check) Bandhu";
    this.item.tooltip = "Ready";
  }
  setError() {
    this.item.text = "$(error) Bandhu";
    this.item.tooltip = "Error";
  }
  dispose() {
    this.item.dispose();
  }
};

// src/chatui.ts
var vscode2 = __toESM(require("vscode"));
var ChatPanel = class {
  panel;
  disposables = [];
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
    this.panel.onDidDispose(() => {
      this.panel = void 0;
    });
    this.disposables.push(this.panel);
  }
  append(msg) {
    if (!this.panel) return;
    this.panel.webview.postMessage({ type: "message", data: msg });
    const type = msg.type;
    const content = msg.content || msg.error || "";
    this.panel.webview.html += `<div class="msg ${type}">${this.escapeHtml(content)}</div>`;
  }
  clear() {
    if (!this.panel) return;
    this.panel.webview.html = this.getHtml();
  }
  dispose() {
    this.disposables.forEach((d) => d.dispose());
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
        body { font-family: var(--vscode-font-family); padding: 10px; color: var(--vscode-foreground); background: var(--vscode-editor-background); }
        .msg { padding: 6px 10px; margin: 4px 0; border-radius: 4px; }
        .response { background: var(--vscode-editor-inactiveSelectionBackground); }
        .tool_result { background: var(--vscode-textBlockQuote-background); }
        .tool_error { background: var(--vscode-inputValidation-errorBackground); color: var(--vscode-errorForeground); }
        .tool_approval { background: var(--vscode-inputValidation-infoBackground); }
    </style>
</head>
<body>
    <div id="messages"></div>
</body>
</html>`;
  }
};

// src/config.ts
function fromEnv() {
  return {
    backendUrl: process.env.BANDHU_BACKEND_URL || "http://127.0.0.1:3000",
    defaultApproval: process.env.BANDHU_DEFAULT_APPROVAL === "true",
    approvalTimeoutSecs: parseInt(process.env.BANDHU_APPROVAL_TIMEOUT_SECS || "300", 10),
    forbiddenCommands: (process.env.BANDHU_FORBIDDEN_CMDS || "").split(",").map((s) => s.trim().toLowerCase()).filter(Boolean),
    forbiddenPaths: (process.env.BANDHU_FORBIDDEN_PATHS || "").split(",").map((s) => s.trim()).filter(Boolean)
  };
}

// src/api.ts
var cfg = fromEnv();
async function sendChat(prompt) {
  const res = await fetch(`${cfg.backendUrl}/chat`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ prompt })
  });
  const data = await res.json();
  if (!res.ok) throw new Error(`chat failed: ${res.status}`);
  return data;
}
async function approve(req) {
  const res = await fetch(`${cfg.backendUrl}/approve`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ request_id: req.id, approved: true })
  });
  return res.ok;
}
async function reject(req) {
  const res = await fetch(`${cfg.backendUrl}/approve`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ request_id: req.id, approved: false })
  });
  return res.ok;
}

// src/approval.ts
var vscode3 = __toESM(require("vscode"));
async function showApproval(req) {
  const choice = await vscode3.window.showWarningMessage(
    `Approve ${req.tool}?`,
    { modal: true },
    "Approve",
    "Reject"
  );
  if (choice === "Approve") {
    await approve(req);
    return true;
  }
  await reject(req);
  return false;
}

// src/controller.ts
var Controller = class {
  constructor(ctx) {
    this.ctx = ctx;
    ctx.subscriptions.push(this);
  }
  ctx;
  status = new StatusBar();
  chat = new ChatPanel();
  async activate() {
    this.chat.create();
    vscode4.commands.registerCommand("bandhu.helloWorld", () => this.chat.create());
    vscode4.commands.registerCommand("bandhu.send", async () => {
      const input = await vscode4.window.showInputBox({ prompt: "Ask Bandhu" });
      if (!input) return;
      this.status.setBusy();
      const res = await sendChat(input);
      this.status.setIdle();
      this.chat.append({ type: "response", content: res.response });
    });
  }
  async handleMessage(msg) {
    if (msg.type === "tool_approval") {
      const req = msg;
      await showApproval(req);
    }
    this.chat.append(msg);
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
