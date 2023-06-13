use riscv::register::*;
use riscv::register::mcause::Exception;
use crate::println;


#[no_mangle]
pub extern "C" fn trap_handle() -> usize {
    println!("trapped");
    match mcause::read().cause() {
        mcause::Trap::Exception(cause) => match cause {
            Exception::Breakpoint => println!("Breakpoint!\n"),
            _ => panic!("Exception: {:?}\n{:#?}", cause, ExceptionFrame::new()),
        }
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
        Self { mhartid, mepc, mtval, mie, mip }
    }
}