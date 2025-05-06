#![no_std]
#![feature(offset_of)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner::test_runner)]
#![reexport_test_harness_main = "run_unit_tests"]
#![no_main]

pub mod allocator;
pub mod efi;
pub mod qemu;
pub mod serial;
pub mod x86;

#[cfg(test)]
pub mod test_runner;

use allocator::ALLOCATOR;
use efi::EfiHandle;
use efi::table::exit_from_efi_boot_services;
use efi::table::EfiSystemTable;
use efi::memory::MemoryMapHolder;

pub type Result<T> = core::result::Result<T, &'static str>;

#[cfg(test)]
#[no_mangle]
pub fn efi_main(image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    init_basic_runtime(image_handle, efi_system_table);
    run_unit_tests()
}

pub fn init_basic_runtime(
    image_handle: EfiHandle,
    efi_system_table: &EfiSystemTable,
) -> MemoryMapHolder {
    let mut memory_map = MemoryMapHolder::new();
    exit_from_efi_boot_services(
        image_handle,
        efi_system_table,
        &mut memory_map,
    );
    ALLOCATOR.init_with_mmap(&memory_map);
    memory_map
}
