# Diff Workflow

Bandhu uses diff previews to make file edits reviewable before execution.

## Flow

1. The model calls `writefile` with a target `path` and full replacement `content`.
2. The queue validates the input and checks `BANDHU_FORBIDDEN_PATHS`.
3. `Writefile::diff` reads the existing file when present.
4. `backend/src/diff.rs` generates a unified diff preview.
5. The backend emits a `tool_approval` message containing the tool input and `diff`.
6. The extension displays the diff and sends the decision to `/approve`.
7. On approval, `writefile` writes the content. On rejection, the loop records a `tool_error`.

## Message

```json
{
    "type": "tool_approval",
    "id": "writefile-1",
    "tool": "writefile",
    "input": {
        "path": "docs/example.md",
        "content": "# Example\n"
    },
    "diff": "--- a/docs/example.md\n+++ b/docs/example.md\n+# Example\n"
}
```

## Configuration

| Variable                       | Default   | Description                                                   |
| ------------------------------ | --------- | ------------------------------------------------------------- |
| `BANDHU_SCHEMA_VALIDATE`       | `true`    | Validates diff-producing tool input before approval.          |
| `BANDHU_TOOL_INPUT_LIMIT`      | `65536`   | Maximum serialized input size for the tool call.              |
| `BANDHU_FORBIDDEN_PATHS`       | _(empty)_ | Comma-separated path substrings blocked before diff approval. |
| `BANDHU_APPROVAL_TIMEOUT_SECS` | `300`     | Intended approval timeout window for pending decisions.       |

## Limits

The current diff generator is intentionally simple. It previews line additions and removals for complete-file replacement, while future roadmap work tracks hunk-level approval and editable suggestions.
