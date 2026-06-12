# Build Loop

The build loop automatically runs the configured build command after successful file edits. It verifies compilation without waiting for the model to invoke `buildtool` manually.

## Flow

```
writefile or applypatch succeeds
        │
        ▼
BANDHU_BUILD_LOOP enabled?
        │
       yes
        │
        ▼
run BANDHU_BUILD_COMMAND in BANDHU_BUILD_WORKDIR
        │
        ▼
emit build_result message to chat stream
        │
        ▼
append result to model context
```

The loop runs inside `backend/src/queue.rs` after approved or direct execution of `writefile` and `applypatch`. Build output is streamed to the extension as a `build_result` message and logged in the VS Code output channel when result reporting is enabled.

## Configuration

| Variable              | Default       | Description                                                                 |
| --------------------- | ------------- | --------------------------------------------------------------------------- |
| `BANDHU_BUILD_LOOP`   | `true`        | Run build automatically after successful edits. Set to `false` to disable.  |
| `BANDHU_BUILD_COMMAND`| `cargo build` | Command executed by the build loop. Example: `npm run build`.               |
| `BANDHU_BUILD_WORKDIR`| `.`           | Working directory for the build loop. Example: `backend`.                   |
| `BANDHU_TOOL_TIMEOUT_SECS` | `120`    | Max seconds allowed for the build command. Example: `300`.                  |

## Message Format

Successful build:

```json
{
    "type": "build_result",
    "result": {
        "command": "cargo build",
        "directory": ".",
        "stdout": "...",
        "stderr": "",
        "status": 0,
        "summary": "passed",
        "failures": []
    }
}
```

Failed build command execution:

```json
{
    "type": "build_result",
    "error": "tool error: command timed out after 120 seconds"
}
```

Build command ran but compilation failed:

```json
{
    "type": "build_result",
    "result": {
        "command": "cargo build",
        "directory": ".",
        "stdout": "...",
        "stderr": "error: could not compile",
        "status": 101,
        "summary": "failed",
        "failures": ["error: could not compile"]
    }
}
```

## Sample `.env`

```env
BANDHU_BUILD_LOOP=true
BANDHU_BUILD_COMMAND=cargo build
BANDHU_BUILD_WORKDIR=backend
BANDHU_TOOL_TIMEOUT_SECS=300
```

## Related

- [Configuration Reference](./config.md)
- [Tool Reference](./tools.md)
- [Result Reporting](./report.md)
