#!/usr/bin/env bash
set -euo pipefail

scripts/install-license-tool.sh
cargo metadata --locked --format-version 1 >/dev/null
dd-rust-license-tool check
