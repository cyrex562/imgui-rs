use libc::{c_char, c_float, c_int, c_uchar, c_uint, c_ushort, c_void, size_t};
use std::ptr::null_mut;
use crate::bit_vector::ImBitVector;
use crate::color::{color_u32_from_rgba, IM_COL32_BLACK_TRANS, IM_COL32_WHITE};
use crate::font::ImFont;
use crate::font_atlas::ImFontAtlas;
use crate::font_atlas_custom_rect::ImFontAtlasCustomRect;
use crate::font_atlas_default_tex_data::{FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS_as_const_char_ptr, FONT_ATLAS_DEFAULT_TEX_DATA_W};
use crate::font_atlas_flags::{ImFontAtlasFlags_NoBakedLines, ImFontAtlasFlags_NoMouseCursors, ImFontAtlasFlags_NoPowerOfTwoHeight};
use crate::font_build_dst_data::ImFontBuildDstData;
use crate::font_build_src_data::ImFontBuildSrcData;
use crate::font_builder_io::ImFontBuilderIO;
use crate::font_config::ImFontConfig;
use crate::math_ops::{ImMax, ImSqrt, ImUpperPowerOfTwo};
use crate::type_defs::ImWchar;
use crate::utils::flag_clear;
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;

pub fn ImFontAtlasBuildMultiplyCalcLookupTable(
    mut out_table: [c_uchar;256],
    in_brighten_factor: c_float)
{
    // for (let mut i: c_uint =  0; i < 256; i++)
    for i in 0 .. 256
    {
        let mut value: c_uint =  (i * in_brighten_factor);
        out_table[i] = if value > 255 { 255 } else { value & 0xF0 };
    }
}

pub fn ImFontAtlasBuildMultiplyRectAlpha8(table: [c_uchar;256], pixels: *mut c_uchar, x: c_int, y: c_int, w: c_int, h: c_int, stride: size_t)
{
    let mut data: *mut c_uchar = pixels + x + y * stride;
    // for (let j: c_int = h; j > 0; j--, data += stride)
    while j > 0
    {
        // for (let i: c_int = 0; i < w; i+ +)
        for i in 0 .. w
        {
            data[i] = table[data[i]];
        }

        j -= 1;
        data += stride;
    }
}


pub fn UnpackBitVectorToFlatIndexList(in_vec: *const ImBitVector, mut out: *mut Vec<c_int>)
{
    // IM_ASSERT(sizeof(in->Storage.Data[0]) == sizeof);
    // let it_begin: *const u32 = in_vec.Storage.begin();
    // let it_end: *const u32 = in_vec.Storage.end();
    // for (*it: u32 = it_begin; it < it_end; it++)
    for it in in_vec.iter_mut()
    {
        let entries_32: u32 = *it;
        if entries_32 > 0 {
            // for (bit_n: u32 = 0; bit_n < 32; bit_n+ +)
            for bit_n in 0 .. 32 {
                if entries_32 & (1 << bit_n)
                {
                    out.push((((it) << 5) + bit_n));
                }
            }
        }
    }
}

