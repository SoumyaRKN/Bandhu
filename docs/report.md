# Result Reporting

The VS Code extension writes build and test output to a dedicated output channel so long command logs stay out of the chat panel.

## Behavior

When the extension receives:

- `build_result` messages from the backend build loop
- `tool_result` messages from `buildtool`
- `tool_result` messages from `testrunner`

…it appends formatted output to the configured output channel. The chat panel still shows a short summary line for build results.

## Configuration

| Variable              | Default   | Description                                                                 |
| --------------------- | --------- | --------------------------------------------------------------------------- |
| `BANDHU_OUTPUT_NAME`  | `Bandhu`  | VS Code output channel name. Example: `Bandhu Build`.                       |
| `BANDHU_OUTPUT_SHOW`  | `true`    | Focus the output channel when build or test results arrive. Set to `false` to keep the current panel focused. |

Extension settings are read from the environment before VS Code starts, matching other Bandhu extension variables documented in [Configuration Reference](./config.md).

## Output Format

Each entry includes a timestamp, command, directory, summary, stdout, stderr, and parsed failure lines when present.

Example:

```
[2026-06-12T10:15:30.123Z] build
command: cargo build
directory: backend
summary: failed
stdout:
   Compiling bandhu-backend v0.1.0
stderr:
error: could not compile
failures:
error: could not compile
```

## Sample `.env`

```env
BANDHU_OUTPUT_NAME=Bandhu Build
BANDHU_OUTPUT_SHOW=true
```

## Related

- [Build Loop](./buildloop.md)
- [Configuration Reference](./config.md)
- [Tool Reference](./tools.md)
