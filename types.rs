use std::os::raw::c_void;



pub type PVOID = *mut c_void;
pub type PVOID_PTR = *mut PVOID;
pub type USHORT = u16;
pub type USHORT_PTR = *mut USHORT;
pub type ULONG = u32;
pub type ULONG_PTR = *mut ULONG;
pub type HANDLE = PVOID;
pub type BOOLEAN = u8; // BYTE is typically represented as u8 in Rust
pub type BOOLEAN_PTR = *mut BOOLEAN;
pub type DWORD = u32;