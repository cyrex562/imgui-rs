// dear imgui: FreeType font builder (used as a replacement for the stb_truetype builder)
// (code)

// Get the latest version at https://github.com/ocornut/imgui/tree/master/misc/freetype
// Original code by @vuhdo (Aleksei Skriabin). Improvements by @mikesart. Maintained since 2019 by @ocornut.

// CHANGELOG
// (minor and older changes stripped away, please see git history for details)
//  2021/08/23: fixed crash when FT_Render_Glyph() fails to render a glyph and returns None.
//  2021/03/05: added ImGuiFreeTypeBuilderFlags_Bitmap to load bitmap glyphs.
//  2021/03/02: set 'atlas->tex_pixels_use_colors = true' to help some backends with deciding of a prefered texture format.
//  2021/01/28: added support for color-layered glyphs via ImGuiFreeTypeBuilderFlags_LoadColor (require Freetype 2.10+).
//  2021/01/26: simplified integration by using '#define IMGUI_ENABLE_FREETYPE'.
//              renamed ImGuiFreeType::XXX flags to ImGuiFreeTypeBuilderFlags_XXX for consistency with other API. removed ImGuiFreeType::BuildFontAtlas().
//  2020/06/04: fix for rare case where FT_Get_Char_Index() succeed but FT_Load_Glyph() fails.
//  2019/02/09: added RasterizerFlags::Monochrome flag to disable font anti-aliasing (combine with ::MonoHinting for best results!)
//  2019/01/15: added support for imgui allocators + added FreeType only override function SetAllocatorFunctions().
//  2019/01/10: re-factored to match big update in STB builder. fixed texture height waste. fixed redundant glyphs when merging. support for glyph padding.
//  2018/06/08: added support for ImFontConfig::glyph_min_advance_x, glyph_max_advance_x.
//  2018/02/04: moved to main imgui repository (away from http://www.github.com/ocornut/imgui_club)
//  2018/01/22: fix for addition of ImFontAtlas::TexUvscale member.
//  2017/10/22: minor inconsequential change to match change in master (removed an unnecessary statement).
//  2017/09/26: fixes for imgui internal changes.
//  2017/08/26: cleanup, optimizations, support for ImFontConfig::RasterizerFlags, ImFontConfig::rasterizer_multiply.
//  2017/08/16: imported from https://github.com/Vuhdo/imgui_freetype into http://www.github.com/ocornut/imgui_club, updated for latest changes in ImFontAtlas, minor tweaks.

// About Gamma Correct Blending:
// - FreeType assumes blending in linear space rather than gamma space.
// - See https://www.freetype.org/freetype2/docs/reference/ft2-base_interface.html#FT_Render_Glyph
// - For correct results you need to be using sRGB and convert to linear space in the pixel shader output.
// - The default dear imgui styles will be impacted by this change (alpha values will need tweaking).

// FIXME: cfg.oversample_h, oversample_v are not supported (but perhaps not so necessary with this rasterizer).



//-------------------------------------------------------------------------
// data
//-------------------------------------------------------------------------

// Default memory allocators
// static void* ImGuiFreeTypeDefaultAllocFunc(size_t size, void* user_data) { IM_UNUSED(user_data); return IM_ALLOC(size); }
// static void  ImGuiFreeTypeDefaultFreeFunc(void* ptr, void* user_data) { IM_UNUSED(user_data); IM_FREE(ptr); }

// current memory allocators
// static void* (*GImGuiFreeTypeAllocFunc)(size_t size, void* user_data) = ImGuiFreeTypeDefaultAllocFunc;
// static void  (*GImGuiFreeTypeFreeFunc)(void* ptr, void* user_data) = ImGuiFreeTypeDefaultFreeFunc;
// static void* GImGuiFreeTypeAllocatorUserData = None;

//-------------------------------------------------------------------------
// Code
//-------------------------------------------------------------------------

// namespace
// {
    // Glyph metrics:
    // --------------
    //
    //                       xmin                     xmax
    //                        |                         |
    //                        |<-------- width -------->|
    //                        |                         |
    //              |         +-------------------------+----------------- ymax
    //              |         |    ggggggggg   ggggg    |     ^        ^
    //              |         |   g:::::::::ggg::::g    |     |        |
    //              |         |  g:::::::::::::::::g    |     |        |
    //              |         | g::::::ggggg::::::gg    |     |        |
    //              |         | g:::::g     g:::::g     |     |        |
    //    offsetX  -|-------->| g:::::g     g:::::g     |  offsetY     |
    //              |         | g:::::g     g:::::g     |     |        |
    //              |         | g::::::g    g:::::g     |     |        |
    //              |         | g:::::::ggggg:::::g     |     |        |
    //              |         |  g::::::::::::::::g     |     |      height
    //              |         |   gg::::::::::::::g     |     |        |
    //  baseline ---*---------|---- gggggggg::::::g-----*--------      |
    //            / |         |             g:::::g     |              |
    //     origin   |         | gggggg      g:::::g     |              |
    //              |         | g:::::gg   gg:::::g     |              |
    //              |         |  g::::::ggg:::::::g     |              |
    //              |         |   gg:::::::::::::g      |              |
    //              |         |     ggg::::::ggg        |              |
    //              |         |         gggggg          |              v
    //              |         +-------------------------+----------------- ymin
    //              |                                   |
    //              |------------- advanceX ----------->|

use std::collections::HashSet;
use crate::font::FontConfig;
use freetype::{Library, RenderMode};
use freetype::face::Face;
use freetype::ffi::FT_New_Memory_Face;
use crate::font::font_config::FontConfig;

