//! Template for writing shellcode in rust.
//!
//! The entrypoint of the shellcode is the `_start` function. It will be what is placed at the
//! very beginning of the binary produced by the Makefile. Modify that function to create the
//! shellcode that you want. Make sure to do an objdump of the binary to check and make sure
//! that that is the case.

#![no_std]
#![no_main]
#![feature(start)]

// This is only needed for the experimental asm architectures
// See https://github.com/rust-lang/rust/issues/93335
// Currently oly needed for mips of the suported architectures
#![feature(asm_experimental_arch)]

mod syscalls;
use crate::syscalls::*;

/// Entry point of the shellcode
///
/// This is the function that you want to modify.
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
    loop {}
}

/// Panic handler
///
/// The Cargo.toml file sets the panic behavior to abort so this function shouldn't actually
/// be used. Just leave it so things compile.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
