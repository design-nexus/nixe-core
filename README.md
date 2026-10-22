# Core

Shared Rust workspace: nixe-core (identity, paths, preferences, control),
nixe-accounts (GOA/EDS metadata discovery), and nixe-sync (WebDAV journal).

Check: `cargo check --locked --workspace`. Headless foundation: `cargo check --locked -p nixe-core --no-default-features`.

Architecture and public API boundaries: [AGENTS.md](AGENTS.md).
