#![feature(no_std,start,core,lang_items)]
#![no_main]
#![no_std]

extern crate core;
use core::prelude::*;

extern {
    fn puts(_: *const u8) -> i32;
}

#[lang = "start"]
#[no_mangle]
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe {
        puts(b"Hello, World!\0".as_ptr());
    }
    0
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
