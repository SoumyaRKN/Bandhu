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
| 10 | Install Python | Install Python 3 for scripts and tooling (if not already present) | `Pending` |

---

## Phase 2: Install Local Model Runtime

| # | Task | Description | Status |
|---|------|-------------|--------|
| 11 | Install Ollama | Run `curl -fsSL https://ollama.com/install.sh \| sh` | `Completed` |
| 12 | Verify Ollama | Run `ollama --version` to confirm installation | `Completed` |
| 13 | Pull Coding Model | Run `ollama pull qwen2.5-coder:7b` | `Completed` |
| 14 | Test Ollama | Run `ollama run qwen2.5-coder:7b` and verify model responds | `Completed` |
| 15 | Configure Ollama | Set Ollama to allow localhost connections, configure host and port if needed | `Pending` |

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
| 23 | Write README.md | Document project overview, setup steps, and usage | `Pending` |
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
| 32 | Add Status Bar Indicator | Add status bar item showing "Bandhu" when extension is active | `Pending` |
| 33 | Verify Extension Loads | Confirm extension activates in new VS Code window without errors | `Pending` |

---

## Phase 5: Create Backend Service

| # | Task | Description | Status |
|---|------|-------------|--------|
| 34 | Initialize Rust Project | Navigate to backend directory and run `cargo init` | `Completed` |
| 35 | Define Cargo Dependencies | Add required dependencies to `Cargo.toml` (axum, serde, tokio, reqwest, etc) | `Completed` |
| 36 | Define API Structure | Design API endpoints for: receive task, accept prompt, return response, execute tool | `Pending` |
| 37 | Define Task Models | Create data structures for: Task, TaskRequest, TaskResponse, ToolCall | `Completed` |
| 38 | Define Error Types | Create custom error types for backend service | `Pending` |
| 39 | Create API Server | Implement HTTP server using Axum framework | `Completed` |
| 40 | Add CORS Middleware | Configure CORS to allow VS Code extension to communicate with backend | `Pending` |
| 41 | Add Health Check Endpoint | Create `/health` endpoint for monitoring backend status | `Pending` |

---

## Phase 6: Connect To Ollama

| # | Task | Description | Status |
|---|------|-------------|--------|
| 42 | Define Ollama Client | Create client module for communicating with Ollama API | `Completed` |
| 43 | Implement Generate Endpoint | Create `POST /api/generate` endpoint that accepts prompt and sends to Ollama | `Pending` |
| 44 | Implement Chat Endpoint | Create `POST /api/chat` endpoint for multi-turn conversations | `Completed` |
| 45 | Handle Streaming Responses | Implement streaming response handling for real-time token output | `Pending` |
| 46 | Add Request Timeout | Configure timeout for Ollama API requests | `Pending` |
| 47 | Test End-to-End Flow | Verify: Extension → Backend → Ollama → Backend → Extension works end-to-end | `Pending` |
| 48 | Add Error Handling | Handle Ollama connection errors, model not found, and timeout scenarios | `Pending` |

---

## Phase 7: Implement Tools

| # | Task | Description | Status |
|---|------|-------------|--------|
| 49 | Define Tool Trait | Create base `Tool` trait with `id`, `name`, `description`, `input_schema`, `execute` method | `Completed` |
| 50 | Implement ReadFile Tool | Create `ReadFile` tool: takes path, returns file content | `Pending` |
| 51 | Implement Search Tool | Create `Search` tool: uses ripgrep to search text patterns | `Pending` |
| 52 | Implement WriteFile Tool | Create `WriteFile` tool: writes content to file (requires approval) | `Pending` |
| 53 | Implement RunCommand Tool | Create `RunCommand` tool: executes shell commands (requires approval) | `Pending` |
| 54 | Add Tool Registry | Create tool registry to map tool IDs to implementations | `Completed` |
| 55 | Add Tool Validation | Implement input schema validation for each tool | `Pending` |
| 56 | Document Tool APIs | Write documentation for each tool's input/output format | `Pending` |

---

## Phase 8: Add Tool Calling Loop

