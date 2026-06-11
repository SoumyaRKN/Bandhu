# Bandhu - Personal Coding AI Agent Roadmap

## Project Overview

Build a local-first VS Code coding AI agent (Bandhu) that runs mostly free, works on Ubuntu, uses local models, reads and edits project files, executes tasks with approval, and scales gradually.

---

## Phase 0: Understand The Target

| # | Task | Description | Status |
|---|------|-------------|--------|
| 1 | Define Scope | Document what Bandhu is and is not built for | `completed` |
| 2 | Set Vision | Confirm Bandhu is a personal coding assistant integrated into VS Code, not a full Cursor clone or cloud platform | `completed` |
| 3 | Document Constraints | List known constraints: local-first, free, Ubuntu, local models | `completed` |

---

## Phase 1: Prepare Environment

| # | Task | Description | Status |
|---|------|-------------|--------|
| 4 | Update Ubuntu | Run `sudo apt update && sudo apt upgrade` | `completed` |
| 5 | Install Git | Run `sudo apt install git` and verify installation | `completed` |
| 6 | Install NodeJS | Install LTS version via apt or nvm, verify with `node -v && npm -v` | `completed` |
| 7 | Install Rust | Run rustup install script, verify with `rustc --version && cargo --version` | `completed` |
| 8 | Install VS Code | Install official VS Code .deb package, verify with `code .` command | `completed` |
| 9 | Install ripgrep | Run `sudo apt install ripgrep` for fast text search | `completed` |
| 10 | Install Python | Install Python 3 for scripts and tooling (if not already present) | `pending` |

---

## Phase 2: Install Local Model Runtime

| # | Task | Description | Status |
|---|------|-------------|--------|
| 11 | Install Ollama | Run `curl -fsSL https://ollama.com/install.sh \| sh` | `completed` |
| 12 | Verify Ollama | Run `ollama --version` to confirm installation | `completed` |
| 13 | Pull Coding Model | Run `ollama pull qwen2.5-coder:7b` | `completed` |
| 14 | Test Ollama | Run `ollama run qwen2.5-coder:7b` and verify model responds | `completed` |
| 15 | Configure Ollama | Set Ollama to allow localhost connections, configure host and port if needed | `pending` |

---

## Phase 3: Create Project Structure

| # | Task | Description | Status |
|---|------|-------------|--------|
| 16 | Make Directory - 'bandhu' | Create main project directory at root | `completed` |
| 17 | Make Directory - 'backend' | Create Rust backend service directory | `completed` |
| 18 | Make Directory - 'docs' | Create documentation directory | `completed` |
| 19 | Make Directory - 'scripts' | Create utility scripts directory | `completed` |
| 20 | Make Directory - 'experiments' | Create experiments and prototyping directory | `completed` |
| 21 | Initialize Git Repo | Run `git init` in project root, configure basic git settings | `completed` |
| 22 | Create .gitignore | Create `.gitignore` file excluding `node_modules`, `.vscode`, `target`, `.ollama`, `__pycache__`, `.env`, etc | `completed` |
| 23 | Write README.md | Document project overview, setup steps, and usage | `in_progress` |
| 24 | Create LICENSE | Add MIT or appropriate open source license file | `completed` |

---

## Phase 4: Build VS Code Extension

| # | Task | Description | Status |
|---|------|-------------|--------|
| 25 | Install Yeoman | Run `npm install -g yo generator-code` | `completed` |
| 26 | Generate Extension Scaffold | Run `yo code` selecting TypeScript and New Extension | `completed` |
| 27 | Configure Extension | Update `package.json` with correct name (`bandhu`), publisher, version, and description | `completed` |
| 28 | Install Dependencies | Run `npm install` in extension directory | `completed` |
| 29 | Configure Extension Manifest | Update `extension.ts` or main entry point with basic activation logic | `completed` |
| 30 | Add Launch Configuration | Ensure `.vscode/launch.json` is properly configured for debugging | `completed` |
| 31 | Test Extension Launch | Press F5 to launch new Extension Development Host window | `pending` |
| 32 | Add Status Bar Indicator | Add status bar item showing "Bandhu" when extension is active | `pending` |
| 33 | Verify Extension Loads | Confirm extension activates in new VS Code window without errors | `pending` |

---

## Phase 5: Create Backend Service

