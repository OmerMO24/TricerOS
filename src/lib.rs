#![no_std]
#![feature(abi_x86_interrupt)]

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod mem;
pub mod vgabuf;

extern crate alloc;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
