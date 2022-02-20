# Rust Shellcode
A template project for writing shellcode in rust.

The goal of this project is to make it easy to write shellcode for the major architectures in rust without a lot of custom configuration.
While this template is not as generic as I wold have liked it to be, it makes it quite easy to get started.

One of the drawbacks of this template is that the shellcode will not be the smallest. x86_64 and aarch64 optimize to be quite small, but
the others can get a little bloated.

## Getting Started
### Binutils
To use this template, a copy of binutils for the target architecture must be installed. Compiling binutils from scratch is quite easy
and the recommended method.

Go to `https://mirrors.ocf.berkeley.edu/gnu/binutils/` and download the newest version of binutils (NOTE: This template was made with 2.38).
Extract the tar archive.

Run:
```bash
./configure --prefix=[your install prefix here] \
            --target=[target triple here] \
            --disable-static \
            --disable-multilib \
            --disable--werror \
            --disable-nls
```

You should use one of the following target triples:
 - x86_64-unknown-linux-gnu
 - i386-unknown-linux-gnu
 - aarch64-unknown-linux-gnu
 - arm-unknown-linux-gnu
 - mipsel-unknown-linux-gnu
 - mips64el-unknown-linux-gnu

### Configuration
The `Makefile` and `.cargo/config.toml` files will need to be configured to build for the proper architecture. You can grep for 'FIXME' to
see what to change. The basics are the `STRIP` and `OBJCOPY` binary that need to point to the binutils version that was installed for the
target architecture, the `TARGET` variable needs to be set to the binary that cargo will produce, and `target` in `.cargo/config.toml`
needs to be set to the target triple.

### Building
Just run `make`. `shellcode.bin` will be built if there are no errors. I recommend running `objdump` from binutils on the binary just to
sanity check the output. I have done limited testing of this template so ymmv.
