# clang_link_test
Testing `libclang` linking on Windows

## Steps to check & reproduce

### `x86_64`
- `rustup default nightly-x86_64-pc-windows-gnu`
- `LIBCLANG_PATH` to `msys\mingw64\bin`
- `cargo run --target=x86_64-pc-windows-gnu`

### `i686`
- `rustup default nightly-i686-pc-windows-gnu`
- `LIBCLANG_PATH` to `msys\mingw32\bin`
- `cargo run --target=i686-pc-windows-gnu`
