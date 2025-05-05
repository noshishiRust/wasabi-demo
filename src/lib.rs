#![no_std]
#![feature(offset_of)]

pub mod efi;

use core::arch::asm;

pub type Result<T> = core::result::Result<T, &'static str>;

pub fn hlt() {
    unsafe {
        asm!("hlt",);
    }
}
