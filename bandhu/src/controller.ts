import * as vscode from 'vscode';
import { StatusBar } from './status';
import { ChatPanel } from './chatui';
import { sendChat, approve, reject } from './api';
import { ChatMessage, ApprovalRequestMsg, WebviewMsg } from './types';

export class Controller implements vscode.Disposable {
    private status: StatusBar = new StatusBar();
    private chat: ChatPanel = new ChatPanel();

    constructor(private ctx: vscode.ExtensionContext) {
        ctx.subscriptions.push(this);
    }

    async activate() {
        this.chat.create();
        const disposables: vscode.Disposable[] = [];

        disposables.push(vscode.commands.registerCommand('bandhu.helloWorld', () => this.chat.create()));

        disposables.push(vscode.commands.registerCommand('bandhu.send', async () => {
            const input = await vscode.window.showInputBox({ prompt: 'Ask Bandhu' });
            if (!input) return;
            this.status.setBusy();
            try {
                const res = await sendChat(input);
                this.status.setIdle();
                this.chat.append({ type: 'response', content: res.response } as ChatMessage);
            } catch (e) {
                this.status.setError();
                this.chat.append({ type: 'error', error: String(e) } as ChatMessage);
            }
        }));

        disposables.push(this.chat.onDidReceiveMessage((msg: WebviewMsg) => this.handleWebviewMsg(msg)));

        for (const d of disposables) {
            this.ctx.subscriptions.push(d);
        }
    }

    private async handleWebviewMsg(msg: WebviewMsg) {
        if (msg.type === 'send' && msg.text) {
            this.status.setBusy();
            try {
                const res = await sendChat(msg.text);
                this.status.setIdle();
                this.chat.append({ type: 'response', content: res.response } as ChatMessage);
            } catch (e) {
                this.status.setError();
                this.chat.append({ type: 'error', error: String(e) } as ChatMessage);
            }
        }
        if (msg.type === 'approve' && msg.id) {
            await approve({ id: msg.id, tool: '', input: {} } as ApprovalRequestMsg);
        }
        if (msg.type === 'reject' && msg.id) {
            await reject({ id: msg.id, tool: '', input: {} } as ApprovalRequestMsg);
        }
    }

    dispose() {
        this.status.dispose();
        this.chat.dispose();
    }
}

export function activate(ctx: vscode.ExtensionContext) {
    const controller = new Controller(ctx);
    controller.activate();
}

export function deactivate() {}