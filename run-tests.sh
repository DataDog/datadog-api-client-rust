./scripts/license-check.sh
if [ $? -ne 0 ]; then
    exit 1
fi
cargo test --test main --
