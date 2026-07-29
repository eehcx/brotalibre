# Contributing to brotalibre

Thanks for helping improve brotalibre. Contributions should make frontend
scaffolding more reliable, easier to understand, or easier to extend.

## Development Setup

Install Rust and the tools needed by the area you are changing:

- Rust stable toolchain
- Node.js and npm
- Angular CLI for Angular scaffolding work
- A package manager supported by the Astro scaffolder for Astro work

Clone the repository and verify the baseline:

```bash
git clone https://github.com/eehcx/brotalibre.git
cd brotalibre
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
```

## Change Workflow

1. Start branches from `develop`.
2. Keep changes focused on one feature, fix, or documentation area.
3. Preserve the Angular flow when changing Astro or template infrastructure.
4. Add or update tests for behavior changes.
5. Run the relevant scaffold smoke test before opening a pull request.
6. Update the README or focused documentation when the public CLI changes.

## Template Changes

Templates are source files under `templates/` and use `.j2` files for dynamic
values. When changing a template:

- Verify the generated path and file contents.
- Cover both filesystem and embedded-template loading when relevant.
- Run `cargo test --all-targets`.
- Run `./scripts/test-scaffold.sh --quick` for Angular changes.
- Run an Astro smoke test with `--skip-install` for Astro changes.

## Pull Requests

Pull requests should include:

- A concise explanation of the user-facing behavior.
- The commands used to verify the change.
- Any generated output or screenshots that help reviewers inspect the result.
- Explicit out-of-scope items when the change prepares a future extension.

Keep commits focused and use conventional commit messages, for example:

```text
feat(astro): add localized documentation scaffolding
fix(cli): preserve Angular defaults in non-interactive mode
docs: explain DDD feature boundaries
```

## License

By contributing, you agree that your contributions are provided under the
project's [GPLv3 license](LICENSE).
