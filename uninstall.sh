#!/usr/bin/env bash
# =============================================================================
# hyprink Uninstall Script
# Removes binaries, config, data and log directories
# =============================================================================

set -euo pipefail
IFS=$'\n\t'

shopt -s inherit_errexit 2>/dev/null || true

# -----------------------------------------------------------------------------
# Configuration
# -----------------------------------------------------------------------------
readonly CONFIG_FILE="${XDG_CONFIG_HOME:-$HOME/.config}/hypr/hyprink.conf"
readonly CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/hyprink"
readonly DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/hyprink"
readonly STATE_DIR="${XDG_STATE_HOME:-$HOME/.local/state}/hyprink"
readonly INSTALL_DIR="${HOME}/.local/bin"
readonly LIB_DIR="${HOME}/.local/lib/hyprink"
readonly INCLUDE_DIR="${HOME}/.local/include/hyprink"

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
log()     { echo -e "${CYAN}[info]${NC} ${INFO_ICON}  $*"; }
success() { echo -e "${GREEN}[ok]${NC}   ${CHECK}  $*"; }
warn()    { echo -e "${YELLOW}[warn]${NC} ${WARN}  $*" >&2; }
error()   { echo -e "${RED}[err]${NC}  ${ERR}  $*" >&2; }
die()     { error "$*"; exit 1; }

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
remove_if_exists() {
    local path="$1"
    local desc="$2"

    if [[ -e "$path" ]]; then
        rm -rf "$path"
        success "Removed $desc"
    else
        warn "$desc not found, skipping"
    fi
}

# -----------------------------------------------------------------------------
# Main
# -----------------------------------------------------------------------------
main() {
    log "Starting hyprink uninstall"

    # Remove binaries
    remove_if_exists "${INSTALL_DIR}/hyprink" "hyprink binary"

    # Remove libraries
    remove_if_exists "$LIB_DIR" "library directory"
    remove_if_exists "$INCLUDE_DIR" "include directory"

    # Remove directories
    remove_if_exists "$CONFIG_FILE" "config file"
    remove_if_exists "$CACHE_DIR" "cache directory"
    remove_if_exists "$DATA_DIR" "data directory"
    remove_if_exists "$STATE_DIR" "state directory"

    echo ""
    echo -e "${PURPLE}[hyprink]${NC} ${CHECK}  Uninstall complete"
}

main "$@"
