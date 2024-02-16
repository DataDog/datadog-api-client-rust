cargo install --quiet dd-rust-license-tool
cargo fetch --target aarch64-apple-darwin
dd-rust-license-tool check
if [ $? -ne 0 ]; then
  echo "Run 'dd-rust-license-tool write' to regenerate license csv file."
fi
exit 0