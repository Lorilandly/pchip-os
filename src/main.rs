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

use alloc::boxed::Box;
use core::panic::PanicInfo;
use riscv::asm::wfi;
use shell::Shell;

use crate::process::reg_frame;

/// Entry point
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    println!("Hello world!");

    let x = Box::new(41);
    println!("Boxed variable: {}", x);

    let frame = reg_frame::new(0x9000_0000);
    syscall::syscall1(frame, sim_prog as *const usize);

    let mut shell = Shell::new();
    shell.shell();

    loop {
        unsafe { wfi() };
    }
}

fn sim_prog() {
    println!("Hello from a program!!");
    syscall::syscall0();
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        unsafe { wfi() };
    }
}