// A structure that describe a glyph.
    #[derive(Default,Debug,Clone)]
    pub struct GlyphInfo
    {
        // int         Width;              // Glyph's width in pixels.
        pub width: u32,
        // int         Height;             // Glyph's height in pixels.
        pub height: u32,
        // FT_Int      OffsetX;            // The distance from the origin ("pen position") to the left of the glyph.
        pub offset_x: u32,
        // FT_Int      OffsetY;            // The distance from the origin to the top of the glyph. This is usually a value < 0.
        pub offset_y: u32,
        // float       AdvanceX;           // The distance from the origin to the origin of the next glyph. This is usually a value > 0.
        pub advance_x: f32,
        // bool        IsColored;          // The glyph is colored
        pub is_colored: bool
    }

    // font parameters and metrics.
    #[derive(Default,Debug,Clone)]
    pub struct FontInfo
    {
        // uint32_t    PixelHeight;        // size this font was generated with.
        pub pixel_height: u32,
        // float       Ascender;           // The pixel extents above the baseline in pixels (typically positive).
        pub ascender: f32,
        // float       Descender;          // The extents below the baseline in pixels (typically negative).
        pub descender: f32,
        // float       LineSpacing;        // The baseline-to-baseline distance. Note that it usually is larger than the sum of the ascender and descender taken as absolute values. There is also no guarantee that no glyphs extend above or below subsequent baselines when using this distance. Think of it as a value the designer of the font finds appropriate.
        pub line_spacing: f32,
        // float       LineGap;            // The spacing in pixels between one row's descent and the next row's ascent.
        pub line_gap: f32,
        // float       MaxAdvanceWidth;    // This field gives the maximum horizontal cursor advance for all glyphs in the font.
        pub max_advanced_width: f32,
    }

    // FreeType glyph rasterizer.
    // NB: No ctor/dtor, explicitly call Init()/Shutdown()
    #[derive(Default,Debug,Clone)]
    pub struct FreeTypeFont
    {
        // bool                    InitFont(FT_Library ft_library, const ImFontConfig& cfg, unsigned int extra_user_flags); // Initialize from an external data buffer. Doesn't copy data, and you must ensure it stays valid up to this object lifetime.
        // void                    CloseFont();
        // void                    SetPixelHeight(int pixel_height); // Change font pixel size. All following calls to RasterizeGlyph() will use this size
        // const FT_Glyph_Metrics* LoadGlyph(uint32_t in_codepoint);
        // const FT_Bitmap*        RenderGlyphAndGetInfo(GlyphInfo* out_glyph_info);
        // void                    BlitGlyph(const FT_Bitmap* ft_bitmap, uint32_t* dst, uint32_t dst_pitch, unsigned char* multiply_table = None);
        // ~FreeTypeFont()         { CloseFont(); }

        // [Internals]
        // FontInfo        Info;               // font descriptor of the current font.
        pub info: FontInfo,
        // FT_Face         Face;
        pub face: Face,
        // unsigned int    UserFlags;          // = ImFontConfig::RasterizerFlags
        pub user_flags: HashSet<RasterizerFlags>,
        // FT_Int32        LoadFlags;
        pub load_flags: i32,
        // FT_Render_Mode  RenderMode;
        pub render_mode: RenderMode,
    }

    impl FreeTypeFont {
        pub fn init_font(&mut self, ft_library: &Library, cfg: &FontConfig, extra_user_flags: u32) -> bool{
            let face = ft_library.new_memory_face(&cfg.font_data, cfg.font_no);
            //  let error = FT_New_Memory_Face(ft_library, cfg.FontData, cfg.FontDataSize, cfg.FontNo, &Face);
            if face.is_err() {
                return false;
            }
            let face_unwrapped = face.unwrap();

        // if (error != 0)
        //     return false;
        // error = FT_Select_Charmap(Face, FT_ENCODING_UNICODE);
        // if (error != 0)
        //     return false;

        // Convert to FreeType flags (NB: Bold and Oblique are processed separately)
        self.user_flags = cfg.font_builder_flags | extra_user_flags;

        LoadFlags = 0;
        if ((UserFlags & FreeTypeBuilderFlags::Bitmap) == 0)
            LoadFlags |= FT_LOAD_NO_BITMAP;

        if (UserFlags & FreeTypeBuilderFlags::NoHinting)
            LoadFlags |= FT_LOAD_NO_HINTING;
        if (UserFlags & FreeTypeBuilderFlags::NoAutoHint)
            LoadFlags |= FT_LOAD_NO_AUTOHINT;
        if (UserFlags & FreeTypeBuilderFlags::ForceAutoHint)
            LoadFlags |= FT_LOAD_FORCE_AUTOHINT;
        if (UserFlags & FreeTypeBuilderFlags::LightHinting)
            LoadFlags |= FT_LOAD_TARGET_LIGHT;
        else if (UserFlags & FreeTypeBuilderFlags::MonoHinting)
            LoadFlags |= FT_LOAD_TARGET_MONO;
        else
            LoadFlags |= FT_LOAD_TARGET_NORMAL;

        if (UserFlags & FreeTypeBuilderFlags::Monochrome)
            RenderMode = FT_RENDER_MODE_MONO;
        else
            RenderMode = FT_RENDER_MODE_NORMAL;

        if (UserFlags & FreeTypeBuilderFlags::LoadColor)
            LoadFlags |= FT_LOAD_COLOR;

        memset(&Info, 0, sizeof(Info));
        SetPixelHeight((uint32_t)cfg.sizePixels);

        return true;
        }

        pub fn close_font(&mut self) {
            if (Face)
        {
            FT_Done_Face(Face);
            Face = None;
        }
        }

        pub fn set_pixel_height(&mut self, pixel_height: i32) {
            // Vuhdo: I'm not sure how to deal with font sizes properly. As far as I understand, currently ImGui assumes that the 'pixel_height'
        // is a maximum height of an any given glyph, i.e. it's the sum of font's ascender and descender. Seems strange to me.
        // NB: FT_Set_Pixel_Sizes() doesn't seem to get us the same result.
        FT_Size_RequestRec req;
        req.type = if(UserFlags & FreeTypeBuilderFlags::Bitmap) { FT_SIZE_REQUEST_TYPE_NOMINAL }else{ FT_SIZE_REQUEST_TYPE_REAL_DIM};
        req.width = 0;
        req.height = pixel_height * 64;
        req.horiResolution = 0;
        req.vertResolution = 0;
        FT_Request_Size(Face, &req);

        // update font info
        FT_Size_Metrics metrics = Face->size.metrics;
        Info.PixelHeight = pixel_height;
        Info.Ascender = FT_CEIL(metrics.ascender);
        Info.Descender = FT_CEIL(metrics.descender);
        Info.LineSpacing = FT_CEIL(metrics.height);
        Info.LineGap = FT_CEIL(metrics.height - metrics.ascender + metrics.descender);
        Info.maxAdvanceWidth = FT_CEIL(metrics.max_advance);
        }

        pub fn load_glyph(&mut self, in_codepoint: u32) -> &GlyphMetrics {
            uint32_t glyph_index = FT_Get_Char_Index(Face, codepoint);
        if (glyph_index == 0)
            return None;

		// If this crash for you: FreeType 2.11.0 has a crash bug on some bitmap/colored fonts.
		// - https://gitlab.freedesktop.org/freetype/freetype/-/issues/1076
		// - https://github.com/ocornut/imgui/issues/4567
		// - https://github.com/ocornut/imgui/issues/4566
		// You can use FreeType 2.10, or the patched version of 2.11.0 in VcPkg, or probably any upcoming FreeType version.
        FT_Error error = FT_Load_Glyph(Face, glyph_index, LoadFlags);
        if (error)
            return None;

        // Need an outline for this to work
        FT_GlyphSlot slot = Face.glyph;
        // IM_ASSERT(slot.format == FT_GLYPH_FORMAT_OUTLINE || slot.format == FT_GLYPH_FORMAT_BITMAP);

        // Apply convenience transform (this is not picking from real "Bold"/"Italic" fonts! Merely applying FreeType helper transform. Oblique == Slanting)
        if (UserFlags & FreeTypeBuilderFlags::Bold)
            FT_GlyphSlot_Embolden(slot);
        if (UserFlags & FreeTypeBuilderFlags::Oblique)
        {
            FT_GlyphSlot_Oblique(slot);
            //FT_BBox bbox;
            //FT_Outline_Get_BBox(&slot->outline, &bbox);
            //slot->metrics.width = bbox.xMax - bbox.xMin;
            //slot->metrics.height = bbox.yMax - bbox.yMin;
        }

        return &slot.metrics;
        }

        pub fn render_glyph_and_get_info(&mut self, out_glyph_info: &mut GlyphInfo) -> &Bitmap {
            FT_GlyphSlot slot = Face.glyph;
        FT_Error error = FT_Render_Glyph(slot, RenderMode);
        if (error != 0)
            return None;

        FT_Bitmap* ft_bitmap = &Face.glyph.bitmap;
        out_glyph_info.width = ft_bitmap.width;
        out_glyph_info.Height = ft_bitmap.rows;
        out_glyph_info.OffsetX = Face.glyph.bitmap_left;
        out_glyph_info.OffsetY = -Face.glyph.bitmap_top;
        out_glyph_info.AdvanceX = FT_CEIL(slot.advance.x);
        out_glyph_info.IsColored = (ft_bitmap.pixel_mode == FT_PIXEL_MODE_BGRA);

        return ft_bitmap;
        }

        pub fn blit_glyph(&mut self, ft_bitmap: &Bitmap, dst: &mut u32, dst_pitch: u32, multiply_table: Option<&mut Vec<u8>>) {
            // IM_ASSERT(ft_bitmap != None);
        const uint32_t w = ft_bitmap.width;
        const uint32_t h = ft_bitmap.rows;
        const uint8_t* src = ft_bitmap.buffer;
        const uint32_t src_pitch = ft_bitmap.pitch;

        switch (ft_bitmap.pixel_mode)
        {
        case FT_PIXEL_MODE_GRAY: // Grayscale image, 1 byte per pixel.
            {
                if (multiply_table == None)
                {
                    for (uint32_t y = 0; y < h; y += 1, src += src_pitch, dst += dst_pitch)
                        for (uint32_t x = 0; x < w; x += 1)
                            dst[x] = IM_COL32(255, 255, 255, src[x]);
                }
                else
                {
                    for (uint32_t y = 0; y < h; y += 1, src += src_pitch, dst += dst_pitch)
                        for (uint32_t x = 0; x < w; x += 1)
                            dst[x] = IM_COL32(255, 255, 255, multiply_table[src[x]]);
                }
                break;
            }
        case FT_PIXEL_MODE_MONO: // Monochrome image, 1 bit per pixel. The bits in each byte are ordered from MSB to LSB.
            {
                uint8_t color0 = if multiply_table { multiply_table[0] }else{ 0};
                uint8_t color1 = if multiply_table { multiply_table[255] }else{ 255};
                for (uint32_t y = 0; y < h; y += 1, src += src_pitch, dst += dst_pitch)
                {
                    uint8_t bits = 0;
                    const uint8_t* bits_ptr = src;
                    for (uint32_t x = 0; x < w; x += 1, bits <<= 1)
                    {
                        if ((x & 7) == 0)
                            bits = *bits_ptr += 1;
                        dst[x] = IM_COL32(255, 255, 255, if (bits & 0x80) { color1} else {color0});
                    }
                }
                break;
            }
        case FT_PIXEL_MODE_BGRA:
            {
                // FIXME: Converting pre-multiplied alpha to straight. Doesn't smell good.
                #define DE_MULTIPLY(color, alpha) (255.0 * color / alpha + 0.5)
                if (multiply_table == None)
                {
                    for (uint32_t y = 0; y < h; y += 1, src += src_pitch, dst += dst_pitch)
                        for (uint32_t x = 0; x < w; x += 1)
                        {
                            uint8_t r = src[x * 4 + 2], g = src[x * 4 + 1], b = src[x * 4], a = src[x * 4 + 3];
                            dst[x] = IM_COL32(DE_MULTIPLY(r, a), DE_MULTIPLY(g, a), DE_MULTIPLY(b, a), a);
                        }
                }
                else
                {
                    for (uint32_t y = 0; y < h; y += 1, src += src_pitch, dst += dst_pitch)
                    {
                        for (uint32_t x = 0; x < w; x += 1)
                        {
                            uint8_t r = src[x * 4 + 2], g = src[x * 4 + 1], b = src[x * 4], a = src[x * 4 + 3];
                            dst[x] = IM_COL32(multiply_table[DE_MULTIPLY(r, a)], multiply_table[DE_MULTIPLY(g, a)], multiply_table[DE_MULTIPLY(b, a)], multiply_table[a]);
                        }
                    }
                }
                #undef DE_MULTIPLY
                break;
            }
        default:
            // IM_ASSERT(0 && "FreeTypeFont::BlitGlyph(): Unknown bitmap pixel mode!");
        }
        }
    }

    // From SDL_ttf: Handy routines for converting from fixed point
    // #define FT_CEIL(X)  (((X + 63) & -64) / 64)

    // bool FreeTypeFont::InitFont(FT_Library ft_library, const ImFontConfig& cfg, unsigned int extra_font_builder_flags)
    // {
    //     FT_Error error = FT_New_Memory_Face(ft_library, cfg.FontData, cfg.FontDataSize, cfg.FontNo, &Face);
    //     if (error != 0)
    //         return false;
    //     error = FT_Select_Charmap(Face, FT_ENCODING_UNICODE);
    //     if (error != 0)
    //         return false;
    //
    //     // Convert to FreeType flags (NB: Bold and Oblique are processed separately)
    //     UserFlags = cfg.FontBuilderFlags | extra_font_builder_flags;
    //
    //     LoadFlags = 0;
    //     if ((UserFlags & ImGuiFreeTypeBuilderFlags_Bitmap) == 0)
    //         LoadFlags |= FT_LOAD_NO_BITMAP;
    //
    //     if (UserFlags & ImGuiFreeTypeBuilderFlags_NoHinting)
    //         LoadFlags |= FT_LOAD_NO_HINTING;
    //     if (UserFlags & ImGuiFreeTypeBuilderFlags_NoAutoHint)
    //         LoadFlags |= FT_LOAD_NO_AUTOHINT;
    //     if (UserFlags & ImGuiFreeTypeBuilderFlags_ForceAutoHint)
    //         LoadFlags |= FT_LOAD_FORCE_AUTOHINT;
    //     if (UserFlags & ImGuiFreeTypeBuilderFlags_LightHinting)
    //         LoadFlags |= FT_LOAD_TARGET_LIGHT;
    //     else if (UserFlags & ImGuiFreeTypeBuilderFlags_MonoHinting)
    //         LoadFlags |= FT_LOAD_TARGET_MONO;
    //     else
    //         LoadFlags |= FT_LOAD_TARGET_NORMAL;
    //
    //     if (UserFlags & ImGuiFreeTypeBuilderFlags_Monochrome)
    //         RenderMode = FT_RENDER_MODE_MONO;
    //     else
    //         RenderMode = FT_RENDER_MODE_NORMAL;
    //
    //     if (UserFlags & ImGuiFreeTypeBuilderFlags_LoadColor)
    //         LoadFlags |= FT_LOAD_COLOR;
    //
    //     memset(&Info, 0, sizeof(Info));
    //     SetPixelHeight((uint32_t)cfg.sizePixels);
    //
    //     return true;
    // }

    // void FreeTypeFont::CloseFont()
    // {
    //     if (Face)
    //     {
    //         FT_Done_Face(Face);
    //         Face = None;
    //     }
    // }

    // void FreeTypeFont::SetPixelHeight(int pixel_height)
    // {
    //     // Vuhdo: I'm not sure how to deal with font sizes properly. As far as I understand, currently ImGui assumes that the 'pixel_height'
    //     // is a maximum height of an any given glyph, i.e. it's the sum of font's ascender and descender. Seems strange to me.
    //     // NB: FT_Set_Pixel_Sizes() doesn't seem to get us the same result.
    //     FT_Size_RequestRec req;
    //     req.type = if(UserFlags & ImGuiFreeTypeBuilderFlags_Bitmap) { FT_SIZE_REQUEST_TYPE_NOMINAL }else{ FT_SIZE_REQUEST_TYPE_REAL_DIM};
    //     req.width = 0;
    //     req.height = pixel_height * 64;
    //     req.horiResolution = 0;
    //     req.vertResolution = 0;
    //     FT_Request_Size(Face, &req);
    //
    //     // update font info
    //     FT_Size_Metrics metrics = Face->size.metrics;
    //     Info.PixelHeight = pixel_height;
    //     Info.Ascender = FT_CEIL(metrics.ascender);
    //     Info.Descender = FT_CEIL(metrics.descender);
    //     Info.LineSpacing = FT_CEIL(metrics.height);
    //     Info.LineGap = FT_CEIL(metrics.height - metrics.ascender + metrics.descender);
    //     Info.maxAdvanceWidth = FT_CEIL(metrics.max_advance);
    // }

    // const FT_Glyph_Metrics* FreeTypeFont::LoadGlyph(uint32_t codepoint)
    // {
    //     uint32_t glyph_index = FT_Get_Char_Index(Face, codepoint);
    //     if (glyph_index == 0)
    //         return None;
    //
	// 	// If this crash for you: FreeType 2.11.0 has a crash bug on some bitmap/colored fonts.
	// 	// - https://gitlab.freedesktop.org/freetype/freetype/-/issues/1076
	// 	// - https://github.com/ocornut/imgui/issues/4567
	// 	// - https://github.com/ocornut/imgui/issues/4566
	// 	// You can use FreeType 2.10, or the patched version of 2.11.0 in VcPkg, or probably any upcoming FreeType version.
    //     FT_Error error = FT_Load_Glyph(Face, glyph_index, LoadFlags);
    //     if (error)
    //         return None;
    //
    //     // Need an outline for this to work
    //     FT_GlyphSlot slot = Face.glyph;
    //     // IM_ASSERT(slot.format == FT_GLYPH_FORMAT_OUTLINE || slot.format == FT_GLYPH_FORMAT_BITMAP);
    //
    //     // Apply convenience transform (this is not picking from real "Bold"/"Italic" fonts! Merely applying FreeType helper transform. Oblique == Slanting)
    //     if (UserFlags & ImGuiFreeTypeBuilderFlags_Bold)
    //         FT_GlyphSlot_Embolden(slot);
    //     if (UserFlags & ImGuiFreeTypeBuilderFlags_Oblique)
    //     {
    //         FT_GlyphSlot_Oblique(slot);
    //         //FT_BBox bbox;
    //         //FT_Outline_Get_BBox(&slot->outline, &bbox);
    //         //slot->metrics.width = bbox.xMax - bbox.xMin;
    //         //slot->metrics.height = bbox.yMax - bbox.yMin;
    //     }
    //
    //     return &slot.metrics;
    // }

    // const FT_Bitmap* FreeTypeFont::RenderGlyphAndGetInfo(GlyphInfo* out_glyph_info)
    // {
    //     FT_GlyphSlot slot = Face.glyph;
    //     FT_Error error = FT_Render_Glyph(slot, RenderMode);
    //     if (error != 0)
    //         return None;
    //
    //     FT_Bitmap* ft_bitmap = &Face.glyph.bitmap;
    //     out_glyph_info.width = ft_bitmap.width;
    //     out_glyph_info.Height = ft_bitmap.rows;
    //     out_glyph_info.OffsetX = Face.glyph.bitmap_left;
    //     out_glyph_info.OffsetY = -Face.glyph.bitmap_top;
    //     out_glyph_info.AdvanceX = FT_CEIL(slot.advance.x);
    //     out_glyph_info.IsColored = (ft_bitmap.pixel_mode == FT_PIXEL_MODE_BGRA);
    //
    //     return ft_bitmap;
    // }

    // void FreeTypeFont::BlitGlyph(const FT_Bitmap* ft_bitmap, uint32_t* dst, uint32_t dst_pitch, unsigned char* multiply_table)
    // {
    //     // IM_ASSERT(ft_bitmap != None);
    //     const uint32_t w = ft_bitmap.width;
    //     const uint32_t h = ft_bitmap.rows;
    //     const uint8_t* src = ft_bitmap.buffer;
    //     const uint32_t src_pitch = ft_bitmap.pitch;
    //
    //     switch (ft_bitmap.pixel_mode)
    //     {
    //     case FT_PIXEL_MODE_GRAY: // Grayscale image, 1 byte per pixel.
    //         {
    //             if (multiply_table == None)
    //             {
    //                 for (uint32_t y = 0; y < h; y += 1, src += src_pitch, dst += dst_pitch)
    //                     for (uint32_t x = 0; x < w; x += 1)
    //                         dst[x] = IM_COL32(255, 255, 255, src[x]);
    //             }
    //             else
    //             {
    //                 for (uint32_t y = 0; y < h; y += 1, src += src_pitch, dst += dst_pitch)
    //                     for (uint32_t x = 0; x < w; x += 1)
    //                         dst[x] = IM_COL32(255, 255, 255, multiply_table[src[x]]);
    //             }
    //             break;
    //         }
    //     case FT_PIXEL_MODE_MONO: // Monochrome image, 1 bit per pixel. The bits in each byte are ordered from MSB to LSB.
    //         {
    //             uint8_t color0 = if multiply_table { multiply_table[0] }else{ 0};
    //             uint8_t color1 = if multiply_table { multiply_table[255] }else{ 255};
    //             for (uint32_t y = 0; y < h; y += 1, src += src_pitch, dst += dst_pitch)
    //             {
    //                 uint8_t bits = 0;
    //                 const uint8_t* bits_ptr = src;
    //                 for (uint32_t x = 0; x < w; x += 1, bits <<= 1)
    //                 {
    //                     if ((x & 7) == 0)
    //                         bits = *bits_ptr += 1;
    //                     dst[x] = IM_COL32(255, 255, 255, if (bits & 0x80) { color1} else {color0});
    //                 }
    //             }
    //             break;
    //         }
    //     case FT_PIXEL_MODE_BGRA:
    //         {
    //             // FIXME: Converting pre-multiplied alpha to straight. Doesn't smell good.
    //             #define DE_MULTIPLY(color, alpha) (255.0 * color / alpha + 0.5)
    //             if (multiply_table == None)
    //             {
    //                 for (uint32_t y = 0; y < h; y += 1, src += src_pitch, dst += dst_pitch)
    //                     for (uint32_t x = 0; x < w; x += 1)
    //                     {
    //                         uint8_t r = src[x * 4 + 2], g = src[x * 4 + 1], b = src[x * 4], a = src[x * 4 + 3];
    //                         dst[x] = IM_COL32(DE_MULTIPLY(r, a), DE_MULTIPLY(g, a), DE_MULTIPLY(b, a), a);
    //                     }
    //             }
    //             else
    //             {
    //                 for (uint32_t y = 0; y < h; y += 1, src += src_pitch, dst += dst_pitch)
    //                 {
    //                     for (uint32_t x = 0; x < w; x += 1)
    //                     {
    //                         uint8_t r = src[x * 4 + 2], g = src[x * 4 + 1], b = src[x * 4], a = src[x * 4 + 3];
    //                         dst[x] = IM_COL32(multiply_table[DE_MULTIPLY(r, a)], multiply_table[DE_MULTIPLY(g, a)], multiply_table[DE_MULTIPLY(b, a)], multiply_table[a]);
    //                     }
    //                 }
    //             }
    //             #undef DE_MULTIPLY
    //             break;
    //         }
    //     default:
    //         // IM_ASSERT(0 && "FreeTypeFont::BlitGlyph(): Unknown bitmap pixel mode!");
    //     }
    // }
