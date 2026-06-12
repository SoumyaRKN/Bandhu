# Bandhu

Personal Coding AI Agent — local-first, runs entirely on your machine using local LLMs via Ollama, with a Rust backend and a TypeScript VS Code extension.

## Features

- **Local-first** — No cloud dependencies. All code execution and LLM inference happens on your machine.
- **Approval-driven** — Every file edit and shell command requires explicit user confirmation.
- **Multi-tool loop** — The agent autonomously chains tools (read, search, write, run, list) to complete tasks.
- **Safety filter** — Dangerous commands and forbidden paths are blocked before execution.
- **Tool validation** — Tool inputs are checked against schemas and size limits before execution.
- **Incremental context** — Only relevant files are sent to the model, keeping the context window efficient.
- **Type-safe** — Rust backend with typed errors; TypeScript extension with strict typing.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     VS Code Host                         │
│  ┌───────────────────────────────────────────────────┐  │
│  │               bandhu (Extension)                   │  │
│  │  ┌────────────┐  ┌────────────┐  ┌─────────────┐  │  │
│  │  │   chatui   │  │   status   │  │   approval  │  │  │
│  │  │   panel    │  │   item     │  │   modal     │  │  │
│  │  └────────────┘  └────────────┘  └─────────────┘  │  │
│  │         │               │                │          │  │
│  │         └───────────────┴────────────────┘          │  │
│  │                    │                               │  │
│  │              api  client                          │  │
│  └────────────────────┼───────────────────────────────┘  │
└───────────────────────┼───────────────────────────────────┘
                         │ HTTP
                         ▼
┌─────────────────────────────────────────────────────────┐
│              backend (Rust / Axum)                       │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐        │
│  │   api      │  │   queue    │  │   safety   │        │
│  │   handler  │  │   engine   │  │   filter   │        │
│  └────────────┘  └────────────┘  └────────────┘        │
│         │                │                │            │
│         └────────────────┼────────────────┘           │
│                          │                             │
│  ┌───────────────────────┼─────────────────────────┐   │
│  │                       ▼                          │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐ │   │
│  │  │   tool     │  │   context  │  │   ollama   │ │   │
│  │  │   registry │  │   builder  │  │   client   │ │   │
│  │  └────────────┘  └────────────┘  └────────────┘ │   │
│  └──────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│                    Ollama Runtime                        │
│  ┌───────────────────────────────────────────────────┐  │
│  │  qwen2.5-coder:7b or qwen3.5:9b (and other local models)       │  │
│  │  http://localhost:11434                            │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

## Prerequisites

