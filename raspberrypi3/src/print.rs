use spin::Mutex;
use lazy_static::lazy_static;
use core::fmt;
use crate::uart;

pub struct QemuSerialPrint;

lazy_static! {
    pub static ref SERIAL1: Mutex<QemuSerialPrint> = {
        unsafe {
        uart::uart_init();
        }
        Mutex::new(QemuSerialPrint{})
    };
}

impl fmt::Write for QemuSerialPrint {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            unsafe {
                //core::ptr::write_volatile(0x3F20_1000 as *mut u8, byte);
                uart::uart_put(byte);
            }
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::print::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}