#![no_std]
#![no_main]

use core::fmt::Write;
use wasabi_demo::efi::bitmap::BitMap;
use wasabi_demo::efi::memory::{EfiMemoryType, MemoryMapHolder};
use wasabi_demo::efi::table::{exit_from_efi_boot_services, EfiSystemTable};
use wasabi_demo::efi::vram::{draw_test_pattern, fill_rect, init_vram, Color, VramTextWriter};
use wasabi_demo::efi::EfiHandle;
use wasabi_demo::hlt;

/// UEFI entry point
///
/// Reference: <https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#uefi-image-entry-point>
#[no_mangle]
fn efi_main(image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    let mut vram = init_vram(efi_system_table).expect("Failed to initialize VRAM");

    let vw = vram.width();
    let vh = vram.height();
    fill_rect(&mut vram, Color::Black as u32, 0, 0, vw, vh).expect("fill_rect failed");
    draw_test_pattern(&mut vram);

    let mut w = VramTextWriter::new(&mut vram);

    for i in 0..4 {
        writeln!(w, "i = {i}").unwrap();
    }

    let mut memory_map = MemoryMapHolder::new();
    let status = efi_system_table
        .boot_services
        .get_memory_map(&mut memory_map);
    writeln!(w, "get_memory_map: {status:?}").unwrap();
    let mut total_memory_pages = 0;
    for e in memory_map.iter() {
        if e.memory_type() != EfiMemoryType::CONVENTIONAL_MEMORY {
            continue;
        }
        total_memory_pages += e.number_of_pages();
        writeln!(w, "{e:?}").unwrap();
    }
    let total_memory_size_mib = total_memory_pages * 4096 / 1024 / 1024;
    writeln!(
        w,
        "Total: {total_memory_pages} pages = {total_memory_size_mib} MiB"
    )
    .unwrap();

    exit_from_efi_boot_services(image_handle, efi_system_table, &mut memory_map);
    writeln!(w, "Hello, Non-UEFI world!").unwrap();

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
