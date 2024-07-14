# rust-os
OS in Rust based on this blog: https://os.phil-opp.com/

## How to build it
```bash
# Install the Rust source code to recompile the core and compiler_builtins libs (.cargo/config.toml)
rustup component add rust-src 

cargo build
```

## Creating a Bootimage
```bash
# Tool that compiles the kernel and bootloader, and then links them together to create a bootable disk image
cargo install bootimage

rustup component add llvm-tools-preview

cargo bootimage
```

## Booting it in [QEMU](https://www.qemu.org/download/)
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust-os.bin
```

Thanks to this configuration in the `./cargo/config.toml` file:
```bash
[target.'cfg(target_os = "none")']
runner = "bootimage runner"
```

You can just run the following command to launch QEMU

```bash
cargo run
```