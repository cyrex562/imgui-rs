use windows::Win32::Foundation::{HANDLE, HWND};
use libc::c_void;

pub enum ViewportPlatformHandle {
    Unset,
    WinHandle(HWND),
    VoidPointer(*mut c_void)
}
