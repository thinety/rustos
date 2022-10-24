_default:
    @just --list

TARGET := 'aarch64-unknown-none-softfloat'
TARGET_CPU := 'cortex-a53'

# Build the kernel file
build:
    RUSTFLAGS='-C target-cpu={{TARGET_CPU}}' cargo rustc \
        --release --target {{TARGET}} -Z build-std=core \
        -- -C link-arg=--script=layout.ld

    llvm-objcopy -O binary \
        target/{{TARGET}}/release/rust-os \
        target/{{TARGET}}/release/kernel8.img

# Run the kernel in QEMU
run: build
    qemu-system-aarch64 \
        -machine raspi3b \
        -serial stdio \
        -display none \
        -kernel target/{{TARGET}}/release/kernel8.img

# Clean workspace
clean:
    cargo clean
