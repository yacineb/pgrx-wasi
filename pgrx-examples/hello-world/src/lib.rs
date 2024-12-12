#![no_std]

// obj file (before link editing)

// HOST env functions
extern "C" {
    // extern void PG_version(void);
    fn PG_version();

    // next step
    // allocator
    // fs
}

#[no_mangle]
/// needed by pglit
pub extern "C" fn PG_init() {
    unsafe { PG_version(); }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() {
    // check whether host function is invokable
    PG_init();
}