//! Example write and exit syscall functions for various architectures

use core::arch::asm;

#[cfg(target_arch = "x86_64")]
pub unsafe fn write(fd: usize, msg: *const u8, len: usize) -> Result<usize, ()> {
    let sys_nr: usize = 1;
    let ret: isize;
    asm!(
    "syscall",
    in("rax") sys_nr,
    in("rdi") fd,
    in("rsi") msg,
    in("rdx") len,
    lateout("rax") ret,
    );
    match ret {
        -1 => Err(()),
        _ => Ok(ret as usize),
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn exit(ret: usize) -> ! {
    let sys_nr: usize = 60;
    asm!(
    "syscall",
    in("rax") sys_nr,
    in("rdi") ret,
    options(noreturn),
    );
}

#[cfg(target_arch = "x86")]
pub unsafe fn write(fd: usize, msg: *const u8, len: usize) -> Result<usize, ()> {
    let sys_nr: usize = 1;
    let ret: isize;
    asm!(
    "syscall",
    in("eax") sys_nr,
    in("ebx") fd,
    in("ecx") msg,
    in("edx") len,
    lateout("eax") ret,
    );
    match ret {
        -1 => Err(()),
        _ => Ok(ret as usize),
    }
}

#[cfg(target_arch = "x86")]
pub unsafe fn exit(ret: usize) -> ! {
    let sys_nr: usize = 60;
    asm!(
    "syscall",
    in("eax") sys_nr,
    in("ebx") ret,
    options(noreturn),
    );
}

#[cfg(target_arch = "arm")]
pub unsafe fn write(fd: usize, msg: *const u8, len: usize) -> Result<usize, ()> {
    let sys_nr: usize = 1;
    let ret: isize;
    asm!(
    "svc #0",
    in("r7") sys_nr,
    in("r0") fd,
    in("r1") msg,
    in("r2") len,
    lateout("r0") ret,
    );
    match ret {
        -1 => Err(()),
        _ => Ok(ret as usize),
    }
}

#[cfg(target_arch = "arm")]
pub unsafe fn exit(ret: usize) -> ! {
    let sys_nr: usize = 60;
    asm!(
    "svc #0",
    in("r7") sys_nr,
    in("r0") ret,
    options(noreturn),
    );
}

#[cfg(target_arch = "aarch64")]
pub unsafe fn write(fd: usize, msg: *const u8, len: usize) -> Result<usize, ()> {
    let sys_nr: usize = 1;
    let ret: isize;
    asm!(
    "svc #0",
    in("x8") sys_nr,
    in("x0") fd,
    in("x1") msg,
    in("x2") len,
    lateout("x0") ret,
    );
    match ret {
        -1 => Err(()),
        _ => Ok(ret as usize),
    }
}

#[cfg(target_arch = "aarch64")]
pub unsafe fn exit(ret: usize) -> ! {
    let sys_nr: usize = 60;
    asm!(
    "svc #0",
    in("x8") sys_nr,
    in("x0") ret,
    options(noreturn),
    );
}

#[cfg(any(target_arch = "mips", target_arch = "mips64"))]
pub unsafe fn write(fd: usize, msg: *const u8, len: usize) -> Result<usize, ()> {
    let sys_nr: usize = 1;
    let ret: isize;
    asm!(
    "syscall",
    in("$2") sys_nr,
    in("$4") fd,
    in("$5") msg,
    in("$6") len,
    lateout("$4") ret,
    );
    match ret {
        -1 => Err(()),
        _ => Ok(ret as usize),
    }
}

#[cfg(any(target_arch = "mips", target_arch = "mips64"))]
pub unsafe fn exit(ret: usize) -> ! {
    let sys_nr: usize = 60;
    asm!(
    "syscall",
    in("$2") sys_nr,
    in("$4") ret,
    options(noreturn),
    );
}
