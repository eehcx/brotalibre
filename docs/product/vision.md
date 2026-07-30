# BrotaLibre Product Vision

BrotaLibre is a local frontend specification compiler. It turns explicit product,
architecture, API, and visual decisions into a coherent frontend foundation that
developers can extend safely.

This document is the durable product direction. It describes the intended
product, not a claim that every capability is available in the current release.
For delivery boundaries, read [the roadmap](roadmap.md). For the target
implementation model, read [the generation architecture](../architecture/target-generation.md)
and [the UI contract](../architecture/ui-contract.md).

## Product Definition

BrotaLibre receives a description of what must be built and produces a
functional, consistent, and structured frontend without forcing every project
to recreate the same infrastructure manually.

```text
Product specification + API description + selected profile
                         |
                         v
                 Application model
                         |
                         v
       Framework, architecture, and UI renderers
                         |
                         v
              Extensible frontend foundation
```

The primary user is a developer. A coding agent is also an important consumer
of the resulting specification: it should be able to discover the selected
architecture, allowed dependencies, UI rules, routes, models, and validation
commands instead of guessing them.

## Product Outputs

BrotaLibre will support three product types. They share configuration concepts,
themes, and template infrastructure, but they are not interchangeable variants
of one generator.

| Output | Initial framework | First purpose | Direction |
|---|---|---|---|
| Administrative web application | Angular | CRUD-oriented admin panels | Primary product path |
| Landing page | Astro | Marketing and presentation sites | Planned after the Angular path is solid |
| Documentation site | Astro | Product and technical documentation | Supported product area, kept separate from CRUD generation |

An admin application models resources, operations, permissions, forms, and
navigation. A landing page models sections, calls to action, and conversion
paths. A documentation site models content, navigation, search, and locales.
Each therefore needs its own product model even when shared generators and
themes are reused.

## Product Principles

### Local-first and private by design

BrotaLibre must run locally and must not require a hosted BrotaLibre service or
an AI provider to process a project's specification. Configuration, API
descriptions, generated context, and validation remain under the developer's
control.

This principle does not promise that package installation or a generated
application's backend API works without a network. Those are separate concerns
and must be declared explicitly when they become product requirements.

### A specification is the source of truth

Flags are useful for quick invocation, but a persistent project configuration
is required for repeatable generation. `brota.yaml` is the planned source of
truth for decisions that materially change generated output. It must not become
a dump of every template implementation detail.

### Opinionated defaults, controlled extension

Fewer user decisions must produce a more complete, coherent result, not an
emptier scaffold. Profiles provide a strong default path; advanced users may
override meaningful decisions without being exposed to an unmaintainable matrix
of combinations.

### Contracts over folder names

An architecture is not a directory layout alone. It includes dependency rules,
required artifacts, validation, documentation, testing expectations, and a
migration story. BrotaLibre must eventually validate its own architectural
claims.

### Functional UI, not decorative scaffolding

Generated UI must provide a usable journey: navigation, loading, empty, and
error states; validated forms; responsive behavior; accessible actions; and
safe destructive operations. `ui: native` means accessible, styled components
without an external component library, not unstyled markup.

### Safe evolution

Initial generation is not enough for a long-lived tool. BrotaLibre must prepare
for deterministic, safe regeneration from the beginning, while avoiding a
premature promise that it can merge arbitrary human edits automatically.

## Current Product Focus

The first web-application focus is an Angular administrative application. It
is intentionally narrower than "any frontend": its first target is a complete
and dependable CRUD-oriented admin experience.

The primary architecture direction is `feature-clean`:

```text
src/app/features/<feature>/
  domain/
  application/
  infrastructure/
  presentation/
```

The distinction is behavioral, not cosmetic. Domain code stays framework-free;
application code depends on domain contracts; infrastructure implements those
contracts; and presentation calls application use cases instead of concrete
adapters.

## What This Vision Does Not Authorize Yet

- Do not treat OpenAPI import, synchronization, authentication, WebSockets,
  React, or community packs as current implementation work without a scoped
  task.
- Do not expand the Angular, UI-library, style-engine, and architecture matrix
  before one golden path is complete.
- Do not make a local web interface or a terminal UI the priority over the
  application model, configuration, UI contract, and safe generation.
- Do not interpret a roadmap item as a public release guarantee until it has
  an explicit implementation task and acceptance criteria.

## Decision Maintenance

This file and its linked documents are the versioned source of truth. Engram
stores a concise, pinned summary so OpenCode can recover the context between
sessions, but a product decision changes only when the relevant document is
updated as well.
