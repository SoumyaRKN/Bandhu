export interface EnvConfig {
    backendUrl: string;
    defaultApproval: boolean;
    approvalTimeoutSecs: number;
    forbiddenCommands: string[];
    forbiddenPaths: string[];
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
    };
}
