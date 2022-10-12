#![allow(non_snake_case)]
//-----------------------------------------------------------------------------
// [SECTION] MISC HELPERS/UTILITIES (String, Format, Hash functions)
//-----------------------------------------------------------------------------

use std::ffi::CStr;
use std::ptr::null;
use libc::{c_char, c_int, c_uchar, c_uint, size_t};
use crate::math::ImMin;
use crate::type_defs::ImWchar;

// Consider using _stricmp/_strnicmp under Windows or strcasecmp/strncasecmp. We don't actually use either ImStricmp/ImStrnicmp in the codebase any more.
// int ImStricmp(const char* str1, const char* str2)
pub unsafe fn ImStricmp(str1: * c_char, str2: * c_char) -> i32 {
    // let mut d = i32;
    // while ((d = toupper(*str2) - toupper(*str1)) == 0 && *str1) { str1+= 1; str2+= 1; }
    // return d;
    libc::stricmp(str1, str2)
}

// int ImStrnicmp(const char* str1, const char* str2, count: size_t)
pub unsafe fn ImStrnicmp(str1: * c_char, str2: * c_char, count: size_t) -> i32 {
    // int d = 0;
    // while (count > 0 && (d = toupper(*str2) - toupper(*str1)) == 0 && *str1) { str1+= 1; str2+= 1; count-= 1; }
    // return d;
    libc::strnicmp(str1, str2, count)
}

// void ImStrncpy(char* dst, const char* src, count: size_t)
pub unsafe fn ImStrncpy(dst: *mut c_char, src: * c_char, count: size_t) {
    // if (count < 1)
    //     return;
    // if (count > 1)
    //     strncpy(dst, src, count - 1);
    // dst[count - 1] = 0;
    libc::strncpy(dst, src, count);
}

// char* ImStrdup(const char* str)
pub unsafe fn ImStrdup(src_str: * c_char) -> *mut c_char {
    // let len = strlen(str);
    // void* buf = IM_ALLOC(len + 1);
    // return (char*)memcpy(buf, (const void*)str, len + 1);
    libc::strdup(src_str)
}

// char* ImStrdupcpy(char* dst, size_t* p_dst_size, const char* src)
pub unsafe fn ImStrdupcpy(mut dst: *mut c_char, p_dst_size: *mut size_t, src: * c_char) -> *mut c_char {
    let mut dst_buf_size = if !p_dst_size.is_null() { *p_dst_size } else { libc::strlen(dst) + 1 };
    let mut src_size = libc::strlen(src) + 1;
    if dst_buf_size < src_size {
        // IM_FREE(dst);
        libc::free(dst);
        // dst = (char*)IM_ALLOC(src_size);
        dst = libc::malloc(src_size);
        if !p_dst_size.is_null() {
            *p_dst_size = src_size;
        }
    }
    return libc::memcpy(dst, src, src_size) as *mut c_char;
}

// const char* ImStrchrRange(const char* str, const char* str_end, char c)
pub unsafe fn ImStrchrRange(str_start: * c_char, str_end: * c_char, c: c_char) -> * c_char {
    // const char* p = (const char*)memchr(str, c, str_end - str);
    // return p;
    libc::memchr(str_start, c as c_int, str_end - str_start) as * c_char
}

// int ImStrlenW(const str: *mut ImWchar)
pub unsafe fn ImStrlenW(str_begin: * ImWchar) -> i32 {
    //return wcslen((const wchar_t*)str);  // FIXME-OPT: Could use this when wchar_t are 16-bit
    let mut n = 0;
    // while (*str++) n+= 1;
    while *str_begin != 0 {
        n += 1;
    }
    return n;
}

// Find end-of-line. Return pointer will point to either first \n, either str_end.
// const char* ImStreolRange(const char* str, const char* str_end)
pub unsafe fn ImStreolRange(str_begin: * c_char, str_end: * c_char) -> * c_char {
    // const char* p = (const char*)memchr(str, '\n', str_end - str);
    let p = libc::memchr(str_begin, '\n' as c_int, str_end - str_begin);
    // return p ? p : str_end;
    return if !p.is_null() {
        p
    } else {
        str_end
    };
}

