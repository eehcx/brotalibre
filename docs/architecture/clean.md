# Clean Architecture

Clean Architecture keeps the application organized by technical layer and
defines dependency direction from outer details toward inner business rules.
Use it when a team wants a predictable structure shared across many features
or when the application has strong platform-level conventions.

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

| Layer | Responsibility | Allowed direction |
|-------|----------------|-------------------|
| `domain` | Entities, value objects, errors, and repository ports | Depends on no outer layer |
| `application` | Use cases, state, and orchestration | Depends on `domain` |
| `infrastructure` | API clients, DTOs, mappers, providers, and repository adapters | Implements `domain` ports |
| `presentation` | Components, routes, forms, and user-facing state | Calls `application` use cases |

## Dependency Rules

- Domain code must not import Angular, HTTP clients, or infrastructure adapters.
- Application code coordinates use cases without knowing concrete adapters.
- Infrastructure implements ports defined by the domain.
- Presentation depends on application contracts instead of constructing adapters directly.

## Generate

```bash
brota new my-app --architecture clean
brota generate feature user \
  --fields name:string,email:string \
  --project-dir ./my-app
```

The feature generator writes files under
`src/app/features/user/{domain,application,infrastructure,presentation}`.
