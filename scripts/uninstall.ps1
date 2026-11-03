# =============================================================================
# HexForge - Windows Uninstaller
# =============================================================================
# Platform : Windows
# Scope    : Current user (no administrator rights required)
# Removes  : Installed executable and its user PATH entry
# =============================================================================

$ErrorActionPreference = 'Stop'

$InstallDir = Join-Path $env:LOCALAPPDATA 'Programs\HexForge'

Write-Host '[HexForge] Uninstalling...' -ForegroundColor Cyan

if (Test-Path -LiteralPath $InstallDir) {
    Remove-Item -LiteralPath $InstallDir -Recurse -Force
    Write-Host "[HexForge] Removed: $InstallDir"
}
else {
    Write-Host '[HexForge] No installed files found.'
}

# Remove only the HexForge entry; preserve all other PATH entries.
$UserPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$UpdatedPath = (@($UserPath -split ';') | Where-Object {
    $_.Trim() -and $_.Trim().TrimEnd('\') -ine $InstallDir.TrimEnd('\')
}) -join ';'

if ($UpdatedPath -ne $UserPath) {
    [Environment]::SetEnvironmentVariable('Path', $UpdatedPath, 'User')
    Write-Host '[HexForge] Removed user PATH entry.'
}

Write-Host '[HexForge] Uninstallation complete.' -ForegroundColor Green
Write-Host '  Open a new terminal to refresh PATH.'
