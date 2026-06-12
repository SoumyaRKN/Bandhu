# Bandhu

Personal Coding AI Agent — local-first, runs entirely on your machine using local LLMs via Ollama, with a Rust backend and a TypeScript VS Code extension.

## Product Overview

Bandhu is an AI-powered coding assistant that provides autonomous code modification capabilities through a VS Code extension interface. Unlike cloud-based solutions, Bandhu operates entirely on your local machine, leveraging Ollama for local LLM inference. The agent can:

- Read and understand your codebase through intelligent context selection
- Search code using ripgrep integration
- Make file modifications with user approval
- Execute build and test commands to verify changes
- Run shell commands within safety boundaries

All operations follow an approval-driven model, ensuring explicit user confirmation before any file modification or command execution.

## Architecture & Workflow

### System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     VS Code Host                         │
│  ┌───────────────────────────────────────────────────┐  │
│  │               bandhu (Extension)                   │  │
│  │  ┌────────────┐  ┌────────────┐  ┌─────────────┐  │  │
│  │  │   chatui   │  │   status   │  │   report    │  │  │
│  │  │   panel    │  │   item     │  │   output    │  │  │
│  │  └────────────┘  └────────────┘  └─────────────┘  │  │
│  │         │               │               │          │  │
│  │         └───────────────┴───────────────┘          │  │
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
│         │                ▼                │            │
│         │  ┌───────────────────────┐      │            │
│         │  │  Tool Call Loop       │      │            │
│         │  │  (iterative tool use) │      │            │
│         │  └───────────────────────┘      │            │
│         │                │                │            │
│         ▼                ▼                ▼            │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐        │
│  │   tool     │  │   context  │  │   ollama   │        │
│  │   registry │  │   builder  │  │   client   │        │
│  └────────────┘  └────────────┘  └────────────┘        │
└─────────────────────────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────┐
│                    Ollama Runtime                        │
│  http://localhost:11434                                 │
└─────────────────────────────────────────────────────────┘
```

### Tool Call Loop Workflow

1. User submits prompt in VS Code chat panel
2. Backend builds context using keywords extracted from the task
3. Loop iterates (max 10 by default):
   - Model called with prompt + context + available tools
   - If model returns tool call JSON:
     - Input validated against schema
     - Safety filter checks command/path against forbidden patterns
     - If approval required, emit `tool_approval` message
     - User approves/rejects (or `BANDHU_DEFAULT_APPROVAL=true` auto-approves)
     - Tool executes and result appended to context
     - If file written: trigger build loop
     - If build passed: trigger test loop
   - If model returns final answer: loop terminates
4. Stream results back to extension via SSE

### Module Interaction

- **Extension** ↔ **Backend**: HTTP POST requests with JSON payloads
- **Backend** ↔ **Ollama**: HTTP requests to local LLM API
- **Queue** → **Tools**: Dispatch execution via registry
- **Queue** → **Context**: Build context at loop start
- **Loop** → **BuildLoop/TestLoop**: Trigger after file writes

## Installation & Setup

### Prerequisites

- **OS**: Ubuntu 22.04+ (or compatible Linux)
- **Node.js**: 18+ LTS
- **Rust**: 1.70+ (via rustup)
- **Ollama**: Latest version
- **ripgrep**: `sudo apt install ripgrep`
- **VS Code**: Latest stable

### Step-by-Step Installation

#### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/bandhu.git
cd bandhu
```

#### 2. Install Ollama

```bash
curl -fsSL https://ollama.com/install.sh | sh
```

Pull a coding model:

```bash
ollama pull qwen2.5-coder:7b
# or
ollama pull qwen3.5:9b
```

Verify Ollama is running:

```bash
ollama --version
curl http://localhost:11434/api/tags
```

#### 3. Build the Backend

```bash
cd backend
cargo build --release
```

The backend binary will be at `backend/target/release/bandhu-backend`.

#### 4. Install the Extension

```bash
cd bandhu
npm install
npm run compile
```

#### 5. Start the Backend

```bash
# From project root
BANDHU_SERVER_PORT=3000 cargo run --release
# Or create a .env file in backend/
```

You should see: `Bandhu backend listening on 127.0.0.1:3000`

#### 6. Launch Extension Development Host

In VS Code: Press `F5` to launch a window with Bandhu installed.

## Usage Guide

### Starting Bandhu

1. Open a project folder in VS Code
2. Activate Bandhu via:
   - Status bar item (left side, shows `$(check) Bandhu` when idle)
   - Command palette: `Ctrl+Shift+P` → "Bandhu: Open Chat"
3. The chat panel appears with an input field

### Basic Interaction

Type your coding task in natural language:

```
Add error handling to the login function
```

```
Find all usages of the Config struct
```

```
Refactor this function to use async/await
```

### Tool Approval Flow

When Bandhu proposes a file edit or command:

1. A `tool_approval` message appears in chat
2. For `writefile`: A diff preview shows additions (`-`) and removals (`+`)
3. Click **Approve** to execute or **Reject** to cancel
4. Results appear inline or in the output channel

### Available Tools

| Tool | Purpose | Requires Approval |
| ---- | ------- | ----------------- |
| `readfile` | Read file content by path | No |
| `search` | Text search via ripgrep | No |
| `writefile` | Write or replace file content | Yes |
| `applypatch` | Apply unified diff patch | Yes |
| `runcommand` | Execute shell command | Yes |
| `buildtool` | Run configured build command | Yes |
| `testrunner` | Run configured test command | Yes |
| `listdir` | List directory entries | No |

### Configuration

Create `.env` files in `backend/` and set environment variables:

| Variable | Default | Description |
| -------- | ------- | ----------- |
| `BANDHU_OLLAMA_MODEL` | `qwen2.5-coder:7b` | LLM model to use |
| `BANDHU_MAX_ITERATIONS` | `10` | Tool loop iterations |
| `BANDHU_BUILD_COMMAND` | `cargo build` | Build command |
| `BANDHU_TEST_COMMAND` | `cargo test` | Test command |
| `BANDHU_FORBIDDEN_CMDS` | `rm -rf,sudo,&` | Blocked commands |

See [Configuration Reference](./docs/config.md) for full settings.

### Example Session

```
User: Add a new function to calculate fibonacci

→ Bandhu searches for similar patterns
→ Bandhu reads relevant files
→ Bandhu proposes writefile tool
→ User approves
→ Build runs automatically
→ Test runs automatically
→ Bandhu: Function added successfully to src/math.rs
```

## Documentation

- [Architecture Overview](./ARCHITECTURE.md)
- [Backend Service](./docs/backend.md)
- [VS Code Extension](./docs/extension.md)
- [Tool Modules](./docs/tools-modules.md)
- [Tool Reference](./docs/tools.md)
- [Configuration Reference](./docs/config.md)
- [Build Loop](./docs/buildloop.md)
- [Approval Flow](./docs/approval.md)
- [Diff Workflow](./docs/diff.md)
- [Result Reporting](./docs/report.md)

## Development

### Backend

```bash
cd backend
cargo check      # Type check
cargo clippy     # Lint
cargo test       # Run tests
cargo fmt        # Format
```

### Extension

```bash
cd bandhu
npm run lint     # Lint
npm run check-types  # Type check
npm run compile  # Build
```

## License

MIT — see [LICENSE](LICENSE) for details.
