use windows::Win32::Foundation::{HANDLE, HWND};

pub enum ViewportPlatformHandle {
    Unset,
    WinHandle(HWND),
}
