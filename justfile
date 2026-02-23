# hyprink justfile

default: build

# === Build Commands ===

# Build release binaries
build:
    ~/code/.dev/scripts/shared/build/build.sh --release

# Build debug binaries
build-debug:
    ~/code/.dev/scripts/shared/build/build.sh

# Run the installation script (Config setup + Build)
install:
    ./install.sh

# Uninstall everything
uninstall:
    ./uninstall.sh

# Clean build artifacts
clean:
    ~/code/.dev/scripts/shared/build/clean.sh

# Clean everything (target, Cargo.lock, .tmp, logs)
clean-full:
    ~/code/.dev/scripts/shared/build/clean.sh --full

# Nuclear clean (includes cargo cache - requires PIN)
clean-nuke:
    ~/code/.dev/scripts/shared/build/clean.sh --nuke

# Show binary sizes
size:
    ~/code/.dev/scripts/shared/build/size.sh

# Analyze binary bloat
bloat CRATE="":
    ~/code/.dev/scripts/shared/build/bloat.sh {{CRATE}}

# === Code Quality ===

# Format code
fmt:
    ~/code/.dev/scripts/shared/code/fmt.sh

# Check format without modifying
fmt-check:
    ~/code/.dev/scripts/shared/code/fmt.sh --check

# Run clippy linter
lint:
    ~/code/.dev/scripts/shared/code/lint.sh

# Run strict linter (pedantic + nursery)
lint-strict:
    ~/code/.dev/scripts/shared/code/lint.sh --strict

# Find TODO/FIXME annotations
todo:
    ~/code/.dev/scripts/shared/code/todo.sh

# Pre-commit checks (format + lint + test)
pre-commit:
    ~/code/.dev/scripts/shared/git/pre-commit.sh

# === Testing ===

# Run all tests
test:
    ~/code/.dev/scripts/shared/test/quick.sh

# Run tests for hi_core only
test-lib:
    cargo test -p hi_core

# Run test coverage analysis
coverage:
    ~/code/.dev/scripts/shared/test/coverage.sh

# Run benchmarks
bench:
    cargo bench

# Run benchmarks for hi_core only
bench-lib:
    cargo bench -p hi_core

# === Dependencies ===

# Audit dependencies (unused + security)
audit:
    ~/code/.dev/scripts/shared/deps/audit.sh

# Check for outdated dependencies
outdated:
    ~/code/.dev/scripts/shared/deps/outdated.sh

# === Documentation ===

# Generate documentation
docs:
    ~/code/.dev/scripts/shared/info/docs.sh

# Generate and open documentation
docs-open:
    ~/code/.dev/scripts/shared/info/docs.sh --open

# === Info ===

# Show lines of code
loc:
    ~/code/.dev/scripts/shared/info/loc.sh

# Show project tree
tree:
    ~/code/.dev/scripts/shared/info/tree.sh

# Show git changes summary
changes:
    ~/code/.dev/scripts/shared/git/changes.sh

# === hyprink Commands ===

# Run hyprink apply
apply:
    ./target/release/hyprink apply

# Add example template
add-waybar:
    ./target/release/hyprink add ./assets/templates/waybar.tpl

# Debug hyprink
debug:
    cargo run --bin hyprink -- --debug

# === Examples ===

# Run Rust Native Example
example-rust:
    @echo "Running Rust Example..."
    cd assets/examples/rust && cargo run
