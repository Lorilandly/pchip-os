use crate::{
    print, println,
    process::RegFrame,
    syscall::syscall1,
    uart::{self, SERIAL},
    xmodem::Xmodem,
};
use alloc::string::String;
use alloc::vec::Vec;
use core::{fmt, ptr};
use crc::{Crc, CRC_32_CKSUM};
use riscv::asm::ebreak;

/// A shell that ask for the next operation to perform
///
/// # Requirements
/// - Inputted characters are outputted immediately
///
/// # Routines
/// - Show register usage (trigger breakpoint)
/// - Receive file
/// - Show file plain text (if exitst)
/// - Show file metadata (if exitst)
/// - Execute file (if exitst)
pub struct Shell {
    file: Option<Vec<u8>>,
}

pub fn user_mod() {
    let mut shell = Shell::new();
    shell.shell();
}

impl Shell {
    pub fn new() -> Self {
        Self { file: None }
    }

    pub fn shell(&mut self) {
        self.help();
        loop {
            let input = readln(&mut *SERIAL.lock());
            match input.as_str() {
                "h" => self.help(),
                "0" => unsafe { ebreak() },
                "1" => {
                    println!("You can now start sending on Xmodem");
                    let modem = Xmodem::new();
                    // Because a mutex is passed in, Dead lock situation is possible if this function crashes or
                    // an interrupt is raised during this function
                    let file = modem.recv(&mut *SERIAL.lock());
                    let file = match file {
                        Ok(f) => f,
                        Err(e) => {
                            println!("\nAborted: {:?}", e);
                            continue;
                        }
                    };
                    let cksum = cksum(&file);
                    println!("File cksum: {}. Is this correct? [Y/n]", cksum);
                    let input = readln(&mut *SERIAL.lock());
                    match input.as_str() {
                        "" | "y" | "yes" | "Y" | "Yes" => {
                            self.file = Some(file);
                            println!("Ok");
                        }
                        _ => println!("Aborted"),
                    }
                }
                "2" => match &self.file {
                    Some(file) => {
                        let frame = RegFrame::new(0x9000_0000);
                        unsafe {
                            ptr::copy_nonoverlapping(
                                file.as_ptr(),
                                0x9000_0000 as *mut u8,
                                file.len(),
                            );
                        }
                        // return return code
                        // print return code or explain
                        if let Err(e) = syscall1(frame, 0x9000_0000 as *const usize) {
                            println!("Err code: {}", e);
                        }
                    }
                    None => println!("Unrecognized Command"),
                },
                "3" => match &self.file {
                    Some(file) => {
                        for b in file {
                            print!("{}", *b as char);
                        }
                        println!();
                    }
                    None => println!("Unrecognized Command"),
                },
                "4" => match &self.file {
                    Some(file) => {
                        for (i, b) in file.iter().enumerate() {
                            if i & 0xf == 0 {
                                print!("{:08x}:", i);
                            }
                            if i & 1 == 0 {
                                print!(" ");
                            }
                            print!("{:02x}", *b);
                            if i & 0xf == 0xf {
                                println!();
                            }
                        }
                        println!();
                    }
                    None => println!("Unrecognized Command"),
                },
                "5" => {
                    if let Some(file) = &self.file {
                        unsafe {
                            ptr::copy_nonoverlapping(
                                file.as_ptr(),
                                0x9000_0000 as *mut u8,
                                file.len(),
                            );
                        }
                    }
                    println!("Type address in hex");
                    let a: usize = usize::from_str_radix(&readln(&mut *SERIAL.lock()), 16).unwrap();
                    let a = a as *const i32;
                    unsafe { println!("{:08x}", *a) };
                }
                _ => println!("Unrecognized Command!"),
            }
        }
    }

    fn help(&self) {
        println!(
            "File status: {}\nPlease select an Operation:\n\th. Show this help message\n\t0. Trigger Breakpoint\n\t1. Recieve a file",
            self.file.is_some(), // include a checksum and a size
        );
        if let Some(_) = self.file {
            print!("\t2. Execute\n\t3. Show file in plain text\n\t4. Show file in hex\n\t5. Print value at memory address\n");
        }
    }
}

fn cksum(file: &Vec<u8>) -> u32 {
    let crc = Crc::<u32>::new(&CRC_32_CKSUM);
    let mut digest = crc.digest();
    digest.update(file);
    let cksum_len = file.len().to_le_bytes();
    let pos = match cksum_len.iter().rposition(|&x| x != 0) {
        Some(n) => n + 1,
        None => 1,
    };
    digest.update(&cksum_len[..pos]);
    digest.finalize()
}

/// Read user input from SERIAL
///
/// must not panic
fn readln<D: uart::Read + fmt::Write>(dev: &mut D) -> String {
    dev.write_str("> ").unwrap();
    let mut buf: String = Default::default();
    loop {
        match dev.get() {
            None => (),
            Some(b'\r' | b'\n') => {
                dev.write_char('\n').unwrap();
                break;
            }
            Some(8 | 127) => {
                // This is a backspace, so we essentially have
                // to write a space and backup again:
                if let Some(_) = buf.pop() {
                    write!(dev, "{}{}{}", 8 as char, ' ', 8 as char).unwrap();
                }
            }
            Some(c) => {
                dev.write_char(c as char).unwrap();
                buf.push(c as char);
            }
        }
    }
    buf
}
