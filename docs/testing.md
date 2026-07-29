# Testing brotalibre

`scripts/test-scaffold.sh` generates a real Angular project in an isolated tempdir using the locally built `brota` binary, then validates that the expected files exist and contain the expected content. It also exercises the `brota generate feature` subcommand and verifies the full feature tree is produced.

## Prerequisites

- **Rust toolchain** — required to build `brota` (the script runs `cargo build -q` once at start)
- **Node.js + npm** — only required for `--build` / `--serve` modes (skipped in `--quick`)
- **Angular CLI** — installed automatically via `npm install` during `--build`; not needed for `--quick`

## Quick start

```bash
# Fastest: validate structure only, no npm install
./scripts/test-scaffold.sh --architecture clean --quick

# Same for DDD
./scripts/test-scaffold.sh --architecture ddd --quick

# Keep the generated project for manual inspection
./scripts/test-scaffold.sh --architecture clean --quick --keep
```

After `--keep`, the project lives under `/tmp/tmp.XXX/brotalibre-<arch>-<ui>-<styles>/`. The tempdir is preserved so you can inspect the tree and file contents.

## CLI reference

| Flag | Values | Default | Description |
|------|--------|---------|-------------|
| `--architecture` | `clean` / `ddd` | `clean` | Architecture profile to test |
| `--ui` | `none` / `material` / `primeng` | `none` | UI library to integrate |
| `--styles` | `none` / `tailwindcss` | `none` | CSS framework to integrate |
| `--package-manager` | `npm` / `pnpm` / `yarn` / `bun` | `npm` | Package manager passed to `ng new` |
| `--skip-git` | (flag) | off | Pass `--skip-git` to `brota new` |
| `--build` | (flag) | off | Run `npm install` + `npx ng build` after generation |
| `--serve` | (flag) | off | Run `npx ng serve --host 0.0.0.0` (implies `--build`) |
| `--keep` | (flag) | off | Don't delete tempdir after the test |
| `--all` | (flag) | off | Run all 12 architecture × UI × styles combinations |
| `--quick` | (flag) | off | Structure check only, skip npm (overrides `--build`) |

## What the script validates

### 1. `brota new` completes

The binary runs with `--yes --skip-install` and must exit 0. The project directory must exist afterwards.

### 2. Angular basics

Required files exist:
- `angular.json`, `tsconfig.json`, `package.json`
- `src/main.ts`, `src/index.html`, `src/styles.scss`
- `src/app/app.config.ts` — must contain `provideRouter`
- `src/app/app.ts` (or `app.component.ts` on older Angular) — must use `styleUrl` (singular)

### 3. Architecture-specific structure

**Clean:**
- `src/app/app.ts` has a `readonly title` property
- `src/app/app.html` contains the `brotalibre` branding
- `src/app/app.config.ts` has `provideRouter`

**DDD:**
- `src/app/features/` directory exists
- `src/app/app.ts`, `app.html`, `app.config.ts` exist

### 4. UI integration

**material:**
- `@angular/material` present in `package.json`

**primeng:**
- `primeng` and `primeicons` present in `package.json`
- `@primeng/themes` referenced in `angular.json`

### 5. Styles

**tailwindcss:**
- `postcss.config.js` exists with `@tailwindcss/postcss` plugin
- `src/styles.scss` imports `tailwindcss`

### 6. `brota generate feature` (new)

After project generation, the script runs:

```bash
brota generate feature test-entity \
  --architecture <arch> \
  --prefix api \
  --fields name:string,age:number,email:string \
  --project-dir <PROJECT_DIR>
```

Then validates that 15 files exist in the correct location:

- **Clean:** files under `src/app/test-entity/{domain,application,infrastructure}/`
- **DDD:** files under `src/app/features/test-entity/{domain,application,infrastructure}/`

| Layer | File |
|-------|------|
| domain | `test-entity.entity.ts` |
| domain | `test-entity-repository.port.ts` |
| domain | `test-entity.errors.ts` |
| domain | `value-objects/test-entity-id.vo.ts` |
| application | `test-entity.store.ts` |
| application | `getall-test-entity.use-case.ts` |
| application | `getbyid-test-entity.use-case.ts` |
| application | `create-test-entity.use-case.ts` |
| application | `update-test-entity.use-case.ts` |
| application | `delete-test-entity.use-case.ts` |
| infrastructure | `dto/test-entity.request.dto.ts` |
| infrastructure | `dto/test-entity.response.dto.ts` |
| infrastructure | `mappers/test-entity.mapper.ts` |
| infrastructure | `test-entity.repository.ts` |
| infrastructure | `test-entity.provider.ts` |

