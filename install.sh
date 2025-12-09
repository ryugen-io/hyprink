#!/usr/bin/env bash
# shellcheck disable=SC2155
# =============================================================================
# Kitchn Install Script
# Sets up config directory, creates default configs, and installs binaries
#
# Usage:
#   From source (in repo):  ./install.sh
#   From release package:   ./install.sh
#   Remote install:         curl -fsSL https://raw.githubusercontent.com/ryugen/kitchn/master/install.sh | bash
#   Specific version:       curl -fsSL ... | bash -s -- v0.2.0
# =============================================================================

set -euo pipefail
IFS=$'\n\t'

# Fail fast on undefined variables and pipe failures
shopt -s inherit_errexit 2>/dev/null || true

# -----------------------------------------------------------------------------
# Configuration
# -----------------------------------------------------------------------------
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || echo "")"
readonly CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/kitchn"
readonly INSTALL_DIR="${HOME}/.local/bin"
readonly LIB_DIR="${HOME}/.local/lib/kitchn"
readonly INCLUDE_DIR="${HOME}/.local/include/kitchn"

# GitHub Release Settings
readonly REPO="ryugen/kitchn"
readonly GITHUB_API="https://api.github.com/repos/${REPO}/releases"

# Installation mode: "source", "package", or "remote"
INSTALL_MODE=""

# Colors (Sweet Dracula palette - 24-bit true color)
readonly GREEN=$'\033[38;2;80;250;123m'
readonly YELLOW=$'\033[38;2;241;250;140m'
readonly CYAN=$'\033[38;2;139;233;253m'
readonly RED=$'\033[38;2;255;85;85m'
readonly PURPLE=$'\033[38;2;189;147;249m'
readonly NC=$'\033[0m'

# Icons (Nerd Font)
readonly CHECK=''
readonly WARN=''
readonly ERR=''
readonly INFO_ICON=''

# -----------------------------------------------------------------------------
# Logging Functions
# -----------------------------------------------------------------------------
log()     { echo -e "${CYAN}[info]${NC} INSTALL  $*"; }
success() { echo -e "${GREEN}[ok]${NC}   INSTALL  $*"; }
warn()    { echo -e "${YELLOW}[warn]${NC} INSTALL  $*" >&2; }
error()   { echo -e "${RED}[err]${NC}  INSTALL  $*" >&2; }
die()     { error "$*"; exit 1; }

# -----------------------------------------------------------------------------
# Cleanup & Signal Handling
# -----------------------------------------------------------------------------
cleanup() {
    local exit_code=$?
    # Add cleanup tasks here if needed
    exit "$exit_code"
}
trap cleanup EXIT
trap 'die "Interrupted"' INT TERM

# -----------------------------------------------------------------------------
# Utility Functions
# -----------------------------------------------------------------------------
command_exists() {
    command -v "$1" &>/dev/null
}

detect_arch() {
    local arch
    arch="$(uname -m)"
    case "$arch" in
        x86_64|amd64)   echo "x86_64-linux" ;;
        aarch64|arm64)  echo "aarch64-linux" ;;
        *)              die "Unsupported architecture: $arch" ;;
    esac
}

detect_install_mode() {
    # Running from source repo?
    if [[ -n "$SCRIPT_DIR" && -f "${SCRIPT_DIR}/Cargo.toml" ]]; then
        INSTALL_MODE="source"
    # Running from extracted release package?
    elif [[ -n "$SCRIPT_DIR" && -d "${SCRIPT_DIR}/bin" && -f "${SCRIPT_DIR}/bin/kitchn" ]]; then
        INSTALL_MODE="package"
    # Running via curl | bash (remote)
    else
        INSTALL_MODE="remote"
    fi
    log "Install mode: ${INSTALL_MODE}"
}

get_latest_release() {
    local url="${GITHUB_API}/latest"
    if command_exists curl; then
        curl -fsSL "$url" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/'
    elif command_exists wget; then
        wget -qO- "$url" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/'
    else
        die "Neither curl nor wget found"
    fi
}

