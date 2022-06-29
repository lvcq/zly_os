use super::WRITER;
use core::fmt::{self, Write};

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules!  print{
  ($($arg:tt)*) => ($crate::vga_buffer::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ()=>($crate::print!("\n"));
    ($($arg:tt)*)=>($crate::print!("{}\n", format_args!($($arg)*)))
}
