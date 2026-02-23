# =============================================================
# build.ps1 - Build the Rust portfolio and generate dist/
# Usage: .\scripts\build.ps1
# =============================================================

$ErrorActionPreference = "Stop"

Write-Host "`n[1/2] Building Rust static site generator..." -ForegroundColor Cyan

# Add cargo to PATH
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"

# Build in release mode
cargo +stable-x86_64-pc-windows-gnu build --release 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo build failed!"
    exit 1
}

Write-Host "[2/2] Generating site..." -ForegroundColor Cyan
cargo +stable-x86_64-pc-windows-gnu run --release 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Error "Site generation failed!"
    exit 1
}

Write-Host "`nBuild complete!" -ForegroundColor Green
Write-Host "Output: .\dist\index.html" -ForegroundColor Yellow
Write-Host "Preview: Start-Process '.\dist\index.html'" -ForegroundColor Yellow
