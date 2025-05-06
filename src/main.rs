#![no_std]
#![no_main]

use core::fmt::Write;
use wasabi_demo::efi::bitmap::BitMap;
use wasabi_demo::efi::vram::{fill_rect, init_vram, Color, VramTextWriter};
use wasabi_demo::efi::{EfiHandle, EfiSystemTable};
use wasabi_demo::hlt;

/// UEFI entry point
///
/// Reference: <https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#uefi-image-entry-point>
#[no_mangle]
fn efi_main(_image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    let mut vram = init_vram(efi_system_table).expect("Failed to initialize VRAM");

    let vw = vram.width();
    let vh = vram.height();
    fill_rect(&mut vram, Color::Black as u32, 0, 0, vw, vh).expect("fill_rect failed");

    let mut w = VramTextWriter::new(&mut vram);

    for i in 0..4 {
        writeln!(w, "i = {i}").unwrap();
    }

    loop {
        hlt();
    }
}

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        hlt();
    }
}