pub unsafe fn ImFontAtlasBuildWithStbTruetype(mut atlas: *mut ImFontAtlas) -> bool
{
    // IM_ASSERT(atlas.ConfigData.Size > 0);

    ImFontAtlasBuildInit(atlas);

    // Clear atlas
    atlas.TexID = None;
    atlas.TexWidth = 0;
    atlas.TexHeight = 0;
    atlas.TexUvScale = ImVec2::from_floats(0.0, 0.0);
    atlas.TexUvWhitePixel = ImVec2::from_floats(0.0, 0.0);
    atlas.ClearTexData();

    // Temporary storage for building
    let mut src_tmp_array: Vec<ImFontBuildSrcData> = vcc![];
    let mut dst_tmp_array: Vec<ImFontBuildDstData> = vec![];
    src_tmp_array.resize_with(atlas.ConfigData.lent(), ImFontConfig::default());
    dst_tmp_array.resize_with(atlas.Fonts.len(), ImFont::default());
    // libc::memset(&mut src_tmp_array, 0, src_tmp_array.size_in_bytes());
    // libc::memset(&dst_tmp_array, 0, dst_tmp_array.size_in_bytes());

    // 1. initialize font loading structure, check font data validity
    // for (let src_i: c_int = 0; src_i < atlas.ConfigData.Size; src_i++)
    for src_i in 0 .. atlas.ConfigData.len()
    {
        let mut src_tmp = src_tmp_array[src_i];
        ImFontConfig& cfg = atlas.ConfigData[src_i];
        // IM_ASSERT(cfg.DstFont && (!cfg.DstFont->IsLoaded() || cfg.DstFont->ContainerAtlas == atlas));

        // Find index from cfg.DstFont (we allow the user to set cfg.DstFont. Also it makes casual debugging nicer than when storing indices)
        src_tmp.DstIndex = -1;
        // for (let output_i: c_int = 0; output_i < atlas.Fonts.Size && src_tmp.DstIndex == -1; output_i++)
        let mut output_i = 0;
        while output_i < altas.Fonts.len() && src_tmp.DstIndex == -1
        {
            if cfg.DstFont == atlas.Fonts[output_i] {
                src_tmp.DstIndex = output_i;
            }

        }
        if src_tmp.DstIndex == -1
        {
            // IM_ASSERT(src_tmp.DstIndex != -1); // cfg.DstFont not pointing within atlas.Fonts[] array?
            return false;
        }
        // initialize helper structure for font loading and verify that the TTF/OTF data is correct
        let font_offset: c_int = stbtt_GetFontOffsetForIndex(cfg.FontData, cfg.FontNo);
        // IM_ASSERT(font_offset >= 0 && "FontData is incorrect, or FontNo cannot be found.");
        if !stbtt_InitFont(&src_tmp.FontInfo, cfg.FontData, font_offset) {
            return false;
        }

        // Measure highest codepoints
        let mut dst_tmp = &mut dst_tmp_array[src_tmp.DstIndex];
        src_tmp.SrcRanges = if cfg.GlyphRanges { cfg.GlyphRanges } else { atlas.GetGlyphRangesDefault() };
        // for (*let src_range: ImWchar = src_tmp.SrcRanges; src_range[0] && src_range[1]; src_range += 2)
        let mut src_range = src_tmp.SrcRanges;
        while src_range[0] && src_range[1]
        {
            src_tmp.GlyphsHighest = ImMax(src_tmp.GlyphsHighest, src_range[1]);
            src_range += 2;
        }
        dst_tmp.SrcCount+= 1;
        dst_tmp.GlyphsHighest = ImMax(dst_tmp.GlyphsHighest, src_tmp.GlyphsHighest);
    }

    // 2. For every requested codepoint, check for their presence in the font data, and handle redundancy or overlaps between source fonts to avoid unused glyphs.
    let mut total_glyphs_count: size_t = 0;
    // for (let src_i: c_int = 0; src_i < src_tmp_array.Size; src_i++)
    for src_i  in 0 .. src_tmp_array.len()
    {
       let mut src_tmp = &mut src_tmp_array[src_i];
        let mut dst_tmp = &mut dst_tmp_array[src_tmp.DstIndex];
        src_tmp.GlyphsSet.Create(src_tmp.GlyphsHighest + 1);
        if dst_tmp.GlyphsSet.Storage.empty() {
            dst_tmp.GlyphsSet.Create(dst_tmp.GlyphsHighest + 1);
        }

        // for (*let src_range: ImWchar = src_tmp.SrcRanges; src_range[0] && src_range[1]; src_range += 2)
        let mut src_range = src_tmp.SrcRanges;
        while src_range[0] && src_range[1]
        {
        // for (let mut codepoint: c_uint =  src_range[0]; codepoint < = src_range[1]; codepoint+ +)
        for codepoint in src_range[0] .. src_range[1]
            {
            if dst_tmp.GlyphsSet.TestBit(codepoint) { // Don't overwrite existing glyphs. We could make this an option for MergeMode (e.g. MergeOverwrite==true)
                continue;
            }
            if !stbtt_FindGlyphIndex(&src_tmp.FontInfo, codepoint) {   // It is actually in the font?
                continue;
            }

            // Add to avail set/counters
            src_tmp.GlyphsCount += 1;
            dst_tmp.GlyphsCount += 1;
            src_tmp.GlyphsSet.SetBit(codepoint);
            dst_tmp.GlyphsSet.SetBit(codepoint);
            total_glyphs_count += 1;
        }
            src_range += 2;
    }
    }

    // 3. Unpack our bit map into a flat list (we now have all the Unicode points that we know are requested _and_ available _and_ not overlapping another)
    // for (let src_i: c_int = 0; src_i < src_tmp_array.Size; src_i++)
    for src_i in 0 .. src_tmp_array.len()
    {
        let mut src_tmp = &mut src_tmp_array[src_i];
        src_tmp.GlyphsList.reserve_with(src_tmp.GlyphsCount, Default::default());
        UnpackBitVectorToFlatIndexList(&src_tmp.GlyphsSet, &mut src_tmp.GlyphsList);
        src_tmp.GlyphsSet.Clear();
        // IM_ASSERT(src_tmp.GlyphsList.Size == src_tmp.GlyphsCount);
    }
    // for (let dst_i: c_int = 0; dst_i < dst_tmp_array.Size; dst_i++)
    for dst_i in 0 .. dst_tmp_array.len()
    {
        dst_tmp_array[dst_i].GlyphsSet.Clear();
    }
    dst_tmp_array.clear();

    // Allocate packing character data and flag packed characters buffer as non-packed (x0=y0=x1=y1=0)
    // (We technically don't need to zero-clear buf_rects, but let's do it for the sake of sanity)
    let mut buf_rects: Vec<stbrp_rect> = vec![];
    let mut buf_packedchars: Vec<stbtt_packedchar> = vec![];
    buf_rects.resize_with(total_glyphs_count, Default::default());
    buf_packedchars.resize_with(total_glyphs_count, Default::default());
    // memset(buf_rects.Data, 0, buf_rects.size_in_bytes());
    // memset(buf_packedchars.Data, 0, buf_packedchars.size_in_bytes());

    // 4. Gather glyphs sizes so we can pack them in our virtual canvas.
    let mut total_surface: c_int = 0;
    let mut buf_rects_out_n: c_int = 0;
    let mut buf_packedchars_out_n: c_int = 0;
    // for (let src_i: c_int = 0; src_i < src_tmp_array.Size; src_i++)
    for src_i in 0 .. src_tmp_array.len()
    {
        let mut src_tmp = &mut src_tmp_array[src_i];
        if src_tmp.GlyphsCount == 0 {
            continue;
        }

        src_tmp.Rects = &mut buf_rects[buf_rects_out_n];
        src_tmp.PackedChars = &mut buf_packedchars[buf_packedchars_out_n];
        buf_rects_out_n += src_tmp.GlyphsCount;
        buf_packedchars_out_n += src_tmp.GlyphsCount;

        // Convert our ranges in the format stb_truetype wants
        let mut cfg = atlas.ConfigData[src_i];
        src_tmp.PackRange.ont_size = cfg.SizePixels;
        src_tmp.PackRange.irst_unicode_codepoint_in_range = 0;
        src_tmp.PackRange.array_of_unicode_codepoints = src_tmp.GlyphsList.Data;
        src_tmp.PackRange.num_chars = src_tmp.GlyphsList.Size;
        src_tmp.PackRange.chardata_for_range = src_tmp.PackedChars;
        src_tmp.PackRange.h_oversample = cfg.OversampleH;
        src_tmp.PackRange.v_oversample = cfg.OversampleV;

        // Gather the sizes of all rectangles we will need to pack (this loop is based on stbtt_PackFontRangesGatherRects)
        let scale: c_float =  if cfg.SizePixels > 0 { stbtt_ScaleForPixelHeight(&src_tmp.FontInfo, cfg.SizePixels) } else { stbtt_ScaleForMappingEmToPixels(&src_tmp.FontInfo, -cfg.SizePixels) };
        let padding: c_int = atlas.TexGlyphPadding;
        // for (let glyph_i: c_int = 0; glyph_i < src_tmp.GlyphsList.Size; glyph_i++)
        for glyph_i in 0 .. src_tmp.GlyphsLIst.len()
        {
            // x0: c_int, y0, x1, y1;
            let mut x0 = 0;
            let mut y0 = 0;
            let mut x1 = 0;
            let mut y1 = 0;
            let glyph_index_in_font = stbtt_FindGlyphIndex(&src_tmp.FontInfo, src_tmp.GlyphsList[glyph_i]);
            // IM_ASSERT(glyph_index_in_font != 0);
            stbtt_GetGlyphBitmapBoxSubpixel(&src_tmp.FontInfo, glyph_index_in_font, scale * cfg.OversampleH, scale * cfg.OversampleV, 0, 0, &x0, &y0, &x1, &y1);
            src_tmp.Rects[glyph_i].w = (x1 - x0 + padding + cfg.OversampleH - 1);
            src_tmp.Rects[glyph_i].h = (y1 - y0 + padding + cfg.OversampleV - 1);
            total_surface += src_tmp.Rects[glyph_i].w * src_tmp.Rects[glyph_i].h;
        }
    }

    // We need a width for the skyline algorithm, any width!
    // The exact width doesn't really matter much, but some API/GPU have texture size limitations and increasing width can decrease height.
    // User can override TexDesiredWidth and TexGlyphPadding if they wish, otherwise we use a simple heuristic to select the width based on expected surface.
    let surface_sqrt: c_int = ImSqrt(total_surface as c_float) + 1;
    atlas.TexHeight = 0;
    if atlas.TexDesiredWidth > 0 {
        atlas.TexWidth = atlas.TexDesiredWidth;
    }
    else {
        atlas.TexWidth = if surface_sqrt >= 4096 * 0.70 { 4096 } else {
            if surface_sqrt >= 2048 * 0.70 {
                2048
            } else { if surface_sqrt >= 1024 * 0.70 { 1024 } else { 512 } }
        };
    }

    // 5. Start packing
    // Pack our extra data rectangles first, so it will be on the upper-left corner of our texture (UV will have small values).
    let TEX_HEIGHT_MAX: c_int = 1024 * 32;
    let mut spc: stbtt_pack_context = stbtt_pack_context::default();
    stbtt_PackBegin(&spc, None, atlas.TexWidth, TEX_HEIGHT_MAX, 0, atlas.TexGlyphPadding, null_mut());
    ImFontAtlasBuildPackCustomRects(atlas, spc.pack_info);

    // 6. Pack each source font. No rendering yet, we are working with rectangles in an infinitely tall texture at this point.
    // for (let src_i: c_int = 0; src_i < src_tmp_array.Size; src_i++)
    for src_i in 0 .. src_tmp_array.len()
    {
        let mut src_tmp = &mut src_tmp_array[src_i];
        if src_tmp.GlyphsCount == 0 {
            continue;
        }

        stbrp_pack_rects(spc.pack_info, src_tmp.Rects, src_tmp.GlyphsCount);

        // Extend texture height and mark missing glyphs as non-packed so we won't render them.
        // FIXME: We are not handling packing failure here (would happen if we got off TEX_HEIGHT_MAX or if a single if larger than TexWidth?)
        // for (let glyph_i: c_int = 0; glyph_i < src_tmp.GlyphsCount; glyph_i++)
        for glyph_i in 0 .. src_tmp.GlyphCount
        {
            if src_tmp.Rects[glyph_i].was_packed {
                atlas.TexHeight = ImMax(atlas.TexHeight, src_tmp.Rects[glyph_i].y + src_tmp.Rects[glyph_i].h);
            }
        }
    }

    // 7. Allocate texture
    atlas.TexHeight = if flag_set(atlas.Flags, ImFontAtlasFlags_NoPowerOfTwoHeight) { (atlas.TexHeight + 1) } else { ImUpperPowerOfTwo(atlas.TexHeight) };
    atlas.TexUvScale = ImVec2::from_floats((1 / atlas.TexWidth) as c_float, (1 / atlas.TexHeight) as c_float);
    atlas.TexPixelsAlpha8 = libc::malloc(atlas.TexWidth * atlas.TexHeight);
    libc::memset(atlas.TexPixelsAlpha8, 0, atlas.TexWidth * atlas.TexHeight);
    spc.pixels = atlas.TexPixelsAlpha8;
    spc.height = atlas.TexHeight;

    // 8. Render/rasterize font characters into the texture
    // for (let src_i: c_int = 0; src_i < src_tmp_array.Size; src_i++)
    for src_i in 0 ..src_tmp_array.len()
    {
        ImFontConfig& cfg = atlas.ConfigData[src_i];
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        if src_tmp.GlyphsCount == 0 {
            continue;
        }

        stbtt_PackFontRangesRenderIntoRects(&spc, &src_tmp.FontInfo, &src_tmp.PackRange, 1, src_tmp.Rects);

        // Apply multiply operator
        if cfg.RasterizerMultiply != 1
        {
            let mut multiply_table: [c_uchar;256] = [0;256];
            ImFontAtlasBuildMultiplyCalcLookupTable(multiply_table, cfg.RasterizerMultiply);
            stbrp_rect* r = &src_tmp.Rects[0];
            // for (let glyph_i: c_int = 0; glyph_i < src_tmp.GlyphsCount; glyph_i++, r++)
            for glyph_i in 0 .. src_tmp.GlyphsCount
            {
                if r.was_packed {
                    ImFontAtlasBuildMultiplyRectAlpha8(multiply_table, atlas.TexPixelsAlpha8, r.x, r.y, r.w, r.h, atlas.TexWidth * 1);
                }
                r += 1;
            }
        }
        src_tmp.Rects= None;
    }

    // End packing
    stbtt_PackEnd(&spc);
    buf_rects.clear();

    // 9. Setup ImFont and glyphs for runtime
    // for (let src_i: c_int = 0; src_i < src_tmp_array.Size; src_i++)
    for src_i in 0 .. src_tmp_array.len()
    {
        let mut src_tmp = &mut src_tmp_array[src_i];
        if (src_tmp.GlyphsCount == 0) {
            continue;
        }

        // When merging fonts with MergeMode=true:
        // - We can have multiple input fonts writing into a same destination font.
        // - dst_font.ConfigData is != from cfg which is our source configuration.
        let mut cfg = &mut atlas.ConfigData[src_i];
        let mut dst_font = cfg.DstFont;

        let font_scale: c_float =  stbtt_ScaleForPixelHeight(&src_tmp.FontInfo, cfg.SizePixels);
        // unscaled_ascent: c_int, unscaled_descent, unscaled_line_gap;
        let mut unscaled_ascent: c_int = 0;
        let mut unscaled_descent: c_int = 0;
        let mut unscaled_line_gap: c_int = 0;
        stbtt_GetFontVMetrics(&src_tmp.FontInfo, &unscaled_ascent, &unscaled_descent, &unscaled_line_gap);

        let ascent: c_float =  ImFloor(unscaled_ascent * font_scale + (if unscaled_ascent > 0 { 1 } else { -1 }));
        let descent: c_float =  ImFloor(unscaled_descent * font_scale + (if unscaled_descent > 0 { 1 }else { -1 }));
        ImFontAtlasBuildSetupFont(atlas, dst_font, cfg, ascent, descent);
        let font_off_x: c_float =  cfg.GlyphOffset.x;
        let font_off_y: c_float =  cfg.GlyphOffset.y + IM_ROUND(dst_font.Ascent);

        // for (let glyph_i: c_int = 0; glyph_i < src_tmp.GlyphsCount; glyph_i++)
        for glyph_i in 0 .. src_tmp.GlyphsCount
        {
            // Register glyph
            let codepoint: c_int = src_tmp.GlyphsList[glyph_i];
            let mut pc: &mut stbtt_packedchar = &mut src_tmp.PackedChars[glyph_i];
            let mut q: stbtt_aligned_quad = stbtt_aligned_quad::default();
            let mut unused_x: c_float =  0.0;
            let mut unused_y: c_float = 0.0;
            stbtt_GetPackedQuad(src_tmp.PackedChars, atlas.TexWidth, atlas.TexHeight, glyph_i, &unused_x, &unused_y, &q, 0);
            dst_font.AddGlyph(cfg, codepoint as ImWchar, q.x0 + font_off_x, q.y0 + font_off_y, q.x1 + font_off_x, q.y1 + font_off_y, q.s0, q.t0, q.s1, q.t1, pc.xadvance);
        }
    }

    // Cleanup
    src_tmp_array.clear_destruct();

    ImFontAtlasBuildFinish(atlas);
    return true;
}