// }

// #ifndef STB_RECT_PACK_IMPLEMENTATION                        // in case the user already have an implementation in the _same_ compilation unit (e.g. unity builds)
// #ifndef IMGUI_DISABLE_STB_RECT_PACK_IMPLEMENTATION
// #define STBRP_ASSERT(x)     do { // IM_ASSERT(x); } while (0)
// #define STBRP_STATIC
// #define STB_RECT_PACK_IMPLEMENTATION

// #ifdef IMGUI_STB_RECT_PACK_FILENAME
// #include IMGUI_STB_RECT_PACK_FILENAME
// #else
// #include "stb_rectpack_h.rs"




struct ImFontBuildSrcGlyphFT
{
    GlyphInfo           Info;
    uint32_t            Codepoint;
    unsigned int*       BitmapData;         // Point within one of the dst_tmp_bitmap_buffers[] array

    ImFontBuildSrcGlyphFT() { memset((void*)this, 0, sizeof(*this)); }
};

struct ImFontBuildSrcDataFT
{
    FreeTypeFont        Font;
    StbRpRect*         Rects;              // Rectangle to pack. We first fill in their size and the packer will give us their position.
    const ImWchar*      SrcRanges;          // ranges as requested by user (user is allowed to request too much, e.g. 0x0020..0xFFFF)
    int                 DstIndex;           // index into atlas->fonts[] and dst_tmp_array[]
    int                 GlyphsHighest;      // Highest requested codepoint
    int                 GlyphsCount;        // Glyph count (excluding missing glyphs and glyphs already set by an earlier source font)
    ImBitVector         GlyphsSet;          // Glyph bit map (random access, 1-bit per codepoint. This will be a maximum of 8KB)
    ImVector<ImFontBuildSrcGlyphFT>   GlyphsList;
};

