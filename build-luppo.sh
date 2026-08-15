#!/usr/bin/env bash
set -euo pipefail

echo "=========================================="
echo " Luppi — Luppo Package Builder"
echo "=========================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export LUPPI_SRC_DIR="${SCRIPT_DIR}"
cd "${SCRIPT_DIR}"

echo "[1/2] Building Rust release binary..."
cargo build --release

echo "[2/2] Creating Luppo package (.luppo)..."
if command -v luppo &>/dev/null; then
    luppo build pspec.xml --no-sandbox --ignore-dependency
else
    echo "Error: luppo command not found!"
    exit 1
fi

echo "Locating generated .luppo package..."

LUPPI_FILE=$(find . /var/luppo /tmp -name "luppi-*.luppo" 2>/dev/null | head -n 1 || true)

if [ -n "${LUPPI_FILE}" ] && [ -f "${LUPPI_FILE}" ]; then
    TARGET_PATH="${SCRIPT_DIR}/$(basename "${LUPPI_FILE}")"
    if [ "${LUPPI_FILE}" != "${TARGET_PATH}" ]; then
        cp -f "${LUPPI_FILE}" "${TARGET_PATH}"
    fi
    
    if [ -n "${SUDO_USER:-}" ]; then
        chown "${SUDO_USER}:" "${TARGET_PATH}" 2>/dev/null || true
    fi
    
    echo ""
    echo "=========================================="
    echo " SUCCESS! .luppo package saved to:"
    echo " ${TARGET_PATH}"
    echo "=========================================="
else
    echo "Error: .luppo package file could not be found."
    exit 1
fi