pub fn ImFontAtlasGetBuilderForStbTruetype() -> *const ImFontBuilderIO {
    // static ImFontBuilderIO io;
    let mut io: ImFontBuildIO = ImFontBuilderIO::default();
    io.FontBuilder_Build = ImFontAtlasBuildWithStbTruetype;
    return &io;
}

pub fn ImFontAtlasBuildSetupFont(atlas: *mut ImFontAtlas, mut font: *mut ImFont, font_config: *mut ImFontConfig, ascent: c_float, descent: c_float)
{
    if !font_config.MergeMode
    {
        font.ClearOutputData();
        font.FontSize = font_config.SizePixels as c_float;
        font.ConfigData = font_config;
        font.ConfigDataCount = 0;
        font.ContainerAtlas = atlas;
        font.Ascent = ascent;
        font.Descent = descent;
    }
    font.ConfigDataCount+= 1;
}

pub fn ImFontAtlasBuildPackCustomRects(
    atlas: *mut ImFontAtlas,
    stbrp_context_opaque: *mut c_void)
{
    let mut pack_context: *mut stbrp_context = stbrp_context_opaque;
    // IM_ASSERT(pack_context != NULL);

    let mut user_rects = &mut atlas.CustomRects;
    // IM_ASSERT(user_rects.Size >= 1); // We expect at least the default custom rects to be registered, else something went wrong.

    let mut pack_rects: Vec<stbrp_rect>  = vec![];
    pack_rects.resize_with(user_rects.len(), Default::default());
    // memset(pack_rects.Data, 0, pack_rects.size_in_bytes());
    // for (let i: c_int = 0; i < user_rects.Size; i++)
    for i in 0 .. user_rects.len()
    {
        pack_rects[i].w = user_rects[i].Width;
        pack_rects[i].h = user_rects[i].Height;
    }
    stbrp_pack_rects(pack_context, &pack_rects[0], pack_rects.Size);
    // for (let i: c_int = 0; i < pack_rects.Size; i++)
    for i in 0 .. pack_rects.len()
    {
        if pack_rects[i].was_packed {
            user_rects[i].X = pack_rects[i].x;
            user_rects[i].Y = pack_rects[i].y;
            // IM_ASSERT(pack_rects[i].w == user_rects[i].Width && pack_rects[i].h == user_rects[i].Height);
            atlas.TexHeight = ImMax(atlas.TexHeight, pack_rects[i].y + pack_rects[i].h);
        }
    }
}

