#!/bin/bash

set -euo pipefail

cargo install genemichaels@0.5.11
genemichaels --log silent examples/*.rs
cargo fmt
# Keep the second pass: rustfmt has an idempotency edge case on generated code.
cargo fmt
