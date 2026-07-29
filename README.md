# brotalibre

Scaffold production-ready Angular v22 projects with Clean Architecture or DDD vertical slicing, and generate features via `brota generate feature`.

```bash
brota new my-app
brota generate feature user --architecture clean --fields name:string,email:string
```

## Features

- **Clean Architecture** — shared `domain/`, `application/`, `infrastructure/`, `presentation/` layers
- **DDD (Domain-Driven Design)** — vertical slicing under `src/app/features/<name>/`
- **Feature generator** — `brota generate feature` scaffolds entity, value objects, repository port/impl, signalStore, use cases, DTOs + mappers
- **Default users CRUD** — every new project includes a mock-backed `users` feature with lazy routes, forms, detail view, and CRUD actions
- **signalStore-only** — no NgRx classic actions, no facades
- **UI Integrations** — Angular Material, PrimeNG, or none
- **CSS Frameworks** — TailwindCSS v4 or none
- **Zero config** — get a working Angular v22 project in seconds

## Install

```bash
cargo install --locked brotalibre
sudo apt install ./brotalibre_<version>-1_amd64.deb
sudo dnf install ./brotalibre-<version>-1.x86_64.rpm
brew install eehcx/tap/brotalibre
```

See [`docs/distribution.md`](docs/distribution.md) for release channels, checksums, and Homebrew tap setup.

## Usage

### Create a new project

```bash
brota new my-app --yes
brota new my-app --architecture ddd --ui material
brota new my-app --architecture clean --styles tailwindcss
brota new my-app --yes --skip-install --skip-git
```

### Generate a feature

```bash
# Clean architecture: files go to src/app/<name>/{domain,application,infrastructure}/
brota generate feature user --architecture clean --fields name:string,email:string,age:number

# DDD: files go to src/app/features/<name>/{domain,application,infrastructure}/
brota generate feature product --architecture ddd --prefix /api/products --fields name:string,price:number

# Generate into a specific project (default: current dir)
brota generate feature user --architecture clean --project-dir ./my-app --fields name:string
```

Flags for `generate feature`:

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `<NAME>` (positional) | kebab-case string | required | Feature name |
| `--architecture` | `clean` / `ddd` | `clean` | Architecture profile |
| `--ui` | `material` / `primeng` / `none` | `none` | Presentation templates to render |
| `--styles` | `css` / `scss` / `sass` / `less` / `tailwindcss` | `css` | When `--ui none` is used, Tailwind presentation templates require `tailwindcss` |
| `--prefix` | string | `api` | API prefix for the repository base URL |
| `--fields` | comma-separated `name:type` | none | Entity fields |
| `--project-dir` | path | cwd | Target project root |

## Architecture

### Clean (layer-based)

```
src/app/
└── <feature>/
    ├── domain/
    ├── application/
    ├── infrastructure/
    └── presentation/
```

### DDD (vertical slice)

```
src/app/
└── features/
    └── <feature>/
        ├── domain/
        │   ├── value-objects/
        │   │   └── <feature>-id.vo.ts
        │   ├── <feature>.entity.ts
        │   ├── <feature>-repository.port.ts
        │   └── <feature>.errors.ts
        ├── application/
        │   ├── <feature>.store.ts
        │   └── {getall,getbyid,create,update,delete}-<feature>.use-case.ts
        └── infrastructure/
            ├── dto/
            │   ├── <feature>.request.dto.ts
            │   └── <feature>.response.dto.ts
            ├── mappers/
            │   └── <feature>.mapper.ts
            ├── <feature>.repository.ts
            └── <feature>.provider.ts
```

## Testing

Run `./scripts/test-scaffold.sh` to validate project generation against real output. See [`docs/testing.md`](docs/testing.md) for the full reference.

```bash
./scripts/test-scaffold.sh --architecture clean --quick
./scripts/test-scaffold.sh --architecture ddd --quick --keep
./scripts/test-scaffold.sh --all --quick
```
