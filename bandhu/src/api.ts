import * as vscode from 'vscode';
import { ChatRequest, ChatResponse, ApprovalRequestMsg } from './types';
import { fromEnv } from './config';

const cfg = fromEnv();

export async function sendChat(prompt: string): Promise<ChatResponse> {
    const res = await fetch(`${cfg.backendUrl}/chat`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt } as ChatRequest),
    });
    const data = await res.json() as ChatResponse;
    if (!res.ok) {
        throw new Error(`chat failed: ${res.status}`);
    }
    return data;
}

export async function approve(req: ApprovalRequestMsg): Promise<boolean> {
    const res = await fetch(`${cfg.backendUrl}/approve`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ request_id: req.id, approved: true }),
    });
    return res.ok;
}

export async function reject(req: ApprovalRequestMsg): Promise<boolean> {
    const res = await fetch(`${cfg.backendUrl}/approve`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ request_id: req.id, approved: false }),
    });
    return res.ok;
}