| # | Task | Description | Status |
|---|------|-------------|--------|
| 34 | Initialize Rust Project | Navigate to backend directory and run `cargo init` | `completed` |
| 35 | Define Cargo Dependencies | Add required dependencies to `Cargo.toml` (axum, serde, tokio, reqwest, etc) | `completed` |
| 36 | Define API Structure | Design API endpoints for: receive task, accept prompt, return response, execute tool | `in_progress` |
| 37 | Define Task Models | Create data structures for: Task, TaskRequest, TaskResponse, ToolCall | `completed` |
| 38 | Define Error Types | Create custom error types for backend service | `pending` |
| 39 | Create API Server | Implement HTTP server using Axum framework | `completed` |
| 40 | Add CORS Middleware | Configure CORS to allow VS Code extension to communicate with backend | `pending` |
| 41 | Add Health Check Endpoint | Create `/health` endpoint for monitoring backend status | `pending` |

---

## Phase 6: Connect To Ollama

| # | Task | Description | Status |
|---|------|-------------|--------|
| 42 | Define Ollama Client | Create client module for communicating with Ollama API | `completed` |
| 43 | Implement Generate Endpoint | Create `POST /api/generate` endpoint that accepts prompt and sends to Ollama | `in_progress` |
| 44 | Implement Chat Endpoint | Create `POST /api/chat` endpoint for multi-turn conversations | `completed` |
| 45 | Handle Streaming Responses | Implement streaming response handling for real-time token output | `pending` |
| 46 | Add Request Timeout | Configure timeout for Ollama API requests | `pending` |
| 47 | Test End-to-End Flow | Verify: Extension → Backend → Ollama → Backend → Extension works end-to-end | `pending` |
| 48 | Add Error Handling | Handle Ollama connection errors, model not found, and timeout scenarios | `in_progress` |

---

## Phase 7: Implement Tools

| # | Task | Description | Status |
|---|------|-------------|--------|
| 49 | Define Tool Trait | Create base `Tool` trait with `id`, `name`, `description`, `input_schema`, `execute` method | `pending` |
| 50 | Implement ReadFile Tool | Create `ReadFile` tool: takes path, returns file content | `pending` |
| 51 | Implement Search Tool | Create `Search` tool: uses ripgrep to search text patterns | `pending` |
| 52 | Implement WriteFile Tool | Create `WriteFile` tool: writes content to file (requires approval) | `pending` |
| 53 | Implement RunCommand Tool | Create `RunCommand` tool: executes shell commands (requires approval) | `pending` |
| 54 | Add Tool Registry | Create tool registry to map tool IDs to implementations | `pending` |
| 55 | Add Tool Validation | Implement input schema validation for each tool | `pending` |
| 56 | Document Tool APIs | Write documentation for each tool's input/output format | `pending` |

---

## Phase 8: Add Tool Calling Loop

| # | Task | Description | Status |
|---|------|-------------|--------|
| 57 | Design Tool Loop | Implement main loop: ask model → parse tool call → execute tool → return result → repeat | `pending` |
| 58 | Implement Loop Controller | Create controller that manages the tool call loop state | `pending` |
| 59 | Add Tool Selection Prompt | Configure system prompt to instruct model on available tools and how to call them | `pending` |
| 60 | Parse Tool Responses | Implement parser for model tool call responses (JSON format) | `pending` |
| 61 | Handle Tool Errors | Implement error handling for failed tool executions within the loop | `pending` |
| 62 | Implement Loop Termination | Define conditions to break the loop (task completed, error encountered, max iterations) | `pending` |
| 63 | Test Simple Tool Chain | Verify a simple sequence: ReadFile → Search → respond works correctly | `pending` |
| 64 | Add Loop Logging | Add structured logging for each iteration of the tool loop | `pending` |

---

## Phase 9: Add Safety

| # | Task | Description | Status |
|---|------|-------------|--------|
| 65 | Define Forbidden Commands | Maintain list of forbidden commands: `rm -rf /`, `sudo`, background execution | `pending` |
| 66 | Implement Command Filter | Add filter to block execution of dangerous commands | `pending` |
| 67 | Add File Edit Confirmation | Require user approval before WriteFile tool is executed | `pending` |
| 68 | Add Command Confirmation | Require user approval before RunCommand tool is executed | `pending` |
| 69 | Add Package Install Confirmation | Require confirmation before any package installation commands | `pending` |
| 70 | Implement Confirmation UI | Create VS Code webview or quick pick for showing tool actions and requesting confirmation | `pending` |
| 71 | Add Approval Logging | Log all approved and rejected tool executions for audit trail | `pending` |
| 72 | Test Safety Mechanisms | Manually verify dangerous commands are blocked and approvals work | `pending` |

