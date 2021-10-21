#![allow(dead_code)]

use core::cell::RefCell;
use core::panic::{self, AssertUnwindSafe, UnwindSafe};

#[derive(core::fmt::Debug, core::cmp::PartialEq)]
struct Foo {
    a: i32,
}

fn assert<T: UnwindSafe + ?Sized>() {}

#[test]
fn panic_safety_traits() {
    assert::<i32>();
    assert::<&i32>();
    assert::<*mut i32>();
    assert::<*const i32>();
    assert::<usize>();
    assert::<str>();
    assert::<&str>();
    assert::<Foo>();
    assert::<&Foo>();
    assert::<Vec<i32>>();
    assert::<String>();
    assert::<RefCell<i32>>();
    assert::<Box<i32>>();
    assert::<Box<[u8]>>();

    {
        trait Trait: UnwindSafe {}
        assert::<Box<dyn Trait>>();
    }

    fn baz<T: UnwindSafe>() {
        assert::<Box<T>>();
        assert::<Vec<T>>();
        assert::<RefCell<T>>();
        assert::<AssertUnwindSafe<T>>();
        assert::<&AssertUnwindSafe<T>>();
    }
}

#[test]
fn catch_unwind_tests() {
    if let Ok(success_value) = panic::catch_unwind(|| {
        42
    }) {
        assert_eq!(success_value, 42);
    } else {
        unreachable!();
    }

    if let Err(failure_value) = panic::catch_unwind(|| {
        core::panic!("Oh, no: {}!", "Failure");
    }) {
        assert_eq!(failure_value, ());
    } else {
        unreachable!();
    }
}
