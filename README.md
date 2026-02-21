# lab-kurrentdb-rust-client

Event-sourced household ledger CLI built with Rust and KurrentDB.

## Prerequisites

- Rust
- Docker

## Setup

```sh
docker compose up -d
```

KurrentDB will be available at `http://localhost:2113`.

## Usage

```
cargo r income  <account> <amount> <description>     # Record income
cargo r expense <account> <amount> <category> <desc> # Record expense (with balance check)
cargo r history <account>                            # Show transaction history
cargo r balance <account>                            # Show current balance
cargo r watch   <account>                            # Real-time monitor
```

### Example

```sh
cargo r income alice 200000 salary
cargo r expense alice 80000 housing rent
cargo r balance alice
cargo r watch alice
```

## License

MIT
