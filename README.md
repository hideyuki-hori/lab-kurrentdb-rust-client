# lab-kurrentdb-rust-client

Event-sourced household ledger CLI built with Rust, KurrentDB, and PostgreSQL.

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

- [Rust × KurrentDBでEventSourcingをやってみた](https://zenn.dev/hideyuki_hori/articles/c11c64d9315e19) (Japanese)
- [家計簿で学ぶ KurrentDB Projection — Rust と TypeScript でサーバサイド常時計算](https://zenn.dev/hideyuki_hori/articles/93923d6cbdcd20) (Japanese)
- [KurrentDB + PostgreSQL — Rust で CQRS Projection](https://zenn.dev/hideyuki_hori/articles/050b8ae00bebeb) (Japanese)

## License

MIT
