param([ValidateSet('test','run','check','fmt')][string]$Command = 'test')
$workspaceRoot = Split-Path $PSScriptRoot -Parent
$env:RUSTUP_HOME = Join-Path $workspaceRoot '.tools\rustup'
$env:CARGO_HOME = Join-Path $workspaceRoot '.tools\cargo'
$cargoExecutable = Join-Path $env:CARGO_HOME 'bin\cargo.exe'
if (-not (Test-Path -LiteralPath $cargoExecutable)) { throw 'Workspace-local toolchain missing. Use your normal cargo installation.' }
Push-Location $workspaceRoot
try { & $cargoExecutable $Command; exit $LASTEXITCODE } finally { Pop-Location }
