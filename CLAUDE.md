# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TSUI is an early-stage Rust UI framework using MVVM (Model-View-ViewModel) architecture with a plugin system. Homepage: https://tsui.edger.dev

## Build Commands

```bash
cargo build                        # Build all workspace crates
cargo test                         # Run all tests
cargo test -p tsui-model           # Run tests for a single crate
cargo clippy                       # Lint all crates
cargo fmt --check                  # Check formatting
cargo fmt                          # Auto-format
bacon                              # Background code checker (available via flox)
```

## Development Environment

The project uses **Flox** for reproducible dev environments. Enter the environment with `flox activate` or automatically via direnv (`.envrc` contains `use flox`). The flox environment provides `bacon` for continuous background checking.

## Workspace Architecture

Rust edition 2024, MSRV 1.93.0, resolver v3. Dual-licensed MIT OR Apache-2.0.

```
core/           - Core abstractions (5 crates)
  tsui-model/     - Data model traits
  tsui-view/      - View abstractions
  tsui-viewmodel/ - ViewModel pattern (MVVM glue)
  tsui-plugin/    - Plugin system abstractions
  tsui-app/       - Application framework
models/         - Domain model implementations
  tsui-registry/  - Registry/catalog model
views/          - GUI implementations
  tsui-egui/      - egui-based GUI
viewmodels/     - Presentation logic
  tsui-layout/    - Layout viewmodel
plugins/        - Extension plugins
  tsui-nu/        - Nushell integration
  tsui-roam/      - Roam RPC integration
apps/           - Applications
  tsui-cli/       - CLI application
```

The dependency flow follows layers: `core/ → models/ → viewmodels/ → views/ → apps/`, with `plugins/` extending `core/tsui-plugin/`.

## Workspace Dependencies

All crate metadata (version, edition, rust-version, license, etc.) is inherited from the workspace root `Cargo.toml` via `field.workspace = true`. Shared dependencies are declared under `[workspace.dependencies]`:

- **log** (with `kv` feature) - logging
- **rootcause** - error diagnostics
- **facet** - serialization
- **rstest** - parameterized testing (dev dependency)

New dependencies should be added to `[workspace.dependencies]` first, then referenced in individual crate `Cargo.toml` files.

## Adding a New Crate

Workspace members use glob patterns (`core/*`, `models/*`, etc.), so new crates placed in existing directories are automatically included. For a new top-level directory, add it to the `members` list in the root `Cargo.toml`.
