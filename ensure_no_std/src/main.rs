#![no_std]
#![no_main]
#[allow(unused_imports)]

use amm_sdk;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    extern "Rust" {
        #[link_name = "\nERROR: Your program contains at least one panicking branch"]
        fn undefined() -> !;
    }
    unsafe { undefined() }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
