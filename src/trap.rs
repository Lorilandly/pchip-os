use crate::println;
use riscv::register::mcause::Exception;
use riscv::register::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct reg_frame {
    a: [usize; 8],
    t: [usize; 7],
    ra: usize,
    sp: usize,
    gp: usize,
    tp: usize,
    s: [usize; 12],
}

/// Handles CPU trap
///
/// When cpu encounters a exception or interrupt, it jumps to the address
/// stored in the `mtvec` register, which is a function called trap_entry
/// written in assembly. That function will store all the volatile registers
/// and call this function.
///
/// The arguments contains the values of the registers at the time the trap is triggered
///
/// Returns the address where the program should return to.
///
/// # Panics
/// Panics for all exceptions except for breakpoint.
#[no_mangle]
pub extern "C" fn trap_handle(frame: reg_frame) -> usize {
    println!("{:x?}", frame);
    match mcause::read().cause() {
        mcause::Trap::Exception(cause) => match cause {
            Exception::Breakpoint => println!("Breakpoint!\n{:#x?}", ExceptionFrame::new()),
            Exception::MachineEnvCall => println!("Syscall!\n{:#x?}", ExceptionFrame::new()),
            _ => panic!("Exception: {:?}\n{:#x?}", cause, ExceptionFrame::new()),
        },
        mcause::Trap::Interrupt(cause) => println!("Interrput: {:?}", cause),
    }

    mepc::read() + 4
}

#[derive(Debug)]
#[allow(dead_code)] // allow dead code as the struct is for the ease of printing
struct ExceptionFrame {
    mhartid: usize,
    mepc: usize,
    mtval: usize,
    mie: bool,
    mip: bool,
}

impl ExceptionFrame {
    fn new() -> Self {
        let mhartid = mhartid::read();
        let mepc = mepc::read();
        let mtval = mtval::read();
        let mstatus = mstatus::read();
        let mie = mstatus.mie();
        let mip = mstatus.mpie();
        Self {
            mhartid,
            mepc,
            mtval,
            mie,
            mip,
        }
    }
}
