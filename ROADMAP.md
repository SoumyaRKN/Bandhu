# Bandhu - Personal Coding AI Agent Roadmap

## Project Overview

Build a local-first VS Code coding AI agent (Bandhu) that runs mostly free, works on Ubuntu, uses local models, reads and edits project files, executes tasks with approval, and scales gradually.

---

## Phase 0: Understand The Target

| # | Task | Description | Status |
|---|------|-------------|--------|
| 1 | Define Scope | Document what Bandhu is and is not built for | `Completed` |
| 2 | Set Vision | Confirm Bandhu is a personal coding assistant integrated into VS Code, not a full Cursor clone or cloud platform | `Completed` |
| 3 | Document Constraints | List known constraints: local-first, free, Ubuntu, local models | `Completed` |

---

## Phase 1: Prepare Environment

| # | Task | Description | Status |
|---|------|-------------|--------|
| 4 | Update Ubuntu | Run `sudo apt update && sudo apt upgrade` | `Completed` |
| 5 | Install Git | Run `sudo apt install git` and verify installation | `Completed` |
| 6 | Install NodeJS | Install LTS version via apt or nvm, verify with `node -v && npm -v` | `Completed` |
| 7 | Install Rust | Run rustup install script, verify with `rustc --version && cargo --version` | `Completed` |
| 8 | Install VS Code | Install official VS Code .deb package, verify with `code .` command | `Completed` |
| 9 | Install ripgrep | Run `sudo apt install ripgrep` for fast text search | `Completed` |
| 10 | Install Python | Install Python 3 for scripts and tooling (if not already present) | `Completed` |

---

## Phase 2: Install Local Model Runtime

| # | Task | Description | Status |
|---|------|-------------|--------|
| 11 | Install Ollama | Run `curl -fsSL https://ollama.com/install.sh \| sh` | `Completed` |
| 12 | Verify Ollama | Run `ollama --version` to confirm installation | `Completed` |
| 13 | Pull Coding Model | Run `ollama pull qwen2.5-coder:7b` | `Completed` |
| 14 | Test Ollama | Run `ollama run qwen2.5-coder:7b` and verify model responds | `Completed` |
| 13 | Pull 2nd Coding Model | Run `ollama pull or qwen3.5:9b` | `Completed` |
| 14 | Test 2nd Ollama | Run `ollama run or qwen3.5:9b` and verify model responds | `Completed` |
| 15 | Configure Ollama | Set Ollama to allow localhost connections, configure host and port if needed | `Completed` |

---

## Phase 3: Create Project Structure

| # | Task | Description | Status |
|---|------|-------------|--------|
| 16 | Make Directory - 'bandhu' | Create main project directory at root | `Completed` |
| 17 | Make Directory - 'backend' | Create Rust backend service directory | `Completed` |
| 18 | Make Directory - 'docs' | Create documentation directory | `Completed` |
| 19 | Make Directory - 'scripts' | Create utility scripts directory | `Completed` |
| 20 | Make Directory - 'experiments' | Create experiments and prototyping directory | `Completed` |
| 21 | Initialize Git Repo | Run `git init` in project root, configure basic git settings | `Completed` |
| 22 | Create .gitignore | Create `.gitignore` file excluding `node_modules`, `.vscode`, `target`, `.ollama`, `__pycache__`, `.env`, etc | `Completed` |
| 23 | Write README.md | Document project overview, setup steps, and usage | `Completed` |
| 24 | Create LICENSE | Add MIT or appropriate open source license file | `Completed` |

---

## Phase 4: Build VS Code Extension

| # | Task | Description | Status |
|---|------|-------------|--------|
| 25 | Install Yeoman | Run `npm install -g yo generator-code` | `Completed` |
| 26 | Generate Extension Scaffold | Run `yo code` selecting TypeScript and New Extension | `Completed` |
| 27 | Configure Extension | Update `package.json` with correct name (`bandhu`), publisher, version, and description | `Completed` |
| 28 | Install Dependencies | Run `npm install` in extension directory | `Completed` |
| 29 | Configure Extension Manifest | Update `extension.ts` or main entry point with basic activation logic | `Completed` |
| 30 | Add Launch Configuration | Ensure `.vscode/launch.json` is properly configured for debugging | `Completed` |
| 31 | Test Extension Launch | Press F5 to launch new Extension Development Host window | `Pending` |
| 32 | Add Status Bar Indicator | Add status bar item showing "Bandhu" when extension is active | `Completed` |
| 33 | Verify Extension Loads | Confirm extension activates in new VS Code window without errors | `Pending` |

