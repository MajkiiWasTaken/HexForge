#!/usr/bin/env bash
# =============================================================================
# HexForge - Linux Uninstaller
# =============================================================================
# Platform : Linux
# Scope    : Current user (no sudo required)
# Removes  : ~/.local/bin/hexforge only
# =============================================================================

set -euo pipefail

EXECUTABLE="${HOME}/.local/bin/hexforge"

printf '[HexForge] Uninstalling...\n'

if [[ -f "$EXECUTABLE" || -L "$EXECUTABLE" ]]; then
  rm -- "$EXECUTABLE"
  printf '[HexForge] Removed: %s\n' "$EXECUTABLE"
else
  printf '[HexForge] No installed executable found.\n'
fi

# Keep ~/.local/bin and PATH untouched: other tools may use them.
printf '[HexForge] Uninstallation complete.\n'
