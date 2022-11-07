use crate::font::ImFont;
use crate::core::type_defs::ImWchar;
use libc::{c_char, c_int, c_uchar, c_uint, c_ushort, size_t};
use std::ptr::null_mut;
use crate::font_atlas::ImFontAtlas;
use crate::GImGui;
use crate::core::math_ops::ImMax;
use crate::core::utils::is_not_null;

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
    accumulative_offsets: &[u16],
    accumulative_offsets_count: size_t,
    out_ranges: &mut String,
) {
    for n in 0..accumulative_offsets_count {
        out_ranges[0] = out_ranges[1] = (base_codepoint + accumulative_offsets[n]);
        base_codepoint += accumulative_offsets[n];
        *out_ranges = out_ranges[2..].to_string();
    }
    out_ranges[0] = 0;
}

pub fn FindFirstExistingGlyph(
    font: &mut ImFont,
    candidate_chars: &[u8],
) -> Option<char> {
    // for (let n: c_int = 0; n < candidate_chars_count; n++)
    for n in 0..candidate_chars.len() {
        if font.FindGlyphNoFallback(candidate_chars[n]).is_some() {
            return Some(candidate_chars[n]);
        }
    }
    return None;
}



// Important: this alone doesn't alter current ImDrawList state. This is called by PushFont/PopFont only.
pub unsafe fn SetCurrentFont(font: ImFont)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(font && font->IsLoaded());    // Font Atlas not created. Did you call io.Fonts.GetTexDataAsRGBA32 / GetTexDataAsAlpha8 ?
    // IM_ASSERT(font->Scale > 0.0);
    g.Font = font.clone();
    g.FontBaseSize = ImMax(1.0, g.IO.FontGlobalScale * g.Font.FontSize * g.Font.Scale);
    g.FontSize = if g.CurrentWindow.is_some() { g.Currentwindow.unwrap().CalcFontSize() } else { 0.0 };

    let atlas = &g.Font.ContainerAtlas;
    g.DrawListSharedData.TexUvWhitePixel = atlas.TexUvWhitePixel;
    g.DrawListSharedData.TexUvLines = atlas.TexUvLines.clone();
    g.DrawListSharedData.Font = g.Font;
    g.DrawListSharedData.FontSize = g.FontSize;
}

pub unsafe fn PushFont(mut font: Option<ImFont>)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if font.is_none() {
        font.replace(GetDefaultFont())
    }
    SetCurrentFont(font.unwrap());
    g.FontStack.push(font.unwrap().clone());
    // g.Currentwindow.DrawList.PushTextureID(font.unwrap().ContainerAtlas.TexID);
    g.CurrentWindow.as_mut().unwrap().DrawList.PushTextureID(font.unwrap().ContainerAtlas.TexID);
}


pub unsafe fn  PopFont()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.CurrentWindow.unwrap().DrawList.PopTextureID();
    g.FontStack.pop_back();
    SetCurrentFont(if g.FontStack.empty() { GetDefaultFont() } else { g.FontStack.last_mut().unwrap() }.clone());
}