- **OS**: Ubuntu 22.04+ (or compatible Linux)
- **Node.js**: 18+ LTS
- **Rust**: 1.70+ (via rustup)
- **Ollama**: Latest (via install script)
- **ripgrep**: `sudo apt install ripgrep`
- **VS Code**: Latest stable

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/bandhu.git
cd bandhu
```

### 2. Install Ollama

```bash
curl -fsSL https://ollama.com/install.sh | sh
```

Pull a coding model:

```bash
ollama pull qwen2.5-coder:7b
```

Or

```bash
ollama pull qwen3.5:9b
```

Verify Ollama is running:

```bash
ollama --version
```

### 3. Build the Backend

```bash
cd backend
cargo build --release
```

The backend binary will be at `backend/target/release/bandhu-server`.

### 4. Install the VS Code Extension

```bash
cd bandhu
npm install
npm run compile
```

Press `F5` in VS Code to launch an Extension Development Host window with Bandhu installed.

### 5. Start the Backend

```bash
cd backend
BANDHU_SERVER_PORT=3000 cargo run --release
```

You should see:

```
Bandhu backend listening on 127.0.0.1:3000
```

## Configuration

All configuration is managed via **environment variables**. Create a `.env` file in the backend root or export variables in your shell.

### Backend Configuration (`backend/.env`)

| Variable | Default | Description |
|----------|---------|-------------|
| `BANDHU_SERVER_HOST` | `127.0.0.1` | Host address for the HTTP server |
| `BANDHU_SERVER_PORT` | `3000` | Port for the HTTP server |
| `BANDHU_OLLAMA_BASE_URL` | `http://localhost:11434` | Ollama API base URL |
| `BANDHU_OLLAMA_MODEL` | `qwen2.5-coder:7b` | Model identifier to use |
| `BANDHU_OLLAMA_STREAM` | `false` | Enable streaming responses (`true`/`false`) |
| `BANDHU_MAX_ITERATIONS` | `10` | Max tool-call loop iterations per request |
| `BANDHU_RG_MAX_COUNT` | `50` | Max ripgrep matches for context building |
| `BANDHU_SCHEMA_VALIDATE` | `true` | Validate tool inputs against registered tool schemas |
| `BANDHU_TOOL_INPUT_LIMIT` | `65536` | Max serialized JSON bytes allowed for a tool input |
| `BANDHU_DEFAULT_APPROVAL` | `false` | Auto-approve all tool calls (`true`/`false`) |
| `BANDHU_APPROVAL_TIMEOUT_SECS` | `300` | Seconds before approval prompt times out |
| `BANDHU_FORBIDDEN_CMDS` | *(empty)* | Comma-separated forbidden command patterns |
| `BANDHU_FORBIDDEN_PATHS` | *(empty)* | Comma-separated forbidden path patterns |
| `BANDHU_CONTEXT_TOKEN_LIMIT` | `8192` | Approximate model context token budget |
| `BANDHU_OLLAMA_TIMEOUT_SECS` | `120` | Max seconds to wait for a single Ollama request |

**Example `.env` file:**

```env
BANDHU_SERVER_HOST=0.0.0.0
BANDHU_SERVER_PORT=3000
BANDHU_OLLAMA_BASE_URL=http://localhost:11434
BANDHU_OLLAMA_MODEL=qwen2.5-coder:7b
BANDHU_OLLAMA_STREAM=false
BANDHU_MAX_ITERATIONS=10
BANDHU_RG_MAX_COUNT=50
BANDHU_SCHEMA_VALIDATE=true
BANDHU_TOOL_INPUT_LIMIT=65536
BANDHU_DEFAULT_APPROVAL=false
BANDHU_APPROVAL_TIMEOUT_SECS=300
BANDHU_FORBIDDEN_CMDS=rm -rf,sudo
BANDHU_FORBIDDEN_PATHS=/etc/passwd,/etc/shadow
BANDHU_CONTEXT_TOKEN_LIMIT=8192
BANDHU_OLLAMA_TIMEOUT_SECS=120
```

Load with:

```bash
source .env
cargo run --release
```

Or use `dotenvy` (already a dependency):

```bash
cargo run --release  # automatically loads .env if present
```

### Extension Configuration (`bandhu/.env`)

Set these variables in your shell before launching VS Code, or load `bandhu/.env` before starting the extension development host.

