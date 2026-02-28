# lab-kurrentdb-rust-client

Event-sourced household ledger CLI built with Rust, KurrentDB, and PostgreSQL.

This repository explores Event Sourcing and CQRS patterns incrementally — from basic event append/replay to server-side projections to PostgreSQL read model projection.

## Architecture Evolution

Each branch represents a stage in the architecture:

| Branch | Stage | Description |
|--------|-------|-------------|
| `main` | **CQRS + PostgreSQL** | Catch-up Subscription projects events into PostgreSQL. Read side uses SQL queries. Value Objects, Vertical Slice Architecture. |
| `projections/kurrentdb` | **Server-side Projection** | KurrentDB's built-in V8 engine runs JavaScript projections. Balance, category stats, budget alerts via `get_state` and `emit()`. |
| `projections/postgres` | **PostgreSQL Projection** | Same as `main`. Kept for branch history. |
| `verify/projection-query` | **Projection verification** | Confirms KurrentDB projections only accept JavaScript (not TypeScript or other languages). |

```
Stage 1 (main~)          Stage 2                     Stage 3 (main)
─────────────────    ─────────────────────    ──────────────────────────
KurrentDB              KurrentDB                  KurrentDB
  │                      │                          │
  │ read_stream          │ JS Projection             │ Catch-up Subscription
  │ (full replay)        │ (get_state)               │
  ▼                      ▼                          ▼
CLI output             CLI output                PostgreSQL
                                                   │
                                                   │ SELECT
                                                   ▼
                                                 CLI output
```

## Key Concepts Demonstrated

- **Event Sourcing** — Append-only event streams as the source of truth
- **Optimistic Concurrency** — Stream revision checks to prevent lost updates
- **Server-side Projections** — KurrentDB V8 engine with TypeScript → ES5 compilation
- **CQRS Projection** — Catch-up Subscription projecting events into PostgreSQL
- **Checkpoint + Transaction** — Atomic checkpoint saves to prevent double-processing on restart
- **Value Objects** — Domain types (`Account`, `Amount`, `Category`, `Description`) enforced at compile time
- **Vertical Slice Architecture** — Each use case as an independent directory with `Operation` / `Presenter` traits

## Prerequisites

- Rust
- Docker

## Setup

```sh
docker compose up -d
```

KurrentDB will be available at `http://localhost:2113`.
PostgreSQL will be available at `localhost:5432` (database: `ledger`).

## Usage

```
cargo r -- income  <account> <amount> <description>     # Record income
cargo r -- expense <account> <amount> <category> <desc> # Record expense
cargo r -- history <account>                            # Show transaction history
cargo r -- balance <account>                            # Show current balance
cargo r -- watch   <account>                            # Real-time monitor
cargo r -- projector run                                # Start projector (KurrentDB → PostgreSQL)
cargo r -- stats category                               # Category expense summary
cargo r -- stats summary                                # Account summary
```

### Example

```sh
# Start projector in one terminal
cargo r -- projector run

# In another terminal
cargo r -- income alice 200000 salary
cargo r -- expense alice 80000 housing rent
cargo r -- balance alice
cargo r -- stats category
cargo r -- stats summary
```

## Posts

- [Rust × KurrentDB で EventSourcing をやってみた](https://zenn.dev/hideyuki_hori/articles/c11c64d9315e19) (Japanese)
- [家計簿で学ぶ KurrentDB Projection — Rust と TypeScript でサーバサイド常時計算](https://zenn.dev/hideyuki_hori/articles/93923d6cbdcd20) (Japanese)
- [KurrentDB + PostgreSQL — Rust で CQRS Projection](https://zenn.dev/hideyuki_hori/articles/050b8ae00bebeb) (Japanese)

## License

MIT