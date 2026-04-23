# Build And Script Rules

## Rule

For build, check, lint, test, run, migration, and codegen operations in this repository:

- This is a Rust workspace. Prefer `cargo` commands, not frontend package-manager workflows.
- Prefer repo-root workspace commands such as `cargo check`, `cargo clippy`, and `cargo test`.
- For crate-scoped work, prefer `cargo <cmd> -p <crate>`.

## Required Before Finishing

Before considering a code change complete, AI must make sure both of these pass unless the user explicitly says not to run them or the environment blocks execution:

```bash
cargo check
cargo clippy
```
