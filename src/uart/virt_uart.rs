use super::Read;
use core::fmt;
use ns16550a::*;
use spin::{Lazy, Mutex};

pub static SERIAL: Lazy<Mutex<Uart>> = Lazy::new(|| {
    let uart = Uart::new(0x1000_0000);
    uart.init(
        WordLength::EIGHT,
        StopBits::ONE,
        ParityBit::DISABLE,
        ParitySelect::EVEN,
        StickParity::DISABLE,
        Break::DISABLE,
        DMAMode::MODE0,
        Divisor::BAUD115200,
    );
    Mutex::new(uart)
});

impl Read for Uart {
    fn get(&self) -> Option<u8> {
        self.get()
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL.lock().write_fmt(args).unwrap();
}
