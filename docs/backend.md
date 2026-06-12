# Backend Service

The Bandhu backend is a Rust-based HTTP service that orchestrates the tool-call loop and LLM interactions.

## Overview

- **Language**: Rust (edition 2021)
- **Framework**: Axum web framework
- **Runtime**: Tokio async runtime
- **Entry Point**: `backend/src/main.rs`

## Core Modules

### `main.rs` - Entry Point & Routing

The main module handles:
- HTTP server initialization with CORS configuration
- Route registration (`/health`, `/chat`, `/chat/stream`, `/call`, `/context`, `/approve`)
- Tool registry initialization and state management
- Shared application state (registry, gate, pending approvals)

### `config.rs` - Configuration Management

Centralizes all configuration via environment variables using the `dotenvy` crate:
- Network settings (`BANDHU_SERVER_HOST`, `BANDHU_SERVER_PORT`)
- Ollama settings (`BANDHU_OLLAMA_BASE_URL`, `BANDHU_OLLAMA_MODEL`, `BANDHU_OLLAMA_STREAM`)
- Tool loop limits (`BANDHU_MAX_ITERATIONS`, `BANDHU_TOOL_INPUT_LIMIT`)
- Safety settings (`BANDHU_FORBIDDEN_CMDS`, `BANDHU_FORBIDDEN_PATHS`, `BANDHU_DEFAULT_APPROVAL`)
- Build/test configuration (`BANDHU_BUILD_COMMAND`, `BANDHU_TEST_COMMAND`, etc.)

### `queue.rs` - Tool Call Loop Engine

Orchestrates the conversation loop:
- Builds prompts with tools list, context, and task
- Calls the Ollama model for responses
- Parses tool calls from model output
- Validates inputs and checks safety filters
- Manages approval requests and user decisions
- Triggers build/test loops after file modifications

### `tool.rs` - Tool Trait Definition

Defines the core `Tool` trait:
- `id()` - Single-word unique identifier
- `name()` - Human-readable label
- `desc()` - Description for model prompts
- `schema()` - JSON schema for validation
- `execute()` - Runs the tool, returns JSON result
- `requires()` - Whether approval is needed

### `registry.rs` - Tool Registry

Manages tool registration and lookup:
- Thread-safe `HashMap` storage for tool implementations
- Schema validation dispatch
- Prevents duplicate registrations

## Endpoints

| Method | Path | Purpose |
| ------ | ---- | ------- |
| GET | `/health` | Liveness check |
| POST | `/chat` | Full prompt/response cycle |
| POST | `/chat/stream` | SSE streaming response |
| POST | `/call` | Direct tool execution |
| POST | `/context` | Context building for task |
| POST | `/approve` | Approval decision handling |

## Error Types (`error.rs`)

- `Config` - Configuration parsing errors
- `Io` - File and process I/O errors
- `Tool` - Tool validation/execution errors
- `Gate` - Safety filter rejections
- `Model` - Ollama-specific errors
- `Http` - HTTP transport errors
- `Parse` - JSON parsing errors
- `Timeout` - Command timeout errors

## Data Flow

```
Request → ContextBuilder → Loop → Model.call() → Parse Tool Call → Safety Check → Approval → Execute Tool → Result
```

## Build & Run

```bash
# Build
cd backend
cargo build --release

# Run (with optional .env)
cargo run --release
```

The binary `bandhu-server` listens on `127.0.0.1:3000` by default.