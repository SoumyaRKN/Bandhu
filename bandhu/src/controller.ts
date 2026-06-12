import * as vscode from 'vscode';
import { Statusbar } from './status';
import { ChatPanel } from './chatui';
import { sendchat, sendchatstream, approve, reject } from './api';
import { ChatMessage, ApprovalRequestMsg, WebviewMsg } from './types';
import { fromEnv } from './config';
import { Report } from './report';

export class Controller implements vscode.Disposable {
    private status: Statusbar = new Statusbar();
    private config = fromEnv();
    private chat: ChatPanel = new ChatPanel(this.config.placeholder);
    private report: Report = new Report(this.config.outputName);
    private active: AbortController | undefined;

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
            if (this.active) {
                this.active.abort();
            }
            const controller = new AbortController();
            this.active = controller;
            this.status.setbusy();
            try {
                if (this.config.streaming) {
                    await sendchatstream(msg.text, resmsg => this.handle(resmsg), controller.signal);
                } else {
                    const res = await sendchat(msg.text, controller.signal);
                    this.show(res);
                }
                if (this.active === controller) {
                    this.active = undefined;
                    this.status.setidle();
                }
            } catch (e) {
                if (this.active === controller) {
                    this.active = undefined;
                    this.status.seterror();
                    this.chat.append({ type: 'error', error: String(e) } as ChatMessage);
                }
            }
        }
        if (msg.type === 'approve' && msg.id) {
            await approve({ id: msg.id, tool: '', input: {} } as ApprovalRequestMsg);
        }
        if (msg.type === 'reject' && msg.id) {
            await reject({ id: msg.id, tool: '', input: {} } as ApprovalRequestMsg);
        }
    }

    private handle(msg: ChatMessage) {
        this.report.log(msg);
        if (this.config.outputShow && (msg.type === 'build_result' || msg.type === 'testresult' || msg.type === 'tool_result')) {
            this.report.show();
        }
        this.chat.append(msg);
    }

    private show(res: { response: string; messages?: ChatMessage[] }) {
        const list = res.messages && res.messages.length > 0
            ? res.messages
            : [{ type: 'response', content: res.response } as ChatMessage];

        for (const msg of list) {
            this.handle(msg);
        }
    }

    dispose() {
        this.status.dispose();
        this.chat.dispose();
        this.report.dispose();
    }
}

export function activate(ctx: vscode.ExtensionContext) {
    const controller = new Controller(ctx);
    controller.activate();
}

export function deactivate() {}
