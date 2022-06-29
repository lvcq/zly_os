pub mod pics_interrupt_index;
pub mod timer_interrupt;

use pic8259::ChainedPics;
use spin;

pub use pics_interrupt_index::InterruptIndex;
pub use timer_interrupt::timer_interrupt_handler;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
