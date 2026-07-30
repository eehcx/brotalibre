# Target Generation Architecture

This document defines the intended architecture for BrotaLibre's generator. It
is a target design, not a description of every current command or template.
Read [the product vision](../product/vision.md), [the roadmap](../product/roadmap.md),
and [the UI contract](ui-contract.md) together with this document.

## Design Goal

BrotaLibre must transform stable product intent into framework-specific code
without making OpenAPI parsers, configuration files, architecture profiles, and
UI templates depend directly on one another.

```text
brota.yaml / OpenAPI / command input
                |
                v
        normalization and validation
                |
                v
          ApplicationModel
                |
                v
 profile + framework + architecture + UI packs
                |
                v
        generated project and .brota metadata
```

The `ApplicationModel` boundary is essential. Input parsers create it; packs
consume it. A renderer must never need to interpret raw OpenAPI itself.

## Product Models and Profiles

The three product outputs share configuration infrastructure but require
different product models.

| Profile | Product model | First renderer target |
|---|---|---|
| `angular-admin` | Resources, operations, forms, navigation, and permissions | Angular admin application |
| `astro-landing` | Sections, navigation, calls to action, and theme | Astro landing page |
| `astro-docs` | Content, navigation, locales, and documentation metadata | Astro documentation site |

Profiles choose coherent defaults. They prevent an early combinatorial matrix
of framework, architecture, UI library, state, API transport, layouts, and
page types. Advanced configuration may override a profile only where that
override has a clear, testable meaning.

## Planned Project Configuration

`brota.yaml` is the planned persistent source of project-generation decisions.
The following is a target shape, not a final public schema:

```yaml
schemaVersion: "1"

project:
  name: inventory-console
  description: Inventory administration panel
  locale: es

profile: angular-admin

target:
  framework: angular
  architecture: feature-clean
  packageManager: pnpm

application:
  initialRoute: welcome

ui:
  library: native
  styleEngine: scss
  density: comfortable

theme:
  primary: "#6750A4"
  radius: 8
  fontFamily: Inter

demo:
  enabled: true
  resource: users

generation:
  mockData: true
  tests: true
  overwrite: safe

templates:
  overrides: ./brota-templates

source:
  type: openapi
  file: ./openapi.yaml
  baseUrl: /api
```

The configuration contains decisions that change the generated product. It
must not expose internal template details such as individual CSS declarations.
One file is easier to validate, version, and share with coding agents; file
composition can be considered after the initial schema is stable.

Template overrides are the controlled escape hatch for deep customization. An
override must retain an explicit pack and file-ownership boundary so it does not
make later generation or validation ambiguous.

## Application Model

The internal model represents the application independently of its inputs and
outputs. Its exact Rust types remain an implementation decision, but it must
cover concepts equivalent to the following:

```text
ApplicationModel
  project
  productType
  resources
  operations
  authentication
  navigation
  theme
  generationPolicy

Resource
  name
  fields
  capabilities
  relationships

Operation
  resource
  kind
  transport
  parameters
  requestSchema
  responseSchema
```

For the first OpenAPI beta, model only the supported REST subset. Unsupported
constructs must produce clear diagnostics rather than silently generating
incorrect code. Future OpenAPI support can grow to references, compositions,
recursive schemas, multipart bodies, and richer security only after their
semantics are represented deliberately.

## Feature-Clean Architecture

The target Angular application uses vertical features with Clean Architecture
dependency direction:

```text
features/<feature>/
  domain/          entities, value objects, errors, ports
  application/     use cases and feature state
  infrastructure/  HTTP, DTOs, mappers, providers, adapters
  presentation/    routes, components, forms, view-specific state
```

Required rules:

- Domain code has no Angular, HTTP, or concrete-adapter dependency.
- Application code depends on domain contracts, not concrete repositories.
- Infrastructure implements ports owned by the domain or application client.
- Presentation calls application use cases and does not construct HTTP
  repositories directly.
- Cross-feature dependencies are explicit, narrow, and validated.

The profile includes layout, dependency rules, generated artifacts, tests,
validation rules, documentation, and eventual migrations. Renaming `clean` or
`ddd` folders without changing these behaviors is not an architecture profile.

## Packs and Renderers

Packs translate the normalized model into code. A target pack boundary should
look conceptually like this:

```text
framework/angular
architecture/feature-clean
ui/native
ui/material
ui/primeng
docs/astro
```

`ui/native`, `ui/material`, and `ui/primeng` are UI renderers. CSS, SCSS, and
Tailwind are style-engine decisions. They must not be conflated: a native UI
can be styled with CSS or Tailwind, and a Material UI can still use project
theme tokens and SCSS.

Only packs with a verified contract should be exposed as supported. PrimeNG and
community packs remain deferred until the native and Material paths are solid.

## Generated-File Ownership and Synchronization

Safe regeneration is more important than a sophisticated terminal interface.
BrotaLibre will prepare for it without promising complete synchronization in
the first OpenAPI beta.

The intended ownership boundary is hybrid:

```text
feature/
  generated/       replaceable output owned by BrotaLibre
  domain/          generated foundation with explicit extension points
  presentation/    user-owned extensions and composition
  feature.config.ts
```

Generation metadata belongs under `.brota/` and should eventually record:

- Generator and schema versions.
- Source document identity and normalized model snapshot.
- Selected profile, architecture, and renderer packs.
- Generated file manifest and hashes.
- Generation timestamp and policy.

This foundation enables later `brota diff` and `brota sync`. Generated regions
inside arbitrary human-edited files are intentionally avoided because they are
fragile and make safe ownership unclear.

## Target Command Surface

The desired command surface is deliberately staged. These names are direction,
not a claim that they are all available today.

```bash
brota init
brota plan
brota generate
brota import openapi ./openapi.yaml
brota add resource product
brota validate
brota doctor
brota context
```

`brota plan` explains what generation would create before writing files.
`brota context` emits machine-readable and human-readable project context for
developers and coding agents. `brota validate` checks configuration,
architecture, UI contracts, and supported API assumptions.

## Developer and Agent Context

The future `brota context` command should expose the selected project contract
without relying on a hosted AI integration. Its target outputs are:

```text
.brota/context.json
.brota/architecture.md
.brota/ui-guidelines.md
```

They describe the selected profile, architecture rules, generated and
user-owned paths, available UI components, routes, resources, naming
conventions, and validation commands. These are generated-project artifacts;
the BrotaLibre product vision remains in this repository's versioned docs and
is recalled by Engram for OpenCode.

## Validation and Determinism

Validation is a product boundary, not an optional utility. The eventual command
must report the violated rule, relevant location, expected dependency direction
or contract, and a practical next step. It may recommend a correction, but it
must not claim to understand or automatically repair every architectural
decision.

Generation must be deterministic for the same configuration, source model, and
pack versions. Tests must cover model normalization, pack rendering,
architecture boundaries, generated-file ownership, and real scaffold builds.
