pub mod bitmap;
pub mod memory;
pub mod protocol;
pub mod table;
pub mod vram;

pub type EfiVoid = u8;
pub type EfiHandle = u64;

/// Reference: <https://uefi.org/specs/UEFI/2.11/Apx_A_GUID_and_Time_Formats.html#guid-and-time-formats>
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EfiGuid {
    pub data0: u32,
    pub data1: u16,
    pub data2: u16,
    pub data3: [u8; 8],
}

/// Reference: <https://uefi.org/specs/UEFI/2.11/Apx_D_Status_Codes.html#status-codes>
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u64)]
pub enum EfiStatus {
    Success = 0,
}
