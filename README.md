# rustbench

Small Rust coding benchmarks with exact answers, checked by `cargo`. Built to
measure coding agents from the very easy end upward: one compiler error to
fix, one failing test, one described edit, one function to write. The tasks
are std-only (no dependencies), so they build offline in seconds.

Any agent can be scored: the runner copies a task's crate to a scratch
directory, runs your agent command with the request in `RB_PROMPT`, then
copies hidden tests in and checks with `cargo build` / `cargo test` / `cargo
run`. Each task's request comes in several styles (terse, chatty, hurried),
so phrasing robustness is measured too.

```sh
cargo build --release
./target/release/rustbench list                                   # dev tasks
./target/release/rustbench run --agent ./my-agent.sh              # every dev task, every prompt style
./target/release/rustbench run --agent ./my-agent.sh --split heldout --out heldout.jsonl
./target/release/rustbench report results.jsonl
./target/release/rustbench validate --split all                   # every task fails as given and passes with its solution
```

The task format and the agent protocol are in [`docs/FORMAT.md`](docs/FORMAT.md).

## Scores

Each run is `pass`, `refused` (the agent declined instead of guessing),
`fail` or `timeout`. Reports give the pass rate per suite, kind, split and
prompt style, and a cost where a wrong change is worse than an honest "I
don't know": fail 10, refused 1, pass 0.

## Held-out tasks

Tasks with `split = "heldout"` are for measurement only. **Don't train on
them, generate training data from them, or let a learning loop's agents read
them.** Report dev and held-out scores separately.

## Suites

| suite | kind | what it asks |
|---|---|---|
| `fix-errors` | fix-error | the crate doesn't compile: fix it without changing behavior |
| `fix-tests` | fix-test | a test fails because of a planted bug: fix the code |
| `edits` | edit | a described change: rename, add a field, derive, change a signature |
| `implement` | implement | write a function from its description |
| `talk` | explain, ask | answer a question about the code exactly; ask when a request is ambiguous |
| `rustc-error-index` | fix-error | examples from rustc's error index (imported, MIT/Apache-2.0) |
