# Suggested Commands

## CRITICAL: Pre-Commit (MANDATORY)
```bash
just pre-commit    # MUST run before EVERY git commit
```

## Build
```bash
just build         # Build release binaries
just build-debug   # Build debug binaries
just install       # Full installation (config + build)
just clean         # Clean build artifacts
just clean-full    # Clean everything (target, Cargo.lock, .tmp, logs)
```

## Testing
```bash
just test          # Run all tests
just test-lib      # Test hi_core only
just bench         # Run all benchmarks
just bench-lib     # Benchmark hi_core only
just coverage      # Test coverage analysis
```

Single test:
```bash
cargo test -p hi_core -- test_name
cargo test -p hi_core -- test_name --nocapture  # With stdout
```

## Code Quality
```bash
just fmt           # Format code
just fmt-check     # Check format without modifying
just lint          # Run clippy
just lint-strict   # Pedantic + nursery lints
just todo          # Find TODO/FIXME annotations
```

## Dependencies
```bash
just audit         # Audit dependencies (unused + security)
just outdated      # Check for outdated deps
```

## Documentation
```bash
just docs          # Generate documentation
just docs-open     # Generate and open in browser
```

## Info
```bash
just loc           # Lines of code
just tree          # Project tree
just changes       # Git changes summary
just size          # Binary sizes
just bloat         # Analyze binary bloat
```

## hyprink Runtime
```bash
just apply         # Run hyprink apply
just debug         # Debug hyprink (cargo run)
```
