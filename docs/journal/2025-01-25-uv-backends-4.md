# 2025-01-25: UV Backends

## Current State

- Backend architecture established (git + uv)
- Shell backends functional (git/shell.rs, uv/shell.rs)
- Release workflow works for Python projects (git/release.rs)
- Policy enforcement exists (policy.rs) with basic allowlist
- Tests passing for implemented features

## Stubbed

- gitoxide git backend (backends/git/gitoxide.rs)
- github-api git backend (backends/git/github_api.rs)
- rust-crate uv backend (backends/uv/rust_crate.rs)

## Missing

- Release command not exposed in CLI (main.rs only wraps git commands)
- Policy enforcement minimal (no least privilege implementation in policy.rs)
- Release workflow only handles Python tags prefixed with `py-*`, not Rust tags (git/release.rs)
- GitHub Action implementation (no action.yml in repository)
- Verified commits via github-api backend (backends/git/github_api.rs returns stub)

## Divergence

- README shows `gcmti release --bump minor --backend github-api` but CLI doesn't accept release subcommand (main.rs only wraps git commands)
- README shows GHA usage with `uses: lmmx/giacometti@v1` but no action.yml exists in repository