pub fn ImFontAtlasBuildRender8bppRectFromString(
    atlas: *mut ImFontAtlas,
    x: c_ushort,
    y: c_ushort,
    w: size_t,
    h: size_t,
    mut in_str: *const c_char,
    in_marker_char: char,
    in_marker_pixel_value: c_uchar) {
    // IM_ASSERT(x >= 0 && x + w <= atlas.TexWidth);
    // IM_ASSERT(y >= 0 && y + h <= atlas.TexHeight);
    let mut out_pixel: *mut c_uchar = atlas.TexPixelsAlpha8 + x + (y * atlas.TexWidth);
    // for (let off_y: c_int = 0; off_y < h; off_y++, out_pixel += atlas.TexWidth, in_str += w)
    let mut off_y = 0;
    while off_y < h
    {
        // for (let off_x: c_int = 0; off_x < w; off_x+ +)
        for off_x in 0 .. w
        {
            out_pixel[off_x] = if in_str[off_x] == in_marker_char {
                in_marker_pixel_value
            } else { 0x00 };
        }
        off_y += 1;
        out_pixel += atlas.TexWidth;
        in_str += w;
    }
}

pub fn ImFontAtlasBuildRender32bppRectFromString(atlas: *mut ImFontAtlas, x: c_int, y: c_ushort, w: size_t, h: size_t, mut in_str: *const c_char, in_marker_char: char, in_marker_pixel_value: c_uint) {
    // IM_ASSERT(x >= 0 && x + w <= atlas.TexWidth);
    // IM_ASSERT(y >= 0 && y + h <= atlas.TexHeight);
    let mut out_pixel: *mut c_uint = atlas.TexPixelsRGBA32 + x + (y * atlas.TexWidth);
    // for (let off_y: c_int = 0; off_y < h; off_y++, out_pixel += atlas.TexWidth, in_str += w)
    let mut off_y = 0;
    while off_y < h {
        // for (let off_x: c_int = 0; off_x < w; off_x+ +)
        for off_x in 0..w {
            out_pixel[off_x] = if in_str[off_x] == in_marker_char {
                in_marker_pixel_value
            } else { IM_COL32_BLACK_TRANS };
        }
        off_y += 1;
        out_pixel += atlas.TexWidth;
        in_str += w;
    }
}