// const ImStrbolW: *mut ImWchar(const buf_mid_line: *mut ImWchar, const buf_begin: *mut ImWchar) // find beginning-of-line
pub fn ImStrbolW(mut buf_mid_line: * ImWchar, buf_begin: * ImWchar) -> * ImWchar {
    while buf_mid_line > buf_begin && buf_mid_line[-1] != '\n' {
        buf_mid_line -= 1;
    }
    return buf_mid_line;
}

// const char* ImStristr(const char* haystack, const char* haystack_end, const char* needle, const char* needle_end)
pub unsafe fn ImStristr(mut haystack: * c_char, haystack_end: * c_char, needle: * c_char, mut needle_end: * c_char) -> * c_char {
    if !needle_end {
        needle_end = needle + libc::strlen(needle);
    }

    let un0 = libc::toupper(*needle as c_int);
    while (haystack_end.is_null() && ((*haystack as c_int) != 0)) || (!haystack_end.is_null() && (haystack < haystack_end)) {
        if libc::toupper(*haystack as c_int) == un0 {
            let b = needle + 1;
            // for (const char* a = haystack + 1; b < needle_end; a++, b++)
            while b < needle_end {
                // if (toupper(*a) != toupper(*b))
                if libc::toupper(*a) != libc::toupper(*b) {
                    break;
                }
            }
            if b == needle_end {
                return haystack;
            }
        }
        haystack += 1;
    }
    return null();
}

// Trim str by offsetting contents when there's leading data + writing a \0 at the trailing position. We use this in situation where the cost is negligible.
// void ImStrTrimBlanks(char* bu0f32)
pub unsafe fn ImStrTrimBlanks(buf: *mut c_char) {
    let mut p = buf;
    while p[0] == ' ' || p[0] == '\t' {    // Leading blanks
        p += 1;
    }
    let mut p_start = p;
    while *p != 0 {                        // Find end of string
        p += 1;
    }
    while p > p_start && (p[-1] == ' ' || p[-1] == '\t') {  // Trailing blanks
        p -= 1;
    }
    if p_start != bu0f32 {                   // Copy memory if we had leading blanks
        libc::memmove(buf, p_start, p - p_start);
    }
    buf[p - p_start] = 0;                   // Zero terminate
}

// const char* ImStrSkipBlank(const char* str)
pub fn ImStrSkipBlank(in_str: * c_char) {
    while str[0] == ' ' || str[0] == '\t' {
        str += 1;
    }
    return str;
}

// A) MSVC version appears to return -1 on overflow, whereas glibc appears to return total count (which may be >= buf_size).
// Ideally we would test for only one of those limits at runtime depending on the behavior the vsnprintf(), but trying to deduct it at compile time sounds like a pandora can of worm.
// B) When buf==NULL vsnprintf() will return the output size.
// #ifndef IMGUI_DISABLE_DEFAULT_FORMAT_FUNCTIONS

// We support stb_sprintf which is much faster (see: https://github.com/nothings/stb/blob/master/stb_sprintf.h)
// You may set IMGUI_USE_STB_SPRINTF to use our default wrapper, or set IMGUI_DISABLE_DEFAULT_FORMAT_FUNCTIONS
// and setup the wrapper yourself. (FIXME-OPT: Some of our high-level operations such as ImGuiTextBuffer::appendfv() are
// designed using two-passes worst case, which probably could be improved using the stbsp_vsprintfcb() function.)
// #ifdef IMGUI_USE_STB_SPRINTF
// #define STB_SPRINTF_IMPLEMENTATION
// #ifdef IMGUI_STB_SPRINTF_FILENAME
// #include IMGUI_STB_SPRINTF_FILENAME
// #else
// #include "stb_sprintf.h"
// #endif
// #endif

// #if defined(_MSC_VER) && !defined(vsnprint0f32)
// #define vsnprintf _vsnprintf
// #endif

