# Angular Admin UI Contract

This document defines what "functional CRUD UI" means for the target
`angular-admin` profile. It is a target contract that native and Material
renderers must satisfy; it is not a claim that every current template already
does so. See [the roadmap](../product/roadmap.md) and [the target generation
architecture](target-generation.md).

## Default Journey

An unattended command must select a coherent, useful profile. Less
configuration must result in stronger defaults, not less generated UI.

```text
brota new inventory --yes
  -> angular-admin profile
  -> feature-clean architecture
  -> responsive admin shell
  -> welcome route
  -> dashboard and users demo CRUD
  -> mock data and validation guidance
```

The default initial route is `welcome`. It introduces the generated project and
links to the dashboard and demo resource. A project may later opt into
`dashboard` or a specific resource route through configuration.

## Application Shell

Every administrative application must include:

- Header and responsive navigation.
- Main content area with an understandable page title and context or
  breadcrumbs.
- A mobile navigation behavior that remains usable without a pointer device.
- A dashboard entry point.
- A 404 page and a recoverable error page.
- Navigation to generated resources and to the welcome route when enabled.

The shell must be functional before visual customization. A renderer may vary
its visual language, but not the navigation and recovery contract.

## Welcome Route

The welcome route is project onboarding, not BrotaLibre marketing. It should
show:

- Project name and short readiness statement.
- Links to the dashboard and demo resource.
- Selected architecture, UI path, and API or mock-data source.
- A validation status or a direct validation command.
- Next steps, including where to update `brota.yaml` and how to add a resource.

The user can disable it or choose a different initial route through explicit
configuration. The default remains welcome because it is useful to an
unconfigured project.

## Resource List Contract

Every generated list must include:

- A clear resource title and an accessible primary create action.
- Loading, empty, and error states that do not look like a successful empty
  result.
- Accessible row actions and a destructive-action confirmation.
- Navigation to detail and edit flows where those capabilities exist.
- Responsive behavior that preserves access to row actions on narrow screens.

The UI contract includes search, sorting, and pagination capabilities when the
configured data source supports them. A renderer must not pretend that an
arbitrary REST API supports server filtering or paging. For the 0.8 OpenAPI
beta, pagination and filters require explicit conventions or configuration and
are not inferred as a universal transport guarantee.

## Detail and Form Contract

Create and edit forms must provide:

- Labels, help text where needed, and field-level validation feedback.
- Creation and edit modes with clear headings and submit actions.
- Pending-submission state that prevents accidental duplicate writes.
- Server-validation feedback when the transport exposes it.
- Cancellation behavior and protection for unsaved changes.
- A route back to the relevant list or detail view after success.

OpenAPI fields must be translated semantically rather than rendered as generic
text inputs whenever the model contains enough information:

| Model signal | Expected control |
|---|---|
| `string` | Text input |
| `string`, `email` format | Email input |
| `string`, `date` format | Date control |
| `boolean` | Checkbox or switch |
| `enum` | Select control |
| `number` or `integer` | Numeric input |
| Reference | Select or autocomplete when a supported relationship model exists |
| Array | Multi-selection when supported |
| Binary | File upload only when uploads are in scope |

Unsupported mappings must be reported by planning or validation, not silently
misrepresented as complete functionality.

## Native UI and Styling

`ui: native` means semantic HTML, keyboard-accessible interaction, visible
focus behavior, and a complete CSS or SCSS presentation without an external
component library. It does not mean no presentation.

UI renderer and style engine remain separate choices:

```yaml
ui:
  library: native
  styleEngine: scss
```

```yaml
ui:
  library: material
  styleEngine: scss
```

Theme values should be neutral tokens such as color, radius, density, and font
family. Each renderer translates those tokens into its own implementation
without leaking library-specific internal options into the product schema.

## Accessibility and Responsiveness

All renderers must meet this baseline:

- Keyboard navigation and visible focus for interactive controls.
- Semantic headings, labels, errors, and action names.
- Destructive confirmations that identify the affected action.
- Status changes communicated accessibly.
- Layouts usable on desktop and narrow mobile viewports.
- Color and density choices that retain readable contrast and touch targets.

## Acceptance Checklist

A renderer supports the admin contract only when a generated project can prove:

- [ ] Welcome, dashboard, list, detail, create, and edit journeys are linked.
- [ ] Loading, empty, error, submit, and destructive-confirmation states work.
- [ ] A keyboard user can reach and operate primary and row actions.
- [ ] A narrow viewport retains navigation and resource actions.
- [ ] The renderer does not bypass feature application use cases or concrete
      infrastructure dependencies.
- [ ] The generated project builds and its contract checks pass.
