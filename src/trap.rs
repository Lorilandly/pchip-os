use crate::{
    println,
    process::{RegFrame, KERNEL_PROCESS},
    syscall,
};
use riscv::interrupt::{
    machine::{Exception, Interrupt},
    Trap,
};
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
pub extern "C" fn trap_handle(mut frame: RegFrame) -> usize {
    println!("{:x?}", frame);
    let mut pc = mepc::read();
    match mcause::read()
        .cause()
        .try_into::<Interrupt, Exception>()
        .unwrap()
    {
        Trap::Exception(cause) => match cause {
            Exception::Breakpoint => {
                println!("Breakpoint!\n{:#x?}", ExceptionFrame::new());
                pc += 4;
            }
            // M-mode syscall
            Exception::MachineEnvCall => {
                println!("Syscall-{}!", frame.a[7]);
                syscall::syscall_handle(&mut frame, &mut pc);
            }
            // if executing frame is guest, switch to kernel and return reason
            _ => {
                let eframe = ExceptionFrame::new();
                unsafe {
                    match &mut KERNEL_PROCESS {
                        // The guest process is running
                        //   Error code is pushed to host a[0]
                        //   Error frame is pushed to host a[1]
                        Some(ref mut kprocess) => {
                            println!("You are seeing this message because an CPU exception ({:?}) is encountered and not handled.\n{:#x?}", cause, eframe);
                            kprocess.frame.a[1] = &eframe as *const ExceptionFrame as usize;
                            syscall::syscall0(cause as usize);
                        }
                        // The kernel process is running
                        None => panic!("Exception: {:?}\n{:#x?}", cause, eframe),
                    }
                }
            }
        },
        Trap::Interrupt(cause) => println!("Interrput: {:?}", cause),
    }
    pc
}

#[derive(Debug)]
#[allow(dead_code)] // allow dead code as the struct is for the ease of printing
struct ExceptionFrame {
    mhartid: usize,
    mepc: usize,
    mtval: usize,
    mie: bool,
    mip: bool,
    mpp: mstatus::MPP,
}

impl ExceptionFrame {
    fn new() -> Self {
        let mhartid = mhartid::read();
        let mepc = mepc::read();
        let mtval = mtval::read();
        let mstatus = mstatus::read();
        let mie = mstatus.mie();
        let mip = mstatus.mpie();
        let mpp = mstatus.mpp();
        Self {
            mhartid,
            mepc,
            mtval,
            mie,
            mip,
            mpp,
        }
    }
}
