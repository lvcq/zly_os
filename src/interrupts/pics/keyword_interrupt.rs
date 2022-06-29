use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};
use x86_64::structures::idt::InterruptStackFrame;

use crate::print;

use super::{InterruptIndex, PICS};

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            layouts::Us104Key,
            ScancodeSet1,
            pc_keyboard::HandleControl::Ignore
        ));
}

pub extern "x86-interrupt" fn keyword_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let mut port: PortGeneric<u8, ReadWriteAccess> = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8())
    }
}
