# Bandhu VS Code Extension

Bandhu is a local-first AI coding agent that integrates directly into VS Code. This extension provides the user interface for interacting with the Bandhu backend service.

## Features

- **Chat Interface**: Webview-based chat panel for natural language coding tasks
- **Tool Approval**: Review and approve/reject all file modifications and commands
- **Diff Preview**: See unified diff previews before approving file changes
- **Build Loop**: Automatic build after file edits (when enabled)
- **Test Loop**: Automatic test execution after successful builds
- **Streaming Support**: Real-time SSE streaming for responsive interactions
- **Output Channel**: Build and test results logged to dedicated output channel

## Requirements

- Bandhu backend server running locally (see root README)
- Ollama installed and running
- VS Code 1.120.0+

## Usage

1. Start the Bandhu backend server
2. Press `F5` in VS Code to launch Extension Development Host
3. Open the chat panel via status bar or "Bandhu: Open Chat" command
4. Type coding tasks in natural language

## Architecture

The extension consists of these core modules:

| Module | Purpose |
| ------ | ------- |
| `extension.ts` | Activation entry point |
| `controller.ts` | Lifecycle orchestrator |
| `chatui.ts` | Webview panel for chat |
| `status.ts` | Status bar item |
| `api.ts` | HTTP client for backend |
| `types.ts` | Shared type definitions |

See the [Architecture Overview](../ARCHITECTURE.md) for full system design.

## Configuration

Environment variables (set before starting VS Code):

| Variable | Default | Description |
| -------- | ------- | ----------- |
| `BANDHU_BACKEND_URL` | `http://127.0.0.1:3000` | Backend server URL |
| `BANDHU_CHAT_PLACEHOLDER` | `Ask Bandhu...` | Input placeholder text |
| `BANDHU_CHAT_STREAMING` | `true` | Use SSE streaming endpoint |
| `BANDHU_CHAT_TIMEOUT_MS` | `120000` | Request timeout in milliseconds |

See [Configuration Reference](../docs/config.md) for all settings.

## Development

```bash
npm install
npm run compile  # Type check, lint, bundle
npm run watch    # Watch mode for development
```

## License

MIT