download_release() {
    local version="$1"
    local arch="$2"
    local url="https://github.com/${REPO}/releases/download/${version}/kitchn-${version}-${arch}.tar.gz"
    local tmp_dir
    tmp_dir="$(mktemp -d)"

    log "Downloading ${url}..."

    if command_exists curl; then
        curl -fsSL "$url" -o "${tmp_dir}/kitchn.tar.gz" || die "Download failed"
    elif command_exists wget; then
        wget -q "$url" -O "${tmp_dir}/kitchn.tar.gz" || die "Download failed"
    fi

    log "Extracting..."
    tar -xzf "${tmp_dir}/kitchn.tar.gz" -C "$tmp_dir"

    # Find extracted directory
    local pkg_dir
    pkg_dir="$(find "$tmp_dir" -maxdepth 1 -type d -name 'kitchn-*' | head -1)"

    if [[ -z "$pkg_dir" ]]; then
        die "Failed to extract release package"
    fi

    echo "$pkg_dir"
}

create_dir() {
    local dir="$1"
    if [[ ! -d "$dir" ]]; then
        mkdir -p "$dir" || die "Failed to create directory: $dir"
        success "Created $dir"
    else
        log "Directory exists: $dir"
    fi
}

write_config() {
    local file="$1"
    local content="$2"
    
    if [[ -f "$file" ]]; then
        warn "Config exists, skipping: $(basename "$file")"
        return 0
    fi
    
    log "Creating $(basename "$file")"
    printf '%s\n' "$content" > "$file" || die "Failed to write: $file"
    success "Created $(basename "$file")"
}

# -----------------------------------------------------------------------------
# Config Templates
# -----------------------------------------------------------------------------
THEME_CONFIG='[meta]
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
size_ui = "11"'

ICONS_CONFIG='[nerdfont]
success = ""
error = ""
warn = ""
info = ""
kitchn = ""
summary = ""
net = "󰖩"

[ascii]
success = "*"
error = "!"
warn = "!!"
info = "i"
kitchn = "K"
summary = "="
net = "#"'

LAYOUT_CONFIG='[tag]
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
write_by_default = true'

DICTIONARY_CONFIG='# User Dictionary Overrides
# The system defaults are embedded in the binary.
# Add your custom presets here to override or extend them.

# Example:
# [presets.my_custom_hook]
# level = "info"
# msg = "hello world"

[presets]
'

# -----------------------------------------------------------------------------
# Installation Functions
# -----------------------------------------------------------------------------
install_from_source() {
    cd "$SCRIPT_DIR" || die "Failed to cd to script directory"

    # Build
    if ! command_exists cargo; then
        die "Cargo not found. Install Rust: https://rustup.rs"
    fi

    log "Building release binaries..."
    # Explicitly build only kitchn and kitchn-log binaries
    if ! cargo build --release --bin kitchn --bin kitchn-log 2>&1; then
        die "Build failed"
    fi
    success "Binaries build complete"

    # Try building FFI library (optional, may fail on musl/alpine)
    log "Attempting to build FFI library (optional)..."
    if cargo build --release -p kitchn_ffi 2>&1; then
        success "FFI library build complete"
    else
        warn "FFI build failed. Retrying with dynamic linking (for Musl/Alpine)..."
        if RUSTFLAGS="-C target-feature=-crt-static" cargo build --release -p kitchn_ffi 2>&1; then
             success "FFI library build complete (dynamic)"
        else
             warn "FFI library build failed (skipping). This is normal on strict static systems."
        fi
    fi

    # Compact binaries if UPX is available
    if command_exists upx; then
        log "Compacting binaries with UPX..."
        compact_binary "target/release/kitchn"
        compact_binary "target/release/kitchn-log"
    fi

    # Install binaries
    for bin in kitchn kitchn-log; do
        local src="target/release/${bin}"
        [[ -f "$src" ]] && cp "$src" "$INSTALL_DIR/" || die "Binary not found: $src"
    done

    # Install FFI library if it exists
    if [[ -f "target/release/libkitchn_ffi.so" ]]; then
        create_dir "$LIB_DIR"
        cp "target/release/libkitchn_ffi.so" "$LIB_DIR/"
    fi

    # Install C header if it exists
    if [[ -f "include/kitchn.h" ]]; then
        create_dir "$INCLUDE_DIR"
        cp "include/kitchn.h" "$INCLUDE_DIR/"
    fi
}

