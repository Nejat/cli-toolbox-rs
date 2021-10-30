function Confirm-Success {
    param([string] $step)

    if ($lastExitCode -ne 0) {
        Write-Host
        Write-Host "$step failed with exit code $lastExitCode" -ForegroundColor Red
        Write-Host
        exit $lastExitCode
    }
}

[string] $script = $PSCommandPath.Split([IO.Path]::DirectorySeparatorChar)[-1];

if (Test-Path -Path $script -PathType Leaf) {
    Write-Host
    Write-Host "Scripts need to be run from the project root, i.e. '.\scripts-ps\$script'" -ForegroundColor Yellow
    Write-Host

    exit -1
}

clear
Write-Host "running clippy debug unoptimized" -ForegroundColor Yellow
cargo clippy --features="debug"
Confirm-Success "clippy debug unoptimized"

Write-Host "running clippy debug optimized" -ForegroundColor Yellow
cargo clippy --release --features="debug"
Confirm-Success "clippy debug optimized"

Write-Host "running clippy eval unoptimized" -ForegroundColor Yellow
cargo clippy --features="eval"
Confirm-Success "clippy eval unoptimized"

Write-Host "running clippy eval optimized" -ForegroundColor Yellow
cargo clippy --release --features="eval"
Confirm-Success "clippy eval optimized"

Write-Host "running clippy release unoptimized" -ForegroundColor Yellow
cargo clippy --features="release"
Confirm-Success "clippy release unoptimized"

Write-Host "running clippy release optimized" -ForegroundColor Yellow
cargo clippy --release --features="release"
Confirm-Success "clippy release optimized"

Write-Host "running clippy report unoptimized" -ForegroundColor Yellow
cargo clippy --features="report"
Confirm-Success "clippy report unoptimized"

Write-Host "running clippy report optimized" -ForegroundColor Yellow
cargo clippy --release --features="report"
Confirm-Success "clippy report optimized"

