# For Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu --bin rustybench-linux64

# For Windows x86_64
cargo build --release --target x86_64-pc-windows-gnu --bin rustybench-windows64
# or if you prefer the MSVC toolchain:
# cargo build --release --target x86_64-pc-windows-msvc --bin rustybench-windows64

# For ARM64 (Linux)
cargo build --release --target aarch64-unknown-linux-gnu --bin rustybench-arm64

