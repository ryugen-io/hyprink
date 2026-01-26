# hyprink Project Overview

## Purpose
hyprink is a system-wide theming tool that unifies configuration across a Linux desktop ecosystem. The core concept is "Single Source of Truth" - edit one central configuration (`~/.config/hypr/hyprink.conf`) and propagate changes to all applications via Tera templates.

## Tech Stack
- **Language**: Rust (2021 edition)
- **Build System**: Cargo workspace with justfile
- **Template Engine**: Tera
- **Serialization**: bincode (binary cache), serde
- **Error Handling**: thiserror (library), anyhow (binary)
- **Logging**: hyprlog (hl_core)
- **CLI**: Clap

## Architecture

### Single Crate Structure
```
hyprink/
├── src/
│   ├── lib.rs              # Library root
│   ├── config.rs           # Config loading
│   ├── template.rs         # Template parsing
│   ├── db.rs               # Store (bincode-based storage)
│   ├── processor.rs        # Tera rendering
│   ├── packager.rs         # .pkg archive handling
│   ├── logger.rs           # hyprlog integration
│   ├── factory.rs          # Factory patterns
│   ├── bin/
│   │   └── hyprink.rs      # Binary entry point
│   └── cli/
│       ├── mod.rs          # CLI module root
│       ├── args.rs         # Clap argument parsing
│       └── commands/       # Subcommands
└── benches/                # Benchmarks
```

### Features
- `default = ["cli"]` - CLI binary enabled by default
- `cli` - Enables CLI dependencies (clap, tracing, etc.)

### Key Modules
- `config` - Config loading from hyprink.conf
- `template` - Template parsing and representation
- `db` - bincode-based template storage (Store)
- `processor` - Template rendering with Tera
- `packager` - .pkg archive handling
- `logger` - hyprlog integration
- `factory` - Factory patterns

## Data Locations
- Config: `~/.config/hypr/hyprink.conf`
- Binary cache: `~/.cache/hyprink/config.bin`
- Data/DB: `~/.local/share/hyprink/`
- Logs: `~/.local/state/hyprlog/logs/`
- Lock file: `~/.cache/hyprink/hyprink.lock`

## CLI Commands
- `hyprink add <path>` - Add template (.tpl) or package (.pkg)
- `hyprink list` - List stored templates
- `hyprink list clear` - Remove all templates
- `hyprink apply` - Apply all templates
- `hyprink pack <dir>` - Create .pkg archive
- `hyprink compile` - Pre-compile config to binary cache
- `hyprink --debug` - Debug viewer
