use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{
    InterruptDescriptorTable as IDescriptorTable, InterruptStackFrame as IStackFrame,
};

lazy_static! {
    static ref IDT: IDescriptorTable = {
        let mut idt = IDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: IStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
