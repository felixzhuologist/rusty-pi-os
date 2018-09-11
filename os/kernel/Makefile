TARGET ?= aarch64-none-elf
CROSS ?= $(TARGET)

CC := $(CROSS)-gcc
TTYWRITE ?= ttywrite
PI_TTY ?= /dev/tty.usbserial
CCFLAGS ?= -Wall -O2 -nostdlib -nostartfiles -ffreestanding -pie -fpie
# LDFLAGS ?= --gc-sections -static -pie -nostdlib -nostartfiles --no-dynamic-linker
LDFLAGS ?= --gc-sections -static -nostdlib -nostartfiles --no-dynamic-linker
XBUILD ?= CARGO_INCREMENTAL=0 RUST_TARGET_PATH="$(shell pwd)" cargo xbuild

LD_LAYOUT := ext/layout.ld

RUST_BINARY := $(shell cat Cargo.toml | grep name | cut -d\" -f 2 | tr - _)
RUST_BUILD_DIR := target/$(TARGET)
RUST_DEBUG_LIB := $(RUST_BUILD_DIR)/debug/lib$(RUST_BINARY).a
RUST_RELEASE_LIB := $(RUST_BUILD_DIR)/release/lib$(RUST_BINARY).a

RUST_DEPS = Cargo.toml build.rs $(LD_LAYOUT) src/* ../pi/src/*
EXT_DEPS = $(BUILD_DIR)/init.o

BUILD_DIR := build
KERNEL := $(BUILD_DIR)/$(RUST_BINARY)
RUST_LIB := $(BUILD_DIR)/$(RUST_BINARY).a

.PHONY: all clean check

VPATH = ext

all: $(KERNEL).hex $(KERNEL).bin

check:
	cargo check --target=$(TARGET)

test:
	CARGO_INCREMENTAL=0 cargo test

install: $(KERNEL).bin
	$(TTYWRITE) -i $< $(PI_TTY)

$(RUST_DEBUG_LIB): $(RUST_DEPS)
	@echo "+ Building $@ [cargo xbuild]"
	@$(XBUILD) --target=$(TARGET).json

$(RUST_RELEASE_LIB): $(RUST_DEPS)
	@echo "+ Building $@ [cargo xbuild --release]"
	@$(XBUILD) --release --target=$(TARGET).json

ifeq ($(DEBUG),1)
$(RUST_LIB): $(RUST_DEBUG_LIB) | $(BUILD_DIR)
	@cp $< $@
else
$(RUST_LIB): $(RUST_RELEASE_LIB) | $(BUILD_DIR)
	@cp $< $@
endif

$(BUILD_DIR):
	@mkdir -p $@

$(BUILD_DIR)/%.o: %.c | $(BUILD_DIR)
	@echo "+ Building $@ [cc $<]"
	@$(CC) $(CCFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: %.S | $(BUILD_DIR)
	@echo "+ Building $@ [as $<]"
	@$(CC) $(CCFLAGS) -c $< -o $@

$(KERNEL).elf: $(EXT_DEPS) $(RUST_LIB) | $(BUILD_DIR)
	@echo "+ Building $@ [ld $^]"
	@$(CROSS)-ld $(LDFLAGS) -T$(LD_LAYOUT) $^ -o $@

$(KERNEL).hex: $(KERNEL).elf | $(BUILD_DIR)
	@echo "+ Building $@ [objcopy $<]"
	@$(CROSS)-objcopy $< -O ihex $@

$(KERNEL).bin: $(KERNEL).elf | $(BUILD_DIR)
	@echo "+ Building $@ [objcopy $<]"
	@$(CROSS)-objcopy $< -O binary $@

clean:
	cargo clean
	rm -rf $(BUILD_DIR)
