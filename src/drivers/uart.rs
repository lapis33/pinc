use crate::utils::mutex::Mutex;
use core::fmt::{Arguments, Result, Write};

#[macro_export]
macro_rules! uart_print {
    ($($arg:tt)*) => {{
        $crate::drivers::uart::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! uart_println {
    () => {
        $crate::uart_print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::uart_print!("{}\n", format_args!($($arg)*))
    }};
}

static UART: Mutex<Uart> = Mutex::new(Uart);

struct Uart;

impl Uart {
    const UART0_BASE: usize = 0x01C28000;
    const UART_THR_OFFSET: usize = 0x00;
    const UART_LSR_OFFSET: usize = 0x14;

    fn read_lsr_thre() -> bool {
        unsafe {
            ((Uart::UART0_BASE + Uart::UART_LSR_OFFSET) as *mut u32).read_volatile() & (1 << 5) != 0
        }
    }

    fn write_thr(byte: u8) {
        unsafe {
            ((Uart::UART0_BASE + Uart::UART_THR_OFFSET) as *mut u8).write_volatile(byte);
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.replace("\n", "\n\r").bytes() {
            loop {
                if Uart::read_lsr_thre() {
                    Uart::write_thr(byte);
                    break;
                }
            }
        }

        Ok(())
    }
}

pub fn _print(args: Arguments) {
    UART.lock()
        .write_fmt(args)
        .unwrap_or_else(|_| unreachable!());
}
