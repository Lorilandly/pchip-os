use ns16550a::*;
use core::fmt;
use spin::Mutex;
use lazy_static::lazy_static;

/*
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
*/
lazy_static! {
    pub static ref SERIAL: Mutex<Uart> = {
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
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let _ = ($crate::virt_uart::_print(format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\r\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", format_args!($($arg)*));
    }};
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL.lock().write_fmt(args).unwrap();
}
