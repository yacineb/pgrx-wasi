#![no_std]

#[no_mangle]
pub extern "C" fn PG_init() {
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}