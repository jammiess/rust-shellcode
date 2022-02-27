# FIXME: point this to bin directory of binutils
CROSS_BINUTILS=/home/jaj/bin/aarch64-unknown-linux-gnu/bin

STRIP=$(CROSS_BINUTILS)/strip
OBJCOPY=$(CROSS_BINUTILS)/objcopy

# FIXME: set this to the correct target triple
TARGET=aarch64-unknown-none

ELF_FILE=target/$(TARGET)/release/shellcode

STRIP_OPTS=-s --strip-unneeded -x -X

.PHONY: clean

all: shellcode.bin

shellcode.bin: $(ELF_FILE)
	$(STRIP) $(STRIP_OPTS) $(ELF_FILE)
	$(OBJCOPY) -O binary $(ELF_FILE) shellcode.bin

$(ELF_FILE): src/*
	cargo build --release --target $(TARGET)

clean:
	cargo clean
	rm -f shellcode.bin
