#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[no_mangle]
static mut GLOBAL_COUNTER:u8 = 1;
//BL address: 0x000004A8
//symbol address 0x00007000

#[no_mangle]
pub unsafe extern "C"  fn nothing(){
    GLOBAL_COUNTER += 1;
}  

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}