| # | Task | Description | Status |
|---|------|-------------|--------|
| 57 | Design Tool Loop | Implement main loop: ask model → parse tool call → execute tool → return result → repeat | `Completed` |
| 58 | Implement Loop Controller | Create controller that manages the tool call loop state | `Completed` |
| 59 | Add Tool Selection Prompt | Configure system prompt to instruct model on available tools and how to call them | `Completed` |
| 60 | Parse Tool Responses | Implement parser for model tool call responses (JSON format) | `Completed` |
| 61 | Handle Tool Errors | Implement error handling for failed tool executions within the loop | `Completed` |
| 62 | Implement Loop Termination | Define conditions to break the loop (task completed, error encountered, max iterations) | `Completed` |
| 63 | Test Simple Tool Chain | Verify a simple sequence: ReadFile → Search → respond works correctly | `Pending` |
| 64 | Add Loop Logging | Add structured logging for each iteration of the tool loop | `Pending` |

---

## Phase 9: Add Safety

| # | Task | Description | Status |
|---|------|-------------|--------|
| 65 | Define Forbidden Commands | Maintain list of forbidden commands: `rm -rf /`, `sudo`, background execution | `Pending` |
| 66 | Implement Command Filter | Add filter to block execution of dangerous commands | `Pending` |
| 67 | Add File Edit Confirmation | Require user approval before WriteFile tool is executed | `Pending` |
| 68 | Add Command Confirmation | Require user approval before RunCommand tool is executed | `Pending` |
| 69 | Add Package Install Confirmation | Require confirmation before any package installation commands | `Pending` |
| 70 | Implement Confirmation UI | Create VS Code webview or quick pick for showing tool actions and requesting confirmation | `Pending` |
| 71 | Add Approval Logging | Log all approved and rejected tool executions for audit trail | `Pending` |
| 72 | Test Safety Mechanisms | Manually verify dangerous commands are blocked and approvals work | `Pending` |

---

## Phase 10: Improve Accuracy

| # | Task | Description | Status |
|---|------|-------------|--------|
| 73 | Implement Context Builder | Create context builder that reads only relevant files instead of entire repo | `Pending` |
| 74 | Build Symbol Map | Create tool to build symbol map (functions, classes, variables) from codebase | `Pending` |
| 75 | Build Dependency Map | Create tool to map file dependencies and imports | `Pending` |
| 76 | Create File Summarizer | Implement tool to generate summaries of large files for context | `Pending` |
| 77 | Optimize Context Window | Implement strategy to fit most relevant context within model's context window | `Pending` |
| 78 | Add Incremental Context | Support adding context incrementally across multiple tool calls | `Pending` |
| 79 | Implement Rerank Strategy | Add strategy to rerank relevant files by task similarity | `Pending` |
| 80 | Test Context Accuracy | Verify agent can solve tasks using context-only approach vs full repo | `Pending` |

---

## Phase 11: Add Diff Approval

| # | Task | Description | Status |
|---|------|-------------|--------|
| 81 | Implement Diff Generator | Create tool to generate unified diff patch from proposed changes | `Pending` |
| 82 | Create Diff View | Implement VS Code webview or TextEditorContentProvider to show diffs | `Pending` |
| 83 | Implement Apply Patch Tool | Create tool to apply diff patch to files after user approval | `Pending` |
| 84 | Add Reject Workflow | Implement workflow to discard changes when user rejects diff | `Pending` |
| 85 | Add Line-by-Line Approval | Support approving/rejecting individual change hunks within a diff | `Pending` |
| 86 | Add Edit Suggestions | Allow user to manually edit proposed changes before applying | `Pending` |
| 87 | Test Diff Workflow | End-to-end test: model proposes change → diff shown → user approves → changes applied | `Pending` |
| 88 | Document Diff System | Write documentation for the diff approval workflow | `Pending` |

---

## Phase 12: Add Testing Loop

| # | Task | Description | Status |
---|---|------|-------------|--------|
| 89 | Implement Build Tool | Create tool to run project build commands (cargo build, npm run build, etc) | `Pending` |
| 90 | Implement Test Runner | Create tool to run project tests and capture output | `Pending` |
| 91 | Add Build Loop | Implement loop that runs build after code edits to verify compilation | `Pending` |
| 92 | Add Test Loop | Implement loop that runs tests after build succeeds | `Pending` |
| 93 | Implement Fix Loop | Implement loop: build fails → send errors to model → fix → rebuild until success | `Pending` |
| 94 | Parse Test Output | Create parser to extract test failures and error messages for the model | `Pending` |
| 95 | Add Test Result Reporting | Display build and test results to user in VS Code output channel | `Pending` |
| 96 | Test Full Testing Loop | Verify complete loop: edit → build → test → fix works for sample project | `Pending` |

---

## Phase 13: Improve Speed

