#!/usr/bin/env bash
# shellcheck disable=SC2155
# =============================================================================
# hyprink Install Script
# Sets up config directory, creates default configs, and installs binaries
#
# Usage:
#   From source (in repo):  ./install.sh
#   From release package:   ./install.sh
#   Remote install:         curl -fsSL https://raw.githubusercontent.com/ryugen-io/hyprink/master/install.sh | bash
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
readonly CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/hypr"
readonly CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/hyprink"
readonly DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/hyprink"
readonly INSTALL_DIR="${HOME}/.local/bin/hypr"

# GitHub Release Settings
readonly REPO="ryugen-io/hyprink"
readonly GITHUB_API="https://api.github.com/repos/${REPO}/releases"

# Installation mode: "source", "package", or "remote"
INSTALL_MODE=""

# -----------------------------------------------------------------------------
# Logging - Source shared lib or use inline fallback
# -----------------------------------------------------------------------------
if [[ -n "$SCRIPT_DIR" && -f "${SCRIPT_DIR}/dev/scripts/lib/log.sh" ]]; then
    # shellcheck source=dev/scripts/lib/log.sh
    source "${SCRIPT_DIR}/dev/scripts/lib/log.sh"
    # Adapter functions for install script
    log()     { log_info "INSTALL" "$*"; }
    success() { log_ok "INSTALL" "$*"; }
    warn()    { log_warn "INSTALL" "$*"; }
    error()   { log_error "INSTALL" "$*"; }
    die()     { log_error "INSTALL" "$*"; exit 1; }
    header()  { log_step "INSTALL" "$*"; }
else
    # Inline fallback (for remote install / extracted packages)
    readonly GREEN=$'\033[38;2;80;250;123m'
    readonly YELLOW=$'\033[38;2;241;250;140m'
    readonly CYAN=$'\033[38;2;139;233;253m'
    readonly RED=$'\033[38;2;255;85;85m'
    readonly PURPLE=$'\033[38;2;189;147;249m'
    readonly NC=$'\033[0m'

    log()     { echo -e "${CYAN}[info]${NC} INSTALL  $*"; }
    success() { echo -e "${GREEN}[ok]${NC}   INSTALL  $*"; }
    warn()    { echo -e "${YELLOW}[warn]${NC} INSTALL  $*" >&2; }
    error()   { echo -e "${RED}[error]${NC} INSTALL  $*" >&2; }
    die()     { error "$*"; exit 1; }
    header()  { echo -e "${PURPLE}[hyprink]${NC} INSTALL  $*"; }
fi

# -----------------------------------------------------------------------------
# Cleanup & Signal Handling
# -----------------------------------------------------------------------------
cleanup() {
    local exit_code=$?
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
    elif [[ -n "$SCRIPT_DIR" && -d "${SCRIPT_DIR}/bin" && -f "${SCRIPT_DIR}/bin/hyprink" ]]; then
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
    local url="https://github.com/${REPO}/releases/download/${version}/hyprink-${version}-${arch}.tar.gz"
    local tmp_dir
    tmp_dir="$(mktemp -d)"

    log "Downloading ${url}..."

    if command_exists curl; then
        curl -fsSL "$url" -o "${tmp_dir}/hyprink.tar.gz" || die "Download failed"
    elif command_exists wget; then
        wget -q "$url" -O "${tmp_dir}/hyprink.tar.gz" || die "Download failed"
    fi

    log "Extracting..."
    tar -xzf "${tmp_dir}/hyprink.tar.gz" -C "$tmp_dir"

    # Find extracted directory
    local pkg_dir
    pkg_dir="$(find "$tmp_dir" -maxdepth 1 -type d -name 'hyprink-*' | head -1)"

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
# Config Template (single hyprink.conf)
# -----------------------------------------------------------------------------
HYPRINK_CONFIG='# hypr metadata
# type = theme
# hyprink.conf - Single Source of Truth for system theming
# Location: ~/.config/hypr/hyprink.conf

[theme]
name = "Sweet Dracula"
active_icons = "nerdfont"

[theme.colors]
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
hyprink = "#BD93F9"
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

[theme.fonts]
mono = "JohtoMono Nerd Font Mono"
ui = "Roboto"
size_mono = "10"
size_ui = "11"

[icons]
[icons.nerdfont]
success = ""
error = ""
warn = ""
info = ""
hyprink = ""
summary = ""
net = "󰖩"

[icons.ascii]
success = "*"
error = "!"
warn = "!!"
info = "i"
hyprink = "H"
summary = "="
net = "#"

[layout]
[layout.tag]
prefix = "["
suffix = "]"
transform = "lowercase"
min_width = 0
alignment = "left"

[layout.labels]
error = "error"
success = "success"
info = "info"
warn = "warn"

[layout.structure]
terminal = "{tag} {scope} {icon} {msg}"
file = "{timestamp} {tag} {msg}"

[layout.logging]
base_dir = "~/.local/state/hyprink/logs"
path_structure = "{year}/{month}/{scope}"
filename_structure = "{level}.{year}-{month}-{day}.log"
timestamp_format = "%H:%M:%S"
write_by_default = true

# Presets - reusable log message definitions
[presets.boot_ok]
level = "success"
scope = "SYSTEM"
msg = "startup complete"

[presets.deploy_fail]
level = "error"
scope = "DEPLOY"
msg = "deployment failed"
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
    if ! cargo build --release --bin hyprink 2>&1; then
        die "Build failed"
    fi
    success "Binary build complete"

    # Compact binaries if UPX is available
    if command_exists upx; then
        log "Compacting binaries with UPX..."
        compact_binary "target/release/hyprink"
    fi

    # Install binaries
    local src="target/release/hyprink"
    [[ -f "$src" ]] && cp "$src" "$INSTALL_DIR/" || die "Binary not found: $src"
}

install_from_package() {
    local pkg_dir="$1"

    # Install binaries
    local src="${pkg_dir}/bin/hyprink"
    [[ -f "$src" ]] && cp "$src" "$INSTALL_DIR/" || die "Binary not found: $src"
    chmod +x "${INSTALL_DIR}/hyprink"

    # Install config from package if it exists
    if [[ -f "${pkg_dir}/config/hyprink.conf" && ! -f "${CONFIG_DIR}/hyprink.conf" ]]; then
        cp "${pkg_dir}/config/hyprink.conf" "${CONFIG_DIR}/"
        success "Created hyprink.conf"
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

    log "Installing hyprink ${version} for ${arch}"

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

    header "starting installation"

    # Detect installation mode
    detect_install_mode

    # Create directories
    create_dir "$CONFIG_DIR"
    create_dir "$INSTALL_DIR"
    create_dir "$CACHE_DIR"
    create_dir "$DATA_DIR"

    # Write config file (only in source/remote mode, package mode has its own)
    if [[ "$INSTALL_MODE" != "package" ]]; then
        write_config "${CONFIG_DIR}/hyprink.conf" "$HYPRINK_CONFIG"
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

    # Use log_summary if available, else fallback
    if type log_summary &>/dev/null; then
        log_summary "summary" "installed successfully to $INSTALL_DIR"
    else
        echo -e "${GREEN}[summary]${NC} summary  installed successfully to $INSTALL_DIR"
    fi

    # PATH check
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        warn "$INSTALL_DIR not in PATH"
        echo "  Add to config.fish: set -Ua fish_user_paths \$HOME/.local/bin"
    fi

    # Show installed version
    if command_exists "${INSTALL_DIR}/hyprink"; then
        log "Installed version: $("${INSTALL_DIR}/hyprink" --version 2>/dev/null || echo "unknown")"
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
