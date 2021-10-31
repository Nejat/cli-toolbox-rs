#!/bin/bash

IFS='/' read -ra script_parts <<< "$0"

NC='\033[0m'
RED='\033[0;31m'
YELLOW='\033[1;33m'

if [[ "$0" == "./${script_parts[-1]}" ]]
then
  echo -e "\n${YELLOW}Scripts need to be run from the project root${NC}, i.e. ./scripts-ps/${script_parts[-1]}\n"
  exit 1
fi

function confirm-success() {
    exit_code=$?
    if [ $exit_code -ne 0 ]
    then
        echo -e "\n$1 ${RED}failed${NC} with exitcode ${exit_code}\n"
        exit $exit_code
    fi
}

clear

for_reals="false"
skip_clean="false"

while test $# -gt 0; do
  case "$1" in
    --for_reals) for_reals="true" && shift ;;
    --skip_clean) skip_clean="true" && shift ;;
    *) shift ;;
  esac
done

if [[ "${for_reals}" == "true" ]] && [[ "${skip_clean}" == "true" ]]
then
    echo -e "${YELLOW}Can not skip clean step when actually publishing${NC}\n"

    skip_clean="false"
fi

if [[ "${skip_clean}" == "true" ]]
then
    cargo clean
    confirm-success "clean"
fi

echo -e "${YELLOW}running clippy debug unoptimized"
cargo clippy --features="debug"
confirm-success "clippy debug unoptimized"

echo -e "${YELLOW}running clippy debug optimized"
cargo clippy --release --features="debug"
confirm-success "clippy debug optimized"

echo -e "${YELLOW}running clippy eval unoptimized"
cargo clippy --features="eval"
confirm-success "clippy eval unoptimized"

echo -e "${YELLOW}running clippy eval optimized"
cargo clippy --release --features="eval"
confirm-success "clippy eval optimized"

echo -e "${YELLOW}running clippy release unoptimized"
cargo clippy --features="release"
confirm-success "clippy release unoptimized"

echo -e "${YELLOW}running clippy release optimized"
cargo clippy --release --features="release"
confirm-success "clippy release optimized"

echo -e "${YELLOW}running clippy report unoptimized"
cargo clippy --features="report"
confirm-success "clippy report unoptimized"

echo -e "${YELLOW}running clippy report optimized"
cargo clippy --release --features="report"
confirm-success "clippy report optimized"

echo -e "${YELLOW}running test all macros unoptimized"
cargo test --features="all" -- --nocapture --test-threads=1
confirm-success "test all macros unoptimized"

echo -e "${YELLOW}running test all macros optimized"
cargo test --features="all" --release -- --nocapture --test-threads=1
confirm-success "test all macros optimized"

if [[ "${for_reals}" == "true" ]]
then
  echo -e "${YELLOW}publish"
  cargo publish --locked --all-features
  confirm-success "publish"
else
  echo -e "${YELLOW}publish dry run"
  cargo publish --locked --all-features --dry-run
  confirm-success "publish dry run"
fi