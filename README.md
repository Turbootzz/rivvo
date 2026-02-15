# Rivvo

Open-source feedback and roadmap tool. Collect feature requests, let users vote, and share your product roadmap — self-hostable, fast, and free.

**Stack:** Rust (Actix-Web) + Vue 3 + PostgreSQL

## Features

- **Feedback boards** — Organize feature requests by topic
- **Voting** — Let users upvote the ideas they care about
- **Public roadmap** — Share what's planned, in progress, and shipped
- **Changelog** — Announce releases and link them to completed requests
- **Self-hostable** — Run it on your own infrastructure

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Bun](https://bun.sh/)
- [Docker](https://docs.docker.com/get-docker/) (for PostgreSQL)

### Setup

```bash
# Clone the repo
git clone https://github.com/turbootzz/rivvo.git
cd rivvo

# Start PostgreSQL
docker compose up -d

# Configure environment
cp .env.example .env
# Edit .env and set a JWT_SECRET (min 32 characters)

# Run the backend (runs migrations automatically)
cargo run

# In a separate terminal, start the frontend
cd web
bun install
bun run dev
```

The app will be available at [http://localhost:5173](http://localhost:5173), with the API at [http://localhost:8080](http://localhost:8080).

## Project Structure

```
rivvo/
├── src/                  # Rust backend
│   ├── handlers/         # HTTP request handlers
│   ├── services/         # Business logic
│   ├── models/           # Database models
│   ├── middleware/        # Auth middleware (JWT)
│   └── utils/            # JWT, slugify helpers
├── migrations/           # PostgreSQL migrations (SQLx)
├── web/                  # Vue 3 frontend
│   └── src/
│       ├── views/        # Page components
│       ├── components/   # Reusable components
│       ├── stores/       # Pinia state management
│       ├── composables/  # Vue composables
│       └── types/        # TypeScript interfaces
└── docker-compose.yml    # Dev database
```

## Development

```bash
# Backend
cargo build                  # Build
cargo test --all-targets     # Test
cargo fmt --all              # Format
cargo clippy --all-targets --all-features -- -D warnings  # Lint

# Frontend (from web/)
bun run dev                  # Dev server
bun run build                # Production build
bun run type-check           # TypeScript check
bun run lint                 # Lint
bun run test:unit            # Unit tests
```

## License

[AGPL-3.0](LICENSE)
