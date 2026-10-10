#!/usr/bin/env bash
# Reference agent: Claude Code headless, working in the task's copy.
# usage: rustbench run --agent agents/claude.sh
# Uses your Claude Code login (an API key in the environment is ignored).
# Model: CLAUDE_MODEL (default claude-opus-5-5).
set -euo pipefail
cd "$RB_WORK"
env -u ANTHROPIC_API_KEY claude -p \
  --model "${CLAUDE_MODEL:-claude-opus-5-5}" \
  --no-session-persistence \
  --permission-mode acceptEdits \
  --allowedTools "Read Edit Write Glob Grep Bash(cargo build:*) Bash(cargo test:*) Bash(cargo run:*) Bash(cargo check:*)" \
  --append-system-prompt "You are working in a small Rust crate in the current directory. Do what the user asks by editing files here. If the request is ambiguous or you should not guess, change nothing and write your question to the file .rustbench-reply. If asked a question about the code, write just the answer to the file ANSWER." \
  < "$RB_PROMPT_FILE" > /dev/null