---

## Phase 10: Improve Accuracy

| # | Task | Description | Status |
|---|------|-------------|--------|
| 73 | Implement Context Builder | Create context builder that reads only relevant files instead of entire repo | `pending` |
| 74 | Build Symbol Map | Create tool to build symbol map (functions, classes, variables) from codebase | `pending` |
| 75 | Build Dependency Map | Create tool to map file dependencies and imports | `pending` |
| 76 | Create File Summarizer | Implement tool to generate summaries of large files for context | `pending` |
| 77 | Optimize Context Window | Implement strategy to fit most relevant context within model's context window | `pending` |
| 78 | Add Incremental Context | Support adding context incrementally across multiple tool calls | `pending` |
| 79 | Implement Rerank Strategy | Add strategy to rerank relevant files by task similarity | `pending` |
| 80 | Test Context Accuracy | Verify agent can solve tasks using context-only approach vs full repo | `pending` |

---

## Phase 11: Add Diff Approval

| # | Task | Description | Status |
|---|------|-------------|--------|
| 81 | Implement Diff Generator | Create tool to generate unified diff patch from proposed changes | `pending` |
| 82 | Create Diff View | Implement VS Code webview or TextEditorContentProvider to show diffs | `pending` |
| 83 | Implement Apply Patch Tool | Create tool to apply diff patch to files after user approval | `pending` |
| 84 | Add Reject Workflow | Implement workflow to discard changes when user rejects diff | `pending` |
| 85 | Add Line-by-Line Approval | Support approving/rejecting individual change hunks within a diff | `pending` |
| 86 | Add Edit Suggestions | Allow user to manually edit proposed changes before applying | `pending` |
| 87 | Test Diff Workflow | End-to-end test: model proposes change → diff shown → user approves → changes applied | `pending` |
| 88 | Document Diff System | Write documentation for the diff approval workflow | `pending` |

---

## Phase 12: Add Testing Loop

| # | Task | Description | Status |
---|---|------|-------------|--------|
| 89 | Implement Build Tool | Create tool to run project build commands (cargo build, npm run build, etc) | `pending` |
| 90 | Implement Test Runner | Create tool to run project tests and capture output | `pending` |
| 91 | Add Build Loop | Implement loop that runs build after code edits to verify compilation | `pending` |
| 92 | Add Test Loop | Implement loop that runs tests after build succeeds | `pending` |
| 93 | Implement Fix Loop | Implement loop: build fails → send errors to model → fix → rebuild until success | `pending` |
| 94 | Parse Test Output | Create parser to extract test failures and error messages for the model | `pending` |
| 95 | Add Test Result Reporting | Display build and test results to user in VS Code output channel | `pending` |
| 96 | Test Full Testing Loop | Verify complete loop: edit → build → test → fix works for sample project | `pending` |

---

## Phase 13: Improve Speed

| # | Task | Description | Status |
|---|------|-------------|--------|
| 97 | Profile Context Generation | Measure time spent on context generation and identify bottlenecks | `pending` |
| 98 | Optimize File Reads | Implement batch file reads and caching for repeated access | `pending` |
| 99 | Reduce Context Size | Implement strategies to reduce context size (summaries, selective inclusion) | `pending` |
| 100 | Optimize Model Calls | Reduce number of model calls via better tool selection and batched requests | `pending` |
| 101 | Add Response Caching | Cache common query responses to avoid redundant model calls | `pending` |
| 102 | Implement Parallel Tool Execution | Add support for parallel tool execution in the loop when tools are independent | `pending` |
| 103 | Profile End-to-End Performance | Measure and document typical task completion times | `pending` |
| 104 | Document Optimizations | Write performance optimization guidelines and best practices | `pending` |

---

## Phase 14: Future Improvements

| # | Task | Description | Status |
|---|------|-------------|--------|
| 105 | Add Git Integration | Implement tool for git operations: status, diff, log, blame | `pending` |
| 106 | Implement Planning System | Add planning module for complex multi-step tasks | `pending` |
| 107 | Add Memory System | Implement short-term and long-term memory for conversation context | `pending` |
| 108 | Add Multi-Project Support | Enable agent to work across multiple related projects | `pending` |
| 109 | Add Embeddings | Implement embeddings for semantic code search | `pending` |
| 110 | Integrate Vector DB | Set up local vector database for codebase embeddings and semantic search | `pending` |
| 111 | Add Streaming UI | Implement real-time streaming of model responses in VS Code | `pending` |
| 112 | Add Multi-Model Support | Support switching between different local models | `pending` |

