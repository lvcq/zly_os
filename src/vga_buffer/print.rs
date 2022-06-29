use core::fmt;

use super::WRITER;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
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
