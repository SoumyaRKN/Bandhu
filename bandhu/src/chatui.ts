import * as vscode from 'vscode';
import { ChatMessage, WebviewMsg } from './types';

export class ChatPanel {
    private panel: vscode.WebviewPanel | undefined;
    private _onDidReceiveMessage = new vscode.EventEmitter<WebviewMsg>();
    readonly onDidReceiveMessage = this._onDidReceiveMessage.event;

    constructor(private placeholder: string = 'Ask Bandhu...') {}

    create(column: vscode.ViewColumn = vscode.ViewColumn.One) {
        if (this.panel) {
            this.panel.reveal(column);
            return;
        }
        this.panel = vscode.window.createWebviewPanel(
            'bandhuChat',
            'Bandhu Chat',
            column,
            { enableScripts: true }
        );

        this.panel.webview.html = this.getHtml();

        this.panel.webview.onDidReceiveMessage((msg: WebviewMsg) => {
            this._onDidReceiveMessage.fire(msg);
        });

        this.panel.onDidDispose(() => {
            this.panel = undefined;
        });
    }

    focus() {
        this.create();
        if (!this.panel) {
            return;
        }
        this.panel.webview.postMessage({ type: 'focus' });
    }

    append(msg: ChatMessage) {
        if (!this.panel) {
            return;
        }
        this.panel.webview.postMessage({ type: 'message', data: msg });
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

    private escapeHtml(text: string): string {
        return text.replace(/[&<>"']/g, m => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[m] || m);
    }

    private getHtml(): string {
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

        function formatBuild(result, error) {
            if (error) {
                return 'build failed: ' + error;
            }
            if (!result) {
                return 'build finished';
            }
            const summary = result.summary || 'unknown';
            const command = result.command || '';
            return 'build ' + summary + ': ' + command;
        }

        function formattest(result, error) {
            if (error) {
                return 'test failed: ' + error;
            }
            if (!result) {
                return 'test finished';
            }
            const summary = result.summary || 'unknown';
            const command = result.command || '';
            return 'test ' + summary + ': ' + command;
        }

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
                } else if (data.type === 'response' || data.type === 'tool_result' || data.type === 'tool_error' || data.type === 'build_result' || data.type === 'testresult' || data.type === 'error') {
                    const text = data.type === 'build_result'
                        ? formatBuild(data.result, data.error)
                        : data.type === 'testresult'
                        ? formattest(data.result, data.error)
                        : (data.content || data.error || '');
                    addMessage(data.type, text);
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
}
