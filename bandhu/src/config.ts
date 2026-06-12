export interface EnvConfig {
    backendUrl: string;
    defaultApproval: boolean;
    approvalTimeoutSecs: number;
    forbiddenCommands: string[];
    forbiddenPaths: string[];
    placeholder: string;
    streaming: boolean;
    outputName: string;
    outputShow: boolean;
}

export function fromEnv(): EnvConfig {
    return {
        backendUrl: process.env.BANDHU_BACKEND_URL || 'http://127.0.0.1:3000',
        defaultApproval: process.env.BANDHU_DEFAULT_APPROVAL === 'true',
        approvalTimeoutSecs: parseInt(process.env.BANDHU_APPROVAL_TIMEOUT_SECS || '300', 10),
        forbiddenCommands: (process.env.BANDHU_FORBIDDEN_CMDS || '')
            .split(',')
            .map(s => s.trim().toLowerCase())
            .filter(Boolean),
        forbiddenPaths: (process.env.BANDHU_FORBIDDEN_PATHS || '')
            .split(',')
            .map(s => s.trim())
            .filter(Boolean),
        placeholder: process.env.BANDHU_CHAT_PLACEHOLDER || 'Ask Bandhu...',
        streaming: process.env.BANDHU_CHAT_STREAMING !== 'false',
        outputName: process.env.BANDHU_OUTPUT_NAME || 'Bandhu',
        outputShow: process.env.BANDHU_OUTPUT_SHOW !== 'false',
    };
}
