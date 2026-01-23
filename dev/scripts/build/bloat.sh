#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${SCRIPT_DIR}/../../.."
source "${SCRIPT_DIR}/../lib/log.sh"

SCOPE="BLOAT"
CRATE="${1:-}"

cd "$PROJECT_ROOT"

log_info "$SCOPE" "=== starting bloat.sh ==="
log_info "$SCOPE" "working directory: $(pwd)"
log_info "$SCOPE" "target crate: ${CRATE:-all binaries}"

log_step "$SCOPE" "checking for cargo-bloat"
if ! cargo bloat --version &>/dev/null; then
    log_error "$SCOPE" "cargo-bloat not installed"
    log_info "$SCOPE" "install with: cargo install cargo-bloat"
    log_info "$SCOPE" "=== bloat.sh failed ==="
    exit 1
fi
log_ok "$SCOPE" "cargo-bloat found"

if [[ -n "$CRATE" ]]; then
    log_info "$SCOPE" "analyzing single crate: ${CRATE}"
    log_step "$SCOPE" "executing: cargo bloat --release -p ${CRATE} --crates"
    echo ""
    cargo bloat --release -p "$CRATE" --crates
else
    log_info "$SCOPE" "analyzing hi_cli binary"
    log_step "$SCOPE" "executing: cargo bloat --release -p hi_cli --crates -n 10"
    echo ""
    if cargo bloat --release -p hi_cli --crates -n 10 2>/dev/null; then
        log_ok "$SCOPE" "hi_cli analysis complete"
    else
        log_warn "$SCOPE" "hi_cli analysis failed (binary may not exist)"
    fi
fi

log_ok "$SCOPE" "bloat analysis complete"
log_info "$SCOPE" "=== bloat.sh finished successfully ==="
