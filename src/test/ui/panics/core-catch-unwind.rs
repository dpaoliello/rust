// build-pass
// compile-flags:-C panic=unwind

#![feature(rustc_private)]
#![feature(rustc_attrs)]
#![feature(alloc_error_handler)]
#![feature(panic_unwind)]
#![feature(start)]
#![feature(catch_unwind_in_libcore)]
#![no_std]

extern crate libc;
extern crate panic_unwind;

// Support for allocations.
struct SomeGlobalAlloc;
unsafe impl core::alloc::GlobalAlloc for SomeGlobalAlloc {
    unsafe fn alloc(&self, _: core::alloc::Layout) -> *mut u8 { panic!() }
    unsafe fn dealloc(&self, _: *mut u8, _: core::alloc::Layout) { panic!() }
}
#[global_allocator]
static GLOBAL: SomeGlobalAlloc = SomeGlobalAlloc;
#[alloc_error_handler]
pub fn rust_oom(_: core::alloc::Layout) -> ! {
    loop {}
}

// Support for panicking.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
#[rustc_std_internal_symbol]
extern "C" fn __rust_drop_panic() -> ! {
    loop {}
}
#[rustc_std_internal_symbol]
extern "C" fn __rust_foreign_exception() -> ! {
    loop {}
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    core::panic::catch_unwind(|| {
        panic!();
    });
    0
}