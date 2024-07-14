# rust-os
OS in Rust based on this blog: https://os.phil-opp.com/

## Notes


## How to build it
```bash
# Install the Rust source code to recompile the core and compiler_builtins libs (.cargo/config.toml)
rustup component add rust-src 

cargo build
```

## Creating a Bootimage
```bash
cargo install bootimage

rustup component add llvm-tools-preview

cargo bootimage
```

## Booting it in QEMU