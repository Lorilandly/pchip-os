#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod assembly;
mod trap;
pub mod virt_uart;

use core::panic::PanicInfo;
use riscv::asm::ebreak;
use riscv::asm::wfi;

/// Entry point
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    println!("This is my operating system!");

    unsafe { ebreak() };

    println!("Still running after the breakpoint!");
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
