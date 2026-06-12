import * as vscode from 'vscode';
import { StatusBar } from './status';
import { ChatPanel } from './chatui';
import { sendChat, approve, reject } from './api';
import { ChatMessage, ApprovalRequestMsg, WebviewMsg } from './types';
import { fromEnv } from './config';

export class Controller implements vscode.Disposable {
    private status: StatusBar = new StatusBar();
    private config = fromEnv();
    private chat: ChatPanel = new ChatPanel(this.config.placeholder);

    constructor(private ctx: vscode.ExtensionContext) {
        ctx.subscriptions.push(this);
    }

    async activate() {
        this.chat.create();
        const disposables: vscode.Disposable[] = [];

        disposables.push(vscode.commands.registerCommand('bandhu.open', () => this.chat.focus()));

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
                this.show(res);
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

    private show(res: { response: string; messages?: ChatMessage[] }) {
        const list = res.messages && res.messages.length > 0
            ? res.messages
            : [{ type: 'response', content: res.response } as ChatMessage];

        for (const msg of list) {
            this.chat.append(msg);
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
