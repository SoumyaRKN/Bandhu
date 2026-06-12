import { ChatRequest, ChatResponse, ApprovalRequestMsg, ChatMessage } from './types';

const chatms = intenv('BANDHU_CHAT_TIMEOUT_MS', 120000);
const chatretries = intenv('BANDHU_CHAT_RETRIES', 2);
const chatdelay = intenv('BANDHU_CHAT_RETRY_DELAY_MS', 500);
const commandms = intenv('BANDHU_COMMAND_TIMEOUT_MS', 30000);
const commandretries = intenv('BANDHU_COMMAND_RETRIES', 1);
const commanddelay = intenv('BANDHU_COMMAND_RETRY_DELAY_MS', 500);
const backend = process.env.BANDHU_BACKEND_URL || 'http://127.0.0.1:3000';

export async function sendchat(prompt: string): Promise<ChatResponse> {
    const res = await postjson(
        `${backend}/chat`,
        { prompt } as ChatRequest,
        chatms,
        chatretries,
        chatdelay
    );
    const data = await res.json() as ChatResponse;
    if (!res.ok) {
        throw new Error(`chat failed: ${res.status}`);
    }
    return data;
}

export async function sendchatstream(prompt: string, onmessage: (msg: ChatMessage) => void): Promise<ChatResponse> {
    const res = await postjson(
        `${backend}/chat/stream`,
        { prompt } as ChatRequest,
        chatms,
        0,
        chatdelay
    );
    if (!res.ok) {
        throw new Error(`chat stream failed: ${res.status}`);
    }
    if (!res.body) {
        throw new Error('chat stream body missing');
    }

    const reader = res.body.getReader();
    const decoder = new TextDecoder();
    let buffer = '';
    let lastmsg: ChatMessage | undefined;

    while (true) {
        const chunk = await reader.read();
        if (chunk.done) {
            break;
        }
        buffer += decoder.decode(chunk.value, { stream: true });
        const parts = buffer.split('\n\n');
        buffer = parts.pop() || '';
        for (const part of parts) {
            const msg = parsesseline(part);
            if (msg) {
                lastmsg = msg;
                onmessage(msg);
            }
        }
    }

    const tail = decoder.decode();
    if (tail) {
        const msg = parsesseline(tail);
        if (msg) {
            lastmsg = msg;
            onmessage(msg);
        }
    }

    return {
        response: lastmsg?.content || '',
        messages: lastmsg ? [lastmsg] : []
    };
}

export async function approve(req: ApprovalRequestMsg): Promise<boolean> {
    const res = await postjson(
        `${backend}/approve`,
        { request_id: req.id, approved: true },
        commandms,
        commandretries,
        commanddelay
    );
    return res.ok;
}

export async function reject(req: ApprovalRequestMsg): Promise<boolean> {
    const res = await postjson(
        `${backend}/approve`,
        { request_id: req.id, approved: false },
        commandms,
        commandretries,
        commanddelay
    );
    return res.ok;
}

async function postjson(url: string, body: unknown, timeout: number, retries: number, delay: number): Promise<Response> {
    let attempt = 0;
    let last: unknown;

    while (attempt <= retries) {
        const controller = new AbortController();
        const timer = setTimeout(() => controller.abort(), timeout);

        try {
            const res = await fetch(url, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(body),
                signal: controller.signal,
            });
            clearTimeout(timer);
            if (res.ok || attempt >= retries) {
                return res;
            }
        } catch (err) {
            last = err;
            clearTimeout(timer);
        } finally {
            clearTimeout(timer);
        }

        attempt += 1;
        if (attempt <= retries) {
            await wait(delay);
        }
    }

    throw last instanceof Error ? last : new Error('request failed');
}

function parsesseline(part: string): ChatMessage | undefined {
    const data = part
        .split('\n')
        .filter(line => line.startsWith('data:'))
        .map(line => line.slice(5).trim())
        .join('\n');
    if (!data) {
        return undefined;
    }
    return JSON.parse(data) as ChatMessage;
}

function intenv(name: string, fallback: number): number {
    const raw = process.env[name];
    const value = Number.parseInt(raw || '', 10);
    return Number.isFinite(value) ? value : fallback;
}

async function wait(ms: number): Promise<void> {
    await new Promise(resolve => setTimeout(resolve, ms));
}