pub unsafe fn ImFontAtlasBuildRenderDefaultTexData(mut atlas: *mut ImFontAtlas)
{
   let mut r: *mut ImFontAtlasCustomRect = atlas.GetCustomRectByIndex(atlas.PackIdMouseCursors);
    // IM_ASSERT(r.IsPacked());

    let w: size_t = atlas.TexWidth;
    if flag_clear(atlas.Flags & ImFontAtlasFlags_NoMouseCursors, 0)
    {
        // Render/copy pixels
        // IM_ASSERT(r.Width == FONT_ATLAS_DEFAULT_TEX_DATA_W * 2 + 1 && r.Height == FONT_ATLAS_DEFAULT_TEX_DATA_H);
        let x_for_white = r.X;
        let x_for_black = r.X + FONT_ATLAS_DEFAULT_TEX_DATA_W + 1;
        if atlas.TexPixelsAlpha8 != None
        {
            ImFontAtlasBuildRender8bppRectFromString(atlas, x_for_white, r.Y, FONT_ATLAS_DEFAULT_TEX_DATA_W, FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS_as_const_char_ptr(), '.', 0xF0);
            ImFontAtlasBuildRender8bppRectFromString(atlas, x_for_black, r.Y, FONT_ATLAS_DEFAULT_TEX_DATA_W, FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS_as_const_char_ptr(), 'X', 0xF0);
        }
        else
        {
            ImFontAtlasBuildRender32bppRectFromString(atlas, x_for_white as c_int, r.Y, FONT_ATLAS_DEFAULT_TEX_DATA_W, FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS_as_const_char_ptr(), '.', IM_COL32_WHITE);
            ImFontAtlasBuildRender32bppRectFromString(atlas, x_for_black as c_int, r.Y, FONT_ATLAS_DEFAULT_TEX_DATA_W, FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS_as_const_char_ptr(), 'X', IM_COL32_WHITE);
        }
    }
    else
    {
        // Render 4 white pixels
        // IM_ASSERT(r.Width == 2 && r.Height == 2);
        let offset = r.X + r.Y * w;
        if atlas.TexPixelsAlpha8 != None
        {
            atlas.TexPixelsAlpha8[offset] = atlas.TexPixelsAlpha8[offset + 1] = atlas.TexPixelsAlpha8[offset + w] = atlas.TexPixelsAlpha8[offset + w + 1] = 0xFF;
        }
        else
        {
            atlas.TexPixelsRGBA32[offset] = atlas.TexPixelsRGBA32[offset + 1] = atlas.TexPixelsRGBA32[offset + w] = atlas.TexPixelsRGBA32[offset + w + 1] = IM_COL32_WHITE;
        }
    }
    atlas.TexUvWhitePixel = ImVec2::from_floats((r.X + 0.5) * atlas.TexUvScale.x, (r.Y + 0.5) * atlas.TexUvScale.y);
}

