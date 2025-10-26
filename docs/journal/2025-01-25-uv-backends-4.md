# OUTLOOK

## Current State

- Backend architecture established (git + uv)
- Shell backends functional (git/shell.rs, uv/shell.rs)
- Release workflow works for Python projects (git/release.rs)
- Policy enforcement exists (policy.rs) with basic allowlist
- Tests passing for implemented features

## Stubbed

- gitoxide git backend (`backends/git/gitoxide.rs`)
- github-api git backend (`backends/git/github_api.rs`)
- rust-crate uv backend (`backends/uv/rust_crate.rs`)

## Missing

- Release command not exposed in CLI (main.rs only wraps git commands)
- Policy enforcement minimal (no least privilege implementation)
- Release workflow only handles Python tags (`py-*`), not Rust
- GitHub Action implementation
- Verified commits via github-api backend

## Vision Gap

- README shows `gcmti release --bump minor --backend github-api`
- CLI currently doesn't accept release subcommand
- README shows GHA usage with `uses: lmmx/giacometti@v1`
- No GitHub Action exists
