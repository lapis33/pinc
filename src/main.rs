#![no_std]
#![no_main]

extern crate alloc;

mod allocator;

mod drivers {
    pub mod uart;
}

mod utils {
    pub mod lazy;
    pub mod mutex;
}

use core::{arch::global_asm, panic::PanicInfo};

global_asm!(include_str!("entry.s"));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    uart_println!("{}", info);
    loop {}
}

#[no_mangle]
extern "C" fn kmain() {
    uart_println!("Hello, world!");
}
