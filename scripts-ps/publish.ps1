param(
    [switch] $ForReals = $false,
    [switch] $SkipClean = $false
)

[string] $script = $PSCommandPath.Split([IO.Path]::DirectorySeparatorChar)[-1];

if (Test-Path -Path $script -PathType Leaf) {
    Write-Host
    Write-Host "Scripts need to be run from the project root, i.e. '.\scripts-ps\$script'" -ForegroundColor Yellow
    Write-Host

    exit -1
}

function Confirm-Success {
    param([string] $step)

    if ($lastExitCode -ne 0) {
        Write-Host
        Write-Host "$step failed with exit code $lastExitCode" -ForegroundColor Red
        Write-Host
        exit $lastExitCode
    }
}

clear

if ($ForReals -and $SkipClean) {
    Write-Host Can not skip clean step when actually publishing -ForegroundColor Yellow
    Write-Host

    $SkipClean = $false
}

if (!$SkipClean) {
    cargo clean
    Confirm-Success "clean"
}

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

Write-Host "running test all macros unoptimized" -ForegroundColor Yellow
cargo test --features="all" -- --nocapture --test-threads=1
Confirm-Success "test"

Write-Host "running test all macros optimized" -ForegroundColor Yellow
cargo test --release --features="all" -- --nocapture --test-threads=1
Confirm-Success "test release"

if ($ForReals) {
    Write-Host "publish" -ForegroundColor Yellow
    cargo publish --locked --all-features
    Confirm-Success "publish"
} else {
    Write-Host "publish dry run" -ForegroundColor Yellow
    cargo publish --locked --all-features --dry-run
    Confirm-Success "publish dry run"
}
