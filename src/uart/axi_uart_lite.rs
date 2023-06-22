use super::Read;
use core::fmt;
use spin::{Lazy, Mutex};

pub static SERIAL: Lazy<Mutex<Uart>> = Lazy::new(|| Mutex::new(Uart::new(0x4060_0000)));

pub struct Uart {
    base_address: usize,
}

#[allow(dead_code)]
impl Uart {
    const RX: usize = 0x00;
    const TX: usize = 0x04;
    const STAT_REG: usize = 0x08;
    const TX_FULL: u32 = 1 << 27;
    const RX_VALID: u32 = 1 << 24;
    const CTRL_REG: usize = 0x0C;

    pub fn new(base_address: usize) -> Self {
        Self { base_address }
    }

    pub fn get(&self) -> Option<u8> {
        let stat = (self.base_address + Self::STAT_REG) as *mut u32;
        let rx = (self.base_address + Self::RX) as *mut u8;
        unsafe {
            match stat.read_volatile() & Self::RX_VALID {
                //0 => None,
                _ => Some(rx.read_volatile()),
            }
        }
    }

    pub fn put(&self, c: u8) {
        let stat = (self.base_address + Self::STAT_REG) as *mut u32;
        let tx = (self.base_address + Self::TX) as *mut u8;
        unsafe {
            while stat.read_volatile() & Self::TX_FULL != 0 {}
            tx.write_volatile(c);
        }
    }

    pub fn put_hex(&self, hex: usize) {
        for idx in (0..16).rev() {
            let nibble: u8 = (hex >> (idx * 4)) as u8 & 0xf;
            self.put(if nibble < 0xa {
                b'0' + nibble
            } else {
                b'a' + nibble - 0xa
            });
        }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.put(b);
        }
        Ok(())
    }
}

impl Read for Uart {
    fn read(&self) -> Option<u8> {
        self.get()
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    Uart::new(0x4060_0000).write_fmt(args).unwrap();
}