// int ImFormatString(char* buf, buf_size: size_t, const char* fmt, ...)
pub fn ImFormatString(buf: *mut c_char, buf_size: usize, fmt_str: * c_char) -> i32 {
//     va_list args;
//     va_start(args, fmt);
// // #ifdef IMGUI_USE_STB_SPRINTF
//     int w = stbsp_vsnprintf(buf, buf_size, fmt, args);
// // #else
//     int w = vsnprintf(buf, buf_size, fmt, args);
// // #endif
//     va_end(args);
//     if (buf == NULL)
//         return w;
//     if (w == -1 || w >= buf_size)
//         w = buf_size - 1;
//     buf[w] = 0;
//     return w;
    todo!()
}

// int ImFormatStringV(char* buf, buf_size: size_t, const char* fmt, va_list args)
pub fn ImFormatStringV(buf: *mut c_char, buf_size: usize, fmt_str: * c_char, args: &Vec<String>) -> i32 {
// // #ifdef IMGUI_USE_STB_SPRINTF
//     int w = stbsp_vsnprintf(buf, buf_size, fmt, args);
// // #else
//     int w = vsnprintf(buf, buf_size, fmt, args);
// // #endif
//     if (buf == NULL)
//         return w;
//     if (w == -1 || w >= buf_size)
//         w = buf_size - 1;
//     buf[w] = 0;
//     return w;
    todo!()
}

// #endif // #ifdef IMGUI_DISABLE_DEFAULT_FORMAT_FUNCTIONS
// void ImFormatStringToTempBuffer(const char** out_buf, const char** out_buf_end, const char* fmt, ...)
pub fn ImFormatStringToTempBuffer(out_buf: *mut *mut c_char, out_buf_end: *mut *mut c_char, fmt_str: * c_char, args: &Vec<String>) {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    // // va_list args;
    //
    // va_start(args, fmt);
    // int buf_len = ImFormatStringV(g.TempBuffer.Data, g.TempBuffer.Size, fmt, args);
    // *out_buf = g.TempBuffer.Data;
    // if (out_buf_end) { *out_buf_end = g.TempBuffer.Data + buf_len; }
    // va_end(args);
    todo!()
}

// void ImFormatStringToTempBufferV(const char** out_buf, const char** out_buf_end, const char* fmt, va_list args)
pub fn ImFormatStringToTempBufferV(out_buf: *mut *mut c_char, out_buf_end: *mut *mut c_char, fmt_str: * c_char, args: &Vec<String>) {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    // int buf_len = ImFormatStringV(g.TempBuffer.Data, g.TempBuffer.Size, fmt, args);
    // *out_buf = g.TempBuffer.Data;
    // if (out_buf_end) { *out_buf_end = g.TempBuffer.Data + buf_len; }
    todo!()
}




//-----------------------------------------------------------------------------
// [SECTION] MISC HELPERS/UTILITIES (ImText* functions)
//-----------------------------------------------------------------------------

