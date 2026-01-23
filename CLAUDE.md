# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

hyprink is a system-wide theming tool that unifies configuration across your Linux desktop ecosystem. The core concept is "Single Source of Truth" - edit one central configuration (`~/.config/hypr/hyprink.conf`) and propagate changes to all applications via Tera templates.

## Build Commands

```bash
just build              # Build release binaries
just install            # Full installation (config + build)
just test               # Run all workspace tests
just test-lib           # Test hi_core only
just lint               # Clippy + format check
just pre-commit         # Full pre-commit checks
just bench-lib          # Benchmark hi_core
```

Single crate/test operations:
```bash
cargo build -p hi_core
cargo test -p hi_core
cargo test -p hi_core -- test_name           # Run single test
cargo test -p hi_core -- test_name --nocapture  # With stdout
```

## Architecture

### Crate Dependency Graph
```
hyprink (CLI binary)
        │
     hi_core (core logic)
```

### Crate Purposes
- **hi_core**: All business logic - config loading, template processing, Store (bincode-based storage), logging via hyprlog
- **hi_cli**: CLI wrapper with Clap-based argument parsing and commands in `src/commands/`

### Key Types in hi_core
- `Config`: Unified config from `hyprink.conf` (theme + icons + layout + presets)
- `Template`: Parsed .tpl file representation
- `Store`: bincode-based template storage
- `ConfigError`: Typed error enum using thiserror

### Data Flow
1. **Config**: `hyprink.conf` -> `Config::load()` -> binary cache (bincode)
2. **Templates**: `.tpl` file -> `Template` -> `Store::add()`
3. **Apply**: `Store::list()` -> Tera render -> target files -> hook execution

## Error Handling Pattern

- Library code (hi_core): Use `thiserror` for typed error enums
- Binary code (hi_cli): Use `anyhow::Result` for propagation

## Single Instance Policy

Uses `flock()` on `~/.cache/hyprink/hyprink.lock` to prevent concurrent modifications. Debug viewer is exempt.

## Config Locations

- Config: `~/.config/hypr/hyprink.conf`
- Binary cache: `~/.cache/hyprink/config.bin`
- Data/DB: `~/.local/share/hyprink/`
- Logs: via hyprlog (`~/.local/state/hyprlog/logs/`)

## CLI Commands Reference

```bash
hyprink add <path>      # Add template (.tpl) or package (.pkg)
hyprink list            # List all stored templates
hyprink list clear      # Remove all templates from store
hyprink apply           # Apply all templates (render + run hooks)
hyprink pack <dir>      # Package .tpl files into a .pkg archive
hyprink compile         # Pre-compile config into binary cache
hyprink --debug         # Spawn debug viewer in separate terminal
```

## Logging

hyprink uses hyprlog (hl_core) for logging. See the hyprlog project for log presets and configuration.
