# FIXME: point this to correct copy of strip
STRIP=strip
# FIXME: point this to correct copy of objcopy
OBJCOPY=objcopy
# FIXME: point this to the output binary of cargo
TARGET=target/x86_64-unknown-linux-gnu/release/shellcode

STRIP_OPTS=-s --strip-unneeded -x -X

.PHONY: clean

all: shellcode.bin

shellcode.bin: $(TARGET)
	$(STRIP) $(STRIP_OPTS) $(TARGET)
	$(OBJCOPY) -O binary $(TARGET) shellcode.bin

$(TARGET): src/*
	cargo build --release

clean:
	cargo clean
	rm -f shellcode.bin
