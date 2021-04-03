#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]

extern crate alloc;
use alloc::boxed::Box;
use linked_list_allocator::LockedHeap;
use core::panic::PanicInfo;
use alloc::alloc::AllocError;
//TODO: We Compile nightly rust which is unstable

#[no_mangle]
static mut GLOBAL_COUNTER:u8 = 1;

const HEAP_START :usize = 0x2000FF00;
const HEAP_SIZE :usize = 0x1000;

//BL address: 0x000004A8
//symbol address 0x00007000

//TODO: Remove the alloc_error_handler and use RCs
//TODO: Lock the HEAP using lock interrupts instead of spin lock
//TODO: compile all hooks (pub extern) by default (Instead os specifying extern)

#[global_allocator]
// static ALLOCATOR: LockedHeap = LockedHeap::empty();
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> !{
    loop{
        
    };
}

pub unsafe extern "C" fn init(){
    ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
}

#[no_mangle]
pub extern "C"  fn nothing(){
    let allocte_box = try_alloc_on_heap();
    
}

fn try_alloc_on_heap() -> Result<Box<u8>, AllocError> {
    Box::try_new(8)
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}