import * as vscode from 'vscode';
import { ApprovalRequestMsg } from './types';
import { approve, reject } from './api';

export async function showApproval(req: ApprovalRequestMsg): Promise<boolean> {
    const choice = await vscode.window.showWarningMessage(
        `Approve ${req.tool}?`,
        { modal: true },
        'Approve',
        'Reject'
    );
    if (choice === 'Approve') {
        await approve(req);
        return true;
    }
    await reject(req);
    return false;
}
