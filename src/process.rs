// one process frame for kernel
//
// one process frame for child process
// - if child process encounter any exception, abort and return to kernel

pub static mut KERNEL_PROCESS: Process = Process::new();

pub struct Process {
    pub frame: reg_frame,
    pub pc: usize,
}

impl Process {
    const fn new() -> Self {
        Self {
            frame: reg_frame::new(0),
            pc: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct reg_frame {
    pub a: [usize; 8],
    pub t: [usize; 7],
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub s: [usize; 12],
}

impl reg_frame {
    pub const fn new(sp: usize) -> Self {
        Self {
            a: [0; 8],
            t: [0; 7],
            ra: 0,
            sp,
            gp: 0,
            tp: 0,
            s: [0; 12],
        }
    }
}
