# Codebase Structure

## Directory Layout
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
│       ├── logging.rs      # CLI logging
│       ├── cli_config.rs   # CLI-specific config
│       └── commands/       # Subcommands
├── dev/
│   └── scripts/          # Build/test/lint scripts
│       ├── build/
│       ├── code/
│       ├── test/
│       ├── deps/
│       ├── git/
│       └── info/
├── assets/
│   ├── templates/        # Example templates
│   └── examples/         # Code examples
├── .github/              # GitHub workflows
├── Cargo.toml            # Workspace definition
├── justfile              # Task runner
├── install.sh            # Installation script
├── uninstall.sh          # Uninstallation script
├── CLAUDE.md             # AI assistant instructions
├── CHANGELOG.md          # Version history
└── README.md             # Project readme
```

## Key Types
- `Config` - Unified config (theme + icons + layout)
- `Template` - Parsed .tpl file
- `Store` - bincode-based template storage
- `ConfigError` - Typed error enum

## Data Flow
1. Config: `hyprink.conf` -> `Config::load()` -> binary cache
2. Templates: `.tpl` -> `Template` -> `Store::add()`
3. Apply: `Store::list()` -> Tera render -> target files -> hooks