---

## Phase 5: Create Backend Service

| # | Task | Description | Status |
|---|------|-------------|--------|
| 34 | Initialize Rust Project | Navigate to backend directory and run `cargo init` | `Completed` |
| 35 | Define Cargo Dependencies | Add required dependencies to `Cargo.toml` (axum, serde, tokio, reqwest, etc) | `Completed` |
| 36 | Define API Structure | Design API endpoints for: receive task, accept prompt, return response, execute tool | `Completed` |
| 37 | Define Task Models | Create data structures for: Task, TaskRequest, TaskResponse, ToolCall | `Completed` |
| 38 | Define Error Types | Create custom error types (`BackendError` enum) for backend service | `Completed` |
| 39 | Create API Server | Implement HTTP server using Axum framework | `Completed` |
| 40 | Add CORS Middleware | Configure CORS in `main.rs` to allow VS Code extension origin | `Completed` |
| 41 | Add Health Check Endpoint | Create `/health` endpoint for monitoring backend status | `Completed` |

---

## Phase 6: Connect To Ollama

| # | Task | Description | Status |
|---|------|-------------|--------|
| 42 | Define Ollama Client | Create `backend/src/model.rs` with `OllamaClient` struct for communicating with Ollama API | `Completed` |
| 43 | Extract Model from Queue | Move `Model` struct and Ollama request/response types from `queue.rs` into `model.rs` | `Completed` |
| 44 | Implement Generate Endpoint | Create `POST /api/generate` endpoint that accepts prompt and sends to Ollama | `Completed` (inside `/chat`) |
| 45 | Implement Chat Endpoint | Create `POST /api/chat` endpoint for multi-turn conversations | `Completed` |
| 46 | Add Request Timeout | Configure timeout for Ollama API requests in `reqwest::Client::builder()` | `Completed` |
| 47 | Test End-to-End Flow | Verify: Extension → Backend → Ollama → Backend → Extension works end-to-end | `Pending` |
| 48 | Add Error Handling | Handle Ollama connection errors, model not found, and timeout scenarios with typed errors | `Completed` |

---

## Phase 7: Implement Tools

| # | Task | Description | Status |
|---|------|-------------|--------|
| 49 | Define Tool Trait | Create base `Tool` trait with `id`, `name`, `desc`, `schema`, `execute`, `requires` method | `Completed` |
| 50 | Implement ReadFile Tool | Create `Readfile` tool: takes path, returns file content | `Completed` |
| 51 | Implement Search Tool | Create `Search` tool: uses ripgrep to search text patterns | `Completed` |
| 52 | Implement WriteFile Tool | Create `Writefile` tool: writes content to file (requires approval) | `Completed` |
| 53 | Implement RunCommand Tool | Create `Runcommand` tool: executes shell commands (requires approval) | `Completed` |
| 54 | Implement ListDir Tool | Create `Listdir` tool: list directory entries | `Completed` |
| 55 | Add Tool Registry | Create tool registry to map tool IDs to implementations | `Completed` |
| 56 | Add Tool Validation | Implement JSON Schema validation for tool inputs before execution | `Completed` |
| 57 | Document Tool APIs | Write documentation for each tool's input/output format | `Completed` |

---

## Phase 8: Add Tool Calling Loop

| # | Task | Description | Status |
|---|------|-------------|--------|
| 58 | Design Tool Loop | Implement main loop: ask model → parse tool call → execute tool → return result → repeat | `Completed` |
| 59 | Implement Loop Controller | Create controller that manages the tool call loop state | `Completed` |
| 60 | Add Tool Selection Prompt | Configure system prompt to instruct model on available tools and how to call them | `Completed` |
| 61 | Parse Tool Responses | Implement parser for model tool call responses (JSON format) | `Completed` |
| 62 | Handle Tool Errors | Implement error handling for failed tool executions within the loop | `Completed` |
| 63 | Implement Loop Termination | Define conditions to break the loop (task completed, error encountered, max iterations) | `Completed` |
| 64 | Test Simple Tool Chain | Verify a simple sequence: ReadFile → Search → respond works correctly | `Completed` |
| 65 | Add Loop Logging | Add structured logging for each iteration of the tool loop | `Completed` |

---

## Phase 9: Add Safety

