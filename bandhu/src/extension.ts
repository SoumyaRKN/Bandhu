import * as vscode from 'vscode';
import { Controller } from './controller';

export function activate(ctx: vscode.ExtensionContext) {
    const controller = new Controller(ctx);
    controller.activate();
}

export function deactivate() {}
