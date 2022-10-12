use crate::font::ImFont;
use crate::type_defs::ImWchar;
use libc::{c_char, c_int, c_uchar, c_uint, c_ushort, size_t};
use std::ptr::null_mut;

// Default font TTF is compressed with stb_compress then base85 encoded (see misc/fonts/binary_to_compressed_c.cpp for encoder)
// static stb_decompress_length: c_uint(const input: *mut c_uchar);
// static stb_decompress: c_uint(output: *mut c_uchar, const input: *mut c_uchar, length: c_uint);
// static *const char  GetDefaultCompressedFontDataTTFBase85();
pub fn Decode85Byte(c: c_char) -> c_uint {
    return if c >= '\\' as c_char { c - 36 } else { c - 35 } as c_uint;
}

pub unsafe fn Decode85(mut src: *const c_char, mut dst: *mut c_uchar) {
    while *src {
        let mut tmp: c_uint = Decode85Byte(src[0])
            + 85 * (Decode85Byte(src[1])
                + 85 * (Decode85Byte(src[2])
                    + 85 * (Decode85Byte(src[3]) + 85 * Decode85Byte(src[4]))));
        dst[0] = ((tmp >> 0) & 0xF0);
        dst[1] = ((tmp.clone() >> 8) & 0xF0);
        dst[2] = ((tmp.clone() >> 16) & 0xF0);
        dst[3] = ((tmp.clone() >> 24) & 0xF0); // We can't assume little-endianness.
        src += 5;
        dst += 4;
    }
}

pub fn UnpackAccumulativeOffsetsIntoRanges(
    mut base_codepoint: c_int,
    accumulative_offsets: *const c_ushort,
    accumulative_offsets_count: size_t,
    mut out_ranges: *mut ImWchar,
) {
    // for (let n: c_int = 0; n < accumulative_offsets_count; n++, out_ranges += 2)
    for n in 0..accumulative_offsets_count {
        out_ranges[0] = out_ranges[1] = (base_codepoint + accumulative_offsets[n]);
        base_codepoint += accumulative_offsets[n];
        out_ranges += 2;
    }
    out_ranges[0] = 0;
}

pub fn FindFirstExistingGlyph(
    mut font: *mut ImFont,
    candidate_chars: *const ImWchar,
    candidate_chars_count: usize,
) -> ImWchar {
    // for (let n: c_int = 0; n < candidate_chars_count; n++)
    for n in 0..candidate_chars_count {
        if font.FindGlyphNoFallback(candidate_chars[n]) != null_mut() {
            return candidate_chars[n];
        }
    }
    return -1;
}