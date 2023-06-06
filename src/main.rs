#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

pub mod asm;

use core::panic::PanicInfo;
use riscv::asm::wfi;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    let uart0 = 0x40600004 as *mut u8;
    unsafe {
        *uart0 = b'a';
    }
    for c in b"Hello from Rust!".iter() {
        unsafe {
            *uart0 = *c as u8;
        }
    }
    
    loop{
        unsafe { wfi() };
    }
}

#[no_mangle]
pub extern "C" fn trap_handle() -> ! {
    loop{
        unsafe { wfi() };
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{
        unsafe { wfi() };
    }
}
