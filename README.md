# brotalibre

Scaffold production-ready Angular projects with clean architecture and optional UI integrations.

```bash
brota new my-app
```

## Features

- **Clean Architecture** — domain, application, infrastructure, presentation layers
- **CDP (Core-Data-Presentation)** — alternative component-driven structure
- **UI Integrations** — Angular Material, PrimeNG, or none
- **CSS Frameworks** — TailwindCSS v4 or none
- **Zero config** — get a working Angular project in seconds

## Usage

```bash
brota new my-app --yes
brota new my-app --architecture cdp --ui material
brota new my-app --architecture clean --styles tailwindcss
brota new my-app --yes --skip-install --skip-git
```

## Architecture

### Clean

```
src/app/
├── domain/
│   ├── entities/
│   └── ports/
├── application/
│   └── use-cases/
├── infrastructure/
│   ├── adapters/
│   └── providers/
└── presentation/
    └── facades/
```

### CDP

```
src/app/
├── core/
│   ├── models/
│   ├── environment/
│   ├── commons/
│   └── auth/
├── data/
│   └── datasource/
│       ├── remote/
│       └── local/
└── presentation/
    └── pages/
```
