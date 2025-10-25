# giacometti

[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/lmmx/giacometti/master.svg)](https://results.pre-commit.ci/latest/github/lmmx/giacometti/master)

Git framework for security hardening

## The Vision

1. CLI usage

```sh
gcmti release --bump minor --backend github-api
```

2. In GHA (compiled Rust binary)

```sh
- uses: lmmx/giacometti@v1
  with:
    bump: minor
    backend: github-api  # or 'git' or 'gitoxide'
    token: ${{ secrets.GITHUB_TOKEN }}
```

## Rationale


1. **Multiple backends** - Git operations support 3 backends (`git`, `gitoxide`, `github-api`). uv operations support 2 backends (`shell`, `rust_crate`).
2. **Standardised** - Allows release process standardisation rather than ad-hoc YAML scripting.
3. **Hardened CI releases** - Applies principle of least privilege to git operations in automated workflows.

Transparent CLI control of a Rust binary is easier to reason about than shell scripts in CI workflows.

## Installation

```sh
cargo binstall giacometti
```

## About

Git release framework designed for security hardening.

Refer to the [Giacometti docs](https://docs.rs/giacometti) for more info.

## Contributing

Maintained by [lmmx](https://github.com/lmmx). Contributions welcome!

1. **Issues & Discussions**: Please open a GitHub issue or discussion for bugs, feature requests, or questions.
2. **Pull Requests**: PRs are welcome!
   - Run tests (`just test`) and include updates to docs or examples if relevant.
   - If reporting a bug, please include the version and the error message/traceback.

## License

Licensed under the [MIT License](https://github.com/lmmx/giacometti/blob/master/LICENSE).
