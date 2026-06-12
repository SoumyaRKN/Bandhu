# Bandhu Architecture

## System Overview

Bandhu is a local-first VS Code coding AI agent. The system runs entirely on the user machine, using local LLMs via Ollama, with a Rust backend providing tool execution and a TypeScript VS Code extension providing the user interface.

## Core Principles

- **Local-first**: No cloud dependencies for core functionality
- **Approval-driven**: All file edits and command executions require explicit user confirmation
- **Single-word naming**: All files, folders, functions, variables, types, and identifiers use single-word naming convention
- **Incremental context**: Only relevant context is sent to the model, never the entire repository

---

## Component Architecture

### High-Level Topology

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
                        │
                        ▼
              (System / Process execution)
```

---

## Backend Architecture

### module map

| module     | purpose                                      |
|------------|----------------------------------------------|
| queue      | manages conversation and tool-call loop      |
| tool       | base trait and tool implementations          |
| context    | selects relevant files for the model         |
| safety     | command filtering and approval enforcement   |
| api        | HTTP endpoints and request routing           |
| model      | Ollama client abstraction                    |
| diff       | unified diff generation and application      |
| applypatch | patch application tool                     |

### type definitions

```
request  — generic inbound request container
response — generic outbound response container
result   — ok/err enum used across all operations
```

### api endpoints

| method | path           | purpose                                                    |
|--------|----------------|------------------------------------------------------------|
| post   | /health        | backend liveness check                                     |
| post   | /chat          | accept prompt, run loop, return response and message array |
| post   | /chat/stream   | accept prompt, stream message array events as SSE          |
| post   | /call          | execute a single tool                                      |
| post   | /context       | build context for a given task description                 |

### data flow: /chat endpoint

1. extension sends `request { task, context }`
2. `queue` builds initial context via `context` builder
3. `queue` starts loop:
   a. send prompt + context to `model` client
   b. parse response for tool call
   c. if tool call present, ask `safety` filter
   d. if safe, execute via `tool` registry
   e. collect output, append to context
   f. repeat until model produces final answer
4. return compatibility response text plus accumulated message array to extension
5. `/chat/stream` emits each accumulated message as an SSE event while the same loop is running

---

## Extension Architecture

### module map

| module      | purpose                                       |
|-------------|-----------------------------------------------|
| api         | HTTP client for backend communication         |
| chatui      | webview panel for conversation display        |
| status      | status bar item showing agent state           |
| approval    | quick-pick modal for tool approval            |
| controller  | orchestrates extension lifecycle             |

### Activation Sequence

1. extension activated → create `status` bar item
2. user invokes command → open `chatui` webview
3. webview sends messages to `api` module
4. `api` forwards to backend `/chat`
5. returned messages rendered in `chatui`
6. tool approval message detected → approval controls shown
7. user accepts/rejects → response sent back to backend
8. final result displayed in `chatui`

---

## Tool System

### tool trait

```
trait:
  id       — single-word unique identifier
  name     — human-readable label
  desc     — one-line description for model prompt
  schema   — JSON schema for input validation
  execute  — runs the tool, returns tool output
  requires — approval requirement flag
```

### built-in tools

| id         | purpose                        | requires approval |
|------------|--------------------------------|-------------------|
| readfile   | read file content by path      | no                |
| search     | text search via ripgrep        | no                |
| writefile  | write or replace file content  | yes               |
| applypatch | apply unified diff patch       | yes               |
| runcommand | execute shell command          | yes               |
| listdir    | list directory entries         | no                |

### Future Tool Addition

The `tool` registry maps tool `id` to boxed trait objects. New tools are added by implementing the trait and registering in the registry map. No core loop changes required.

---

## Design Patterns

### pattern: Strategy (tool selection)

The model selects which tool to invoke. The backend does not hardcode tool dispatch. A registry maps string identifiers to tool implementations.

### pattern: Pipeline (context building)

Context flows through stages: `search` → `select` → `summarize` → `pack`. Each stage transforms the context set. Stages are composable and replaceable.

### pattern: Circuit Breaker (safety filter)

Before any `runcommand` or `writefile` execution, the `safety` filter checks against forbidden patterns (`rm -rf`, `sudo`, background operators). Package install commands are matched against configurable install patterns and tagged for explicit install approval. Blocked commands short-circuit the loop with a rejection notification.

### pattern: Approval Gate (user confirmation)

Tools flagged `requires = true` trigger the approval gate. Execution halts until the extension returns accept or reject. Rejection produces a rejection message injected back into the model context.

### pattern: Diff-Patch (writefile execution)

`writefile` does not overwrite directly. It produces a unified diff patch, shows it in the `approval` modal, and only applies patch after acceptance.

---

## Execution Strategies

### queue / tool-call loop

```
loop {
  prompt = build_prompt(context, question)
  model_output = call_ollama(prompt)
  parsed = parse_tool_call(model_output)
  
  if parsed.is_final_answer {
    return parsed.content
  }
  
  if parsed.is_tool_call {
    tool = registry.get(parsed.tool_id)
    approval = safety.check(tool, parsed.args)
    
    if approval.required {
      decision = wait_for_user_approval(approval.view)
      if decision.rejected {
        context.add("tool", tool.id, "rejected by user")
        continue
      }
    }
    
    output = tool.execute(parsed.args)
    context.add("tool", tool.id, output)
  }
  
  if iterations > max_iterations {
    return "max iterations reached"
  }
}
```

### context builder strategy

1. parse task description for keywords (files, symbols, imports)
2. run `search` with extracted keywords to find candidate files
3. rank candidates by match score and file size
4. read top N files (configurable limit)
5. extract symbol definitions and dependency edges
6. serialize into model context (file_path + content blocks)

Goal: keep context under 8k tokens.

### api client strategy (extension side)

- use `fetch` with timeout
- retry on network error (max 2 retries)
- stream SSE from `/chat/stream` when `BANDHU_CHAT_STREAMING=true`
- fall back to `/chat` response when streaming is disabled
- queue multiple requests; drop stale ones on new user input

### ollama strategy

- endpoint: `http://localhost:11434`
- default model: `qwen2.5-coder:7b`
- non-streaming for `/api/generate`
- streaming for `/api/chat` when implemented
- timeout: 120 seconds per request
- failures are returned as typed backend errors for timeout, connection, status, and parse cases

---

## Directory Layout

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
    diff.rs              diff generation module
    applypatch.rs        patch application tool
  cargo.toml             dependencies

docs/                    design and user documentation
scripts/                 build and setup scripts
experiments/             prototyping and benchmarks
```

---

## Dependency Summary

### Rust (backend)

| crate            | purpose                       |
|------------------|-------------------------------|
| axum             | HTTP server and routing       |
| tokio            | async runtime                 |
| reqwest          | HTTP client for Ollama        |
| serde/serde_json | serialization               |
| tower            | middleware                    |

### TypeScript (extension)

| package              | purpose                        |
|----------------------|--------------------------------|
| vscode               | VS Code extensibility API      |
| @types/vscode        | TypeScript bindings            |
| typescript-eslint     | linting                       |
| esbuild              | bundling                       |

---

## Open Work

| area                | status  |
| --------------------- | -------------- |
| extension scaffold    | done           |
| backend scaffold      | done           |
| ollama connection     | done           |
| tool trait + registry | done           |
| context builder       | done           |
| tool-call loop        | done           |
| safety filter         | done           |
| approval modal        | done           |
| diff approval         | done           |
| test loop             | todo           |
| P0 compile fixes      | done           |
