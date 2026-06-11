import * as vscode from 'vscode';

export class StatusBar {
    private item: vscode.StatusBarItem;

    constructor() {
        this.item = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
        this.item.command = 'bandhu.helloWorld';
        this.item.show();
    }

    setBusy() {
        this.item.text = '$(loading~spin) Bandhu';
        this.item.tooltip = 'Working';
    }

    setIdle() {
        this.item.text = '$(check) Bandhu';
        this.item.tooltip = 'Ready';
    }

    setError() {
        this.item.text = '$(error) Bandhu';
        this.item.tooltip = 'Error';
    }

    dispose() {
        this.item.dispose();
    }
}