| # | Task | Description | Status |
|---|------|-------------|--------|
| 66 | Define Forbidden Commands | Maintain list of forbidden commands: `rm -rf /`, `sudo`, background execution | `Completed` |
| 67 | Implement Command Filter | Add filter to block execution of dangerous commands in `gate.rs` | `Completed` |
| 68 | Add File Edit Confirmation | Require user approval before WriteFile tool is executed | `Completed` |
| 69 | Add Command Confirmation | Require user approval before RunCommand tool is executed | `Completed` |
| 70 | Add Package Install Confirmation | Require confirmation before any package installation commands | `Completed` |
| 71 | Implement Confirmation UI | Create VS Code webview or quick pick for showing tool actions and requesting confirmation | `Completed` |
| 72 | Add Approval Logging | Log all approved and rejected tool executions for audit trail | `Completed` |
| 73 | Test Safety Mechanisms | Manually verify dangerous commands are blocked and approvals work end-to-end | `Completed` |

---

## Phase 10: Improve Accuracy (Context Builder)

| # | Task | Description | Status |
|---|------|-------------|--------|
| 74 | Create Context Module | Create `backend/src/context.rs` module for context building pipeline | `Completed` |
| 75 | Implement Search Stage | Extract relevant files by searching for keywords from task description | `Completed` |
| 76 | Implement Select Stage | Rank candidate files by match score and file size | `Completed` |
| 77 | Implement Summarize Stage | Generate summaries of large files for context | `Completed` (truncation placeholder replaces summaries in initial implementation) |
| 78 | Implement Pack Stage | Serialize selected context into model-readable blocks (path + content) | `Completed` |
| 79 | Wire Context to Loop | Call context builder at start of each loop iteration in `queue.rs` | `Completed` |
| 80 | Add Context Size Limit | Enforce ~8k token limit on context window | `Completed` (via `BANDHU_CONTEXT_MAX_FILE_BYTES` and `BANDHU_CONTEXT_TOP_N`) |
| 81 | Test Context Accuracy | Verify agent can solve tasks using context-only approach vs full repo | `Pending` |

---

## Phase 11: Add Diff Approval

| # | Task | Description | Status |
|---|------|-------------|--------|
| 82 | Implement Diff Generator | Create tool to generate unified diff patch from proposed file changes | `Completed` |
| 83 | Modify WriteFile Tool | Change `Writefile` to produce diff patch instead of direct write | `Completed` |
| 84 | Create Diff View UI | Implement VS Code webview panel to display unified diffs | `Completed` |
| 85 | Add Apply Patch Tool | Create backend tool to apply diff patch after user approval | `Completed` |
| 86 | Add Reject Workflow | Implement workflow to discard changes when user rejects diff | `Completed` |
| 87 | Add Line-by-Line Approval | Support approving/rejecting individual change hunks within a diff | `Pending` |
| 88 | Add Edit Suggestions | Allow user to manually edit proposed changes before applying | `Pending` |
| 89 | Test Diff Workflow | End-to-end test: model proposes change → diff shown → user approves → changes applied | `Pending` |
| 90 | Document Diff System | Write documentation for the diff approval workflow | `Completed` |

---

## Phase 12: Extension Approval Integration

| # | Task | Description | Status |
|---|------|-------------|--------|
| 91 | Fix Approval Flow in Controller | Wire `handleMessage` to webview; send `tool_approval` messages to webview panel | `Completed` |
| 92 | Add Webview Approval JS | Add JavaScript in webview HTML to receive approval messages and show UI buttons | `Completed` |
| 93 | Replace InputBox with Chat UI | Replace `vscode.window.showInputBox` with proper webview chat input | `Completed` |
| 94 | Stream Responses Backend | Add SSE streaming support to `/chat` endpoint | `Completed` |
| 95 | Stream Responses Extension | Update extension to consume SSE stream and render tokens incrementally | `Completed` |
| 96 | Add Request Cancellation | Support cancelling in-flight requests when user sends new input | `Pending` |
| 143 | Add Fetch Timeout And Retry | Configure extension `/chat` and `/approve` calls with environment-controlled timeout, retry count, and retry delay | `Completed` |

---

## Phase 13: Add Testing Loop

| # | Task | Description | Status |
|---|------|-------------|--------|
| 97 | Implement Build Tool | Create tool to run project build commands (cargo build, npm run build, etc) | `Pending` |
| 98 | Implement Test Runner | Create tool to run project tests and capture output | `Pending` |
| 99 | Add Build Loop | Implement loop that runs build after code edits to verify compilation | `Pending` |
| 100 | Add Test Loop | Implement loop that runs tests after build succeeds | `Pending` |
| 101 | Implement Fix Loop | Implement loop: build fails → send errors to model → fix → rebuild until success | `Pending` |
| 102 | Parse Test Output | Create parser to extract test failures and error messages for the model | `Pending` |
| 103 | Add Test Result Reporting | Display build and test results to user in VS Code output channel | `Pending` |
| 104 | Test Full Testing Loop | Verify complete loop: edit → build → test → fix works for sample project | `Pending` |

