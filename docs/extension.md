# VS Code Extension

The Bandhu extension is a TypeScript-based VS Code extension providing the user interface for the coding agent.

## Overview

- **Language**: TypeScript (targeting VS Code extension API)
- **Entry Point**: `bandhu/src/extension.ts`
- **Build**: esbuild bundling via `bandhu/esbuild.js`

## Core Modules

### `extension.ts` - Activation Entry

Minimal entry point that creates the `Controller` and calls `activate()`.

### `controller.ts` - Lifecycle Orchestrator

Central coordination point:
- Manages `Statusbar`, `ChatPanel`, `Report`, and `Config` instances
- Handles webview messages and routes to appropriate handlers
- Coordinates chat requests (streaming and non-streaming)
- Manages abort signals for request cancellation

### `chatui.ts` - Webview Panel

VS Code webview implementation:
- Creates chat panel with VS Code theming
- Renders different message types with appropriate styling
- Handles approval UI with inline buttons
- Displays diff previews for file edits
- Manages message scroll and input handling

### `status.ts` - Status Bar Item

VS Code status bar integration:
- Shows idle/busy/error states with icons
- Configurable text via environment variables
- Click handler opens chat panel

### `api.ts` - HTTP Client

Backend communication:
- REST API calls to backend endpoints
- Timeout handling with configurable limits
- Retry logic for transient failures
- SSE streaming support for `/chat/stream`
- Approval decision submission

### `types.ts` - Type Definitions

Shared TypeScript interfaces:
- `ChatMessage` - All message types for display
- `ApprovalRequestMsg` - Approval request structure
- `ChatRequest` - Chat request payload
- `ChatResponse` - Response structure
- `WebviewMsg` - Webview communication types

### `config.ts` - Environment Configuration

Client-side configuration:
- Reads environment variables at startup
- Provides typed configuration interface
- Defaults for all optional settings

### `report.ts` - Output Channel

Build and test result logging:
- Writes to VS Code output channel
- Formats build/test results with timestamps
- Displays failures extracted from output

### `approval.ts` - Approval Modal

Alternative approval interface (currently unused in favor of chat UI):
- Shows VS Code warning message with buttons
- Sends approval/rejection to backend

## Message Types

| Type | Purpose |
| ---- | ------- |
| `response` | Final model answer |
| `tool_result` | Successful tool execution |
| `tool_error` | Tool execution failure |
| `tool_approval` | Request for user approval |
| `build_result` | Build loop completion |
| `testresult` | Test loop completion |
| `error` | General error |
| `complete` | Loop termination |

## Commands

| Command | Purpose |
| ------- | ------- |
| `bandhu.open` | Opens the chat panel |

## Development

```bash
cd bandhu
npm install
npm run compile  # Type check, lint, bundle
npm run watch    # Watch mode for development
```

To test: Press F5 in VS Code to launch Extension Development Host.