| # | Task | Description | Status |
|---|------|-------------|--------|
| 97 | Profile Context Generation | Measure time spent on context generation and identify bottlenecks | `Pending` |
| 98 | Optimize File Reads | Implement batch file reads and caching for repeated access | `Pending` |
| 99 | Reduce Context Size | Implement strategies to reduce context size (summaries, selective inclusion) | `Pending` |
| 100 | Optimize Model Calls | Reduce number of model calls via better tool selection and batched requests | `Pending` |
| 101 | Add Response Caching | Cache common query responses to avoid redundant model calls | `Pending` |
| 102 | Implement Parallel Tool Execution | Add support for parallel tool execution in the loop when tools are independent | `Pending` |
| 103 | Profile End-to-End Performance | Measure and document typical task completion times | `Pending` |
| 104 | Document Optimizations | Write performance optimization guidelines and best practices | `Pending` |

---

## Phase 14: Future Improvements

| # | Task | Description | Status |
|---|------|-------------|--------|
| 105 | Add Git Integration | Implement tool for git operations: status, diff, log, blame | `Pending` |
| 106 | Implement Planning System | Add planning module for complex multi-step tasks | `Pending` |
| 107 | Add Memory System | Implement short-term and long-term memory for conversation context | `Pending` |
| 108 | Add Multi-Project Support | Enable agent to work across multiple related projects | `Pending` |
| 109 | Add Embeddings | Implement embeddings for semantic code search | `Pending` |
| 110 | Integrate Vector DB | Set up local vector database for codebase embeddings and semantic search | `Pending` |
| 111 | Add Streaming UI | Implement real-time streaming of model responses in VS Code | `Pending` |
| 112 | Add Multi-Model Support | Support switching between different local models | `Pending` |

---

## General / Cross-Phase Tasks

| # | Task | Description | Status |
|---|------|-------------|--------|
| 113 | Rename Files | Apply single-word naming convention to all files and directories | `Pending` |
| 114 | Rename Functions | Apply single-word naming convention to all functions and methods | `Pending` |
| 115 | Rename Variables | Apply single-word naming convention to all variables | `Pending` |
| 116 | Run Formatter | Configure and run formatter (rustfmt for Rust, prettier for TypeScript/JS) on all code | `Pending` |
| 117 | Run Linter | Configure and run linter (clippy for Rust, eslint for TypeScript/JS) and fix issues | `Pending` |
| 118 | Type Check | Run type checker (tsc for TypeScript, cargo check for Rust) and fix type errors | `Pending` |
| 119 | Format Markdown | Format all markdown files (`README.md`, `brain.md`, `ROADMAP.md`, docs) with consistent style | `Pending` |
| 120 | Update README | Keep README.md up-to-date with setup instructions, usage examples, and feature status | `Pending` |
| 121 | Write API Docs | Document all API endpoints, request/response formats, and error codes | `Pending` |
| 122 | Write Tool Docs | Document all implemented tools with examples and usage guidelines | `Pending` |
| 123 | Write User Guide | Write comprehensive user guide for installing, configuring, and using Bandhu | `Pending` |
| 124 | Write Developer Guide | Write developer guide for contributing, building, testing, and extending Bandhu | `Pending` |
| 125 | Add Code Comments | Add descriptive comments to complex logic and public APIs | `Pending` |
| 126 | Create CHANGELOG | Initialize CHANGELOG.md with version history and release notes | `Pending` |
| 127 | Clean Up Experiments | Review `experiments/` directory, archive or delete obsolete experiments | `Pending` |
| 128 | Clean Up Scripts | Review `scripts/` directory, remove unused scripts, document remaining ones | `Pending` |
| 129 | Clean Up Temporary Files | Remove temporary files, build artifacts, debug logs, and caches from repo | `Pending` |
| 130 | Organize Docs Structure | Organize `docs/` directory with clear subdirectories (api, tools, guides, etc) | `Pending` |
| 131 | Git Add Configuration | Verify `.gitignore` covers all build artifacts and sensitive files | `Pending` |
| 132 | Git Commit Structure | Verify commit history is clean and project is ready for version control | `Pending` |
| 133 | Add Git Hooks | Set up pre-commit hooks for formatting, linting, and type checking | `Pending` |
| 134 | Create CI Config | Set up CI pipeline (GitHub Actions or similar) for lint, type check, and test | `Pending` |
| 135 | Review Security | Audit code for security issues: no hardcoded secrets, safe command execution, input validation | `Pending` |

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
| `Pending` | Not yet started |
| `In Progress` | Currently being worked on |
| `Completed` | Finished and verified |
| `Blocked` | Waiting on dependency or external factor |
