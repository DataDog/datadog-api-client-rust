#!/usr/bin/env bash
set -euo pipefail

readonly REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${REPO_ROOT}"

scripts/install-license-tool.sh
export CARGO_RESOLVER_LOCKFILE_PATH="${REPO_ROOT}/.ci/license/Cargo.lock"
cargo update
dd-rust-license-tool write
