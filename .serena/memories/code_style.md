# Code Style and Conventions

## Rust Idioms
- Standard Rust 2021 edition idioms
- Use `#[derive(...)]` macros for common traits
- Public structs with `pub` fields where appropriate
- Module organization: one file per module in `src/`

## Error Handling
- **Library code (hi_core)**: Use `thiserror` for typed error enums
- **Binary code (hi_cli)**: Use `anyhow::Result` for propagation

Example:
```rust
// In hi_core
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config: {0}")]
    ReadError(#[from] std::io::Error),
}

// In hi_cli
fn main() -> anyhow::Result<()> {
    // ...
}
```

## Serialization
- Use `serde` with `Serialize, Deserialize` derives
- Binary cache uses `bincode`
- Config files use custom parsing

## Naming
- Snake_case for functions and variables
- PascalCase for types and traits
- SCREAMING_SNAKE_CASE for constants

## Documentation
- Doc comments for public API
- No excessive inline comments
- Keep code self-documenting

## Testing
- Tests in same file with `#[cfg(test)]` module
- Use descriptive test names: `test_config_load_valid`

## Release Profile
- LTO enabled
- Single codegen unit
- Stripped binaries
- Optimized for size (`opt-level = "z"`)
- Panic = abort
