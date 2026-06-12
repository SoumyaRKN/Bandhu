# Approval Flow

## Overview

Every `writefile` and `runcommand` execution goes through input validation and the **Approval Gate** before the safety filter permits execution.

## UI Types

The extension supports two approval UI modes:

1. **Quick Pick Dialog** - Shows `vscode.window.showWarningMessage` with Approve/Reject buttons (used by `approval.ts`)
2. **Webview Panel** - Inline approval buttons in the chat interface (primary method in `chatui.ts`)

## Sequence

1. The model calls a tool that marks `requires == true`.
2. The tool input is validated against its schema and size limit.
3. The safety filter scans the tool input against forbidden commands and paths.
4. If the input is blocked, the loop returns a `tool_error` message.
5. If the input passes, the backend emits a `tool_approval` message in the `/chat` response `messages` array.
6. The controller forwards returned messages to the webview panel.
7. The webview displays inline approval buttons (Approve/Reject) with tool details:
   - Tool name and target path/command
   - For writefile: a unified diff showing the proposed changes
   - JSON representation of the input
8. The user clicks Approve or Reject in the webview.
9. The controller sends the decision to `/approve` endpoint.
10. The backend resumes or aborts execution based on the decision.

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

The gate checks `BANDHU_FORBIDDEN_CMDS`, `BANDHU_INSTALL_CMDS`, and `BANDHU_FORBIDDEN_PATHS` from `backend/.env`.

- `runcommand` input is checked as a lowercase command string.
- The default forbidden command patterns block `rm -rf`, `sudo`, and shell ampersand operators before approval.
- Package install commands are matched against `BANDHU_INSTALL_CMDS`. Matching approvals include `kind: "install"` and the matched `pattern`.
- `writefile`, `readfile`, and `runcommand` path inputs are checked as path substrings when a `path` field is present.
- Matching is case-insensitive for commands and paths.

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `BANDHU_DEFAULT_APPROVAL` | `false` | When `true`, bypasses the user prompt. |
| `BANDHU_APPROVAL_TIMEOUT_SECS` | `300` | Timeout for pending approval. |
| `BANDHU_APPROVAL_LOG` | _(empty)_ | Optional JSONL audit log path for approved, rejected, and timed out decisions. |
| `BANDHU_FORBIDDEN_CMDS` | `rm -rf,sudo,&` | Command patterns blocked before approval and execution. |
| `BANDHU_INSTALL_CMDS` | `apt install,apt-get install,npm install,yarn add,pnpm add,cargo install,pip install,pip3 install,uv pip install,poetry add,gem install,go install,brew install` | Package install command patterns that tag `runcommand` approvals as install approvals. |
| `BANDHU_SCHEMA_VALIDATE` | `true` | Validates tool inputs before approval and execution. |
| `BANDHU_TOOL_INPUT_LIMIT` | `65536` | Max serialized JSON bytes allowed for a tool input. |

## Messages

The `/chat` response includes `response` for compatibility and `messages` for structured rendering. The controller appends each returned message to the webview panel. The loop emits these approval-related message types:

| Type | Meaning |
|---|---|
| `tool_approval` | Tool requires user approval before execution. Includes `diff` field for writefile and `kind: "install"` for package installs. |
| `tool_result` | Tool executed successfully. |
| `tool_error` | Tool failed (blocked by gate, path error, or execution error). |

## Audit Log

When `BANDHU_APPROVAL_LOG` is set, each decision is appended as one JSON object per line:

```json
{"decision":"approved","id":"writefile-1","tool":"writefile"}
```
