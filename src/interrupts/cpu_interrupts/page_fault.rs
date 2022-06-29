use x86_64::structures::idt::{InterruptStackFrame,PageFaultErrorCode};

use crate::{println, hlt_loop};

pub(crate) extern "x86-interrupt" fn page_fault_handler(
    stack_frame:InterruptStackFrame,
    stack_code:PageFaultErrorCode
){
    use x86_64::registers::control::Cr2;
    println!("EXCEPTION: PAGE FAULT.");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}",stack_code);
    println!("{:#?}",stack_frame);
    hlt_loop();
}