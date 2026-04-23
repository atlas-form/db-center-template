# Build And Script Rules

## Rule

For build, check, lint, test, run, migration, and codegen operations in this repository:

- This is a Rust workspace. Prefer `cargo` commands, not frontend package-manager workflows.
- Before other Rust build or verification commands, run `cargo fmt` first unless the user explicitly says not to.
- Prefer repo-root workspace commands such as `cargo check`, `cargo clippy`, and `cargo test`.
- For crate-scoped work, prefer `cargo <cmd> -p <crate>`.

## Required Before Finishing

Before considering a code change complete, AI must make sure all of these are handled unless the user explicitly says not to run them or the environment blocks execution:

```bash
cargo fmt
cargo check
cargo clippy
```

- After API changes, sync the Markdown API docs under `API_CONTRACTS/` before finishing.
