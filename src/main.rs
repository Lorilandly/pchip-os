#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

extern crate alloc;

mod allocator;
mod assembly;
pub mod shell;
mod trap;
pub mod uart;
pub mod xmodem;

use alloc::boxed::Box;
use core::panic::PanicInfo;
use riscv::asm::wfi;
use shell::Shell;

/// Entry point
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    println!("Hello world!");

    let x = Box::new(41);
    println!("Boxed variable: {}", x);

    let mut shell = Shell::new();
    shell.shell();

    loop {
        unsafe { wfi() };
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        unsafe { wfi() };
    }
}
