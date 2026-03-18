# Rust SDK

## Live Integration Tests

The repository root `.env` can point the Rust SDK at a dedicated live Gitea
instance for end-to-end verification.

The current live smoke suite expects at least:
- `GITEA_HOST`
- `GITEA_HTTP_PORT`
- `GITEA_SSH_PORT`
- `GITEA_USER_NAME`
- `GITEA_USER_PASS`
- `GITEA_TOKEN_VALUE`
- one of `ED25519_PUBLIC_KEY` or `RSA_PUBLIC_KEY`

Run the ignored live suite with:

```bash
cargo test --test live_integration_test -- --ignored --nocapture
```

The suite currently verifies:
- `server_version()`
- `users().get_my_info()`
- `users().list_my_public_keys()`
- public key create/get/delete lifecycle
- shared repo fixture create/delete lifecycle
- shared org fixture create/delete lifecycle when the live account is allowed to create orgs

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
