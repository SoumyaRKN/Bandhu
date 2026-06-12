import * as vscode from 'vscode';
import { ChatMessage } from './types';

export class Report {
    private channel: vscode.OutputChannel;

    constructor(name: string) {
        this.channel = vscode.window.createOutputChannel(name);
    }

    show() {
        this.channel.show(true);
    }

    log(msg: ChatMessage) {
        if (msg.type === 'build_result') {
            this.logbuild(msg);
            return;
        }
        if (msg.type === 'testresult') {
            this.logtestmsg(msg);
            return;
        }
        if (msg.type !== 'tool_result') {
            return;
        }
        const tool = msg.id || '';
        const result = msg.result as Record<string, unknown> | undefined;
        if (!result) {
            return;
        }
        if (tool === 'buildtool') {
            this.logbuild({ type: 'build_result', result });
            return;
        }
        if (tool === 'testrunner') {
            this.logtest(result);
        }
    }

    dispose() {
        this.channel.dispose();
    }

    private logtestmsg(msg: ChatMessage) {
        const result = msg.result as Record<string, unknown> | undefined;
        const error = msg.error;
        const stamp = new Date().toISOString();
        this.channel.appendLine(`[${stamp}] test`);
        if (error) {
            this.channel.appendLine(`error: ${error}`);
            this.channel.appendLine('');
            return;
        }
        if (!result) {
            return;
        }
        this.writesection('test', result);
    }

    private logbuild(msg: ChatMessage) {
        const result = msg.result as Record<string, unknown> | undefined;
        const error = msg.error;
        const stamp = new Date().toISOString();
        this.channel.appendLine(`[${stamp}] build`);
        if (error) {
            this.channel.appendLine(`error: ${error}`);
            this.channel.appendLine('');
            return;
        }
        if (!result) {
            return;
        }
        this.writesection('build', result);
    }

    private logtest(result: Record<string, unknown>) {
        const stamp = new Date().toISOString();
        this.channel.appendLine(`[${stamp}] test`);
        this.writesection('test', result);
    }

    private writesection(kind: string, result: Record<string, unknown>) {
        this.channel.appendLine(`command: ${result.command || ''}`);
        this.channel.appendLine(`directory: ${result.directory || ''}`);
        this.channel.appendLine(`summary: ${result.summary || ''}`);
        if (result.stdout) {
            this.channel.appendLine('stdout:');
            this.channel.appendLine(String(result.stdout));
        }
        if (result.stderr) {
            this.channel.appendLine('stderr:');
            this.channel.appendLine(String(result.stderr));
        }
        this.logfailures(result.failures);
        this.channel.appendLine('');
    }

    private logfailures(failures: unknown) {
        if (!Array.isArray(failures) || failures.length === 0) {
            return;
        }
        this.channel.appendLine('failures:');
        for (const line of failures) {
            this.channel.appendLine(String(line));
        }
    }
}
