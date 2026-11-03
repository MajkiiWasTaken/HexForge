#!/usr/bin/env bash
# =============================================================================
# HexForge - Linux Installer
# =============================================================================
# Platform : Linux x86-64
# Requires : Bash, curl, tar, sha256sum
# Scope    : Current user (no sudo required)
# Source   : GitHub Releases
# =============================================================================

set -euo pipefail

REPOSITORY='MajkiiWasTaken/HexForge'
INSTALL_DIR="${HOME}/.local/bin"

log()  { printf '[HexForge] %s\n' "$*"; }
fail() { printf '[HexForge] ERROR: %s\n' "$*" >&2; exit 1; }

[[ "$(uname -s)" == 'Linux' ]] || fail 'Linux is required.'
[[ "$(uname -m)" == 'x86_64' ]] || fail 'Only Linux x86-64 is supported.'

for command in curl tar sha256sum mktemp; do
  command -v "$command" >/dev/null 2>&1 || fail "Missing dependency: $command"
done

TMP_DIR="$(mktemp -d)"
trap 'rm -rf -- "$TMP_DIR"' EXIT

log 'Checking latest release...'
RELEASE_URL="$(curl -fLsS --retry 3 -o /dev/null -w '%{url_effective}' \
  "https://github.com/${REPOSITORY}/releases/latest")"
TAG="${RELEASE_URL##*/}"
[[ "$TAG" == v* ]] || fail 'Could not determine the latest release.'

ARCHIVE="hexforge-${TAG}-linux-x86_64.tar.gz"
BASE_URL="https://github.com/${REPOSITORY}/releases/download/${TAG}"

log "Downloading HexForge ${TAG}..."
curl -fLsS --retry 3 -o "${TMP_DIR}/${ARCHIVE}" "${BASE_URL}/${ARCHIVE}"

# Check the published archive checksum when available.
if curl -fLsS --retry 3 -o "${TMP_DIR}/SHA256SUMS.txt" "${BASE_URL}/SHA256SUMS.txt"; then
  EXPECTED="$(awk -v filename="$ARCHIVE" '$2 == filename || $2 == "*" filename { print $1; exit }' \
    "${TMP_DIR}/SHA256SUMS.txt")"
  [[ -n "$EXPECTED" ]] || fail "Checksum missing for ${ARCHIVE}."

  ACTUAL="$(sha256sum "${TMP_DIR}/${ARCHIVE}")"
  ACTUAL="${ACTUAL%% *}"
  [[ "${EXPECTED,,}" == "${ACTUAL,,}" ]] || fail 'SHA256 verification failed.'
  log 'SHA256 verified.'
else
  printf '[HexForge] WARNING: Checksums unavailable; verification skipped.\n' >&2
fi

log 'Installing...'
tar -xzf "${TMP_DIR}/${ARCHIVE}" -C "$TMP_DIR" -- hexforge
[[ -f "${TMP_DIR}/hexforge" ]] || fail 'hexforge not found in the archive.'
mkdir -p -- "$INSTALL_DIR"
install -m 755 "${TMP_DIR}/hexforge" "${INSTALL_DIR}/hexforge"

log "Installed HexForge ${TAG}"
printf '  Location : %s\n' "${INSTALL_DIR}/hexforge"
printf '  Verify   : hexforge --version\n'
printf '  Help     : hexforge --help\n'

if [[ ":${PATH}:" != *":${INSTALL_DIR}:"* ]]; then
  printf '\nAdd ~/.local/bin to your PATH if needed:\n'
  printf '  export PATH="$HOME/.local/bin:$PATH"\n'
fi
