use core::panic::PanicInfo;
#![no_std]
fn main() {
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