The generated feature also includes a mock repository adapter. Use `--ui material` or `--ui primeng` to render those component sets explicitly. Use `--styles tailwindcss` with `--ui none` to render the Tailwind-style presentation templates; plain CSS leaves presentation optional.

## Modes explained

### `--quick` (default for fast iteration)

Skips npm entirely. Runs `cargo build -q` once, generates the project, validates file structure, runs `brota generate feature`, validates feature tree, deletes tempdir (unless `--keep`). Total runtime: a few seconds.

### `--build`

After structure validation, runs `npm install --silent` and then `npx ng build`. If TypeScript compilation fails, reports `✗ ng build failed` and counts the test as failed. Useful for catching template/type errors that structure checks can't detect.

### `--serve`

After a successful build, runs `npx ng serve --host 0.0.0.0` in the foreground. The script prints the browser URL (`http://localhost:4200/`). Press `Ctrl+C` to stop; a trap handler cleans up the tempdir on exit. Use this to visually inspect the generated app.

### `--all`

Runs the full 12-combination matrix:

```
clean × {none, material, primeng} × {none, tailwindcss}
ddd   × {none, material, primeng} × {none, tailwindcss}
```

`--serve` is silently ignored in `--all` mode (would launch 12 servers). Pair with `--quick` for fast full-matrix validation.

## Inspecting the generated project

With `--keep`, the tempdir survives. Useful inspection commands:

```bash
# Find the most recent generated project
PROJECT=$(ls -d /tmp/tmp.*/brotalibre-* | tail -1)

# Full file tree (excluding node_modules and git)
find "$PROJECT" -type f -not -path '*/node_modules/*' -not -path '*/.git/*' | sort

# Compare Clean vs DDD layout
tree "$PROJECT/src/app" -I 'node_modules'

# Read a generated file
cat "$PROJECT/src/app/domain/test-entity.entity.ts"

# Diff two generation runs
diff -r /tmp/tmp.AAA/brotalibre-clean-none-none/src/app \
        /tmp/tmp.BBB/brotalibre-ddd-none-none/src/app
```

## Exit codes

- `0` — all tests passed (or only `--serve` was used and exited cleanly)
- `1` — at least one test failed; see the `✗` markers in the output and the final summary line

## Output legend

- `✓` green — assertion passed
- `✗` red — assertion failed (increments the error counter)
- `→` cyan — info / progress message
- `⚠` yellow — warning (non-fatal, e.g. `--serve` ignored in `--all` mode)
- `⏸` yellow — test paused, keeping tempdir (`--keep`) or handing off to serve mode

## Troubleshooting

### "brota generate feature failed"

The script suppresses stderr by default. To see the real error, run the subcommand manually:

```bash
TMPDIR=$(mktemp -d)
ln -s "$PWD/templates" "$TMPDIR/templates"
"$PWD/target/debug/brota" new test-app --yes --architecture clean --skip-install
(cd "$TMPDIR" && "$PWD/target/debug/brota" generate feature user \
  --architecture clean --fields name:string --project-dir "$TMPDIR/test-app")
```

### "Template not found: template XYZ does not exist"

The `brota` binary first looks for `templates/angular/...` beside its executable, as provided by a release archive. For development it falls back to `cwd/templates/angular/...`. If neither location exists, it uses the templates embedded in the binary, which keeps `cargo install brotalibre` functional.

### "brota new command failed"

Likely a `cargo build` failure. Run `cargo build` directly to see the compilation error.

### Type errors in generated files (kebab-case identifiers like `test-entity`)

If you see `import { test-entityId } from ...` (invalid TypeScript identifier), the PascalCase conversion in `src/infrastructure/seeder/templates/clean.rs` or `ddd.rs` is broken. The dispatcher must compute `name_pascal` from `name_kebab` before passing it to the template context as `name`.

### `--all` takes too long

Add `--quick` to skip all npm operations. The full 12-combination matrix in `--quick` mode finishes in well under a minute.
