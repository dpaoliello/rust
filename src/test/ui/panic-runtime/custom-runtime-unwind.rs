// build-pass
// compile-flags:-C panic=unwind

#![feature(lang_items)]
#![feature(panic_runtime)]
#![feature(start)]
#![feature(rustc_private)]
#![panic_runtime]
#![no_std]

extern crate libc;

#[no_mangle]
#[lang = "eh_personality"]
pub extern "C" fn rust_eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    panic!()
}