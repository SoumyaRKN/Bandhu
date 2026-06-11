# Personal Coding AI Agent Project Guide - Bandhu

## Goal

Build a local-first VS Code coding AI agent that:

- Runs mostly free
- Works on Ubuntu
- Uses local models
- Reads and edits project files
- Executes tasks with approval
- Scales gradually

------------------------------------------------------------------------

# Phase 0 --- Understand The Target

You are NOT building:

- Full Cursor clone
- Fully autonomous AGI
- Huge cloud platform

You ARE building:

> A personal coding assistant integrated into VS Code.

------------------------------------------------------------------------

# Phase 1 --- Prepare Environment

## Install Required Software

### Update Ubuntu

``` bash
sudo apt update
sudo apt upgrade
```

### Install Git

``` bash
sudo apt install git
```

### Install NodeJS

Install LTS version.

Verify:

``` bash
node -v
npm -v
```

### Install Rust

``` bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify:

``` bash
rustc --version
cargo --version
```

### Install VS Code

Install official VS Code.

Verify:

``` bash
code .
```

------------------------------------------------------------------------

# Phase 2 --- Install Local Model Runtime

Install Ollama:

``` bash
curl -fsSL https://ollama.com/install.sh | sh
```

Verify:

``` bash
ollama --version
```

Pull a coding model:

``` bash
ollama pull qwen2.5-coder:7b
```

Test:

``` bash
ollama run qwen2.5-coder:7b
```

------------------------------------------------------------------------

# Phase 3 --- Create Project Structure

Create folders:

``` text
/home/nsoumyaprakash/Desktop/Personal/Projects/Bandhu/

├── bandhu/
├── backend/
├── docs/
├── scripts/
└── experiments/
```

------------------------------------------------------------------------

# Phase 4 --- Build VS Code Extension

Create extension:

``` bash
npm install -g yo generator-code

yo code
```

Choose:

- TypeScript
- New Extension

Run:

``` bash
npm install

code .
```

Press:

F5

A new VS Code window should open.

------------------------------------------------------------------------

# Phase 5 --- Create Backend Service

Go backend folder:

``` bash
cargo init
```

Create API:

Responsibilities:

- Receive tasks
- Call model
- Execute tools
- Return results

------------------------------------------------------------------------

# Phase 6 --- Connect To Ollama

Backend should:

1. Receive prompt
2. Send prompt to Ollama
3. Receive response
4. Return response

Example API flow:

``` text
Extension

↓

Backend

↓

Ollama

↓

Backend

↓

Extension
```

------------------------------------------------------------------------

# Phase 7 --- Implement Tools

Start SMALL.

## Tool 1

READ_FILE

Input:

``` text
path
```

Output:

``` text
file content
```

## Tool 2

SEARCH

Use:

ripgrep

Install:

``` bash
sudo apt install ripgrep
```

Example:

``` bash
rg "login"
```

## Tool 3

WRITE_FILE

Only after confirmation.

## Tool 4

RUN_COMMAND

Examples:

``` bash
npm test

cargo test

pytest
```

Require confirmation.

------------------------------------------------------------------------

# Phase 8 --- Add Tool Calling Loop

Workflow:

``` text
User Task

↓

Model decides tool

↓

Execute tool

↓

Return result

↓

Repeat
```

Pseudo loop:

``` text
while not finished:

ask model

execute tool

return output
```

------------------------------------------------------------------------

# Phase 9 --- Add Safety

Never allow:

- unrestricted rm
- unrestricted sudo
- background execution

Require approval for:

- file edits
- command execution
- package installs

------------------------------------------------------------------------

# Phase 10 --- Improve Accuracy

DO NOT send entire repo.

Instead:

Task

↓

Search

↓

Select files

↓

Send relevant context

------------------------------------------------------------------------

Create:

- symbol map
- dependency map
- file summaries

------------------------------------------------------------------------

# Phase 11 --- Add Diff Approval

Bad:

``` text
Agent edits directly
```

Better:

``` text
Generate patch

↓

Show diff

↓

Approve

↓

Apply
```

------------------------------------------------------------------------

# Phase 12 --- Add Testing Loop

After edits:

``` text
Build

↓

Run tests

↓

Fix failures

↓

Return result
```

------------------------------------------------------------------------

# Phase 13 --- Improve Speed

Focus on:

- Context reduction
- Fast search
- Fewer model calls

Avoid:

- giant prompts
- reading hundreds of files

------------------------------------------------------------------------

# Phase 14 --- Future Improvements

Possible additions:

- Git integration
- Planning system
- Memory
- Multi project support
- Embeddings
- Local vector DB

Do NOT start here.

------------------------------------------------------------------------

# Suggested Development Order

1. Local model works
2. Extension works
3. Chat UI works
4. Read files
5. Search
6. Edit files
7. Run commands
8. Diff approval
9. Tests
10. Better context

------------------------------------------------------------------------

# Suggested Weekly Plan

Week 1:

Environment + models

Week 2:

Extension

Week 3:

Backend + tools

Week 4:

Tool calling loop

Week 5:

Approval system

Week 6:

Testing + improvements

------------------------------------------------------------------------

# Final Advice

Optimize for:

- reliability
- visibility
- approvals
- iteration speed

Not:

- autonomy
- huge models
- complexity

Your laptop can realistically support a useful personal coding agent.
