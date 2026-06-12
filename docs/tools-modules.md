# Tool Modules

Bandhu's tool system provides capabilities for file operations, command execution, and code search. All tools implement the `Tool` trait defined in `backend/src/tool.rs`.

## File Tools

### `readfile.rs`

**Purpose**: Read file content from disk.

- **ID**: `readfile`
- **Requires Approval**: No
- **Input Schema**: `{ "path": string }` (required)
- **Output Schema**: `{ "path": string, "content": string }`

**Implementation Details**:
- Uses `std::fs::read_to_string` for UTF-8 file reading
- No approval required (safe read operation)
- Path validation checks for empty string

### `writefile.rs`

**Purpose**: Write or replace file content.

- **ID**: `writefile`
- **Requires Approval**: Yes
- **Input Schema**: `{ "path": string, "content": string }` (both required)
- **Output Schema**: `{ "path": string, "status": "written" }`

**Implementation Details**:
- Creates parent directories automatically via `create_dir_all`
- Generates unified diff preview via `diff::generate()`
- Checks against `BANDHU_FORBIDDEN_PATHS` before execution
- Content size limit enforced (65536 bytes by default)

### `applypatch.rs`

**Purpose**: Apply unified diff patch to an existing file.

- **ID**: `applypatch`
- **Requires Approval**: Yes
- **Input Schema**: `{ "path": string, "patch": string }` (both required)
- **Output Schema**: `{ "path": string, "status": "applied" }`

**Implementation Details**:
- Uses `diff::apply()` to apply patch to existing content
- Non-destructive preview before execution
- Rejections produce `tool_error` in context

## Search & Discovery Tools

### `search.rs`

**Purpose**: Text search using ripgrep.

- **ID**: `search`
- **Requires Approval**: No
- **Input Schema**: `{ "pattern": string, "path?": string }` (pattern required)
- **Output Schema**: `{ "pattern": string, "path": string, "matches": [{ "path": string, "line": number, "text": string }] }`

**Implementation Details**:
- Invokes `rg --json --line-number --max-count` for structured output
- Ignores `target/**`, `node_modules/**`, `.git/**`, `dist/**` by default
- Parses ripgrep JSON output format
- Match limit configurable via `BANDHU_RG_MAX_COUNT`

### `listdir.rs`

**Purpose**: List immediate directory entries.

- **ID**: `listdir`
- **Requires Approval**: No
- **Input Schema**: `{ "path?": string }`
- **Output Schema**: `{ "path": string, "entries": [{ "name": string, "kind": "file" | "dir" }] }`

**Implementation Details**:
- Uses `std::fs::read_dir` for directory reading
- Returns name and kind (file/directory) for each entry
- Defaults to current working directory

## Execution Tools

### `runcommand.rs`

**Purpose**: Execute arbitrary shell commands.

- **ID**: `runcommand`
- **Requires Approval**: Yes
- **Input Schema**: `{ "command": string }` (required)
- **Output Schema**: `{ "stdout": string, "stderr": string, "status": number }`

**Implementation Details**:
- Shell wrapper: `sh -c` (Unix) or `cmd /C` (Windows)
- Timeout enforcement via `BANDHU_TOOL_TIMEOUT_SECS`
- Captures stdout, stderr, and exit code
- Subject to `BANDHU_FORBIDDEN_CMDS` filter

### `buildtool.rs`

**Purpose**: Execute configured build command.

- **ID**: `buildtool`
- **Requires Approval**: Yes
- **Input Schema**: `{ "command?": string, "directory?": string }`
- **Output Schema**: `{ "command": string, "directory": string, "stdout": string, "stderr": string, "status": number, "summary": "passed" | "failed", "failures": string[] }`

**Implementation Details**:
- Uses `commandtool::run()` for execution
- Defaults from `BANDHU_BUILD_COMMAND` and `BANDHU_BUILD_WORKDIR`
- Extracts failure lines containing "failed", "error:", or "panic"
- Triggers build loop after file edits

### `testrunner.rs`

**Purpose**: Execute configured test command.

- **ID**: `testrunner`
- **Requires Approval**: Yes
- **Input Schema**: `{ "command?": string, "directory?": string }`
- **Output Schema**: `{ "command": string, "directory": string, "stdout": string, "stderr": string, "status": number, "summary": "passed" | "failed", "failures": string[] }`

**Implementation Details**:
- Uses `commandtool::run()` for execution
- Defaults from `BANDHU_TEST_COMMAND` and `BANDHU_TEST_WORKDIR`
- Triggers automatically after successful build when `BANDHU_TEST_LOOP=true`
- Failure extraction mirrors `buildtool`

## Shared Infrastructure

### `commandtool.rs`

Common command execution utilities:

- `run()` - Executes command with timeout and output capture
- `failures()` - Extracts failure lines from combined stdout/stderr
- `shell()` - Creates process with proper shell invocation
- `command()` - Environment variable lookup for commands
- `directory()` - Environment variable lookup for directories

### `diff.rs`

Diff generation and application:

- `generate()` - Creates unified diff between old and new content
- `apply()` - Applies patch to original content
- `hunks()` - Parses patch into `Hunk` structures (for future use)

### `context.rs` - Context Building

Four-stage context pipeline:

1. **Search Stage** - Extract keywords, run search, collect candidates
2. **Select Stage** - Rank and truncate to top N files
3. **Read Stage** - Read file contents, apply size limits
4. **Pack Stage** - Serialize into `ContextItem` array

### `gate.rs` - Safety Filter

Pre-execution safety checks:

- `check()` - Validates against forbidden commands and paths
- `install()` - Detects package install command patterns
- `requires_approval()` - Determines if tool needs approval

### `buildloop.rs` & `testloop.rs`

Automated execution after file modifications:

- `buildloop` - Runs after `writefile`/`applypatch` success
- `testloop` - Runs after `buildloop` success
- Configurable via `BANDHU_BUILD_LOOP`, `BANDHU_TEST_LOOP`