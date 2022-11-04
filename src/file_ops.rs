#![allow(non_snake_case)]
//-----------------------------------------------------------------------------
// [SECTION] MISC HELPERS/UTILITIES (File functions)
//-----------------------------------------------------------------------------

// Default file functions
// #ifndef IMGUI_DISABLE_DEFAULT_FILE_FUNCTIONS

use std::ptr::null_mut;
use libc::{c_char, c_int, c_void, size_t};
use windows_sys::Win32;
use crate::type_defs::{ImFileHandle, ImWchar};

// ImFileOpen: ImFileHandle(const char* filename, const char* mode)
pub unsafe fn ImFileOpen(filename: * c_char, mode: * c_char) -> ImFileHandle {
    // #if defined(_WIN32) && !defined(IMGUI_DISABLE_WIN32_FUNCTIONS) && !defined(__CYGWIN__) && !defined(__GNUC__)
    // We need a fopen() wrapper because MSVC/Windows fopen doesn't handle UTF-8 filenames.
    // Previously we used ImTextCountCharsFromUtf8/ImTextStrFromUtf8 here but we now need to support ImWchar16 and ImWchar32!
    if cfg!(windows) {
        let filename_wsize = Win32::Globalization::MultiByteToWideChar(Win32::Globalization::CP_UTF8, 0, filename, -1, None, 0);
        let mode_wsize = Win32::Globalization::MultiByteToWideChar(Win32::Globalization::CP_UTF8, 0, mode, -1, None, 0);
        // ImVector<ImWchar> buf;
        let mut buf: Vec<ImWchar> = vec![];
        buf.resize((filename_wsize + mode_wsize) as usize, 0);
        Win32::Globalization::MultiByteToWideChar(Win32::Globalization::CP_UTF8, 0, filename, -1, &buf[0], filename_wsize);
        Win32::Globalization::MultiByteToWideChar(Win32::Globalization::CP_UTF8, 0, mode, -1, &buf[filename_wsize], mode_wsize);
        return ::_wfopen(&buf[0], &buf[filename_wsize]);
    }
// #else
    return libc::fopen(filename, mode);
// #endif
}

// We should in theory be using fseeko()/ftello() with off_t and _fseeki64()/_ftelli64() with __int64, waiting for the PR that does that in a very portable pre-C++11 zero-warnings way.
// bool    ImFileClose(0: ImFileHandle.0)     { return fclose(0.0) == 0; }
pub unsafe fn ImFileClose(f: ImFileHandle) -> c_int {
    libc::fclose(f)
}

pub unsafe fn ImFileGetSize(f: ImFileHandle) -> u64 {
    // long off = 0, sz = 0;
    let mut off = 0;
    let mut sz = 0;
    off = libc::ftell(f);
    let seek_result_1 = libc::fseek(f, 0, libc::SEEK_END);
    sz = libc::ftell(f);
    let seek_result_2 = libc::fseek(f, off, libc::SEEK_SET);

    if (off != -1) && !seek_result_1 > 0 && sz != -1 && !seek_result_2 > 0 {
        return sz as u64;
    }
    return -1;
}


pub unsafe fn ImFileRead(data: *mut c_void, sz: u64, count: u64, f: ImFileHandle) -> u64 {
    libc::fread(data, sz as size_t, count as size_t, f) as u64
}


pub unsafe fn ImFileWrite(data: *mut c_void, sz: u64, count: u64, f: ImFileHandle) -> u64 {
    libc::fwrite(data, sz as size_t, count as size_t, f) as u64
}
// #endif // #ifndef IMGUI_DISABLE_DEFAULT_FILE_FUNCTIONS

// Helper: Load file content into memory
// Memory allocated with IM_ALLOC(), must be freed by user using IM_FREE() == MemFree()
// This can't really be used with "rt" because fseek size won't match read size.
// void*   ImFileLoadToMemory(const char* filename, const char* mode, size_t* out_file_size, int padding_bytes)
pub unsafe fn ImFileLoadToMemory(filename: * c_char, mode: * c_char, out_file_size: *mut size_t, padding_bytes: i32) -> *mut c_void {
    // IM_ASSERT(filename && mode);
    if out_file_size.is_null() == false {
        *out_file_size = 0;
    }

    // f: ImFileHandle;
    let mut f: ImFileHandle = ImFileOpen(filename, mode);
    if f.is_null() {
        return None;
    }

    let file_size = ImFileGetSize(f);
    if file_size == -1 {
        ImFileClose(f);
        return None;
    }

    // void* file_data = IM_ALLOC(file_size + padding_bytes);
    let mut file_data = libc::malloc((file_size + padding_bytes) as size_t);
    if file_data.is_null() {
        ImFileClose(f);
        return None;
    }
    if ImFileRead(file_data, 1, file_size, f) != file_size {
        ImFileClose(null_mut());
        libc::free(file_data);
        return None;
    }
    if padding_bytes > 0 {
        libc::memset(((file_data) + file_size), 0, padding_bytes as size_t);
    }

    ImFileClose(null_mut());
    if out_file_size {
        *out_file_size = file_size as size_t;
    }

    return file_data;
}
