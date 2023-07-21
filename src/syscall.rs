use crate::{
    println,
    process::{Process, RegFrame, KERNEL_PROCESS},
};

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
pub fn syscall0(code: usize) {
    unsafe { syscall(code, 0, 0, 0, 0, 0, 0) };
}

/// Syscall to run a program
pub fn syscall1(frame: RegFrame, pc: *const usize) -> Result<(), usize> {
    let frame = &frame as *const RegFrame as usize;
    let pc = pc as usize;
    let code;
    unsafe {
        code = syscall(frame, pc, 0, 0, 0, 0, 1);
    }
    match code {
        0 => Ok(()),
        n => Err(n),
    }
}

pub fn syscall_handle(frame: &mut RegFrame, pc: &mut usize) {
    match frame.a[7] {
        // arg 0: exit status
        0 => unsafe {
            if let Some(kprocess) = &KERNEL_PROCESS {
                // load kernel frame with return value at a0
                let code = frame.a[0];
                *pc = kprocess.pc + 4;
                *frame = kprocess.frame;
                frame.a[0] = code;
                KERNEL_PROCESS = None;
            }
        },
        // arg 0: reg_frame
        // arg 1: pc
        1 => unsafe {
            if let None = KERNEL_PROCESS {
                // save kernel frame
                KERNEL_PROCESS = Some(Process {
                    frame: *frame,
                    pc: *pc,
                });
                // switch to new reg frame
                *pc = frame.a[1];
                *frame = *(frame.a[0] as *const RegFrame);
            }
        },
        _ => println!("Unknown Syscall number"),
    }
}
