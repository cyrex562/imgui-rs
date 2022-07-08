use std::fs;
use std::fs::File;
use std::io::{Read, Write};

// ImFileHandle ImFileOpen(const char* filename, const char* mode)
pub fn ImFileOpen(filename: &String, mode: &String) -> ImFileHandle
{
// #if defined(_WIN32) && !defined(IMGUI_DISABLE_WIN32_FUNCTIONS) && !defined(__CYGWIN__) && !defined(__GNUC__)
//     // We need a fopen() wrapper because MSVC/windows fopen doesn't handle UTF-8 filenames.
//     // Previously we used ImTextCountCharsFromUtf8/ImTextStrFromUtf8 here but we now need to support ImWchar16 and ImWchar32!
//     const int filename_wsize = ::MultiByteToWideChar(CP_UTF8, 0, filename, -1, NULL, 0);
//     const int mode_wsize = ::MultiByteToWideChar(CP_UTF8, 0, mode, -1, NULL, 0);
//     ImVector<ImWchar> buf;
//     buf.resize(filename_wsize + mode_wsize);
//     ::MultiByteToWideChar(CP_UTF8, 0, filename, -1, (wchar_t*)&buf[0], filename_wsize);
//     ::MultiByteToWideChar(CP_UTF8, 0, mode, -1, (wchar_t*)&buf[filename_wsize], mode_wsize);
//     return ::_wfopen((const wchar_t*)&buf[0], (const wchar_t*)&buf[filename_wsize]);
// #else
//     return fopen(filename, mode);
// #endif
    fs::File::open(filename)
}

// We should in theory be using fseeko()/ftello() with off_t and _fseeki64()/_ftelli64() with __int64, waiting for the PR that does that in a very portable pre-C++11 zero-warnings way.
// bool    ImFileClose(ImFileHandle f)     { return fclose(f) == 0; }
pub fn ImFileClose(f: &fs::File) -> bool {
    todo!()
}

// ImU64   ImFileGetSize(ImFileHandle f)   { long off = 0, sz = 0; return ((off = ftell(f)) != -1 && !fseek(f, 0, SEEK_END) && (sz = ftell(f)) != -1 && !fseek(f, off, SEEK_SET)) ? sz : -1; }
pub fn ImFileGetSize(f: &fs::File) -> usize {
    let mut off = 0;
    let mut sz = 0;
    todo!()
}
// ImU64   ImFileRead(void* data, ImU64 sz, ImU64 count, ImFileHandle f)           { return fread(data, (size_t)sz, (size_t)count, f); }
pub fn ImFileRead(data: &mut Vec<u8>, sz: usize, count: usize, f: &mut fs::File) -> usize {
    f.read(data.as_mut_slice()).unwrap()
}
// ImU64   ImFileWrite(const void* data, ImU64 sz, ImU64 count, ImFileHandle f)    { return fwrite(data, (size_t)sz, (size_t)count, f); }
pub fn ImFileWrite(data: &mut Vec<u8>, sz: usize, count: usize, f: &mut fs::File) -> usize {
    f.write(data.as_slice()).unwrap()
}
// #endif // #ifndef IMGUI_DISABLE_DEFAULT_FILE_FUNCTIONS

// Helper: Load file content into memory
// Memory allocated with IM_ALLOC(), must be freed by user using IM_FREE() == ImGui::MemFree()
// This can't really be used with "rt" because fseek size won't match read size.
// void*   ImFileLoadToMemory(const char* filename, const char* mode, size_t* out_file_size, int padding_bytes)
pub fn ImFileLoadToMemory(filename: &String, mode: &String, out_file_size: &mut usize, padding_bytes: i32) -> Option<Vec<u8>>
{
    // IM_ASSERT(filename && mode);
    if out_file_size {
        *out_file_size = 0;
    }

    // ImFileHandle f;
    let mut f: fs::File;
    // if ((f = ImFileOpen(filename, mode)) == NULL) {
    //     return NULL;
    // }
    f = ImFileOpen(filename, mode);

    // size_t file_size = (size_t)ImFileGetSize(f);
    let file_size = ImFileGetSize(&f);
    if file_size == -1
    {
        ImFileClose(&f);
        // return NULL;
        return None;
    }

    // void* file_data = IM_ALLOC(file_size + padding_bytes);
    let mut file_data: Vec<u8> = Vec::new();
    file_data.reserve((file_size + padding_bytes) as usize);
    if (file_data == NULL)
    {
        ImFileClose(&mut f);
        return NULL;
    }
    if (ImFileRead(&mut file_data, 1, file_size, &mut f) != file_size)
    {
        ImFileClose(&mut f);
        // IM_FREE(file_data);
        return None;
    }
    if (padding_bytes > 0) {
        // memset((void *)(((char *)file_data) + file_size), 0, (size_t)
        // padding_bytes);
        // TODO
    }

    ImFileClose(&mut f);
    if (out_file_size) {
        *out_file_size = file_size;
    }

    return Some(file_data);
}
