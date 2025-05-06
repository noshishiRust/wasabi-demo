use crate::efi::memory::MemoryMapHolder;
use crate::efi::{EfiGuid, EfiHandle, EfiStatus, EfiVoid};
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

    _reserved1: [u64; 21],

    /// Reference: <https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-exitbootservices>
    exit_boot_services: extern "win64" fn(image_handle: EfiHandle, map_key: usize) -> EfiStatus,

    _reserved2: [u64; 10],

    // Reference: <https://uefi.org/specs/UEFI/2.11/07_Services_Boot_Services.html#efi-boot-services-locateprotocol>
    pub locate_protocol: extern "win64" fn(
        protocol: *const EfiGuid,
        registration: *const EfiVoid,
        interface: *mut *mut EfiVoid,
    ) -> EfiStatus,
}
const _: () = assert!(offset_of!(EfiBootServicesTable, get_memory_map) == 56); // 7 * 8 (u64) = 56
const _: () = assert!(offset_of!(EfiBootServicesTable, exit_boot_services) == 232); // 7 * 8 (u64) + 8 (get_memory_map) + 21 * 8 (u64) = 232
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

pub fn exit_from_efi_boot_services(
    image_handle: EfiHandle,
    efi_system_table: &EfiSystemTable,
    memory_map: &mut MemoryMapHolder,
) {
    loop {
        let status = efi_system_table.boot_services.get_memory_map(memory_map);
        assert_eq!(status, EfiStatus::Success);
        let status =
            (efi_system_table.boot_services.exit_boot_services)(image_handle, memory_map.map_key);
        if status == EfiStatus::Success {
            break;
        }
    }
}
