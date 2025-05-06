use crate::efi::memory::MemoryMapHolder;
use crate::efi::{EfiGuid, EfiStatus, EfiVoid};
use core::mem::offset_of;

/// EFI System Table
///
/// Reference: <https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#id6>
#[repr(C)]
pub struct EfiSystemTable {
    _reserved0: [u64; 12],
    pub boot_services: &'static EfiBootServicesTable,
}
const _: () = assert!(offset_of!(EfiSystemTable, boot_services) == 96); // 12 * 8 (u64) = 96

/// EFI Boot Services Table
///
/// Reference: <https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-boot-services-table>
#[repr(C)]
pub struct EfiBootServicesTable {
    _reserved0: [u64; 7],

    /// Reference: <https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-getmemorymap>
    get_memory_map: extern "win64" fn(
        memory_map_size: *mut usize,
        memory_map: *mut u8,
        map_key: *mut usize,
        descriptor_size: *mut usize,
        descriptor_version: *mut u32,
    ) -> EfiStatus,
    _reserved1: [u64; 32],

    // Reference: <https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-locateprotocol>
    pub locate_protocol: extern "win64" fn(
        protocol: *const EfiGuid,
        registration: *const EfiVoid,
        interface: *mut *mut EfiVoid,
    ) -> EfiStatus,
}
const _: () = assert!(offset_of!(EfiBootServicesTable, get_memory_map) == 56); // 7 * 8 (u64) + 8 (fn ptr) = 56
const _: () = assert!(offset_of!(EfiBootServicesTable, locate_protocol) == 320); // 40 * 8 (u64) = 320

impl EfiBootServicesTable {
    /// Get the memory map
    ///
    /// Reference: <https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-getmemorymap>
    pub fn get_memory_map(&self, map: &mut MemoryMapHolder) -> EfiStatus {
        (self.get_memory_map)(
            &mut map.memory_map_size,
            map.memory_map_buffer.as_mut_ptr(),
            &mut map.map_key,
            &mut map.descriptor_size,
            &mut map.descriptor_version,
        )
    }
}
