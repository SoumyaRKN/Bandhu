# Approval Flow

## Overview

Every `writefile` and `runcommand` execution goes through the **Approval Gate** before the safety filter permits execution.

## Sequence

1. The model calls a tool that marks `requires == true`.
2. The safety filter scans the tool input against forbidden commands and paths.
3. If the input is blocked, the loop returns a `tool_error` message.
4. If the input passes, the backend emits a `tool_approval` message.
5. The extension shows an approval prompt to the user.
6. The user accepts or rejects.
7. The backend resumes or aborts execution based on the decision.

## Safety Filter

The gate checks `BANDHU_FORBIDDEN_CMDS` and `BANDHU_FORBIDDEN_PATHS` from `backend/.env`.

- `runcommand` input is checked as a lowercase command string.
- `writefile` and `runcommand` path inputs are checked as path substrings.
- Matching is case-insensitive for commands, case-sensitive for paths.

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `BANDHU_DEFAULT_APPROVAL` | `false` | When `true`, bypasses the user prompt. |
| `BANDHU_APPROVAL_TIMEOUT_SECS` | `300` | Timeout for pending approval. |

## Messages

The loop emits these approval-related message types:

| Type | Meaning |
|---|---|
| `tool_approval` | Tool requires user approval before execution. |
| `tool_result` | Tool executed successfully. |
| `tool_error` | Tool failed (blocked by gate, path error, or execution error). |
