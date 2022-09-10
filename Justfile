_default:
    @just --list

# Build the kernel file
build:
    cargo build \
        -Z build-std=core \
        -Z build-std-features=compiler-builtins-mem \
        --target targets/raspi3.json \
        --release
    llvm-objcopy -O binary \
        target/raspi3/release/rust-os.elf \
        target/raspi3/release/kernel8.img

# Run the kernel in QEMU
run: build
    qemu-system-aarch64 \
        -machine raspi3b \
        -serial stdio \
        -display none \
        -kernel target/raspi3/release/kernel8.img

# Clean workspace
clean:
    cargo clean
