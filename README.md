# brotalibre

Frontend scaffolding from architecture to a runnable project foundation.

[![status: alpha](https://img.shields.io/badge/status-alpha-orange.svg)](https://github.com/eehcx/brotalibre/releases)
[![Crates.io](https://img.shields.io/crates/v/brotalibre?logo=rust)](https://crates.io/crates/brotalibre)
[![Downloads](https://img.shields.io/crates/d/brotalibre)](https://crates.io/crates/brotalibre)
[![Docs.rs](https://img.shields.io/docsrs/brotalibre?logo=docs.rs)](https://docs.rs/brotalibre)
[![CI](https://github.com/eehcx/brotalibre/actions/workflows/rust.yml/badge.svg)](https://github.com/eehcx/brotalibre/actions/workflows/rust.yml)
[![Release](https://github.com/eehcx/brotalibre/actions/workflows/release.yml/badge.svg)](https://github.com/eehcx/brotalibre/actions/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/eehcx/brotalibre?logo=github)](https://github.com/eehcx/brotalibre/releases)
[![Homebrew](https://img.shields.io/badge/homebrew-eehcx%2Ftap%2Fbrotalibre-orange?logo=homebrew)](https://github.com/eehcx/homebrew-tap)
[![License](https://img.shields.io/github/license/eehcx/brotalibre)](LICENSE)


`brotalibre` turns frontend architecture decisions into runnable project foundations and repeatable feature scaffolds. It currently supports Angular applications and Astro documentation sites, with additional frontend targets planned as the template system grows.

```bash
brota new my-app
brota new docs-site --framework astro --docs-engine starlight --i18n en,es
brota generate feature user --fields name:string,email:string
```

## Quick start

```bash
# Interactive: choose Angular or Astro, then answer the relevant prompts
brota new my-app

# Angular without prompts
brota new my-app --yes

# Astro Docs without prompts
brota new docs-site --framework astro --yes
cd docs-site
npm run dev
```

## Features

- **Clean Architecture** — shared `domain/`, `application/`, `infrastructure/`, `presentation/` layers
- **DDD (Domain-Driven Design)** — vertical slicing under `src/app/features/<name>/`
- **Feature generator** — `brota generate feature` scaffolds entity, value objects, repository port/impl, signalStore, use cases, DTOs + mappers
- **Default users CRUD** — every new Angular project includes a mock-backed `users` feature with lazy routes, forms, detail view, and CRUD actions
- **signalStore-only** — no NgRx classic actions, no facades
- **UI Integrations** — Angular Material, PrimeNG, or none
- **CSS Frameworks** — TailwindCSS v4 or none
- **Astro documentation** — Starlight or native Astro with localized routes and content
- **Interactive scaffolding** — choose Angular or Astro when `--framework` is omitted
- **Zero config** — get a working Angular v22 project in seconds

## Install

```bash
cargo install --locked brotalibre
sudo apt install ./brotalibre_<version>-1_amd64.deb
sudo dnf install ./brotalibre-<version>-1.x86_64.rpm
brew install eehcx/tap/brotalibre
```

See [`docs/distribution.md`](docs/distribution.md) for release channels, checksums, and Homebrew tap setup.

## Requirements

- Rust toolchain for building from source.
- Node.js and a package manager (`npm`, `pnpm`, `yarn`, or `bun`) for generated projects.
- Angular CLI for Angular scaffolding; Astro CLI is fetched by the selected package manager.

## Documentation

| Document | Purpose |
|----------|---------|
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Development setup, template changes, tests, and pull requests |
| [`docs/architecture/clean.md`](docs/architecture/clean.md) | Clean Architecture rules and generated structure |
| [`docs/architecture/ddd.md`](docs/architecture/ddd.md) | DDD boundaries and vertical-slice structure |
| [`docs/testing.md`](docs/testing.md) | Angular scaffold test reference |
| [`docs/distribution.md`](docs/distribution.md) | Release channels, packages, checksums, and Homebrew |
| [`CHANGELOG.md`](CHANGELOG.md) | Version history |

## Usage

### Create a new project

```bash
# Interactive: choose Angular or Astro
brota new my-app

# Angular defaults: no prompts
brota new my-app --yes
brota new my-app --architecture ddd --ui material
brota new my-app --architecture clean --styles tailwindcss
brota new my-app --yes --skip-install --skip-git
```

### Create an Astro documentation site

```bash
# Starlight documentation with English and Spanish content
brota new docs-site \
  --framework astro \
  --docs-engine starlight \
  --i18n en,es

# Native Astro documentation without installing dependencies or initializing git
brota new docs-site \
  --framework astro \
  --docs-engine native \
  --i18n en,es \
  --yes \
  --skip-install \
  --skip-git
```

When `--framework astro` is used interactively, the CLI asks for the
documentation engine and locales. With `--yes`, the defaults are Starlight,
English (`en`), and npm. Astro scaffolding requires Node.js and the selected
package manager.

Flags for `new`:

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `<PROJECT_NAME>` (positional) | project name | required | Directory to create |
| `--framework` | `angular` / `astro` | interactive; Angular with `--yes` | Target framework |
| `--docs-engine` | `starlight` / `native` | Starlight for Astro | Astro documentation engine |
| `--i18n` | comma-separated locales | `en` for Astro with `--yes` | Astro documentation locales |
| `--package-manager` | `npm` / `pnpm` / `yarn` / `bun` | npm with `--yes` | Package manager used for scaffolding |
| `--architecture` | `clean` / `ddd` | `clean` | Angular application architecture |
| `--ui` | `material` / `primeng` / `none` | `none` | Angular UI integration |
| `--styles` | `css` / `scss` / `sass` / `less` / `tailwindcss` | `css` | Angular styles option |
| `--skip-install` | flag | off | Skip dependency installation |
| `--skip-git` | flag | off | Skip git initialization |
| `--yes` | flag | off | Accept defaults and skip prompts |

### Generate a feature

`generate feature` is currently Angular-only. It remains focused on CRUD and
application features; Astro documentation content is created through
`brota new --framework astro`.

```bash
# Uses the architecture, UI library, and style engine from brota.yaml.
brota generate feature user --fields name:string,email:string,age:number

# Short alias for generate feature.
brota g f product --prefix /api/products --fields name:string,price:number

# Generate into a specific project (default: current dir)
brota generate feature user --project-dir ./my-app --fields name:string
```

The project must contain a valid `brota.yaml`. `generate feature` uses it as the
single source of truth for architecture, UI library, and style engine.

Flags for `generate feature`:

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `<NAME>` (positional) | kebab-case string | required | Feature name |
| `--prefix` | string | `api` | API prefix for the repository base URL |
| `--fields` | comma-separated `name:type` | none | Entity fields |
| `--project-dir` | path | cwd | Target project root |

## Architecture guides

Choose an architecture according to the coupling and ownership boundaries
your application needs. The CLI applies the selected profile to Angular
projects and feature generation.

| Architecture | Best fit | Generated layout | Guide |
|--------------|----------|------------------|-------|
| Clean Architecture | Feature-oriented applications with explicit dependency direction | `src/app/features/<feature>/{domain,application,infrastructure,presentation}` | [`docs/architecture/clean.md`](docs/architecture/clean.md) |
| Domain-Driven Design | Feature teams and vertical slices with strong domain ownership | `src/app/features/<feature>/{domain,application,infrastructure,presentation}` | [`docs/architecture/ddd.md`](docs/architecture/ddd.md) |

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the development workflow and
[`docs/testing.md`](docs/testing.md) for scaffold verification.

## Testing

Run the Rust test suite and scaffold checks before contributing. The scaffold script currently validates Angular project generation and feature output; Astro generation is covered by the Rust tests and should also be smoke-tested with `--skip-install`.

```bash
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
./scripts/test-scaffold.sh --architecture clean --quick
./scripts/test-scaffold.sh --architecture ddd --quick --keep
./scripts/test-scaffold.sh --all --quick

# Astro smoke test
cargo run -- new docs-site \
  --framework astro \
  --docs-engine starlight \
  --i18n en,es \
  --yes \
  --skip-install \
  --skip-git
```

See [`docs/testing.md`](docs/testing.md) for the Angular scaffold test reference.

## License

brotalibre is distributed under the GNU General Public License v3.0. See
[`LICENSE`](LICENSE) for the complete terms.
