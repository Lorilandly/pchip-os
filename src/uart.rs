mod virt_uart;
use riscv::asm::delay;
pub use virt_uart::{_print, SERIAL};

pub trait Read {
    /// A simple wrapper
    fn get(&self) -> Option<u8>;

    /// Read with timeout
    /// Time out at around 3 seconds on qemu running on M1 Pro Mac
    fn read(&self) -> Result<u8, Error> {
        for _ in 0..1000000 {
            match self.get() {
                //Some(CAN) => return Err(Error::Canceled),
                Some(c) => return Ok(c),
                None => unsafe { delay(1000) },
            }
        }
        Err(Error::Timeout)
    }
}

#[derive(Debug)]
pub enum Error {
    Timeout,
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let _ = ($crate::uart::_print(format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\r\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!("{}\r\n", format_args!($($arg)*));
    }};
}
