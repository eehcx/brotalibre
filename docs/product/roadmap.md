# BrotaLibre Product Roadmap

This roadmap protects the product from implementing its entire future at once.
Release numbers describe intended sequencing, not dates or promises that a
capability already exists. Read [the product vision](vision.md) first.

## Decision Status

| Status | Meaning |
|---|---|
| Confirmed | May guide scoped implementation work. |
| Planned | Direction is accepted, but it is not current implementation work. |
| Deferred | Deliberately excluded until a later milestone. |

## Confirmed Direction

| Topic | Decision |
|---|---|
| Web applications | Angular is the only application framework for the initial beta path. |
| Initial product | Administrative panels and CRUD applications, not arbitrary visual products. |
| Astro | Astro serves landing pages and documentation sites; it is not part of Angular CRUD generation. |
| Architecture | The intended primary profile is `feature-clean`, with vertical features and enforceable dependency rules. |
| Defaults | An unattended generation uses an opinionated, complete profile rather than a minimal empty scaffold. |
| Configuration | `brota.yaml` will persist project decisions and guide later feature generation. |
| Validation | `brota validate` is a core product capability that reports deviations and recommendations. |
| Regeneration | Safe synchronization is strategic, but complete sync is deferred until the generation model is stable. |

## 0.5.0: Truthful Angular Admin Foundation

**Goal:** prove that BrotaLibre can generate a useful Angular administrative
application before OpenAPI becomes the central input.

### In scope

- Persist project decisions in an initial `brota.yaml`.
- Establish the `angular-admin` profile and the `feature-clean` architecture.
- Generate a responsive administrative shell with header, navigation, content
  area, breadcrumbs or page context, mobile navigation, 404 handling, and an
  error page.
- Generate an initial welcome route that links to the dashboard and demo CRUD.
- Generate a connected users demo CRUD using mock data.
- Provide a native CSS or SCSS UI baseline and stabilize Angular Material as
  the supported component-library integration.
- Implement loading, empty, and error states; destructive-action confirmation;
  validated create and edit forms; and responsive accessible actions.
- Make feature generation read the existing project configuration.
- Introduce `brota validate` and `brota doctor` with actionable diagnostics.

### Deferred from 0.5.0

- OpenAPI parsing and HTTP repository generation.
- Automatic synchronization of a changed API.
- Authentication, realtime data, and complex data relationships.
- A fully supported PrimeNG path. It may remain experimental, but it must not
  dilute the native and Material quality bar.
- Astro landing-page generation expansion. Existing Astro documentation work
  remains separate and must not regress.

### Exit condition

`brota new <project> --yes` must produce a visible, navigable Angular admin
application whose defaults are useful without asking the user to choose every
technical detail.

## 0.6.0: Enforce the UI Contract

**Goal:** harden the UI behavior introduced in 0.5.0 so native and Material
renderers meet the same functional contract.

- Define and test common shell, list, form, state, responsiveness, and
  accessibility requirements.
- Complete the native renderer as a first-class styled UI path.
- Keep the UI library independent from the style engine.
- Make renderer gaps visible through validation and scaffold tests.

## 0.7.0: Application Model and Experimental OpenAPI

**Goal:** introduce the internal model that prevents parsers and templates from
being coupled directly.

- Parse the supported subset of OpenAPI with a real parser, never regular
  expressions.
- Normalize OpenAPI and `brota.yaml` into an application model.
- Prototype resource detection, basic REST operation mapping, and a generation
  plan before files are written.
- Keep the feature experimental while the schema and generation semantics
  evolve.

## 0.8.0: OpenAPI CRUD Beta

**Goal:** generate the first reliable Angular admin CRUD path from a documented
REST API description. The intended public beta label is `0.8.0-beta.1`, not
1.0.

### In scope

- Supported OpenAPI 3 input and explicit limits.
- Detection of conventional CRUD operations: collection and item reads,
  create, update, and delete.
- DTOs, models, HTTP repositories, basic error handling, and CRUD forms.
- Basic scalar fields, enums, required or optional fields, defaults, and common
  validation constraints.
- `brota plan` output before generation writes files.
- A `.brota/` manifest, source-model snapshot, generation metadata, and file
  hashes that prepare later synchronization.

### Not a beta guarantee

- Server pagination or filtering conventions.
- Complex relationships, nested resources, uploads, authentication, or
  WebSockets.
- Safe automatic merge of generated and human-edited files.

Pagination and filtering are intentionally not inferred from arbitrary REST
APIs. They require declared conventions or explicit configuration rather than
guessing query names and response envelopes.

## 0.9.0: Stabilization and Migration

**Goal:** make the beta upgradeable instead of adding broad new surface area.

- Stabilize generated-project migrations and configuration evolution.
- Prove deterministic output across supported profiles and renderer packs.
- Define compatibility boundaries for future external packs without opening an
  unverified plugin ecosystem.

## 1.0.0: Stabilize Contracts

Before calling the product 1.0, stabilize the following public contracts:

- `brota.yaml` schema and migrations.
- Application-model semantics.
- Generation ownership and overwrite policy.
- Pack and renderer compatibility boundaries.
- Validation behavior and documentation output.
- Deterministic generation and a supported upgrade story.

## Post-1.0 Direction

| Area | Planned sequence |
|---|---|
| 1.1 | Pagination and filters after explicit API conventions are supported. |
| 1.2 | Authentication as a compatible configuration and generation addition. |
| 1.3 | Relationships, nested resources, and richer references after the base resource model is proven. |
| 1.4 | Diff and safe regeneration after file ownership is established. |
| 1.5 | WebSockets and live data after the REST path is stable. |
| Ecosystem | External packs, template registry, local web UI, and additional frontend targets after core contracts mature. |

## Delivery Rules

- A task must identify its target milestone, acceptance criteria, tests, and
  explicit exclusions.
- A planned item does not authorize speculative refactoring in an earlier
  milestone.
- Preserve the Angular golden path when extending Astro, templates, UI, or
  configuration behavior.
- Update this roadmap and the linked architecture documents before changing a
  long-term product decision.
