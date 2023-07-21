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

use core::panic::PanicInfo;
use riscv::asm::wfi;
use shell::Shell;

/// Entry point
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    println!("Hello world!");

    let mut shell = Shell::new();
    shell.shell();

    // Put CPU into idle.
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