---

## Phase 14: Improve Speed

| # | Task | Description | Status |
|---|------|-------------|--------|
| 105 | Profile Context Generation | Measure time spent on context generation and identify bottlenecks | `Pending` |
| 106 | Optimize File Reads | Implement batch file reads and caching for repeated access | `Pending` |
| 107 | Reduce Context Size | Implement strategies to reduce context size (summaries, selective inclusion) | `Pending` |
| 108 | Optimize Model Calls | Reduce number of model calls via better tool selection and batched requests | `Pending` |
| 109 | Add Response Caching | Cache common query responses to avoid redundant model calls | `Pending` |
| 110 | Implement Parallel Tool Execution | Add support for parallel tool execution in the loop when tools are independent | `Pending` |
| 111 | Profile End-to-End Performance | Measure and document typical task completion times | `Pending` |
| 112 | Document Optimizations | Write performance optimization guidelines and best practices | `Pending` |

---

## Phase 15: Future Improvements

| # | Task | Description | Status |
|---|------|-------------|--------|
| 113 | Add Git Integration | Implement tool for git operations: status, diff, log, blame | `Pending` |
| 114 | Implement Planning System | Add planning module for complex multi-step tasks | `Pending` |
| 115 | Add Memory System | Implement short-term and long-term memory for conversation context | `Pending` |
| 116 | Add Multi-Project Support | Enable agent to work across multiple related projects | `Pending` |
| 117 | Add Embeddings | Implement embeddings for semantic code search | `Pending` |
| 118 | Integrate Vector DB | Set up local vector database for codebase embeddings and semantic search | `Pending` |
| 119 | Add Multi-Model Support | Support switching between different local models | `Pending` |

---

## General / Cross-Phase Tasks

| # | Task | Description | Status |
|---|------|-------------|--------|
| 120 | Rename Files | Apply single-word naming convention to all files and directories | `Pending` |
| 121 | Rename Functions | Apply single-word naming convention to all functions and methods | `Pending` |
| 122 | Rename Variables | Apply single-word naming convention to all variables | `Pending` |
| 123 | Run Formatter | Configure and run formatter (rustfmt for Rust, prettier for TypeScript/JS) on all code | `Pending` |
| 124 | Run Linter | Configure and run linter (clippy for Rust, eslint for TypeScript/JS) and fix issues | `Pending` |
| 125 | Type Check | Run type checker (tsc for TypeScript, cargo check for Rust) and fix type errors | `Pending` |
| 126 | Format Markdown | Format all markdown files (`README.md`, `ARCHITECTURE.md`, `ROADMAP.md`, docs) with consistent style | `Pending` |
| 127 | Update README | Keep README.md up-to-date with setup instructions, usage examples, and feature status | `Pending` |
| 128 | Write API Docs | Document all API endpoints, request/response formats, and error codes | `Pending` |
| 129 | Write Tool Docs | Document all implemented tools with examples and usage guidelines | `Pending` |
| 130 | Write User Guide | Write comprehensive user guide for installing, configuring, and using Bandhu | `Pending` |
| 131 | Write Developer Guide | Write developer guide for contributing, building, testing, and extending Bandhu | `Pending` |
| 132 | Add Code Comments | Add descriptive comments to complex logic and public APIs (avoiding over-commenting) | `Pending` |
| 133 | Create CHANGELOG | Initialize CHANGELOG.md with version history and release notes | `Pending` |
| 134 | Clean Up Experiments | Review `experiments/` directory, archive or delete obsolete experiments | `Pending` |
| 135 | Clean Up Scripts | Review `scripts/` directory, remove unused scripts, document remaining ones | `Pending` |
| 136 | Clean Up Temporary Files | Remove temporary files, build artifacts, debug logs, and caches from repo | `Pending` |
| 137 | Organize Docs Structure | Organize `docs/` directory with clear subdirectories (api, tools, guides, etc) | `Pending` |
| 138 | Git Add Configuration | Verify `.gitignore` covers all build artifacts and sensitive files | `Pending` |
| 139 | Git Commit Structure | Verify commit history is clean and project is ready for version control | `Pending` |
| 140 | Add Git Hooks | Set up pre-commit hooks for formatting, linting, and type checking | `Pending` |
| 141 | Create CI Config | Set up CI pipeline (GitHub Actions or similar) for lint, type check, build, and test | `Pending` |
| 142 | Review Security | Audit code for security issues: no hardcoded secrets, safe command execution, input validation, path traversal | `Pending` |

