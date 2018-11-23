extern crate libc;

use libc::{wchar_t, c_int};


#[repr(C)]
pub struct Find_Result {
    /// Zero if no Windows SDK found.
    windows_sdk_version: c_int,

    windows_sdk_root: *mut wchar_t,
    windows_sdk_um_library_path: *mut wchar_t,
    windows_sdk_ucrt_library_path: *mut wchar_t,

    vs_exe_path: *mut wchar_t,
    vs_library_path: *mut wchar_t,
}

extern "C" {
    pub fn vswhom_find_visual_studio_and_windows_sdk() -> Find_Result;

    pub fn vswhom_free_resources(result: *mut Find_Result);
}
