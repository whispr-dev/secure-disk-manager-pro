param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"

Write-Host "== Secure DiskManager Rust Ops build helper =="
Write-Host "Working directory: $(Get-Location)"

if (-not (Test-Path ".\Cargo.toml")) {
    throw "Cargo.toml not found. Run this script from the secure_diskmanager_ops_rust crate root."
}
if (-not (Test-Path ".\src\main.rs")) {
    throw "src\main.rs not found. This is not the CLI-patched crate folder."
}

Write-Host "`n== Cargo targets =="
$cargoToml = Get-Content ".\Cargo.toml" -Raw
if ($cargoToml -notmatch '\[\[bin\]\]') {
    throw "Cargo.toml does not contain an explicit [[bin]] target. Use v0.1.4 or later."
}
Select-String -Path ".\Cargo.toml" -Pattern 'name =|version =|\[\[bin\]\]|\[\[example\]\]|path =' | ForEach-Object { $_.Line }

Write-Host "`n== Cleaning old target output =="
cargo clean

$profile = if ($Release) { "release" } else { "debug" }
$releaseArg = if ($Release) { "--release" } else { "" }

Write-Host "`n== Building binary target =="
if ($Release) {
    cargo build --release --bin secure_diskmanager_ops
} else {
    cargo build --bin secure_diskmanager_ops
}

$exe = Join-Path ".\target\$profile" "secure_diskmanager_ops.exe"
if (-not (Test-Path $exe)) {
    Write-Host "`nExpected exe missing: $exe"
    Write-Host "All exe files Cargo produced:"
    Get-ChildItem -Path ".\target" -Recurse -Filter "*.exe" | Select-Object -ExpandProperty FullName
    throw "Binary build completed but expected exe was not found. Paste this full output back into chat."
}

Write-Host "`n== Building demo examples =="
if ($Release) {
    cargo build --release --example demo
    cargo build --release --example python_ops_demo
    $demoExe = ".\target\release\examples\demo.exe"
    $pythonDemoExe = ".\target\release\examples\python_ops_demo.exe"
} else {
    cargo build --example demo
    cargo build --example python_ops_demo
    $demoExe = ".\target\debug\examples\demo.exe"
    $pythonDemoExe = ".\target\debug\examples\python_ops_demo.exe"
}

foreach ($expected in @($demoExe, $pythonDemoExe)) {
    if (-not (Test-Path $expected)) {
        Write-Host "`nExpected example exe missing: $expected"
        Write-Host "All exe files Cargo produced:"
        Get-ChildItem -Path ".\target" -Recurse -Filter "*.exe" | Select-Object -ExpandProperty FullName
        throw "Example build completed but expected example exe was not found. Paste this full output back into chat."
    }
}

Write-Host "`n== Exes found =="
Get-ChildItem -Path ".\target" -Recurse -Filter "*.exe" | Select-Object -ExpandProperty FullName

Write-Host "`n== CLI help smoke test =="
& $exe help

Write-Host "`nSUCCESS"
Write-Host "Runnable CLI:  $((Resolve-Path $exe).Path)"
Write-Host "Runnable demo: $((Resolve-Path $demoExe).Path)"
Write-Host "Runnable Python-ops demo: $((Resolve-Path $pythonDemoExe).Path)"
