// build-pass
// compile-flags:-C panic=abort

#![feature(panic_runtime)]
#![feature(start)]
#![feature(rustc_private)]
#![panic_runtime]
#![no_std]

extern crate libc;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    panic!();
}