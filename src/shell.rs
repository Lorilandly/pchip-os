use crate::xmodem::Xmodem;
use crate::{
    print, println,
    uart::{self, SERIAL},
};
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
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
    file: Option<File>,
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
                "1" => 'a: {
                    println!("You can now start sending on Xmodem");
                    let modem = Xmodem::new();
                    // Because a mutex is passed in, Dead lock situation is possible if this function crashes or
                    // an interrupt is raised during this function
                    let file = modem.recv(&mut *SERIAL.lock());
                    let file = match file {
                        Ok(f) => f,
                        Err(e) => {
                            println!("\nAborted: {:?}", e);
                            break 'a;
                        }
                    };
                    let cksum = cksum(&file);
                    println!("File cksum: {}. Is this correct? [Y/n]", cksum);
                    let input = readln(&mut *SERIAL.lock());
                    match input.as_str() {
                        "" | "y" | "yes" | "Y" | "Yes" => {
                            self.file = Some(File { file, cksum });
                            println!("Ok");
                        }
                        _ => println!("Aborted"),
                    }
                }
                "2" => match &self.file {
                    Some(file) => {
                        for b in &file.file {
                            print!("{}", *b as char);
                        }
                    }
                    None => println!("Unrecognized Command"),
                },
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
            print!("\t2. Show file in plain text\n\t3. file metadata\n\t4. Execute\n");
        }
    }
}

#[allow(unused)]
struct File {
    file: Vec<u8>,
    cksum: u32,
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
