use core::fmt;

pub struct Uart {
    base_address: *mut u8,
}

#[allow(dead_code)]
impl Uart {
    pub fn new(address: usize) -> Self {
        let base_address = address as *mut u8;
        Self { base_address }
    }

    pub fn putc(&self, c: u8) {
        unsafe {
            while self.stat_reg().read_volatile() & (1 << 27) != 0 {}
            self.tx().write_volatile(c);
        }
    }

    pub fn puts(&self, s: &str) {
        for b in s.bytes() {
            self.putc(b);
        }
    }

    pub fn put_hex(&self, hex: usize) {
        for idx in (0..16).rev() {
            let nibble: u8 = (hex >> (idx * 4)) as u8 & 0xf;
            self.putc(if nibble < 0xa {
                b'0' + nibble
            } else {
                b'a' + nibble - 0xa
            });
        }
    }

    unsafe fn tx(&self) -> *mut u8 {
        self.base_address.add(0x04)
    }

    unsafe fn rx(&self) -> *mut u8 {
        self.base_address
    }

    unsafe fn stat_reg(&self) -> *mut u32 {
        self.base_address.add(0x08).cast::<u32>()
    }

    unsafe fn ctrl_reg(&self) -> *mut u32 {
        self.base_address.add(0x0C).cast::<u32>()
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.puts(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let _ = ($crate::axi_uart_lite::_print(format_args!($($arg)*)));
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
    Uart::new(0x4060_0000).write_fmt(args).unwrap();
}
