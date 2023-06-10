#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod asm;
pub mod virt_uart;

use core::panic::PanicInfo;
use riscv::asm::wfi;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    let mut uart = virt_uart::Uart::new(0x1000_0000);
    uart.init();
    uart.put(b'h');
    uart.put(b'e');
    uart.put(b'l');
    uart.put(b'l');
    //println!("This is my operating system!");
	//println!("I'm so awesome. If you start typing something, I'll show you what you typed!");

    
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
