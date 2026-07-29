# Domain-Driven Design

The DDD profile organizes code by business capability instead of placing all
domains, use cases, or adapters in shared technical folders. Use it when
features are owned independently, evolve at different speeds, or need strong
boundaries around domain behavior.

## Structure

```text
src/app/
└── features/
    └── <feature>/
        ├── domain/
        ├── application/
        ├── infrastructure/
        └── presentation/
```

| Layer | Responsibility | Example files |
|-------|----------------|---------------|
| `domain` | Business rules and contracts | Entity, value objects, errors, repository port |
| `application` | Feature use cases and state | Signal store, create/update/delete use cases |
| `infrastructure` | Feature-specific external adapters | DTOs, mappers, repository, provider, mock |
| `presentation` | Feature UI and routes | List, form, and detail views |

## Dependency Rules

- A feature owns its domain rules and infrastructure adapters.
- Domain code stays independent from Angular and external services.
- Application code depends on the feature domain, not concrete adapters.
- Presentation uses application contracts and does not bypass use cases.
- Cross-feature dependencies should be explicit and kept narrow.

## Generate

```bash
brota new my-app --architecture ddd
brota generate feature product \
  --architecture ddd \
  --prefix /api/products \
  --fields name:string,price:number \
  --project-dir ./my-app
```

The feature generator writes files under
`src/app/features/product/{domain,application,infrastructure,presentation}`.
