#!/usr/bin/env bash
set -euo pipefail

readonly LICENSE_TOOL_VERSION="1.0.6"

if command -v dd-rust-license-tool >/dev/null 2>&1 &&
  [[ "$(dd-rust-license-tool --version)" == "dd-rust-license-tool ${LICENSE_TOOL_VERSION}" ]]; then
  exit 0
fi

cargo install --quiet --locked --force \
  --version "${LICENSE_TOOL_VERSION}" dd-rust-license-tool
