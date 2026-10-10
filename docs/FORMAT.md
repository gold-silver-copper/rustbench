# Task format

Every task is a directory `tasks/<suite>/<id>/`:

```text
tasks/fix-errors/e0384-assign-twice/
  task.toml        what the task is, how it is asked, how it is checked
  workspace/       the crate the agent starts from (Cargo.toml, src/...)
  hidden/          files copied over the workspace after the agent finishes
  solution/        files copied over the workspace that make the task pass
```

## `task.toml`

```toml
id = "e0384-assign-twice"     # unique within the suite, the directory name
suite = "fix-errors"          # the directory above
kind = "fix-error"            # see "Kinds" below
split = "dev"                 # "dev" or "heldout"
difficulty = 1                # 1 (one obvious change) .. 5 (a real feature)
summary = "a variable is assigned twice without `mut`"   # for reports, not shown to agents

[prompt]                      # the request, in several styles; the runner picks one per run
terse = "fix the build"
chatty = "Hey, this little program won't compile anymore, could you take a look and get it building again?"
hurried = "doesnt compile pls fix"

[check]
build = true                  # `cargo build` must succeed (default true)
test = true                   # `cargo test` must pass (default: true if the crate has tests after `hidden/` is applied)
stdout = "6\n"                # optional: `cargo run` must print exactly this
timeout_secs = 120            # per check command (default 120)
max_shrink = 0.2              # optional: fail if the agent removed more than 20% of the Rust code
```

Every prompt style is a full request a person might send. Styles a task
should have: `terse`, `chatty`, `hurried` (lowercase, typos, missing words).
More are welcome.

## The rules every task follows

- **Std only, edition 2024.** No dependencies, so everything builds offline
  and fast.
- **The workspace fails the check; workspace + solution passes it.** CI
  verifies both for every task (the `noop` and `reference` agents).
- **Hidden files stop shortcuts.** Tests that the fix must satisfy live in
  `hidden/` and are copied in after the agent finishes, overwriting any file
  of the same path. Visible tests in the workspace may exist too (they are
  part of the request, e.g. "make this test pass"). An agent that deletes or
  weakens a visible test still faces the hidden ones.
- **One clear intended change**, small for difficulty 1–2. Ambiguity belongs
  only in tasks of kind `ask`, where asking is the right answer.
- **Original code.** Write it fresh; don't copy from other benchmarks or
  repositories. The `imported` suites are the exception and record their
  source and license in their `task.toml` (`source = "...", license = "..."`).

## Kinds

| kind | the request | passes when |
|---|---|---|
| `fix-error` | the crate doesn't compile | it builds, and hidden tests pass |
| `fix-test` | a test fails | all visible and hidden tests pass |
| `edit` | a described change: rename, add a field, derive a trait, change a signature | it builds and the hidden tests that check the change pass |
| `implement` | write the body of something from a description | the hidden tests pass |
| `explain` | a question about the code | the answer file `ANSWER` matches `[check] answer` (exact, after trimming) |
| `ask` | an ambiguous request | the agent refuses or asks instead of changing code (`[check] expect_refusal = true`) |

## The agent protocol

The runner copies `workspace/` to a fresh directory and runs the agent
command (`--agent "..."`, run with `sh -c`) with:

| variable | value |
|---|---|
| `RB_WORK` | the working copy to change |
| `RB_PROMPT` | the request text |
| `RB_PROMPT_FILE` | the same text in a file |
| `RB_TASK` | `<suite>/<id>` (for logs only; agents must not use it to look anything up) |
| `RB_STYLE` | the prompt style used |

The agent edits files under `RB_WORK`. To refuse or ask instead of changing
code, it writes its message to `RB_WORK/.rustbench-reply` and exits 0 (any
other text it wants to say goes there too). Then the runner copies `hidden/`
over the working copy and runs the checks.

## Outcomes and scores

Each run is one of:

- `pass`: every check passed;
- `refused`: the agent declined (wrote a reply and left the code unchanged)
  and the task did not expect it (for `ask` tasks, refusing is `pass`);
- `fail`: checks failed;
- `timeout`: the agent or a check ran out of time.

Reports give the pass rate per suite, kind and split, and a cost in the
style "wrong is worse than I don't know": `fail` 10, `refused` 1, `pass` 0.
