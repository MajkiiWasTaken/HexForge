# =============================================================================
# HexForge - Windows Installer
# =============================================================================
# Platform : Windows x86-64
# Requires : PowerShell 5.1+
# Scope    : Current user (no administrator rights required)
# Source   : GitHub Releases
# =============================================================================

$ErrorActionPreference = 'Stop'

$Repository = 'MajkiiWasTaken/HexForge'
$InstallDir = Join-Path $env:LOCALAPPDATA 'Programs\HexForge'
$Executable = Join-Path $InstallDir 'hexforge.exe'
$Headers = @{
    'User-Agent' = 'HexForge-Installer'
    'Accept' = 'application/vnd.github+json'
}

function Write-Step {
    param([string]$Message)
    Write-Host "[HexForge] $Message" -ForegroundColor Cyan
}

function Write-Success {
    param([string]$Message)
    Write-Host "[HexForge] $Message" -ForegroundColor Green
}

function Test-PathEntry {
    param([string]$PathValue, [string]$Directory)

    $Normalized = $Directory.TrimEnd('\')
    foreach ($Entry in ($PathValue -split ';')) {
        if ($Entry.Trim().TrimEnd('\') -ieq $Normalized) {
            return $true
        }
    }
    return $false
}

# Windows ARM64 is supported only when x86-64 emulation is available.
if (-not [Environment]::Is64BitOperatingSystem) {
    throw 'HexForge requires a 64-bit Windows installation.'
}

$TempDir = Join-Path ([System.IO.Path]::GetTempPath()) (
    'hexforge-' + [Guid]::NewGuid().ToString('N')
)
New-Item -ItemType Directory -Path $TempDir -Force | Out-Null

try {
    Write-Step 'Checking latest release...'
    $Release = Invoke-RestMethod `
        -Uri "https://api.github.com/repos/$Repository/releases/latest" `
        -Headers $Headers

    $Tag = [string]$Release.tag_name
    if (-not $Tag) { throw 'Could not determine the latest release.' }

    $AssetName = "hexforge-$Tag-windows-x86_64.zip"
    $Asset = $Release.assets |
        Where-Object { $_.name -eq $AssetName } |
        Select-Object -First 1

    if (-not $Asset) { throw "Release archive not found: $AssetName" }

    $ArchivePath = Join-Path $TempDir $AssetName
    Write-Step "Downloading HexForge $Tag..."
    Invoke-WebRequest -Uri $Asset.browser_download_url `
        -Headers $Headers -OutFile $ArchivePath -UseBasicParsing

    # Verify archive against release checksums when provided.
    $ChecksumAsset = $Release.assets |
        Where-Object { $_.name -eq 'SHA256SUMS.txt' } |
        Select-Object -First 1

    if ($ChecksumAsset) {
        $ChecksumPath = Join-Path $TempDir 'SHA256SUMS.txt'
        Invoke-WebRequest -Uri $ChecksumAsset.browser_download_url `
            -Headers $Headers -OutFile $ChecksumPath -UseBasicParsing

        $Entry = Get-Content -LiteralPath $ChecksumPath |
            Where-Object { $_ -match ('\s\*?' + [regex]::Escape($AssetName) + '$') } |
            Select-Object -First 1

        if (-not $Entry) { throw "Checksum missing for $AssetName" }

        $Expected = ($Entry.Trim() -split '\s+')[0]
        $Actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $ArchivePath).Hash

        if ($Expected -ine $Actual) { throw 'SHA256 verification failed.' }
        Write-Success 'SHA256 verified.'
    }
    else {
        Write-Warning 'SHA256SUMS.txt was not published; checksum verification skipped.'
    }

    $ExtractDir = Join-Path $TempDir 'extract'
    Expand-Archive -LiteralPath $ArchivePath -DestinationPath $ExtractDir -Force

    $SourceExe = Join-Path $ExtractDir 'hexforge.exe'
    if (-not (Test-Path -LiteralPath $SourceExe -PathType Leaf)) {
        throw 'hexforge.exe was not found in the release archive.'
    }

    Write-Step 'Installing...'
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    Copy-Item -LiteralPath $SourceExe -Destination $Executable -Force

    # Register PATH for future terminals, without duplicating the entry.
    $UserPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    if (-not (Test-PathEntry -PathValue $UserPath -Directory $InstallDir)) {
        $NewPath = (@($UserPath, $InstallDir) |
            Where-Object { $_ -and $_.Trim() }) -join ';'
        [Environment]::SetEnvironmentVariable('Path', $NewPath, 'User')
        Write-Step 'Added install directory to user PATH.'
    }

    if (-not (Test-PathEntry -PathValue $env:Path -Directory $InstallDir)) {
        $env:Path += ";$InstallDir"
    }

    Write-Success "Installed HexForge $Tag"
    Write-Host "  Location : $Executable"
    Write-Host '  Verify   : hexforge --version'
    Write-Host '  Help     : hexforge --help'
    Write-Host '  Open a new terminal if the command is not yet recognized.'
}
finally {
    Remove-Item -LiteralPath $TempDir -Recurse -Force -ErrorAction SilentlyContinue
}
