use x86_64::structures::idt::InterruptStackFrame;

pub(crate) extern "x86-interrupt" fn double_fault_handler(
  stack_frame: InterruptStackFrame, _error_code:u64
)->!{
  panic!("EXCEPTION: DOUBLIE FAULT\n{:#?}",stack_frame);
}