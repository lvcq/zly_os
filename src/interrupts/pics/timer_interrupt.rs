use x86_64::structures::idt::InterruptStackFrame;

use crate::print;
use super::{PICS,InterruptIndex};

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!("A");
    unsafe{
      PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