// Convert UTF-8 to 32-bit character, process single character input.
// A nearly-branchless UTF-8 decoder, based on work of Christopher Wellons (https://github.com/skeeto/branchless-utf8).
// We handle UTF-8 decoding error by skipping forward.
// int ImTextCharFromUtf8(unsigned int* out_char, const char* in_text, const char* in_text_end)
pub unsafe fn ImTextCharFromUtf8(out_char: *mut c_uint, in_text: *c_char, mut in_text_end: *c_char) -> c_int
{
    pub const lengths: [c_char;32] = [ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 3, 3, 4, 0 ];
    pub const masks: [c_int;5]  = [ 0x00, 0x7f, 0x1f, 0x0f32, 0x07 ];
    pub const mins: [u32;5] = [ 0x400000, 0, 0x80, 0x800, 0x10000 ];
    pub const shiftc: [c_int;5] = [ 0, 18, 12, 6, 0 ];
    pub const shifte: [c_int;5] = [ 0, 6, 4, 2, 0 ];
    let mut len = lengths[*in_text >> 3];
    let mut wanted = len + !len;

    if in_text_end.is_null() {
        in_text_end = in_text + wanted;
    } // Max length, nulls will be taken into account.

    // Copy at most 'len' bytes, stop copying at 0 or past in_text_end. Branch predictor does a good job here,
    // so it is fast even with excessive branching.
    let mut s: [c_uchar;4] = [0;4];
    s[0] = if in_text + 0 < in_text_end { in_text[0] } else { 0 };
    s[1] = if in_text + 1 < in_text_end { in_text[1] } else { 0 };
    s[2] = if in_text + 2 < in_text_end { in_text[2] } else { 0 };
    s[3] = if in_text + 3 < in_text_end { in_text[3] } else { 0 };

    // Assume a four-byte character and load four bytes. Unused bits are shifted out.
    *out_char  = (s[0] & masks[len]) << 18;
    *out_char |= (s[1] & 0x30f32) << 12;
    *out_char |= (s[2] & 0x30f32) <<  6;
    *out_char |= (s[3] & 0x30f32) <<  0;
    *out_char >>= shiftc[len];

    // Accumulate the various error conditions.
    let mut e = 0;
    e  = (*out_char < mins[len]) << 6; // non-canonical encoding
    e |= ((*out_char >> 11) == 0x1b) << 7;  // surrogate half?
    e |= (*out_char > IM_UNICODE_CODEPOINT_MAX) << 8;  // out of range?
    e |= (s[1] & 0xc0) >> 2;
    e |= (s[2] & 0xc0) >> 4;
    e |= (s[3]       ) >> 6;
    e ^= 0x2a; // top two bits of each tail byte correct?
    e >>= shifte[len];

    if e
    {
        // No bytes are consumed when *in_text == 0 || in_text == in_text_end.
        // One byte is consumed in case of invalid first byte of in_text.
        // All available bytes (at most `len` bytes) are consumed on incomplete/invalid second to last bytes.
        // Invalid or incomplete input may consume less bytes than wanted, therefore every byte has to be inspected in s.
        wanted = ImMin(wanted, !!s[0] + !!s[1] + !!s[2] + !!s[3]);
        *out_char = IM_UNICODE_CODEPOINT_INVALID;
    }

    return wanted;
}

// int ImTextStrFromUtf8(buf: *mut ImWchar, int buf_size, const char* in_text, const char* in_text_end, const char** in_text_remaining)
pub unsafe fn ImTextStrFromUtf8(buf: *mut ImWchar, buf_size: i32, mut in_text: *const c_char, in_text_end: *const c_char, in_text_remaining: *mut *const c_char)
{
    // buf_out: *mut ImWchar = buf;
    let mut buf_out = buf;
    // buf_end: *mut ImWchar = buf + buf_size;
    let mut buf_end = buf + buf_size;
    while buf_out < buf_end - 1 && (in_text_end.is_null() || in_text < in_text_end) && *in_text != 0
    {
        let mut c: c_uint = 0;
        in_text += ImTextCharFromUtf8(&mut c, in_text, in_text_end);
        if c == 0 {
            break;
        }
        *buf_out = c;
        buf_out += 1;
    }
    *buf_out = 0;
    if in_text_remaining.is_null() == false {
        *in_text_remaining = in_text;
    }
    return buf_out - buf;
}

// int ImTextCountCharsFromUtf8(const char* in_text, const char* in_text_end)
pub unsafe fn ImTextCountCharsFromUtf8(mut in_text: *const c_char, in_text_end: *const c_char) -> i32
{
    // int char_count = 0;
    let mut char_count: i32 = 0;
    while (in_text_end.is_null() || in_text < in_text_end) && *in_text != 0
    {
        // unsigned int c;
        let mut c: c_uint = 0;
        in_text += ImTextCharFromUtf8(&mut c, in_text, in_text_end);
        if c == 0 {
            break;
        }
        char_count += 1;
    }
    return char_count;
}