install_from_package() {
    local pkg_dir="$1"

    # Install binaries
    for bin in kitchn kitchn-log; do
        local src="${pkg_dir}/bin/${bin}"
        [[ -f "$src" ]] && cp "$src" "$INSTALL_DIR/" || die "Binary not found: $src"
        chmod +x "${INSTALL_DIR}/${bin}"
    done

    # Install FFI library if it exists
    if [[ -f "${pkg_dir}/lib/libkitchn_ffi.so" ]]; then
        create_dir "$LIB_DIR"
        cp "${pkg_dir}/lib/libkitchn_ffi.so" "$LIB_DIR/"
    fi

    # Install C header if it exists
    if [[ -f "${pkg_dir}/include/kitchn.h" ]]; then
        create_dir "$INCLUDE_DIR"
        cp "${pkg_dir}/include/kitchn.h" "$INCLUDE_DIR/"
    fi

    # Install configs from package if they exist
    if [[ -d "${pkg_dir}/config" ]]; then
        for cfg in theme.toml icons.toml layout.toml cookbook.toml; do
            if [[ -f "${pkg_dir}/config/${cfg}" && ! -f "${CONFIG_DIR}/${cfg}" ]]; then
                cp "${pkg_dir}/config/${cfg}" "${CONFIG_DIR}/"
                success "Created ${cfg}"
            fi
        done
    fi
}

install_from_remote() {
    local version="${1:-}"
    local arch

    arch="$(detect_arch)"

    # Get version
    if [[ -z "$version" ]]; then
        log "Fetching latest release..."
        version="$(get_latest_release)"
    fi

    if [[ -z "$version" ]]; then
        die "Could not determine release version"
    fi

    log "Installing kitchn ${version} for ${arch}"

    # Download and extract
    local pkg_dir
    pkg_dir="$(download_release "$version" "$arch")"

    # Install from package
    install_from_package "$pkg_dir"

    # Cleanup
    rm -rf "$(dirname "$pkg_dir")"
}

# -----------------------------------------------------------------------------
# Main Installation
# -----------------------------------------------------------------------------
main() {
    local requested_version="${1:-}"

    # Manual scope change to match standard header
    echo -e "${PURPLE}[kitchn]${NC} INSTALL  starting installation"

    # Detect installation mode
    detect_install_mode

    # Create directories
    create_dir "$CONFIG_DIR"
    create_dir "$INSTALL_DIR"

    # Write config files (only in source/remote mode, package mode has its own)
    if [[ "$INSTALL_MODE" != "package" ]]; then
        write_config "${CONFIG_DIR}/theme.toml" "$THEME_CONFIG"
        write_config "${CONFIG_DIR}/icons.toml" "$ICONS_CONFIG"
        write_config "${CONFIG_DIR}/layout.toml" "$LAYOUT_CONFIG"
        write_config "${CONFIG_DIR}/cookbook.toml" "$DICTIONARY_CONFIG"
    fi

    # Install based on mode
    case "$INSTALL_MODE" in
        source)
            install_from_source
            ;;
        package)
            install_from_package "$SCRIPT_DIR"
            ;;
        remote)
            install_from_remote "$requested_version"
            ;;
    esac

    # Manual scope change to match standard footer
    echo -e "${GREEN}[summary]${NC} summary  installed successfully to $INSTALL_DIR"

    # PATH check
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        warn "$INSTALL_DIR not in PATH"
        echo "  Add to config.fish: set -Ua fish_user_paths \$HOME/.local/bin"
    fi

    # Show installed version
    if command_exists "${INSTALL_DIR}/kitchn"; then
        log "Installed version: $("${INSTALL_DIR}/kitchn" --version 2>/dev/null || echo "unknown")"
    fi
}

compact_binary() {
    local bin="$1"
    if [[ -f "$bin" ]]; then
        local size_before=$(stat -c%s "$bin")
        upx --best --lzma --quiet "$bin" > /dev/null
        local size_after=$(stat -c%s "$bin")
        local saved=$(( size_before - size_after ))
        local percent=$(( (saved * 100) / size_before ))
        
        # Convert bytes to readable format
        local size_before_fmt=$(numfmt --to=iec-i --suffix=B "$size_before")
        local size_after_fmt=$(numfmt --to=iec-i --suffix=B "$size_after")
        
        log "Optimized $(basename "$bin"): ${size_before_fmt} -> ${size_after_fmt} (-${percent}%)"
    fi
}

main "$@"