// Temporary data for one destination ImFont* (multiple source fonts can be merged into one destination ImFont)
struct ImFontBuildDstDataFT
{
    int                 SrcCount;           // Number of source fonts targeting this destination font.
    int                 GlyphsHighest;
    int                 GlyphsCount;
    ImBitVector         GlyphsSet;          // This is used to resolve collision when multiple sources are merged into a same destination font.
};

bool ImFontAtlasBuildWithFreeTypeEx(FT_Library ft_library, ImFontAtlas* atlas, unsigned int extra_flags)
{
    // IM_ASSERT(atlas.ConfigData.size > 0);

    ImFontAtlasBuildInit(atlas);

    // clear atlas
    atlas.TexID = None;
    atlas.TexWidth = atlas.TexHeight = 0;
    atlas.TexUvScale = Vector2D::new(0.0, 0.0);
    atlas.TexUvWhitePixel = Vector2D::new(0.0, 0.0);
    atlas.ClearTexData();

    // Temporary storage for building
    bool src_load_color = false;
    ImVector<ImFontBuildSrcDataFT> src_tmp_array;
    ImVector<ImFontBuildDstDataFT> dst_tmp_array;
    src_tmp_array.resize(atlas.ConfigData.size);
    dst_tmp_array.resize(atlas.Fonts.size);
    memset((void*)src_tmp_array.data, 0, src_tmp_array.size_in_bytes());
    memset((void*)dst_tmp_array.data, 0, dst_tmp_array.size_in_bytes());

    // 1. Initialize font loading structure, check font data validity
    for (int src_i = 0; src_i < atlas.ConfigData.size; src_i += 1)
    {
        ImFontBuildSrcDataFT& src_tmp = src_tmp_array[src_i];
        ImFontConfig& cfg = atlas.ConfigData[src_i];
        FreeTypeFont& font_face = src_tmp.font;
        // IM_ASSERT(cfg.DstFont && (!cfg.DstFont.IsLoaded() || cfg.DstFont.container_atlas == atlas));

        // Find index from cfg.dst_font (we allow the user to set cfg.dst_font. Also it makes casual debugging nicer than when storing indices)
        src_tmp.DstIndex = -1;
        for (int output_i = 0; output_i < atlas.Fonts.size && src_tmp.DstIndex == -1; output_i += 1)
            if (cfg.DstFont == atlas.Fonts[output_i])
                src_tmp.DstIndex = output_i;
        // IM_ASSERT(src_tmp.DstIndex != -1); // cfg.dst_font not pointing within atlas->fonts[] array?
        if (src_tmp.DstIndex == -1)
            return false;

        // Load font
        if (!font_face.InitFont(ft_library, cfg, extra_flags))
            return false;

        // Measure highest codepoints
        src_load_color |= (cfg.font_builder_flags & FreeTypeBuilderFlags::LoadColor) != 0;
        ImFontBuildDstDataFT& dst_tmp = dst_tmp_array[src_tmp.DstIndex];
        src_tmp.SrcRanges = if cfg.GlyphRanges { cfg.GlyphRanges }else{ atlas.GetGlyphRangesDefault()};
        for (const ImWchar* src_range = src_tmp.SrcRanges; src_range[0] && src_range[1]; src_range += 2)
            src_tmp.GlyphsHighest = ImMax(src_tmp.GlyphsHighest, src_range[1]);
        dst_tmp.SrcCount += 1;
        dst_tmp.GlyphsHighest = ImMax(dst_tmp.GlyphsHighest, src_tmp.GlyphsHighest);
    }

    // 2. For every requested codepoint, check for their presence in the font data, and handle redundancy or overlaps between source fonts to avoid unused glyphs.
    int total_glyphs_count = 0;
    for (int src_i = 0; src_i < src_tmp_array.size; src_i += 1)
    {
        ImFontBuildSrcDataFT& src_tmp = src_tmp_array[src_i];
        ImFontBuildDstDataFT& dst_tmp = dst_tmp_array[src_tmp.DstIndex];
        src_tmp.GlyphsSet.Create(src_tmp.GlyphsHighest + 1);
        if (dst_tmp.GlyphsSet.Storage.empty())
            dst_tmp.GlyphsSet.Create(dst_tmp.GlyphsHighest + 1);

        for (const ImWchar* src_range = src_tmp.SrcRanges; src_range[0] && src_range[1]; src_range += 2)
            for (int codepoint = src_range[0]; codepoint <= src_range[1]; codepoint += 1)
            {
                if (dst_tmp.GlyphsSet.TestBit(codepoint))    // Don't overwrite existing glyphs. We could make this an option (e.g. MergeOverwrite)
                    continue;
                uint32_t glyph_index = FT_Get_Char_Index(src_tmp.font.Face, codepoint); // It is actually in the font? (FIXME-OPT: We are not storing the glyph_index..)
                if (glyph_index == 0)
                    continue;

                // Add to avail set/counters
                src_tmp.GlyphsCount += 1;
                dst_tmp.GlyphsCount += 1;
                src_tmp.GlyphsSet.SetBit(codepoint);
                dst_tmp.GlyphsSet.SetBit(codepoint);
                total_glyphs_count += 1;
            }
    }

    // 3. Unpack our bit map into a flat list (we now have all the Unicode points that we know are requested _and_ available _and_ not overlapping another)
    for (int src_i = 0; src_i < src_tmp_array.size; src_i += 1)
    {
        ImFontBuildSrcDataFT& src_tmp = src_tmp_array[src_i];
        src_tmp.GlyphsList.reserve(src_tmp.GlyphsCount);

        // IM_ASSERT(sizeof(src_tmp.GlyphsSet.Storage.data[0]) == sizeof);
        const ImU32* it_begin = src_tmp.GlyphsSet.Storage.begin();
        const ImU32* it_end = src_tmp.GlyphsSet.Storage.end();
        for (const ImU32* it = it_begin; it < it_end; it += 1)
            if (ImU32 entries_32 = *it)
                for (ImU32 bit_n = 0; bit_n < 32; bit_n += 1)
                    if (entries_32 & (1 << bit_n))
                    {
                        ImFontBuildSrcGlyphFT src_glyph;
                        src_glyph.Codepoint = (ImWchar)(((it - it_begin) << 5) + bit_n);
                        //src_glyph.GlyphIndex = 0; // FIXME-OPT: We had this info in the previous step and lost it..
                        src_tmp.GlyphsList.push_back(src_glyph);
                    }
        src_tmp.GlyphsSet.Clear();
        // IM_ASSERT(src_tmp.GlyphsList.size == src_tmp.GlyphsCount);
    }
    for (int dst_i = 0; dst_i < dst_tmp_array.size; dst_i += 1)
        dst_tmp_array[dst_i].GlyphsSet.Clear();
    dst_tmp_array.clear();

    // Allocate packing character data and flag packed characters buffer as non-packed (x0=y0=x1=y1=0)
    // (We technically don't need to zero-clear buf_rects, but let's do it for the sake of sanity)
    ImVector<StbRpRect> buf_rects;
    buf_rects.resize(total_glyphs_count);
    memset(buf_rects.data, 0, buf_rects.size_in_bytes());

    // Allocate temporary rasterization data buffers.
    // We could not find a way to retrieve accurate glyph size without rendering them.
    // (e.g. slot->metrics->width not always matching bitmap->width, especially considering the Oblique transform)
    // We allocate in chunks of 256 KB to not waste too much extra memory ahead. Hopefully users of FreeType won't find the temporary allocations.
    let BITMAP_BUFFERS_CHUNK_SIZE = 256 * 1024;
    int buf_bitmap_current_used_bytes = 0;
    ImVector<unsigned char*> buf_bitmap_buffers;
    buf_bitmap_buffers.push_back((unsigned char*)IM_ALLOC(BITMAP_BUFFERS_CHUNK_SIZE));

    // 4. Gather glyphs sizes so we can pack them in our virtual canvas.
    // 8. Render/rasterize font characters into the texture
    int total_surface = 0;
    int buf_rects_out_n = 0;
    for (int src_i = 0; src_i < src_tmp_array.size; src_i += 1)
    {
        ImFontBuildSrcDataFT& src_tmp = src_tmp_array[src_i];
        ImFontConfig& cfg = atlas.ConfigData[src_i];
        if (src_tmp.GlyphsCount == 0)
            continue;

        src_tmp.Rects = &buf_rects[buf_rects_out_n];
        buf_rects_out_n += src_tmp.GlyphsCount;

        // Compute multiply table if requested
        let multiply_enabled = (cfg.RasterizerMultiply != 1.0);
        unsigned char multiply_table[256];
        if (multiply_enabled)
            ImFontAtlasBuildMultiplyCalcLookupTable(multiply_table, cfg.RasterizerMultiply);

        // Gather the sizes of all rectangles we will need to pack
        let padding = atlas.TexGlyphPadding;
        for (int glyph_i = 0; glyph_i < src_tmp.GlyphsList.size; glyph_i += 1)
        {
            ImFontBuildSrcGlyphFT& src_glyph = src_tmp.GlyphsList[glyph_i];

            const FT_Glyph_Metrics* metrics = src_tmp.font.LoadGlyph(src_glyph.Codepoint);
            if (metrics == None)
                continue;

            // Render glyph into a bitmap (currently held by FreeType)
            const FT_Bitmap* ft_bitmap = src_tmp.font.RenderGlyphAndGetInfo(&src_glyph.Info);
            if (ft_bitmap == None)
                continue;

            // Allocate new temporary chunk if needed
            let bitmap_size_in_bytes = src_glyph.Info.width * src_glyph.Info.Height * 4;
            if (buf_bitmap_current_used_bytes + bitmap_size_in_bytes > BITMAP_BUFFERS_CHUNK_SIZE)
            {
                buf_bitmap_current_used_bytes = 0;
                buf_bitmap_buffers.push_back((unsigned char*)IM_ALLOC(BITMAP_BUFFERS_CHUNK_SIZE));
            }

            // Blit rasterized pixels to our temporary buffer and keep a pointer to it.
            src_glyph.BitmapData = (unsigned int*)(buf_bitmap_buffers.back() + buf_bitmap_current_used_bytes);
            buf_bitmap_current_used_bytes += bitmap_size_in_bytes;
            src_tmp.font.BlitGlyph(ft_bitmap, src_glyph.BitmapData, src_glyph.Info.width, multiply_enabled ? multiply_table : None);

            src_tmp.Rects[glyph_i].w = (stbrp_coord)(src_glyph.Info.width + padding);
            src_tmp.Rects[glyph_i].h = (stbrp_coord)(src_glyph.Info.Height + padding);
            total_surface += src_tmp.Rects[glyph_i].w * src_tmp.Rects[glyph_i].h;
        }
    }

    // We need a width for the skyline algorithm, any width!
    // The exact width doesn't really matter much, but some API/GPU have texture size limitations and increasing width can decrease height.
    // User can override tex_desired_width and tex_glyph_padding if they wish, otherwise we use a simple heuristic to select the width based on expected surface.
    let surface_sqrt = ImSqrt(total_surface) + 1;
    atlas.TexHeight = 0;
    if (atlas.TexDesiredWidth > 0)
        atlas.TexWidth = atlas.TexDesiredWidth;
    else
        atlas.TexWidth = if (surface_sqrt >= 4096 * 0.7) ? 4096 : (surface_sqrt >= 2048 * 0.7) ? 2048 : (surface_sqrt >= 1024 * 0.7) { 1024 }else{ 512};

    // 5. Start packing
    // Pack our extra data rectangles first, so it will be on the upper-left corner of our texture (UV will have small values).
    let TEX_HEIGHT_MAX = 1024 * 32;
    let num_nodes_for_packing_algorithm = atlas.TexWidth - atlas.TexGlyphPadding;
    ImVector<StbRpNode> pack_nodes;
    pack_nodes.resize(num_nodes_for_packing_algorithm);
    StbRpContext pack_context;
    stbrp_init_target(&pack_context, atlas.TexWidth, TEX_HEIGHT_MAX, pack_nodes.data, pack_nodes.size);
    ImFontAtlasBuildPackCustomRects(atlas, &pack_context);

    // 6. Pack each source font. No rendering yet, we are working with rectangles in an infinitely tall texture at this point.
    for (int src_i = 0; src_i < src_tmp_array.size; src_i += 1)
    {
        ImFontBuildSrcDataFT& src_tmp = src_tmp_array[src_i];
        if (src_tmp.GlyphsCount == 0)
            continue;

        stbrp_pack_rects(&pack_context, src_tmp.Rects, src_tmp.GlyphsCount);

        // Extend texture height and mark missing glyphs as non-packed so we won't render them.
        // FIXME: We are not handling packing failure here (would happen if we got off TEX_HEIGHT_MAX or if a single if larger than tex_width?)
        for (int glyph_i = 0; glyph_i < src_tmp.GlyphsCount; glyph_i += 1)
            if (src_tmp.Rects[glyph_i].was_packed)
                atlas.TexHeight = ImMax(atlas.TexHeight, src_tmp.Rects[glyph_i].y + src_tmp.Rects[glyph_i].h);
    }

    // 7. Allocate texture
    atlas.TexHeight = if (atlas.flags & ImFontAtlasFlags_NoPowerOfTwoHeight) { (atlas.TexHeight + 1) }else{ ImUpperPowerOfTwo(atlas.TexHeight)};
    atlas.TexUvScale = Vector2D::new(1.0 / atlas.TexWidth, 1.0 / atlas.TexHeight);
    if (src_load_color)
    {
        size_t tex_size = atlas.TexWidth * atlas.TexHeight * 4;
        atlas.TexPixelsRGBA32 = (unsigned int*)IM_ALLOC(tex_size);
        memset(atlas.TexPixelsRGBA32, 0, tex_size);
    }
    else
    {
        size_t tex_size = atlas.TexWidth * atlas.TexHeight * 1;
        atlas.TexPixelsAlpha8 = (unsigned char*)IM_ALLOC(tex_size);
        memset(atlas.TexPixelsAlpha8, 0, tex_size);
    }

    // 8. Copy rasterized font characters back into the main texture
    // 9. Setup ImFont and glyphs for runtime
    bool tex_use_colors = false;
    for (int src_i = 0; src_i < src_tmp_array.size; src_i += 1)
    {
        ImFontBuildSrcDataFT& src_tmp = src_tmp_array[src_i];
        if (src_tmp.GlyphsCount == 0)
            continue;

        // When merging fonts with merge_mode=true:
        // - We can have multiple input fonts writing into a same destination font.
        // - dst_font->config_data is != from cfg which is our source configuration.
        ImFontConfig& cfg = atlas.ConfigData[src_i];
        ImFont* dst_font = cfg.DstFont;

        let ascent = src_tmp.font.Info.Ascender;
        let descent = src_tmp.font.Info.Descender;
        ImFontAtlasBuildSetupFont(atlas, dst_font, &cfg, ascent, descent);
        let font_off_x = cfg.GlyphOffset.x;
        let font_off_y = cfg.GlyphOffset.y + IM_ROUND(dst_font.Ascent);

        let padding = atlas.TexGlyphPadding;
        for (int glyph_i = 0; glyph_i < src_tmp.GlyphsCount; glyph_i += 1)
        {
            ImFontBuildSrcGlyphFT& src_glyph = src_tmp.GlyphsList[glyph_i];
            StbRpRect& pack_rect = src_tmp.Rects[glyph_i];
            // IM_ASSERT(pack_rect.was_packed);
            if (pack_rect.w == 0 && pack_rect.h == 0)
                continue;

            GlyphInfo& info = src_glyph.Info;
            // IM_ASSERT(info.width + padding <= pack_rect.w);
            // IM_ASSERT(info.Height + padding <= pack_rect.h);
            let tx = pack_rect.x + padding;
            let ty = pack_rect.y + padding;

            // Register glyph
            let x0 =  info.OffsetX + font_off_x;
            let y0 =  info.OffsetY + font_off_y;
            let x1 =  x0 + info.width;
            let y1 =  y0 + info.Height;
            let u0 =  (tx) / atlas.TexWidth;
            let v0 =  (ty) / atlas.TexHeight;
            let u1 =  (tx + info.width) / atlas.TexWidth;
            let v1 =  (ty + info.Height) / atlas.TexHeight;
            dst_font.AddGlyph(&cfg, (ImWchar)src_glyph.Codepoint, x0, y0, x1, y1, u0, v0, u1, v1, info.AdvanceX);

            ImFontGlyph* dst_glyph = &dst_font.Glyphs.back();
            // IM_ASSERT(dst_glyph.Codepoint == src_glyph.Codepoint);
            if (src_glyph.Info.IsColored)
                dst_glyph.Colored = tex_use_colors = true;

            // Blit from temporary buffer to final texture
            size_t blit_src_stride = src_glyph.Info.width;
            size_t blit_dst_stride = atlas.TexWidth;
            unsigned int* blit_src = src_glyph.BitmapData;
            if (atlas.TexPixelsAlpha8 != None)
            {
                unsigned char* blit_dst = atlas.TexPixelsAlpha8 + (ty * blit_dst_stride) + tx;
                for (int y = 0; y < info.Height; y += 1, blit_dst += blit_dst_stride, blit_src += blit_src_stride)
                    for (int x = 0; x < info.width; x += 1)
                        blit_dst[x] = (unsigned char)((blit_src[x] >> IM_COL32_A_SHIFT) & 0xFF);
            }
            else
            {
                unsigned int* blit_dst = atlas.TexPixelsRGBA32 + (ty * blit_dst_stride) + tx;
                for (int y = 0; y < info.Height; y += 1, blit_dst += blit_dst_stride, blit_src += blit_src_stride)
                    for (int x = 0; x < info.width; x += 1)
                        blit_dst[x] = blit_src[x];
            }
        }

        src_tmp.Rects = None;
    }
    atlas.TexPixelsUseColors = tex_use_colors;

    // Cleanup
    for (int buf_i = 0; buf_i < buf_bitmap_buffers.size; buf_i += 1)
        IM_FREE(buf_bitmap_buffers[buf_i]);
    src_tmp_array.clear_destruct();

    ImFontAtlasBuildFinish(atlas);

    return true;
}

