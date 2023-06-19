use crate::println;
use riscv::register::mcause::Exception;
use riscv::register::*;

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
pub extern "C" fn trap_handle(
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) -> usize {
    println!(
        "a0: {:#04x}\na1: {:#04x}\na2: {:#04x}\na3: {:#04x}\na4: {:#04x}\na5: {:#04x}\na6: {:#04x}\na7: {:#04x}",
        a0, a1, a2, a3, a4, a5, a6, a7
    );
    match mcause::read().cause() {
        mcause::Trap::Exception(cause) => match cause {
            Exception::Breakpoint => println!("Breakpoint!\n{:#?}", ExceptionFrame::new()),
            Exception::MachineEnvCall => println!("Syscall!\n{:#?}", ExceptionFrame::new()),
            _ => panic!("Exception: {:?}\n{:#?}", cause, ExceptionFrame::new()),
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
