# ngseed

`ngseed` is a Rust-powered CLI to scaffold production-ready Angular apps. Generate a Clean or CDP (Core-Data-Presentation) architecture baseline with optional UI integrations in seconds.

## Features

- Creates Angular apps via Angular CLI with fixed defaults:
  - Standalone API enabled
  - Router enabled
  - SCSS styles
  - SSR disabled
  - npm package manager
- Supports architecture profiles:
  - `clean`: domain/application/infrastructure/presentation
  - `cdp`: core/data/presentation
- Clean profile starter structure:
  - `domain`
  - `application`
  - `infrastructure`
  - `presentation`
- Optional UI selection:
  - `material`
  - `primeng`
  - `none`

## Usage

```bash
ngseed new my-app
```

Non-interactive mode:

```bash
ngseed new my-app --yes --architecture cdp --ui material --package-manager pnpm
```

Flags:

- `--ui <material|primeng|none>`
- `--package-manager <npm|pnpm|yarn|bun>`
- `--architecture <clean|cdp>`
- `--skip-install`
- `--skip-git`
- `--yes`

Version:

```bash
ngseed -V
ngseed --version
```

## Requirements

- Node.js
- npm
- Angular CLI (`ng`)

## Versioning and Release Automation

This project uses:
- **Conventional Commits** for semantic intent (`feat`, `fix`, `BREAKING CHANGE`)
- **release-please** for automatic SemVer bump, changelog and `vX.Y.Z` tags
- **GitHub Actions** for CI and multi-OS release artifacts

Workflows:
- `.github/workflows/rust.yml`: quality CI (fmt, clippy, test, release build)
- `.github/workflows/release.yml`: release orchestration, binary artifacts (Linux/macOS/Windows x64) and crates.io publish

Required repository secret:
- `CRATES_IO_TOKEN`

## Publish to crates.io (manual fallback)

1. Update version in `Cargo.toml`.
2. Login once: `cargo login`.
3. Dry run: `cargo publish --dry-run`.
4. Publish: `cargo publish`.

## License

GPL-3.0-only. See [`LICENSE`](./LICENSE).
