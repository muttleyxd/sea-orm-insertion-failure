# sea-orm-insertion-failure

This repository allows for easy reproducing of a bug with insertion of an item, where it has a OffsetDateTime member in primary key.

Requirements:
- cargo
- installed Rust toolchain
- docker-compose (for database)

Usage:
```
./create-database.sh
cargo run
```

Run `./destroy-database.sh` to cleanup leftover files