---

## Suggested Weekly Plan

| Week | Focus Area | Key Tasks |
|------|-----------|-----------|
| 1 | Environment + Models | #4-#15 (Phase 1 + Phase 2) |
| 2 | VS Code Extension | #16-#33 (Phase 3 + Phase 4) |
| 3 | Backend + Tools | #34-#57 (Phase 5 + Phase 6 + Phase 7) |
| 4 | Tool Calling Loop | #58-#65 (Phase 8) |
| 5 | Approval System | #66-#73 (Phase 9) |
| 6 | Context + Extension Approval | #74-#90 (Phase 10 + Phase 11 + Phase 12) |
| 7 | Testing Loop | #91-#104 (Phase 13) |
| 8 | Performance + Polish | #105-#112 (Phase 14) |
| 9 | Future + Documentation | #113-#142 (Phase 15 + General Tasks) |

---

## Suggested Development Order

1. Local model works (#11-#15)
2. Extension works (#25-#33)
3. Chat UI works (Extension connects to backend)
4. Read files (#50)
5. Search (#51)
6. Edit files (#52) with approval
7. Run commands (#53) with approval
8. Diff approval (#82-#90)
9. Tests (#97-#104)
10. Better context (#74-#81)

---

## Status Key

| Value | Meaning |
|-------|---------|
| `Pending` | Not yet started |
| `In Progress` | Currently being worked on |
| `Completed` | Finished and verified |
| `Blocked` | Waiting on dependency or external factor |

---

## Outstanding Issues Requiring Immediate Attention

These are high-priority items discovered during the audit that block further development:

| Priority | Issue | Location | Action Required | Status |
|----------|-------|----------|-----------------|--------|
| P0 | `listdir.rs` compile bug — missing `Value` import | `backend/src/listdir.rs:1` | Add `use serde_json::Value;` import | `Completed` |
| P0 | `registry.rs` compile bug — missing `Value` import | `backend/src/registry.rs:2` | Add `use serde_json::Value;` import | `Completed` |
| P0 | `writefile.rs` compile bug — missing `Gate` import | `backend/src/writefile.rs:1` | Add `use crate::gate::Gate;` import | `Completed` |
| P0 | `readfile.rs` compile bug — wrong `PathBuf` import and missing `resolve` function | `backend/src/readfile.rs:3` | Fix import and use `PathBuf::from()` directly | `Completed` |
| P0 | `search.rs` tests missing `Tool` trait import in module | `backend/src/search.rs:108` | Add `use crate::tool::Tool;` in test module | `Completed` |
| P0 | `tool.rs` test using wrong `serde` types | `backend/src/tool.rs:18` | Use `serde_json::json!({})` instead of `serde::{Map, Value}` | `Completed` |
| P0 | `registry.rs` test missing `validate` implementation | `backend/src/registry.rs:85` | Add `validate` method to Dummy struct | `Completed` |
| P0 | `queue.rs` unused import | `backend/src/queue.rs:2` | Remove unused `ApprovalRequest` import | `Completed` |
| P0 | `main.rs` missing `Arc` import | `backend/src/main.rs:1` | Add `use std::sync::Arc;` import | `Completed` |
| P0 | No `README.md` at project root | Project root | Create README.md | `Completed` |
| P1 | Ollama client is inline in `queue.rs` instead of `model.rs` | `backend/src/queue.rs:219` | Extract into `backend/src/model.rs` | `Completed` |
| P1 | `writefile` overwrites directly — no diff/patch system | `backend/src/writefile.rs` | Implement diff generation and patch application | `Completed` |
| P1 | No `BackendError` enum — errors are bare `String`s | Throughout backend | Create typed error enum | `Completed` |
| P1 | No JSON Schema validation on tool inputs | `backend/src/tool.rs` | Add `validate(input) -> Result<()>` to `Tool` trait | `Completed` |
| P2 | Extension routing types mismatch — `types.ts` `id` field vs backend `request_id` | `bandhu/src/types.ts`, `backend/src/queue.rs` | Align field names | `Completed` |
| P2 | No streaming support for `/chat` | `backend/src/main.rs`, `bandhu/src/api.ts` | Implement SSE streaming | `Completed` |
| P2 | `gate.rs` does package install detection only via broad command filter | `backend/src/gate.rs` | Add specific package manager pattern matching | `Completed` |
