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
Write-Host "running test all macros unoptimized" -ForegroundColor Yellow
cargo test --features="all" -- --nocapture --test-threads=1
Confirm-Success "test unoptimized"

Write-Host "running test all macros optimized" -ForegroundColor Yellow
cargo test --features="all" --release -- --nocapture --test-threads=1
Confirm-Success "test optimized"
