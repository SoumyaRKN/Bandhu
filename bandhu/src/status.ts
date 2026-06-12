import * as vscode from 'vscode';

export class Statusbar {
    private item: vscode.StatusBarItem;

    constructor() {
        this.item = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
        this.item.command = 'bandhu.open';
        this.setidle();
        this.item.show();
    }

    setbusy() {
        this.item.text = env('BANDHU_STATUS_BUSY_TEXT', '$(loading~spin) Bandhu');
        this.item.tooltip = env('BANDHU_STATUS_BUSY_TOOLTIP', 'Working');
    }

    setidle() {
        this.item.text = env('BANDHU_STATUS_TEXT', '$(check) Bandhu');
        this.item.tooltip = env('BANDHU_STATUS_TOOLTIP', 'Ready');
    }

    seterror() {
        this.item.text = env('BANDHU_STATUS_ERROR_TEXT', '$(error) Bandhu');
        this.item.tooltip = env('BANDHU_STATUS_ERROR_TOOLTIP', 'Error');
    }

    dispose() {
        this.item.dispose();
    }
}

function env(name: string, fallback: string): string {
    return process.env[name] || fallback;
}
