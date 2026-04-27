# Author: William C. Canin <https://williamcanin.github.io>

param(
    [switch]$Uninstall
)

$ErrorActionPreference = "Stop"

$NAME             = "seedctl"
$REPO             = "evolvbits/seedctl"
$API_URL          = "https://api.github.com/repos/$REPO/releases/latest"
$BINARY_NAME      = "seedctl"
$ARCH             = "windows-x86_64"
$INSTALLATION_DIR = "$env:LOCALAPPDATA\Programs\$NAME"

# ----- libs -----
function Title($msg) {
    Write-Host "[ $msg ]" -ForegroundColor Magenta
}

function Info($msg, $val = $null) {
    Write-Host "-> $msg" -ForegroundColor Cyan -NoNewline
    if ($null -ne $val) { Write-Host $val } else { Write-Host "" }
}

function Finish($msg) {
    Write-Host "* $msg" -ForegroundColor Green
}

function Warning($msg, $val = $null) {
    Write-Host "! $msg" -ForegroundColor Yellow -NoNewline
    if ($null -ne $val) { Write-Host $val } else { Write-Host "" }
}

function Err($msg) {
    Write-Host "x $msg" -ForegroundColor Red
}

# ----- Ignore Administrator -----
$principal = [Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
if ($principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Err "Error: This script should not be run as Administrator."
    exit 1
}

# ----- Uninstall mode -----
if ($Uninstall) {
    Title "$NAME Uninstall"

    $target = "$INSTALLATION_DIR\$BINARY_NAME.exe"
    if (Test-Path $target) {
        Info "Removing from: " $INSTALLATION_DIR
        Remove-Item $target -Force -Verbose

        # ----- Remove from PATH -----
        $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
        if ($currentPath -like "*$INSTALLATION_DIR*") {
            $newPath = ($currentPath -split ";" | Where-Object { $_ -ne $INSTALLATION_DIR }) -join ";"
            [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        }

        Finish "Uninstallation completed!"
    } else {
        Warning "No installation found."
    }

    exit 0
}

# ----- Download mode -----
Title "$NAME Installation"

try {
    $release     = Invoke-RestMethod -Uri $API_URL -UseBasicParsing
    $VERSION_TAG = $release.tag_name -replace '^v', ''
} catch {
    Err "Error: Could not retrieve the latest release version from GitHub."
    exit 1
}

if (-not $VERSION_TAG) {
    Err "Error: Could not retrieve the latest release version from GitHub."
    exit 1
}

$TARGET_FILE  = "$BINARY_NAME-$VERSION_TAG-$ARCH.exe"
$DOWNLOAD_URL = "https://github.com/$REPO/releases/download/v$VERSION_TAG/$TARGET_FILE"
$TMP_FILE     = "$env:TEMP\$TARGET_FILE"

Info "Latest version: " $VERSION_TAG
Info "Target file: "    $TARGET_FILE
Info "Download link: "  $DOWNLOAD_URL

try {
    Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile $TMP_FILE -UseBasicParsing
    Finish "Download completed successfully."
} catch {
    Err "Error: Failed to download the latest release."
    Remove-Item $TMP_FILE -Force -ErrorAction SilentlyContinue
    exit 1
}

Info "Target file rename to: " "$BINARY_NAME.exe"

# ----- Show SHA256 Binary -----
$hash = (Get-FileHash $TMP_FILE -Algorithm SHA256).Hash.ToLower()
Info "SHA256SUM Binary: " "$hash  $TARGET_FILE"

# ----- Install mode -----
if (-not (Test-Path $INSTALLATION_DIR)) {
    New-Item -ItemType Directory -Path $INSTALLATION_DIR -Force | Out-Null
}

$DEST = "$INSTALLATION_DIR\$BINARY_NAME.exe"
Remove-Item $DEST -Force -ErrorAction SilentlyContinue
Copy-Item $TMP_FILE $DEST -Force
Remove-Item $TMP_FILE -Force -ErrorAction SilentlyContinue

# ----- Add to PATH -----
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$INSTALLATION_DIR*") {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$INSTALLATION_DIR", "User")
}

# ----- Info mode -----
Finish "Installation completed successfully!"
Warning "$NAME was installed on: "; Write-Host $INSTALLATION_DIR
Warning "NOTE: "; Write-Host "Restart your terminal to apply the PATH changes."