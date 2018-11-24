//! Pure FFI to [Jon Blow's VS discovery script](https://pastebin.com/3YvWQa5c).
//!
//! The rest of this crate's documentation is copied borderline verbatim
//! from the original C++ source code.
//!
//! HOW TO USE THIS CODE
//!
//! The purpose of this file is to find the folders that contain libraries
//! you may need to link against, on Windows, if you are linking with any
//! compiled C or C++ code. This will be necessary for many non-C++ programming
//! language environments that want to provide compatibility.
//!
//! We find the place where the Visual Studio libraries live (for example,
//! libvcruntime.lib), where the linker and compiler executables live
//! (for example, link.exe), and where the Windows SDK libraries reside
//! (kernel32.lib, libucrt.lib).
//!
//! We all wish you didn't have to worry about so many weird dependencies,
//! but we don't really have a choice about this, sadly.
//!
//! I don't claim that this is the absolute best way to solve this problem,
//! and so far we punt on things (if you have multiple versions of Visual Studio
//! installed, we return the first one, rather than the newest). But it
//! will solve the basic problem for you as simply as I know how to do it,
//! and because there isn't too much code here, it's easy to modify and expand.
//!
//!
//! Here is the API you need to know about:
//!
//!
//! Call `vswhom_find_visual_studio_and_windows_sdk()`, look at the resulting
//! paths, then call `vswhom_free_resources()` on the result.


extern crate libc;

use libc::{wchar_t, c_int};


#[repr(C)]
pub struct Find_Result {
    /// Zero if no Windows SDK found.
    pub windows_sdk_version: c_int,

    pub windows_sdk_root: *mut wchar_t,
    pub windows_sdk_um_library_path: *mut wchar_t,
    pub windows_sdk_ucrt_library_path: *mut wchar_t,

    pub vs_exe_path: *mut wchar_t,
    pub vs_library_path: *mut wchar_t,
}

extern "C" {
    pub fn vswhom_find_visual_studio_and_windows_sdk() -> Find_Result;

    pub fn vswhom_free_resources(result: *mut Find_Result);
}
