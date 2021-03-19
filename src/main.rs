#![no_std]
#![no_main]
use core::panic::PanicInfo;

const GLOBAL_STRING : &'static str = "ABC";

#[no_mangle]
pub fn nothing() -> &'static str{
    GLOBAL_STRING
}  

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

// #[lang = "eh_personality"]
// extern "C" fn eh_personality() {}