// Based on stb_to_utf8() from github.com/nothings/stb/
// static inline int ImTextCharToUtf8_inline(char* buf, int buf_size, unsigned int c)
pub fn ImTextCharToUtf8_inline(buf: *mut c_char, buf_size: c_int, c: c_uint) -> c_int
{
    if c < 0x80
    {
        buf[0] = c;
        return 1;
    }
    if c < 0x800
    {
        if (buf_size < 2) { return 0; }
        buf[0] = (0xc0 + (c >> 6));
        buf[1] = (0x80 + (c & 0x3F));
        return 2;
    }
    if (c < 0x10000)
    {
        if (buf_size < 3) { return 0; }
        buf[0] = (0xe0 + (c >> 12));
        buf[1] = (0x80 + ((c >> 6) & 0x3F));
        buf[2] = (0x80 + ((c ) & 0x3F));
        return 3;
    }
    if (c <= 0x10FFFF)
    {
        if (buf_size < 4) { return 0; }
        buf[0] = (0xf0 + (c >> 18));
        buf[1] = (0x80 + ((c >> 12) & 0x3F));
        buf[2] = (0x80 + ((c >> 6) & 0x3F));
        buf[3] = (0x80 + ((c ) & 0x3F));
        return 4;
    }
    // Invalid code point, the max unicode is 0x10FFFF
    return 0;
}

// const char* ImTextCharToUtf8(char out_buf[5], unsigned int c)
pub fn ImTextCharToUtf8(mut out_buf: [c_char;5]) -> *const c_char
{
    let mut  count = ImTextCharToUtf8_inline(out_buf.as_mut_ptr(), 5, c);
    out_buf[count] = 0;
    return out_buf.as_ptr();
}

// Not optimal but we very rarely use this function.
// int ImTextCountUtf8BytesFromChar(const char* in_text, const char* in_text_end)
pub unsafe fn ImTextCountUtf8BytesFromChar(in_text: *const c_char, in_text_end: *const c_char) -> c_int {
    // unsigned int unused = 0;
    let mut unused: c_uint = 0;
    return ImTextCharFromUtf8(&mut unused, in_text, in_text_end);
}

// static inline int ImTextCountUtf8BytesFromChar(unsigned int c)
pub fn ImTextCountUtf8BytesFromChar2(c: c_uint) -> c_int
{
    if c < 0x80 { return 1; };
    if c < 0x800 { return 2; };
    if c < 0x10000 { return 3; };
    if c <= 0x10FFFF { return 4; };
    return 3;
}

// int ImTextStrToUtf8(char* out_buf, int out_buf_size, const in_text: *mut ImWchar, const in_text_end: *mut ImWchar)
pub unsafe fn ImTextStrToUtf8(out_buf: *mut c_char, out_buf_size: c_int, int_text: *const ImWchar, in_text_end: *const ImWchar) -> c_int
{
    let mut buf_p = out_buf;
    let buf_end = out_buf + out_buf_size;
    while buf_p < buf_end - 1 && (in_text_end.is_null || in_text < in_text_end) && *in_text != 0
    {
        // let c = (*in_text++);
        let mut c = *in_text;
        in_text += 1;
        if c < 0x80 {
            // *buf_p ++ = c;
            *buf_p = c;
            buf_p += 1;
        }
        else {
            buf_p += ImTextCharToUtf8_inline(buf_p, (buf_end - buf_p - 1), c);
        }
    }
    *buf_p = 0;
    return buf_p - out_buf;
}

// int ImTextCountUtf8BytesFromStr(const in_text: *mut ImWchar, const in_text_end: *mut ImWchar)
pub unsafe fn ImTextCountUtf8BytesFromStr(mut in_text: *const ImWchar, in_text_end: *const ImWchar) -> c_int
{
    let mut bytes_count = 0;
    while (in_text_end.is_null() || in_text < in_text_end) && *in_text != 0
    {
        // unsigned int c = (*in_text++);
        let mut c = *in_text;
        in_text += 1;
        if c < 0x80 {
            bytes_count += 1;
        }
        else {
            bytes_count += ImTextCountUtf8BytesFromChar2(c);
        }
    }
    return bytes_count;
}

pub unsafe fn str_to_const_c_char_ptr(in_str: &str) -> *const c_char {
    CStr::from_bytes_with_nul_unchecked(in_str.as_bytes()).as_ptr()
}
