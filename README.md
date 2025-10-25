# giacometti

[![License](https://img.shields.io/pypi/l/giacometti.svg)](https://pypi.python.org/pypi/giacometti)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/lmmx/giacometti/master.svg)](https://results.pre-commit.ci/latest/github/lmmx/giacometti/master)

Git framework for security hardening

## The Vision

1. CLI usage

```sh
gcmti release --bump minor --backend github-api
```

2. In GHA (no JS actions, pure compiled binary)

```sh
- uses: lmmx/giacometti@v1
  with:
    bump: minor
    backend: github-api  # or 'git' or 'gitoxide'
    token: ${{ secrets.GITHUB_TOKEN }}
```

## Rationale

Why you’d bother with such a contortion just to achieve the same as ad-hoc YAML is that you can generalise releasing as a trivial function of metadata.
Risk of making a mistake is higher if these are always one offs (turning it into a program and then from there externalising program config as TOML
makes it easier and reduces context switching). When it’s trivial to copy to new repos you do it more.

1. **3 backends** - Choose based on environment/needs:
   - `git` - Shell out (simple, works everywhere git is available)
   - `gitoxide` - Pure Rust (no git binary needed)
   - `github-api` - Direct API calls (used in the CI GitHub Action, Verified commits)
2. **Standardised** - Allows release process standardisation rather than ad-hoc YAML scripting.
3. **Hardened CI releases** - Giacometti is designed to harden the security posture of CI release automation:
   - The same way we apply package-wide linter settings that prevent mistakes, we should be able to
     apply a principle of least privilege to git operations that are done via automated CI workflows.
   - The ad-hoc nature of CI release may not pose a security risk in itself, but a lack of automation
     and easily intelligible/portable configuration around it prevents clarity around it (which does).

I think this would lead to a more natural user experience when it comes to understanding how CI
works: a GitHub Action with transparent CLI flag-equivalent control of a Rust binary's config would
be naturally much easier to follow in Rust than shell or other languages. I see a trend in the Python
ecosystem towards devtools written in Rust in part because they are very easy to reason about and
perform excellently.

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
