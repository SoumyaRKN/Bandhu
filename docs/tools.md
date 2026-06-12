# Tool Reference

Bandhu tools are registered in `backend/src/registry.rs` and implement the `Tool` trait in `backend/src/tool.rs`. Tool input is JSON, validated before execution when `BANDHU_SCHEMA_VALIDATE=true`, and capped by `BANDHU_TOOL_INPUT_LIMIT`.

## `readfile`

Reads a UTF-8 file from disk.

Input:

```json
{
    "path": "backend/src/main.rs"
}
```

Output:

```json
{
    "path": "backend/src/main.rs",
    "content": "..."
}
```

Approval: no.

## `search`

Searches text with ripgrep and returns JSON match records.

Input:

```json
{
    "pattern": "Config",
    "path": "backend/src"
}
```

Output:

```json
{
    "pattern": "Config",
    "path": "/workspace/backend/src",
    "matches": [
        {
            "path": "/workspace/backend/src/config.rs",
            "line": 4,
            "text": "pub struct Config {"
        }
    ]
}
```

Approval: no.

Configuration: `BANDHU_RG_MAX_COUNT` limits matches per file.

## `listdir`

Lists immediate directory entries.

Input:

```json
{
    "path": "backend/src"
}
```

Output:

```json
{
    "path": "/workspace/backend/src",
    "entries": [
        {
            "name": "main.rs",
            "kind": "file"
        }
    ]
}
```

Approval: no.

## `writefile`

Writes complete file content. The queue generates a diff preview before approval.

Input:

```json
{
    "path": "docs/example.md",
    "content": "# Example\n"
}
```

Output:

```json
{
    "path": "docs/example.md",
    "status": "written"
}
```

Approval: yes.

Configuration: `BANDHU_FORBIDDEN_PATHS` blocks sensitive path substrings.

## `applypatch`

Applies a unified diff patch to a file.

Input:

```json
{
    "path": "docs/example.md",
    "patch": "--- a/docs/example.md\n+++ b/docs/example.md\n-old\n+new\n"
}
```

Output:

```json
{
    "path": "docs/example.md",
    "status": "applied"
}
```

Approval: yes.

## `runcommand`

Runs a shell command through the operating system shell.

Input:

```json
{
    "command": "cargo test"
}
```

Output:

```json
{
    "stdout": "...",
    "stderr": "",
    "status": 0
}
```

Approval: yes.

Configuration: `BANDHU_FORBIDDEN_CMDS` blocks dangerous command substrings before approval and execution. The default blocks `rm -rf`, `sudo`, and shell ampersand operators, and can be replaced through the environment. `BANDHU_INSTALL_CMDS` tags package install approvals with `kind: "install"` and the matched `pattern`.

## `buildtool`

Runs the configured build command and returns captured output.

Input:

```json
{
    "command": "cargo build",
    "directory": "backend"
}
```

Output:

```json
{
    "command": "cargo build",
    "directory": "backend",
    "stdout": "...",
    "stderr": "",
    "status": 0,
    "summary": "passed",
    "failures": []
}
```

Approval: yes.

See also [Build Loop](./buildloop.md) for automatic post-edit builds.

Configuration: `BANDHU_BUILD_COMMAND` sets the default build command, `BANDHU_BUILD_WORKDIR` sets the default working directory, and `BANDHU_TOOL_TIMEOUT_SECS` caps command runtime. Input fields override the environment defaults for one tool call.

## `testrunner`

Runs the configured test command, captures output, and returns a parsed failure summary.

Input:

```json
{
    "command": "cargo test",
    "directory": "backend"
}
```

Output:

```json
{
    "command": "cargo test",
    "directory": "backend",
    "stdout": "...",
    "stderr": "",
    "status": 0,
    "summary": "passed",
    "failures": []
}
```

Approval: yes.

Configuration: `BANDHU_TEST_COMMAND` sets the default test command, `BANDHU_TEST_WORKDIR` sets the default working directory, and `BANDHU_TOOL_TIMEOUT_SECS` caps command runtime. Input fields override the environment defaults for one tool call.
