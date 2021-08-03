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

if (!$SkipClean)
{
    cargo clean
    Confirm-Success "clean"
}

cargo clippy
Confirm-Success "clippy"

cargo clippy --release
Confirm-Success "clippy release"

cargo test  -- --nocapture --test-threads=1
Confirm-Success "test"

cargo test  --release -- --nocapture --test-threads=1
Confirm-Success "test release"

cargo publish --locked --dry-run
Confirm-Success "publish dry run"

cargo publish --locked
Confirm-Success "publish"
