# Kitchn Features

## Ingredient Management
- **.ing Files**: The fundamental unit. A TOML file defining an ingredient (metadata + hooks).
- **.bag Files**: A zipped package containing multiple `.ing` files. Created via `kitchn wrap`.
- **Pantry**: A local database (`~/.local/share/kitchn/pantry.db`) storing managed ingredients.

## Commands
- `stock`: Adds ingredients/bags to the pantry.
- `wrap`: Packages a directory of ingredients into a portable `.bag`.
- `cook`: Applies all ingredients from the pantry to the system. This executes the hooks and renders templates.
- `pantry`: Lists all stocked ingredients.
- `bake`: Pre-compiles configuration files (theme, layout, icons) into a binary format for faster subsequent runs.

## Debug Mode (v0.2.0+)
- **Flag**: `--debug` (Global flag).
- **Standalone**: Running `kitchn --debug` opens a terminal window that tails the debug log.
- **Persistent**: Running `kitchn --debug <cmd>` or just `kitchn <cmd>` (while listener is running) logs verbose debug info to `/tmp/kitchn-debug.log`.
- **Automatic Detection**: It tries to detect your preferred terminal emulator (via `TERMINAL` env var) or falls back to standard ones (alacritty, kitty, etc.).
