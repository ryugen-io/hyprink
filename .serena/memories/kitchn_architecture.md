# Kitchn Architecture

Kitchn is designed as a modular ingredient manager for Hyprcore, written in Rust.

## Core Components

### Kitchn CLI (`kitchn_cli`)
- The user-facing command-line interface.
- Built with `clap` for argument parsing.
- Handles high-level commands: `stock`, `wrap`, `cook`, `pantry`, `bake`.
- Manages the debug mode initialization and terminal spawning.
- Calls into `kitchn_lib` for logic.

### Kitchn Lib (`kitchn_lib`)
- The heavy lifter containing business logic.
- **Modules**:
    - `db`: Manages the ingredient database (`pantry.db`). Uses `bincode` for serialization.
    - `packager`: Handles `.bag` (zip) creation and extraction.
    - `processor`: Applies ingredients (`.ing` TOML files) using the `tera` templating engine.
    - `config`: Handles `theme.toml`, `layout.toml`, etc., and supports "baking" them into a binary cache.
    - `logger`: Legacy/User-facing output formatter (relies on `kitchn-log`).

### Kitchn Log (`kitchn_log`)
- A standalone binary for standardized, themed logging output.
- `kitchn_cli` spawns this binary for pretty user notifications (e.g., "stocked x", "cooked y").

### Kitchn FFI (`kitchn_ffi`)
- Foreign Function Interface bindings (likely for C/C++ integration).

## Data Flow
- **Stocking**: `.ing` files or `.bag` archives are parsed/unzipped and stored in `pantry.db`.
- **Cooking**: Ingredients are retrieved from `pantry.db`. Logic in `processor.rs` renders templates (using keys from `cookbook.toml` and `theme.toml`) and runs hooks.
- **Baking**: Configuration TOMLs are pre-parsed and saved as a binary `pastry.bin` to speed up startup times.

## Debugging
- Uses `log` crate + `simplelog` for internal verbose logging (`debug!`).
- Logs to `/tmp/kitchn-debug.log`.
- `kitchn --debug` acts as a log listener (tailing the file).