pub fn ImFontAtlasBuildRenderLinesTexData(mut atlas: *mut ImFontAtlas)
{
    if atlas.Flags & ImFontAtlasFlags_NoBakedLines {
        return;
    }

    // This generates a triangular shape in the texture, with the various line widths stacked on top of each other to allow interpolation between them
    r: *mut ImFontAtlasCustomRect = atlas.GetCustomRectByIndex(atlas.PackIdLines);
    // IM_ASSERT(r.IsPacked());
    // for (let mut n: c_uint =  0; n < IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 1; n++) // +1 because of the zero-width row
    for n in 0 .. IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 1
    {
        // Each line consists of at least two empty pixels at the ends, with a line of solid pixels in the middle
        let mut y: c_uint =  n;
        let mut line_width: c_uint =  n;
        let mut pad_left: c_uint =  (r.Width - line_width) / 2;
        let mut pad_right: c_uint =  r.Width - (pad_left + line_width);

        // Write each slice
        // IM_ASSERT(pad_left + line_width + pad_right == r.Width && y < r.Height); // Make sure we're inside the texture bounds before we start writing pixels
        if atlas.TexPixelsAlpha8 != None
        {
            let mut write_ptr: *mut c_uchar = &mut atlas.TexPixelsAlpha8[r.X + ((r.Y + y) * atlas.TexWidth)];
            // for (let mut i: c_uint =  0; i < pad_left; i++)
            for i in 0 .. pad_left
            {
                *(write_ptr + i) = 0x00;
            }

            // for (let mut i: c_uint =  0; i < line_width; i++)
            for i in 0 .. line_width
            {
                *(write_ptr + pad_left + i) = 0xFF;
            }

            // for (let mut i: c_uint =  0; i < pad_right; i++)
            for i in 0 .. pad_right
            {
                *(write_ptr + pad_left + line_width + i) = 0x00;
            }
        }
        else
        {
            write_ptr: *mut c_uint = &mut atlas.TexPixelsRGBA32[r.X + ((r.Y + y) * atlas.TexWidth)];
            // for (let mut i: c_uint =  0; i < pad_left; i++)
            for i in 0 .. pad_left
            {
                *(write_ptr + i) = color_u32_from_rgba(255, 255, 255, 0);
            }

            // for (let mut i: c_uint =  0; i < line_width; i++)
            for i in 0 .. line_width
            {
                *(write_ptr + pad_left + i) = IM_COL32_WHITE;
            }

            // for (let mut i: c_uint =  0; i < pad_right; i++)
            for i in 0 .. pad_right
            {
                *(write_ptr + pad_left + line_width + i) = color_u32_from_rgba(255, 255, 255, 0);
            }
        }

        // Calculate UVs for this line
        let uv0: ImVec2 = ImVec2::from_floats((r.X + pad_left - 1), (r.Y + y)) * atlas.TexUvScale;
        let uv1: ImVec2 = ImVec2::from_floats((r.X + pad_left + line_width + 1), (r.Y + y + 1)) * atlas.TexUvScale;
        let half_v: c_float =  (uv0.y + uv1.y) * 0.5; // Calculate a constant V in the middle of the row to avoid sampling artifacts
        atlas.TexUvLines[n] = ImVec4::from_floats(uv0.x, half_v, uv1.x, half_v);
    }
}

