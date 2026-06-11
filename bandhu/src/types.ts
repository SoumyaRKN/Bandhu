export interface ToolCall {
    id: string;
    tool: string;
    input: unknown;
}

export interface ChatMessage {
    type: 'response' | 'tool_result' | 'tool_error' | 'tool_approval' | 'error' | 'complete';
    content?: string;
    id?: string;
    tool?: string;
    result?: unknown;
    error?: string;
    messages?: ChatMessage[];
    iterations?: number;
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
}