// FreeType memory allocation callbacks
static void* FreeType_Alloc(FT_Memory /*memory*/, long size)
{
    return GImGuiFreeTypeAllocFunc(size, GImGuiFreeTypeAllocatorUserData);
}

static void FreeType_Free(FT_Memory /*memory*/, void* block)
{
    GImGuiFreeTypeFreeFunc(block, GImGuiFreeTypeAllocatorUserData);
}

static void* FreeType_Realloc(FT_Memory /*memory*/, long cur_size, long new_size, void* block)
{
    // Implement realloc() as we don't ask user to provide it.
    if (block == None)
        return GImGuiFreeTypeAllocFunc(new_size, GImGuiFreeTypeAllocatorUserData);

    if (new_size == 0)
    {
        GImGuiFreeTypeFreeFunc(block, GImGuiFreeTypeAllocatorUserData);
        return None;
    }

    if (new_size > cur_size)
    {
        void* new_block = GImGuiFreeTypeAllocFunc(new_size, GImGuiFreeTypeAllocatorUserData);
        memcpy(new_block, block, cur_size);
        GImGuiFreeTypeFreeFunc(block, GImGuiFreeTypeAllocatorUserData);
        return new_block;
    }

    return block;
}

static bool ImFontAtlasBuildWithFreeType(ImFontAtlas* atlas)
{
    // FreeType memory management: https://www.freetype.org/freetype2/docs/design/design-4.html
    FT_MemoryRec_ memory_rec = {};
    memory_rec.user = None;
    memory_rec.alloc = &FreeType_Alloc;
    memory_rec.free = &FreeType_Free;
    memory_rec.realloc = &FreeType_Realloc;

    // https://www.freetype.org/freetype2/docs/reference/ft2-module_management.html#FT_New_Library
    FT_Library ft_library;
    FT_Error error = FT_New_Library(&memory_rec, &ft_library);
    if (error != 0)
        return false;

    // If you don't call FT_Add_Default_Modules() the rest of code may work, but FreeType won't use our custom allocator.
    FT_Add_Default_Modules(ft_library);

    bool ret = ImFontAtlasBuildWithFreeTypeEx(ft_library, atlas, atlas.font_builder_flags);
    FT_Done_Library(ft_library);

    return ret;
}

const ImFontBuilderIO* ImGuiFreeType::GetBuilderForFreeType()
{
    static ImFontBuilderIO io;
    io.FontBuilder_Build = ImFontAtlasBuildWithFreeType;
    return &io;
}

void ImGuiFreeType::SetAllocatorFunctions(void* (*alloc_func)(size_t sz, void* user_data), void (*free_func)(void* ptr, void* user_data), void* user_data)
{
    GImGuiFreeTypeAllocFunc = alloc_func;
    GImGuiFreeTypeFreeFunc = free_func;
    GImGuiFreeTypeAllocatorUserData = user_data;
}