// Note: this is called / shared by both the stb_truetype and the FreeType builder
pub fn ImFontAtlasBuildInit(mut atlas: *mut ImFontAtlas)
{
    // Register texture region for mouse cursors or standard white pixels
    if atlas.PackIdMouseCursors < 0
    {
        if flag_clear(atlas.Flags, ImFontAtlasFlags_NoMouseCursors) {
            atlas.PackIdMouseCursors = atlas.AddCustomRectRegular(FONT_ATLAS_DEFAULT_TEX_DATA_W * 2 + 1, FONT_ATLAS_DEFAULT_TEX_DATA_H);
        }
        else {
            atlas.PackIdMouseCursors = atlas.AddCustomRectRegular(2, 2);
        }
    }

    // Register texture region for thick lines
    // The +2 here is to give space for the end caps, whilst height +1 is to accommodate the fact we have a zero-width row
    if atlas.PackIdLines < 0
    {
        if flag_clear(atlas.Flags, ImFontAtlasFlags_NoBakedLines) {
            atlas.PackIdLines = atlas.AddCustomRectRegular(IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 2, IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 1);
        }
    }
}

// This is called/shared by both the stb_truetype and the FreeType builder.
pub unsafe fn ImFontAtlasBuildFinish(atlas: *mut ImFontAtlas)
{
    // Render into our custom data blocks
    // IM_ASSERT(atlas.TexPixelsAlpha8 != NULL || atlas.TexPixelsRGBA32 != NULL);
    ImFontAtlasBuildRenderDefaultTexData(atlas);
    ImFontAtlasBuildRenderLinesTexData(atlas);

    // Register custom rectangle glyphs
    // for (let i: c_int = 0; i < atlas.CustomRects.Size; i++)
    for i in 0 .. atlas.CustomRects.len()
    {
        let mut r: *mut ImFontAtlasCustomRect = &mut atlas.CustomRects[i];
        if r.Font == None || r.GlyphID == 0 {
            continue;
        }

        // Will ignore ImFontConfig settings: GlyphMinAdvanceX, GlyphMinAdvanceY, GlyphExtraSpacing, PixelSnapH
        // IM_ASSERT(r.Font->ContainerAtlas == atlas);
        // uv0: ImVec2, uv1;
        let mut uv0 = ImVec2::default();
        let mut uv1 = ImVec2::default();
        atlas.CalcCustomRectUV(r, &uv0, &uv1);
        r.Font.AddGlyph(None, r.GlyphID, r.GlyphOffset.x, r.GlyphOffset.y, r.GlyphOffset.x + r.Width, r.GlyphOffset.y + r.Height, uv0.x, uv0.y, uv1.x, uv1.y, r.GlyphAdvanceX);
    }

    // Build all fonts lookup tables
    // for (let i: c_int = 0; i < atlas.Fonts.Size; i++)
    for i in 0 .. atlas.Fonts.len()
    {
        if atlas.Fonts[i].DirtyLookupTables
        {
            atlas.Fonts[i].BuildLookupTable();
        }
    }

    atlas.TexReady = true;
}
