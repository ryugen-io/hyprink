# hyprink justfile

default: build

# === Build Commands ===

# Build release binaries
build:
    ./dev/scripts/build/build.sh --release

# Build debug binaries
build-debug:
    ./dev/scripts/build/build.sh

# Run the installation script (Config setup + Build)
install:
    ./install.sh

# Uninstall everything
uninstall:
    ./uninstall.sh

# Clean build artifacts
clean:
    ./dev/scripts/build/clean.sh

# Clean everything (target, Cargo.lock, .tmp, logs)
clean-full:
    ./dev/scripts/build/clean.sh --full

# Nuclear clean (includes cargo cache - requires PIN)
clean-nuke:
    ./dev/scripts/build/clean.sh --nuke

# Show binary sizes
size:
    ./dev/scripts/build/size.sh

# Analyze binary bloat
bloat CRATE="":
    ./dev/scripts/build/bloat.sh {{CRATE}}

# === Code Quality ===

# Format code
fmt:
    ./dev/scripts/code/fmt.sh

# Check format without modifying
fmt-check:
    ./dev/scripts/code/fmt.sh --check

# Run clippy linter
lint:
    ./dev/scripts/code/lint.sh

# Run strict linter (pedantic + nursery)
lint-strict:
    ./dev/scripts/code/lint.sh --strict

# Find TODO/FIXME annotations
todo:
    ./dev/scripts/code/todo.sh

# Pre-commit checks (format + lint + test)
pre-commit:
    ./dev/scripts/git/pre-commit.sh

# === Testing ===

# Run all tests
test:
    ./dev/scripts/test/quick.sh

# Run tests for hi_core only
test-lib:
    cargo test -p hi_core

# Run test coverage analysis
coverage:
    ./dev/scripts/test/coverage.sh

# Run benchmarks
bench:
    cargo bench

# Run benchmarks for hi_core only
bench-lib:
    cargo bench -p hi_core

# === Dependencies ===

# Audit dependencies (unused + security)
audit:
    ./dev/scripts/deps/audit.sh

# Check for outdated dependencies
outdated:
    ./dev/scripts/deps/outdated.sh

# === Documentation ===

# Generate documentation
docs:
    ./dev/scripts/info/docs.sh

# Generate and open documentation
docs-open:
    ./dev/scripts/info/docs.sh --open

# === Info ===

# Show lines of code
loc:
    ./dev/scripts/info/loc.sh

# Show project tree
tree:
    ./dev/scripts/info/tree.sh

# Show git changes summary
changes:
    ./dev/scripts/git/changes.sh

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
