# Restore after reinstall

This workspace is backed up on branch `wy-dev-feature`.

## Clone

```sh
git clone --recurse-submodules git@github.com:kissyanw/rupta_rustui_workspace.git
cd rupta_rustui_workspace
git checkout wy-dev-feature
git submodule update --init --recursive
```

## Rust toolchain

The repository pins the Rust toolchain in `rust-toolchain.toml`:

```sh
rustup toolchain install nightly-2025-05-09
rustup component add rust-src rustc-dev llvm-tools-preview --toolchain nightly-2025-05-09
```

## Fast sanity checks

```sh
cargo build --manifest-path rupta/Cargo.toml --bin cargo-pta
cargo build --manifest-path rustdsl/Cargo.toml
cargo test --manifest-path lite_cast_erase/test_programs/cast_risk_matrix/Cargo.toml
```

## Not backed up in Git

These local/generated directories are intentionally not committed because they are large or reproducible:

- `.cargo-home/`
- `target/` directories
- `analysis_results/`
- `.aider*`
