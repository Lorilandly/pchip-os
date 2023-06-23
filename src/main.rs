#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

extern crate alloc;

mod allocator;
mod assembly;
mod trap;
pub mod uart;
pub mod xmodem;

use alloc::boxed::Box;
use core::panic::PanicInfo;
use crc::{Crc, CRC_32_ISO_HDLC};
use riscv::asm::ebreak;
use riscv::asm::wfi;
use uart::SERIAL;

/// Entry point
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    println!("This is my operating system!");

    let crc: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    loop {
        let mut modem = xmodem::Xmodem::new();
        // Because a mutex is passed in, Dead lock situation is possible if this function crashes or
        // an interrupt is raised during this function
        let file = modem.recv(&mut *SERIAL.lock()).unwrap();
        for b in &file {
            print!("{}", *b as char);
        }
        println!(
            "\nLength: {}, Cksum: {}, Completed!",
            file.len(),
            crc.checksum(&file),
        );
    }

    let x = Box::new(41);
    println!("Boxed variable: {}", x);

    unsafe { ebreak() };
    println!("Still running after the breakpoint!");

    loop {
        let i = SERIAL.lock().get();
        match i {
            Some(c) => match c {
                8 => {
                    // This is a backspace, so we essentially have
                    // to write a space and backup again:
                    print!("{}{}{}", 8 as char, ' ', 8 as char);
                }
                10 | 13 => {
                    // Newline or carriage-return
                    println!();
                }
                _ => {
                    print!("{}", c as char);
                }
            },
            None => (),
        }
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
