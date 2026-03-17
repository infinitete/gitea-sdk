# Rust SDK

## Git Hooks

This repository uses a local Git `pre-commit` hook under `.githooks/pre-commit`.

Enable it with:

```bash
git config core.hooksPath .githooks
```

Before each commit, the hook runs:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```
