use core::fmt;

pub struct Uart {
    base_address: usize,
}

impl Uart {
    pub fn new(base_address: usize) -> Self {
        Self { base_address }
    }
    
    pub fn init(&mut self) {
		let ptr = self.base_address as *mut u8;
		unsafe {
            let lcr: u8 = (1 << 0) | (1 << 1);
			ptr.add(3).write_volatile(lcr);
            ptr.add(2).write_volatile(1 << 0);
            ptr.add(1).write_volatile(1 << 0);
            let divisor: u16 = 592;
			let divisor_least: u8 =
				(divisor & 0xff).try_into().unwrap();
			let divisor_most: u8 =
				(divisor >> 8).try_into().unwrap();
            ptr.add(3).write_volatile(lcr | 1 << 7);
            ptr.add(0).write_volatile(divisor_least);
			ptr.add(1).write_volatile(divisor_most);
            ptr.add(3).write_volatile(lcr);
		}
    }

    pub fn put(&mut self, c: u8) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            ptr.add(0).write_volatile(c);
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
    Uart::new(0x1000_0000).write_fmt(args).unwrap();
}