| Variable | Default | Description |
|----------|---------|-------------|
| `BANDHU_BACKEND_URL` | `http://127.0.0.1:3000` | Backend server URL |
| `BANDHU_CHAT_PLACEHOLDER` | `Ask Bandhu...` | Placeholder shown in the webview chat input |
| `BANDHU_CHAT_TIMEOUT_MS` | `120000` | Milliseconds before `/chat` requests are aborted |
| `BANDHU_CHAT_RETRIES` | `2` | Number of retry attempts for `/chat` failures after the first attempt |
| `BANDHU_CHAT_RETRY_DELAY_MS` | `500` | Milliseconds to wait between `/chat` retry attempts |
| `BANDHU_CHAT_STREAMING` | `true` | Use the backend `/chat/stream` SSE endpoint for incremental chat messages |
| `BANDHU_COMMAND_TIMEOUT_MS` | `30000` | Milliseconds before `/approve` requests are aborted |
| `BANDHU_COMMAND_RETRIES` | `1` | Number of retry attempts for `/approve` failures after the first attempt |
| `BANDHU_COMMAND_RETRY_DELAY_MS` | `500` | Milliseconds to wait between `/approve` retry attempts |
| `BANDHU_STATUS_TEXT` | `$(check) Bandhu` | Text shown in the VS Code status bar when idle |
| `BANDHU_STATUS_BUSY_TEXT` | `$(loading~spin) Bandhu` | Text shown while a chat request is running |
| `BANDHU_STATUS_ERROR_TEXT` | `$(error) Bandhu` | Text shown after a chat request fails |
| `BANDHU_STATUS_TOOLTIP` | `Ready` | Tooltip shown when idle |
| `BANDHU_STATUS_BUSY_TOOLTIP` | `Working` | Tooltip shown while a chat request is running |
| `BANDHU_STATUS_ERROR_TOOLTIP` | `Error` | Tooltip shown after a chat request fails |
| `BANDHU_DEFAULT_APPROVAL` | `false` | Auto-approve all tool calls |
| `BANDHU_APPROVAL_TIMEOUT_SECS` | `300` | Approval prompt timeout in seconds |
| `BANDHU_FORBIDDEN_CMDS` | *(empty)* | Comma-separated forbidden command patterns |
| `BANDHU_FORBIDDEN_PATHS` | *(empty)* | Comma-separated forbidden path patterns |

## Usage

1. Open a project folder in VS Code.
2. Activate Bandhu via the status bar item or command palette (`Ctrl+Shift+P` → "Bandhu: Open Chat").
3. Type your coding task in the chat panel (e.g., *"Add error handling to the login function"*).
4. Review tool actions in the approval modal.
5. Accept or reject each proposed change.
6. View the final result in the chat panel.

## Available Tools

| Tool ID | Purpose | Requires Approval |
|---------|---------|-------------------|
| `readfile` | Read file content by path | No |
| `search` | Text search via ripgrep | No |
| `writefile` | Write or replace file content | Yes |
| `runcommand` | Execute shell command | Yes |
| `listdir` | List directory entries | No |

## Project Structure

```
bandhu/                  VS Code extension (TypeScript)
  src/
    extension.ts         entry point
    api.ts               HTTP client
    chatui.ts            webview panel
    status.ts            bar item
    approval.ts          modal logic
    controller.ts        lifecycle orchestrator
  test/
    extension.test.ts    test suite

backend/                 Rust backend service
  src/
    main.rs              entry, routing
    config.rs            environment configuration
    queue.rs             tool-call loop controller
    tool.rs              tool trait definition
    registry.rs          tool registry
    readfile.rs          file read tool
    search.rs            ripgrep search tool
    writefile.rs         file write tool
    runcommand.rs        shell command tool
    listdir.rs           directory listing tool
    gate.rs              safety filter
  Cargo.toml             dependencies

docs/                    design and user documentation
scripts/                 build and setup scripts
experiments/             prototyping and benchmarks
```

## Development

### Backend

```bash
cd backend
cargo check          # type check
cargo clippy         # lint
cargo test           # run tests
cargo fmt            # format
```

### Extension

```bash
cd bandhu
npm run lint         # lint
npm run test         # run tests
```

## Safety

- All `writefile` and `runcommand` executions require explicit approval.
- The safety filter blocks known dangerous patterns (`rm -rf`, `sudo`, etc.) and path traversal attempts.
- Approval decisions are logged for audit.

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Backend won't start | Check if port 3000 is already in use. Set `BANDHU_SERVER_PORT` to a different value. |
| Ollama connection refused | Ensure Ollama is running: `ollama serve`. Verify with `curl http://localhost:11434/api/tags`. |
| Extension not activating | Open the Extension Development Host window with `F5`. Check the Debug Console for errors. |
| Tool approval modal not showing | Ensure the backend `/approve` endpoint is reachable from the extension. |

## License

MIT — see [LICENSE](LICENSE) for details.
