// one process frame for kernel
//
// one process frame for child process
// - if child process encounter any exception, abort and return to kernel

pub static mut KERNEL_PROCESS: Option<Process> = None;

pub struct Process {
    pub frame: reg_frame,
    pub pc: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct reg_frame {
    pub a: [usize; 8],
    pub t: [usize; 7],
    pub s: [usize; 12],
    pub ra: usize,
    pub gp: usize,
    pub tp: usize,
    pub sp: usize,
}

impl reg_frame {
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