---

## General / Cross-Phase Tasks

| # | Task | Description | Status |
|---|------|-------------|--------|
| 113 | Rename Files | Apply single-word naming convention to all files and directories | `pending` |
| 114 | Rename Functions | Apply single-word naming convention to all functions and methods | `pending` |
| 115 | Rename Variables | Apply single-word naming convention to all variables | `pending` |
| 116 | Run Formatter | Configure and run formatter (rustfmt for Rust, prettier for TypeScript/JS) on all code | `pending` |
| 117 | Run Linter | Configure and run linter (clippy for Rust, eslint for TypeScript/JS) and fix issues | `pending` |
| 118 | Type Check | Run type checker (tsc for TypeScript, cargo check for Rust) and fix type errors | `pending` |
| 119 | Format Markdown | Format all markdown files (`README.md`, `brain.md`, `ROADMAP.md`, docs) with consistent style | `pending` |
| 120 | Update README | Keep README.md up-to-date with setup instructions, usage examples, and feature status | `pending` |
| 121 | Write API Docs | Document all API endpoints, request/response formats, and error codes | `pending` |
| 122 | Write Tool Docs | Document all implemented tools with examples and usage guidelines | `pending` |
| 123 | Write User Guide | Write comprehensive user guide for installing, configuring, and using Bandhu | `pending` |
| 124 | Write Developer Guide | Write developer guide for contributing, building, testing, and extending Bandhu | `pending` |
| 125 | Add Code Comments | Add descriptive comments to complex logic and public APIs | `pending` |
| 126 | Create CHANGELOG | Initialize CHANGELOG.md with version history and release notes | `pending` |
| 127 | Clean Up Experiments | Review `experiments/` directory, archive or delete obsolete experiments | `pending` |
| 128 | Clean Up Scripts | Review `scripts/` directory, remove unused scripts, document remaining ones | `pending` |
| 129 | Clean Up Temporary Files | Remove temporary files, build artifacts, debug logs, and caches from repo | `pending` |
| 130 | Organize Docs Structure | Organize `docs/` directory with clear subdirectories (api, tools, guides, etc) | `pending` |
| 131 | Git Add Configuration | Verify `.gitignore` covers all build artifacts and sensitive files | `pending` |
| 132 | Git Commit Structure | Verify commit history is clean and project is ready for version control | `pending` |
| 133 | Add Git Hooks | Set up pre-commit hooks for formatting, linting, and type checking | `pending` |
| 134 | Create CI Config | Set up CI pipeline (GitHub Actions or similar) for lint, type check, and test | `pending` |
| 135 | Review Security | Audit code for security issues: no hardcoded secrets, safe command execution, input validation | `pending` |

---

## Suggested Weekly Plan

| Week | Focus Area | Key Tasks |
|------|-----------|-----------|
| 1 | Environment + Models | #4-#15 (Phase 1 + Phase 2) |
| 2 | VS Code Extension | #16-#33 (Phase 3 + Phase 4) |
| 3 | Backend + Tools | #34-#56 (Phase 5 + Phase 6 + Phase 7) |
| 4 | Tool Calling Loop | #57-#64 (Phase 8) |
| 5 | Approval System | #65-#72 (Phase 9) |
| 6 | Testing + Improvements | #73-#88 (Phase 10 + Phase 11 + Phase 12) |
| 7 | Performance + Polish | #89-#104 (Phase 13) |
| 8 | Future + Documentation | #105-#135 (Phase 14 + General Tasks) |

---

## Suggested Development Order

1. Local model works (#11-#15)
2. Extension works (#25-#33)
3. Chat UI works (Extension connects to backend)
4. Read files (#50)
5. Search (#51)
6. Edit files (#52) with approval
7. Run commands (#53) with approval
8. Diff approval (#81-#88)
9. Tests (#89-#96)
10. Better context (#73-#80)

---

## Status Key

| Value | Meaning |
|-------|---------|
| `pending` | Not yet started |
| `in_progress` | Currently being worked on |
| `completed` | Finished and verified |
| `blocked` | Waiting on dependency or external factor |
