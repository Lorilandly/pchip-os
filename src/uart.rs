mod virt_uart;
pub use virt_uart::{_print, SERIAL};

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
        $crate::print!("{}\n", format_args!($($arg)*));
    }};
}
