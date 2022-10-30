TARGET := aarch64-unknown-none-softfloat
TARGET_CPU := cortex-a53

target/$(TARGET)/release/kernel8.img: target/$(TARGET)/release/rustos
	@llvm-objcopy -O binary $^ $@

target/$(TARGET)/release/rustos: target/$(TARGET)/release/librustos.a target/$(TARGET)/release/init.o src/layout.ld
	@ld.lld --gc-sections $^ -o $@

target/$(TARGET)/release/librustos.a: RUST_SRC
	@RUSTFLAGS='-C target-cpu=$(TARGET_CPU)' cargo build --release --target $(TARGET) -Z build-std=core

target/$(TARGET)/release/init.o: src/init.s
	@llvm-mc --triple $(TARGET) --mcpu $(TARGET_CPU) --assemble --filetype obj $^ -o $@

clean:
	@cargo clean

run: target/$(TARGET)/release/kernel8.img
	@qemu-system-aarch64 \
		-machine raspi3b \
		-serial stdio \
		-display none \
		-kernel $^

.PHONY: RUST_SRC clean run
