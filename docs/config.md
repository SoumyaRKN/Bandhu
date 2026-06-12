# Bandhu Configuration Reference

All configurable parameters are set via environment variables or a `.env` file in `backend/`.

---

## Extension Settings

| Variable | Default | Description |
|---|---|---|
| `BANDHU_BACKEND_URL` | `http://127.0.0.1:3000` | Backend HTTP endpoint for extension API calls. |
| `BANDHU_CHAT_PLACEHOLDER` | `Ask Bandhu...` | Placeholder shown in the webview chat input. Example: `Describe the coding task...` |

The extension uses this endpoint for `/chat` and `/approve`. `/chat` returns a compatibility `response` string and a structured `messages` array that the controller forwards to the webview.

The `Bandhu: Open Chat` command and status bar item open the webview chat panel. User prompts are submitted from the webview input, not from a VS Code InputBox.

---

## Server Settings

| Variable | Default | Description |
|---|---|---|
| `BANDHU_SERVER_HOST` | `127.0.0.1` | Backend bind address. |
| `BANDHU_SERVER_PORT` | `3000` | Backend bind port. |
| `BANDHU_CORS_ORIGINS` | `*` | Comma-separated HTTP origins allowed to call the backend. Use `*` for local development or explicit values such as `http://127.0.0.1:3000,vscode-webview://localhost`. |

---

## Ollama Settings

| Variable | Default | Description |
|---|---|---|
| `BANDHU_OLLAMA_BASE_URL` | `http://localhost:11434` | Ollama server address. |
| `BANDHU_OLLAMA_MODEL` | `qwen2.5-coder:7b` | Default model name. |
| `BANDHU_OLLAMA_STREAM` | `false` | Enable streaming output. |

---

## Tool Loop Settings

| Variable | Default | Description |
|---|---|---|
| `BANDHU_MAX_ITERATIONS` | `10` | Max tool-call loop iterations per request. |
| `BANDHU_RG_MAX_COUNT` | `50` | Max ripgrep matches returned per search. |
| `BANDHU_SCHEMA_VALIDATE` | `true` | Validate tool input against each tool schema before execution. Set to `false` only for debugging. |
| `BANDHU_TOOL_INPUT_LIMIT` | `65536` | Max serialized JSON bytes allowed for a tool input payload. |

---

## Approval & Safety Settings

| Variable | Default | Description |
|---|---|---|
| `BANDHU_DEFAULT_APPROVAL` | `false` | Auto-approve all tools when `true`. Disable for production. |
| `BANDHU_APPROVAL_TIMEOUT_SECS` | `300` | Seconds before pending approval is aborted. |
| `BANDHU_APPROVAL_LOG` | _(empty)_ | Optional JSONL audit log path for approval decisions. Example: `./approval.jsonl` |
| `BANDHU_FORBIDDEN_CMDS` | `rm -rf,sudo,&` | Comma-separated lowercase command patterns blocked by the safety filter. Set to an empty value only for controlled debugging. Example: `rm -rf,sudo,&,del /f` |
| `BANDHU_INSTALL_CMDS` | `apt install,apt-get install,npm install,yarn add,pnpm add,cargo install,pip install,pip3 install,uv pip install,poetry add,gem install,go install,brew install` | Comma-separated lowercase package install command patterns. Matching `runcommand` approvals are tagged with `kind: "install"` and the matched `pattern`. |
| `BANDHU_FORBIDDEN_PATHS` | _(empty)_ | Comma-separated path substrings blocked by the safety filter. Example: `/etc/passwd,.env` |

---

## Context Builder Settings

| Variable | Default | Description |
|---|---|---|
| `BANDHU_CONTEXT_TOKEN_LIMIT` | `8192` | Approximate token budget used when packing context for the model. |
| `BANDHU_CONTEXT_TOP_N` | `10` | Max candidate files selected during the select stage. |
| `BANDHU_CONTEXT_MAX_FILE_BYTES` | `65536` | Max file size in bytes before truncation during the read stage. |

---

## Ollama Runtime Settings

| Variable | Default | Description |
|---|---|---|
| `BANDHU_OLLAMA_TIMEOUT_SECS` | `120` | Max seconds to wait for a single Ollama request before timing out. |

Ollama request failures are surfaced as typed backend errors. Connection failures use the model error path, HTTP status failures use HTTP or model errors, response decoding failures use parse errors, and elapsed requests use timeout errors configured by `BANDHU_OLLAMA_TIMEOUT_SECS`.

---

## Prompt Template

| Variable | Default | Description |
|---|---|---|
| `BANDHU_PROMPT_TEMPLATE` | _(built-in)_ | Python-style format string for the tool loop prompt. Use `{}` placeholders for tools list, context, and task. |

---

## Tool Validation

Tool validation runs before safety checks and execution in both the tool-call loop and the `/call` endpoint.

| Setting | Default | Behavior |
|---|---|---|
| `BANDHU_SCHEMA_VALIDATE=true` | `true` | Rejects tool inputs that fail the registered tool schema, such as missing required fields or wrong types. |
| `BANDHU_SCHEMA_VALIDATE=false` | `false` | Skips schema validation while still enforcing `BANDHU_TOOL_INPUT_LIMIT`. |
| `BANDHU_TOOL_INPUT_LIMIT=65536` | `65536` | Rejects serialized JSON inputs larger than the configured byte limit. |

Sample validation failure:

```json
{
  "type": "tool_error",
  "tool": "readfile",
  "error": "tool error: path is empty"
}
```

---

## Logging

| Variable | Default | Description |
|---|---|---|
| `BANDHU_LOG_LEVEL` | `info` | Logging verbosity. Accepted values: `error`, `warn`, `info`, `debug`, `trace`. |

---

## Sample `.env`

```env
BANDHU_SERVER_HOST=127.0.0.1
BANDHU_SERVER_PORT=3000
BANDHU_CHAT_PLACEHOLDER=Ask Bandhu...
BANDHU_CORS_ORIGINS=*
BANDHU_OLLAMA_BASE_URL=http://localhost:11434
BANDHU_OLLAMA_MODEL=qwen2.5-coder:7b
BANDHU_OLLAMA_STREAM=false
BANDHU_MAX_ITERATIONS=10
BANDHU_RG_MAX_COUNT=50
BANDHU_SCHEMA_VALIDATE=true
BANDHU_TOOL_INPUT_LIMIT=65536
BANDHU_DEFAULT_APPROVAL=false
BANDHU_APPROVAL_TIMEOUT_SECS=300
BANDHU_APPROVAL_LOG=./approval.jsonl
BANDHU_FORBIDDEN_CMDS=rm -rf,sudo,&,del /f
BANDHU_INSTALL_CMDS=apt install,apt-get install,npm install,yarn add,pnpm add,cargo install,pip install,pip3 install,uv pip install,poetry add,gem install,go install,brew install
BANDHU_FORBIDDEN_PATHS=/etc/passwd,.env
BANDHU_PROMPT_TEMPLATE=Available tools:\n{}\n\nContext:\n{}\n\nTask: {}
BANDHU_CONTEXT_TOKEN_LIMIT=8192
BANDHU_OLLAMA_TIMEOUT_SECS=120
```
