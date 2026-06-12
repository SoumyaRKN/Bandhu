export interface ChatMessage {
    type: 'response' | 'tool_result' | 'tool_error' | 'tool_approval' | 'error' | 'complete';
    content?: string;
    id?: string;
    tool?: string;
    result?: unknown;
    error?: string;
    messages?: ChatMessage[];
    iterations?: number;
    diff?: string;
    input?: unknown;
    kind?: string;
    pattern?: string;
}

export interface ApprovalRequestMsg {
    id: string;
    tool: string;
    input: unknown;
}

export interface ChatRequest {
    prompt: string;
}

export interface ChatResponse {
    response: string;
    messages?: ChatMessage[];
}

export interface WebviewMsg {
    type: 'send' | 'approve' | 'reject' | 'message';
    data?: ChatMessage;
    id?: string;
    text?: string;
}
