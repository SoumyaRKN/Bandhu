# Bandhu Configuration Reference

All configurable parameters are set via environment variables or a `.env` file in `backend/`.

---

## Extension Settings

| Variable | Default | Description |
|---|---|---|
| `BANDHU_BACKEND_URL` | `http://127.0.0.1:3000` | Backend HTTP endpoint for extension API calls. |

---

## Server Settings

| Variable | Default | Description |
|---|---|---|
| `BANDHU_SERVER_HOST` | `127.0.0.1` | Backend bind address. |
| `BANDHU_SERVER_PORT` | `3000` | Backend bind port. |

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

---

## Approval & Safety Settings

| Variable | Default | Description |
|---|---|---|
| `BANDHU_DEFAULT_APPROVAL` | `false` | Auto-approve all tools when `true`. Disable for production. |
| `BANDHU_APPROVAL_TIMEOUT_SECS` | `300` | Seconds before pending approval is aborted. |
| `BANDHU_FORBIDDEN_CMDS` | _(empty)_ | Comma-separated lowercase command patterns blocked by the safety filter. Example: `rm -rf,sudo,del /f` |
| `BANDHU_FORBIDDEN_PATHS` | _(empty)_ | Comma-separated path substrings blocked by the safety filter. Example: `/etc/passwd,.env` |

---

## Context Builder Settings

| Variable | Default | Description |
|---|---|---|
| `BANDHU_CONTEXT_TOP_N` | `10` | Max candidate files selected during the select stage. |
| `BANDHU_CONTEXT_MAX_FILE_BYTES` | `65536` | Max file size in bytes before truncation during the read stage. |

---

## Prompt Template

| Variable | Default | Description |
|---|---|---|
| `BANDHU_PROMPT_TEMPLATE` | _(built-in)_ | Python-style format string for the tool loop prompt. Use `{}` placeholders for tools list, context, and task. |

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
BANDHU_OLLAMA_BASE_URL=http://localhost:11434
BANDHU_OLLAMA_MODEL=qwen2.5-coder:7b
BANDHU_OLLAMA_STREAM=false
BANDHU_MAX_ITERATIONS=10
BANDHU_RG_MAX_COUNT=50
BANDHU_DEFAULT_APPROVAL=false
BANDHU_APPROVAL_TIMEOUT_SECS=300
BANDHU_FORBIDDEN_CMDS=rm -rf,sudo,del /f
BANDHU_FORBIDDEN_PATHS=/etc/passwd,.env
BANDHU_PROMPT_TEMPLATE=Available tools:\n{}\n\nContext:\n{}\n\nTask: {}
```
