# Kitchn Justfile

default: build

# Build release binaries
build:
    cargo build --release

# Run the installation script (Config setup + Build)
install:
    ./install.sh

# Run tests
test:
    cargo test --workspace

# Clean build artifacts
clean:
    cargo clean

# Check code quality
lint:
    cargo clippy -- -D warnings
    cargo fmt -- --check

# Run kitchn-log example
run-log preset="test_pass":
    ./target/release/k-log {{preset}}

# Run kitchn cook (sync)
cook:
    ./target/release/kitchn cook

# Stock example ingredient
stock-waybar:
    ./target/release/kitchn stock ./assets/ingredients/waybar.ing



# Uninstall everything
uninstall:
    ./uninstall.sh

# Pre-commit checks (Lint + Format)
pre-commit:
    cargo clippy --all-targets --all-features -- -D warnings

# Show demo logs
show:
    ../utils/kitchn/demo_logs.sh

# Audit dependencies
audit:
    cargo audit

# Run benchmarks (Criterion + Hyperfine)
bench:
    cargo bench
    ../utils/kitchn/bench.sh

# Run benchmarks for k-lib only
bench-lib:
    cargo bench -p k-lib

# Run tests for k-lib only
test-lib:
    cargo test -p k-lib

# Run C++ FFI Example
example-cpp: build
    @echo "Running C++ Example..."
    cd assets/examples/cpp && make main && ./main

# Run Python FFI Example
example-python: build
    @echo "Running Python Example..."
    cd assets/examples/python && python3 main.py

# Run Rust Native Example
example-rust:
    @echo "Running Rust Example..."
    cd assets/examples/rust && cargo run

# Run all examples
examples: example-cpp example-python example-rust

# Memory leak check (ASan + LSan)
memcheck: build
    ../utils/kitchn/memcheck.sh .

# Show project statistics (LOC, Sizes)
stats:
    ../utils/kitchn/stats.sh .

# Debug Kitchn (Spawns listener)
debug:
    cargo run --bin kitchn -- --debug

# Optimize binaries with UPX
compact: build
    @echo "Compacting binaries..."
    @upx --best --lzma target/release/kitchn
    @upx --best --lzma target/release/k-log

# Package release tarball (for CI/CD)
# Usage: just package <target> <name>
# Example: just package x86_64-unknown-linux-gnu x86_64-linux
package target name:
    #!/usr/bin/env bash
    set -euo pipefail
    VERSION="${GITHUB_REF_NAME:-dev}"
    DIST="dist"
    PKG="kitchn-${VERSION}-{{name}}"

    # Determine release directory (with or without target triple)
    if [[ -d "target/{{target}}/release" ]]; then
        RELEASE_DIR="target/{{target}}/release"
    elif [[ -d "target/release" ]]; then
        RELEASE_DIR="target/release"
    else
        echo "[package] ERROR: No release build found. Run 'cargo build --release' first."
        exit 1
    fi

    echo "[package] Creating ${PKG}..."
    echo "[package] Using binaries from: ${RELEASE_DIR}"
    rm -rf "${DIST}/${PKG}"
    mkdir -p "${DIST}/${PKG}"/{bin,lib,include,config}

    # Binaries
    cp "${RELEASE_DIR}/kitchn" "${DIST}/${PKG}/bin/"
    cp "${RELEASE_DIR}/k-log" "${DIST}/${PKG}/bin/"
    chmod +x "${DIST}/${PKG}/bin/"*

    # FFI library (optional)
    [[ -f "${RELEASE_DIR}/libk_ffi.so" ]] && \
        cp "${RELEASE_DIR}/libk_ffi.so" "${DIST}/${PKG}/lib/" || true

    # C header
    [[ -f "include/kitchn.h" ]] && cp "include/kitchn.h" "${DIST}/${PKG}/include/"

    # Default configs
    cat > "${DIST}/${PKG}/config/theme.toml" << 'TOML'
    [meta]
    name = "Sweet Dracula"

    [settings]
    active_icons = "nerdfont"

    [colors]
    bg = "#161925"
    fg = "#F8F8F2"
    cursor = "#8BE9FD"
    selection_bg = "#44475A"
    selection_fg = "#F8F8F2"
    tabs = "#11131C"
    tabs_active = "#BD93F9"
    primary = "#FF79C6"
    secondary = "#BD93F9"
    success = "#50FA7B"
    error = "#FF5555"
    warn = "#F1FA8C"
    info = "#8BE9FD"
    kitchn = "#BD93F9"
    summary = "#50FA7B"
    black = "#44475A"
    red = "#DE312B"
    green = "#2FD651"
    yellow = "#D0D662"
    blue = "#9C6FCF"
    magenta = "#DE559C"
    cyan = "#6AC5D3"
    white = "#D7D4C8"
    bright_black = "#656B84"
    bright_red = "#FF5555"
    bright_green = "#50FA7B"
    bright_yellow = "#F1FA8C"
    bright_blue = "#BD93F9"
    bright_magenta = "#FF79C6"
    bright_cyan = "#8BE9FD"
    bright_white = "#F8F8F2"

    [fonts]
    mono = "JohtoMono Nerd Font Mono"
    ui = "Roboto"
    size_mono = "10"
    size_ui = "11"
    TOML

    cat > "${DIST}/${PKG}/config/icons.toml" << 'TOML'
    [nerdfont]
    success = ""
    error = ""
    warn = ""
    info = ""
    kitchn = ""
    summary = ""
    net = "󰖩"

    [ascii]
    success = "*"
    error = "!"
    warn = "!!"
    info = "i"
    kitchn = "K"
    summary = "="
    net = "#"
    TOML

    cat > "${DIST}/${PKG}/config/layout.toml" << 'TOML'
    [tag]
    prefix = "["
    suffix = "]"
    transform = "lowercase"
    min_width = 0
    alignment = "left"

    [labels]
    error = "error"
    success = "success"
    info = "info"
    warn = "warn"

    [structure]
    terminal = "{tag} {scope} {icon} {msg}"
    file = "{timestamp} {tag} {msg}"

    [logging]
    base_dir = "~/.local/state/kitchn/logs"
    path_structure = "{year}/{month}/{scope}"
    filename_structure = "{level}.{year}-{month}-{day}.log"
    timestamp_format = "%H:%M:%S"
    write_by_default = true
    TOML

    cat > "${DIST}/${PKG}/config/cookbook.toml" << 'TOML'
    # User Cookbook - Add custom presets here
    [presets]
    TOML

    # Documentation
    [[ -f "LICENSE" ]] && cp "LICENSE" "${DIST}/${PKG}/"
    [[ -f "README.md" ]] && cp "README.md" "${DIST}/${PKG}/"
    [[ -f "CHANGELOG.md" ]] && cp "CHANGELOG.md" "${DIST}/${PKG}/"

    # Install scripts
    cp install.sh uninstall.sh "${DIST}/${PKG}/"
    chmod +x "${DIST}/${PKG}/"*.sh

    # Create tarball
    tar -czvf "${DIST}/${PKG}.tar.gz" -C "${DIST}" "${PKG}"
    rm -rf "${DIST}/${PKG}"

    SIZE=$(du -h "${DIST}/${PKG}.tar.gz" | cut -f1)
    echo "[package] Created: ${DIST}/${PKG}.tar.gz (${SIZE})"
