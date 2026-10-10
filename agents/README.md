# Agents

An agent is any command. `rustbench run --agent "<command>"` runs it with `sh -c`
in an environment that holds the request (see `docs/FORMAT.md`, "The agent protocol").

- `noop` and `reference` are built in: they change nothing / apply the task's solution
  (the floor and the ceiling; `rustbench validate` uses both).
- `claude.sh`: Claude Code in headless mode, as a strong LLM baseline.
