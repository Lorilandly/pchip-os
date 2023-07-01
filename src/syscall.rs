use crate::println;
use crate::process::{reg_frame, KERNEL_PROCESS};

extern "C" {
    /// System call
    ///
    /// Can return a value by editing the register frame
    fn syscall(
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
        syscall_no: usize,
    ) -> usize;
}

/// Syscall to exit from a program
pub fn syscall0() {
    unsafe { syscall(0, 0, 0, 0, 0, 0, 0) };
}

/// Syscall to run a program
// TODO: change this function to accept `Process`
pub fn syscall1(frame: reg_frame, pc: *const usize) {
    let frame = &frame as *const reg_frame as usize;
    let pc = pc as usize;
    unsafe {
        syscall(frame, pc, 0, 0, 0, 0, 1);
    }
}

pub fn syscall_handle(frame: &mut reg_frame, pc: &mut usize) {
    println!("Handling Syscall!!");
    match frame.a[7] {
        // no arg
        0 => unsafe {
            println!("Syscall 0");
            //GUEST_PROCESS.frame = *frame;
            //GUEST_PROCESS.pc = *pc;
            *frame = KERNEL_PROCESS.frame;
            *pc = KERNEL_PROCESS.pc;
        },
        // arg 0: reg_frame
        // arg 1: pc
        1 => unsafe {
            println!("Syscall 1");
            KERNEL_PROCESS.frame = *frame;
            KERNEL_PROCESS.pc = *pc;
            // new reg frame with correct sp
            *pc = frame.a[1];
            *frame = *(frame.a[0] as *const reg_frame);
        },
        _ => println!("Unknown Syscall number"),
    }
}
