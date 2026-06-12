# Approval Flow

## Overview

Every `writefile` and `runcommand` execution goes through the **Approval Gate** before the safety filter permits execution.

## UI Types

The extension supports two approval UI modes:

1. **Quick Pick Dialog** - Shows `vscode.window.showWarningMessage` with Approve/Reject buttons (used by `approval.ts`)
2. **Webview Panel** - Inline approval buttons in the chat interface (primary method in `chatui.ts`)

## Sequence

1. The model calls a tool that marks `requires == true`.
2. The safety filter scans the tool input against forbidden commands and paths.
3. If the input is blocked, the loop returns a `tool_error` message.
4. If the input passes, the backend emits a `tool_approval` message.
5. The webview displays inline approval buttons (Approve/Reject) with tool details:
   - Tool name and target path/command
   - For writefile: a unified diff showing the proposed changes
   - JSON representation of the input
6. The user clicks Approve or Reject in the webview.
7. The controller sends the decision to `/approve` endpoint.
8. The backend resumes or aborts execution based on the decision.

## Diff Preview

When the `writefile` tool is triggered:

1. The diff generator reads the existing file content (if any).
2. A unified diff is generated showing changes with `-` (removals) and `+` (additions).
3. The diff is included in the `tool_approval` message under the `diff` field.
4. The webview renders the diff in a formatted `<pre>` block with syntax highlighting.

Example diff output:
```
--- a/src/main.rs
+++ b/src/main.rs
-line1
+line1 modified
 line2
```

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
| `tool_approval` | Tool requires user approval before execution. Includes `diff` field for writefile. |
| `tool_result` | Tool executed successfully. |
| `tool_error` | Tool failed (blocked by gate, path error, or execution error). |
