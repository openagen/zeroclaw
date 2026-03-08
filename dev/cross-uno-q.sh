#!/usr/bin/env bash
# Cross-compile ZeroClaw for Arduino UNO Q (aarch64 Debian Linux).
#
# Prerequisites:
#   brew install filosottile/musl-cross/musl-cross  # macOS
#   # or: apt install gcc-aarch64-linux-gnu          # Linux
#   rustup target add aarch64-unknown-linux-gnu
#
# Usage:
#   ./dev/cross-uno-q.sh          # release build
#   ./dev/cross-uno-q.sh --debug  # debug build

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

TARGET="aarch64-unknown-linux-gnu"
PROFILE="release"

if [[ "${1:-}" == "--debug" ]]; then
    PROFILE="dev"
fi

echo "==> Cross-compiling ZeroClaw for $TARGET ($PROFILE)"

# Check if cross is available (preferred)
if command -v cross &>/dev/null; then
    echo "    Using 'cross' (Docker-based cross-compilation)"
    cd "$PROJECT_DIR"
    if [[ "$PROFILE" == "release" ]]; then
        cross build --target "$TARGET" --release --features hardware
    else
        cross build --target "$TARGET" --features hardware
    fi
else
    # Native cross-compilation
    echo "    Using native toolchain"

    # Ensure target is installed
    rustup target add "$TARGET" 2>/dev/null || true

    # Detect linker
    if command -v aarch64-linux-gnu-gcc &>/dev/null; then
        LINKER="aarch64-linux-gnu-gcc"
    elif command -v aarch64-unknown-linux-gnu-gcc &>/dev/null; then
        LINKER="aarch64-unknown-linux-gnu-gcc"
    else
        echo "Error: No aarch64 cross-compiler found."
        echo "Install with:"
        echo "  macOS: brew tap messense/macos-cross-toolchains && brew install aarch64-unknown-linux-gnu"
        echo "  Linux: apt install gcc-aarch64-linux-gnu"
        echo "  Or install 'cross': cargo install cross"
        exit 1
    fi

    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="$LINKER"

    cd "$PROJECT_DIR"
    if [[ "$PROFILE" == "release" ]]; then
        cargo build --target "$TARGET" --release --features hardware
    else
        cargo build --target "$TARGET" --features hardware
    fi
fi

BINARY="$PROJECT_DIR/target/$TARGET/$( [[ $PROFILE == release ]] && echo release || echo debug )/zeroclaw"

if [[ -f "$BINARY" ]]; then
    SIZE=$(du -h "$BINARY" | cut -f1)
    echo "==> Build complete: $BINARY ($SIZE)"
    echo ""
    echo "Deploy to Uno Q:"
    echo "  zeroclaw peripheral deploy-uno-q --host <uno-q-ip>"
    echo ""
    echo "Or manually:"
    echo "  scp $BINARY arduino@<uno-q-ip>:~/zeroclaw/"
else
    echo "Error: binary not found at $BINARY"
    exit 1
fi
