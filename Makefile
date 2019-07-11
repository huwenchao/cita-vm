evm/testdata:
	cd /tmp/ && git clone https://github.com/ethereum/tests jsondata && cd jsondata && git checkout 74cc22b8f

ci:
	cargo fmt --all -- --check
	cargo clippy --all --tests --all-targets -- -D warnings
	cargo test


TARGET := riscv64-unknown-elf
CC := $(TARGET)-gcc
LD := $(TARGET)-gcc
CFLAGS := -Os -DCKB_NO_MMU -D__riscv_soft_float -D__riscv_float_abi_soft
APP_CFLAGS := $(CFLAGS) -Isrc/riscv/duktape -Ic -Wall -Werror
LDFLAGS := -lm -Wl,-static -fdata-sections -ffunction-sections -Wl,--gc-sections -Wl,-s
CURRENT_DIR := $(shell pwd)

riscv/example/c_sdk:
	$(CC) -I./src/riscv/c/ -o ./build/riscv_c_sdk ./examples/riscv_c_sdk.c

riscv/example/c_sdk/docker:
	docker run -v $(CURRENT_DIR):/src nervos/ckb-riscv-gnu-toolchain:bionic bash -c "cd /src && make riscv/example/c_sdk"

riscv/example/c_fibonacci:
	$(CC) -I./src/riscv/c/ -o ./build/riscv_c_fibonacci ./examples/riscv_c_fibonacci.c

riscv/example/c_fibonacci/docker:
	docker run -v $(CURRENT_DIR):/src nervos/ckb-riscv-gnu-toolchain:bionic bash -c "cd /src && make riscv/example/c_fibonacci"

riscv/duktape:
	$(CC) $(APP_CFLAGS) src/riscv/c/entry.c -c -o build/entry.o
	$(CC) $(APP_CFLAGS) src/riscv/duktape/duktape.c -c -o build/duktape.o
	$(LD) build/entry.o build/duktape.o -o build/duktape $(LDFLAGS)

riscv/duktape/docker:
	docker run -v $(CURRENT_DIR):/src nervos/ckb-riscv-gnu-toolchain:bionic bash -c "cd /src && make riscv/duktape"

riscv/all: riscv/example/c_sdk/docker riscv/example/c_fibonacci/docker riscv/duktape/docker

.PHONY: \
	evm/testdata \
	riscv/example/c_sdk \
	riscv/example/c_sdk/docker \
	riscv/duktape \
	riscv/duktape/docker \
	riscv/all \
	ci
