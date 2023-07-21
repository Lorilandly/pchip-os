// Global variable stores process frame for kernel
// - if child process encounter any exception, abort and return to kernel
//
// TODO: Create a global variable to store the child process. This will
//       allow the child process to be paused.

pub static mut KERNEL_PROCESS: Option<Process> = None;

pub struct Process {
    pub frame: RegFrame,
    pub pc: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RegFrame {
    pub a: [usize; 8],
    pub t: [usize; 7],
    pub s: [usize; 12],
    pub ra: usize,
    pub gp: usize,
    pub tp: usize,
    pub sp: usize,
}

impl RegFrame {
    pub const fn new(sp: usize) -> Self {
        Self {
            a: [0; 8],
            t: [0; 7],
            s: [0; 12],
            ra: 0,
            gp: 0,
            tp: 0,
            sp,
        }
    }
}
