//! Template for writing shellcode in rust.
//!
//! The entrypoint of the shellcode is the `_start` function. It will be what is placed at the
//! very beginning of the binary produced by the Makefile. Modify that function to create the
//! shellcode that you want. Make sure to do an objdump of the binary to check and make sure
//! that it is actually placed at the beginning of the shellcode.

#![no_std]
#![no_main]

// This is only needed for the experimental asm architectures
// See https://github.com/rust-lang/rust/issues/93335
// Currently oly needed for mips of the suported architectures
// #![feature(asm_experimental_arch)]

mod syscalls;
use crate::syscalls::*;

/// Entry point of the shellcode
///
/// This is the function that you want to modify. Other functions and modules can be added.
/// Just make sure to check that everything is laid out properly in the binary.
///
/// The size of the generated assembly will be affected by the return value of the function.
/// If it will never return, change the return value to `!` and no function prologue or
/// epilogue will be generated.
/// 
/// The signature can be changed to whatever you want. It will take and return parameters in
/// the standard sysv c abi.
#[no_mangle]
pub extern "C" fn _start() {
    let message: &str = "Hello, Shellcode!";
    unsafe {
        let _ = write(1, message.as_bytes().as_ptr(), message.len());
        exit(0);
    }
}

/// Panic handler
///
/// The Cargo.toml file sets the panic behavior to abort so I don't think this functioin will
/// be used. Just leave it so things compile.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
