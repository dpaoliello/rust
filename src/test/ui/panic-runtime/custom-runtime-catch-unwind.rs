// build-pass
// compile-flags:-C panic=unwind

#![feature(lang_items)]
#![feature(panic_runtime)]
#![feature(start)]
#![feature(rustc_private)]
#![feature(rustc_attrs)]
#![feature(catch_unwind_in_libcore)]
#![panic_runtime]
#![no_std]

extern crate libc;

#[no_mangle]
#[lang = "eh_personality"]
pub extern "C" fn rust_eh_personality() {}

#[rustc_std_internal_symbol]
pub extern "C" fn __rust_panic_cleanup_and_drop(_payload: *mut u8) {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    core::panic::catch_unwind(|| {
        panic!();
    });
    0
}