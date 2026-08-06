use std::arch::asm;

/// Execute linux x86_64 syscall with one arguments.
///
/// # Safety
///
/// Caller must ensure:
/// - syscall number is valid
/// - arguments follow linux syscall ABI
/// - pointers passed to kernel are valid
pub unsafe fn syscall1(nr: usize, arg1: usize) -> isize {
    unsafe {
        let ret: isize;

        asm!(
            "syscall",

            inlateout("rax") nr as isize => ret,

            in("rdi") arg1,

            lateout("rcx") _,
            lateout("r11") _,

            options(nostack),
        );

        ret
    }
}

/// Execute linux syscall with three arguments
///
/// # Safety
///
/// Caller must ensure all arguments are valid according to
/// Linux syscall ABI
pub unsafe fn syscall3(nr: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    unsafe {
        let ret: isize;

        asm!(

            "syscall",

            inlateout("rax") nr as isize => ret,

            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,

            lateout("rcx") _,
            lateout("r11") _,

            options(nostack),
        );

        ret
    }
}

/// Execute Linux syscall with four arguments
///
/// # Safety
///
/// Caller must ensure syscall number and arguments are valid
pub unsafe fn syscall4(nr: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> isize {
    unsafe {
        let ret: isize;

        asm!(
            "syscall",

            inlateout("rax") nr as isize => ret,

            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,

            lateout("rcx") _,
            lateout("r11") _,

            options(nostack),
        );

        ret
    }
}
