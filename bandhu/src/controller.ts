import * as vscode from 'vscode';
import { StatusBar } from './status';
import { ChatPanel } from './chatui';
import { sendChat, showApproval } from './api';
import { ChatMessage, ApprovalRequestMsg } from './types';

class Controller implements vscode.Disposable {
    private status: StatusBar = new StatusBar();
    private chat: ChatPanel = new ChatPanel();

    constructor(private ctx: vscode.ExtensionContext) {
        ctx.subscriptions.push(this);
    }

    async activate() {
        this.chat.create();
        vscode.commands.registerCommand('bandhu.helloWorld', () => this.chat.create());
        vscode.commands.registerCommand('bandhu.send', async () => {
            const input = await vscode.window.showInputBox({ prompt: 'Ask Bandhu' });
            if (!input) return;
            this.status.setBusy();
            const res = await sendChat(input);
            this.status.setIdle();
            this.chat.append({ type: 'response', content: res.response } as ChatMessage);
        });
    }

    async handleMessage(msg: ChatMessage) {
        if (msg.type === 'tool_approval') {
            const req = msg as ApprovalRequestMsg;
            await showApproval(req);
        }
        this.chat.append(msg);
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
