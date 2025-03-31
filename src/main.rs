#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

extern crate alloc;

mod allocator;
mod assembly;
pub mod process;
pub mod shell;
pub mod syscall;
mod trap;
pub mod uart;
pub mod xmodem;

use core::arch::asm;
use core::panic::PanicInfo;
use riscv::asm::wfi;
use riscv::register::*;
use shell::user_mod;

/// Entry point
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    println!("Hello world!");

    unsafe {
        // configure PMP CSR
        pmpcfg0::set_pmp(0, Range::NAPOT, Permission::RWX, false);
        pmpaddr0::write(0x80FF_FFFF >> 2);
        pmpcfg0::set_pmp(1, Range::NAPOT, Permission::RWX, false);
        pmpaddr1::write(0x1000_00FF >> 2);
        // set target jump addr
        mepc::write(user_mod as usize);
        // set target jump mode
        mstatus::set_mpp(mstatus::MPP::User);
        // set stack
        asm!("la sp, _u_stack_base");
        asm!("mret");
    }

    // Put CPU into idle.
    loop {
        wfi();
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        wfi();
    }
}
