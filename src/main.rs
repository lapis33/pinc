#![no_std]
#![no_main]

extern crate alloc;

mod allocator;
mod executor;

mod drivers {
    pub mod uart;
}

mod tasks {
    pub mod init;
}

mod utils {
    pub mod const_assert;
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
    executor::push(tasks::init::main());
    executor::run();
}
