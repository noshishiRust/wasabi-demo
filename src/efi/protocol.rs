use crate::efi::table::EfiSystemTable;
use crate::efi::{EfiGuid, EfiStatus, EfiVoid};
use crate::Result;
use core::mem::size_of;

/// Reference: <https://uefi.org/specs/UEFI/2.11/12_Protocols_Console_Support.html#efi-graphics-output-protocol>
const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data0: 0x9042a9de,
    data1: 0x23dc,
    data2: 0x4a38,
    data3: [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
};

#[repr(C)]
#[derive(Debug)]
pub struct EfiGraphicsOutputProtocolPixelInfo {
    version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    _padding0: [u32; 5],
    pub pixels_per_scan_line: u32,
}
const _: () = assert!(size_of::<EfiGraphicsOutputProtocolPixelInfo>() == 36);

#[repr(C)]
#[derive(Debug)]
pub struct EfiGraphicsOutputProtocolMode<'a> {
    pub max_mode: u32,
    pub mode: u32,
    pub info: &'a EfiGraphicsOutputProtocolPixelInfo,
    pub size_of_info: u64,
    pub frame_buffer_base: usize,
    pub frame_buffer_size: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct EfiGraphicsOutputProtocol<'a> {
    reserved: [u64; 3],
    pub mode: &'a EfiGraphicsOutputProtocolMode<'a>,
}

pub fn locate_graphic_protocol<'a>(
    efi_system_table: &EfiSystemTable,
) -> Result<&'a EfiGraphicsOutputProtocol<'a>> {
    let mut graphics_output_protocol: *mut EfiGraphicsOutputProtocol = core::ptr::null_mut();
    let status = (efi_system_table.boot_services.locate_protocol)(
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID as *const _,
        core::ptr::null(),
        &mut graphics_output_protocol as *mut _ as *mut *mut EfiVoid,
    );
    if status != EfiStatus::Success {
        return Err("Failed to locate Graphics Output Protocol");
    }
    Ok(unsafe { &*graphics_output_protocol })
}
