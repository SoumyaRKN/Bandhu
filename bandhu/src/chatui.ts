import * as vscode from 'vscode';
import { ChatMessage } from './types';

export class ChatPanel {
    private panel: vscode.WebviewPanel | undefined;
    private disposables: vscode.Disposable[] = [];

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

        this.panel.onDidDispose(() => {
            this.panel = undefined;
        });

        this.disposables.push(this.panel);
    }

    append(msg: ChatMessage) {
        if (!this.panel) return;
        this.panel.webview.postMessage({ type: 'message', data: msg });
        const type = msg.type;
        const content = (msg as any).content || (msg as any).error || '';
        this.panel.webview.html += `<div class="msg ${type}">${this.escapeHtml(content)}</div>`;
    }

    clear() {
        if (!this.panel) return;
        this.panel.webview.html = this.getHtml();
    }

    dispose() {
        this.disposables.forEach(d => d.dispose());
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
}
