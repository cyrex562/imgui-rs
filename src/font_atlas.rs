#![allow(non_snake_case)]

use libc::{c_char, c_float, c_int, c_uchar, c_uint, c_void};
use crate::font::ImFont;
use crate::font_atlas_custom_rect::ImFontAtlasCustomRect;
use crate::font_atlas_flags::ImFontAtlasFlags;
use crate::font_builder_io::ImFontBuilderIO;
use crate::font_config::ImFontConfig;
use crate::font_glyph::ImFontGlyph;
use crate::mouse_cursor::ImGuiMouseCursor;
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::type_defs::{ImTextureID, ImWchar};

// Load and rasterize multiple TTF/OTF fonts into a same texture. The font atlas will build a single texture holding:
//  - One or more fonts.
//  - Custom graphics data needed to render the shapes needed by Dear ImGui.
//  - Mouse cursor shapes for software cursor rendering (unless setting 'Flags |= ImFontAtlasFlags_NoMouseCursors' in the font atlas).
// It is the user-code responsibility to setup/build the atlas, then upload the pixel data into a texture accessible by your graphics api.
//  - Optionally, call any of the AddFont*** functions. If you don't call any, the default font embedded in the code will be loaded for you.
//  - Call GetTexDataAsAlpha8() or GetTexDataAsRGBA32() to build and retrieve pixels data.
//  - Upload the pixels data into a texture within your graphics system (see imgui_impl_xxxx.cpp examples)
//  - Call SetTexID(my_tex_id); and pass the pointer/identifier to your texture in a format natural to your graphics API.
//    This value will be passed back to you during rendering to identify the texture. Read FAQ entry about ImTextureID for more details.
// Common pitfalls:
// - If you pass a 'glyph_ranges' array to AddFont*** functions, you need to make sure that your array persist up until the
//   atlas is build (when calling GetTexData*** or Build()). We only copy the pointer, not the data.
// - Important: By default, AddFontFromMemoryTTF() takes ownership of the data. Even though we are not writing to it, we will free the pointer on destruction.
//   You can set font_cfg.FontDataOwnedByAtlas=false to keep ownership of your data and it won't be freed,
// - Even though many functions are suffixed with "TTF", OTF data is supported just as well.
// - This is an old API and it is currently awkward for those and and various other reasons! We will address them in the future!
#[derive(Default, Debug, Clone)]
pub struct ImFontAtlas {
    //-------------------------------------------
    // Members
    //-------------------------------------------
    pub Flags: ImFontAtlasFlags,
    // Build flags (see ImFontAtlasFlags_)
    pub TexID: ImTextureID,
    // User data to refer to the texture once it has been uploaded to user's graphic systems. It is passed back to you during rendering via the ImDrawCmd structure.
    pub TexDesiredWidth: c_int,
    // Texture width desired by user before Build(). Must be a power-of-two. If have many glyphs your graphics API have texture size restrictions you may want to increase texture width to decrease height.
    pub TexGlyphPadding: c_int,
    // Padding between glyphs within texture in pixels. Defaults to 1. If your rendering method doesn't rely on bilinear filtering you may set this to 0 (will also need to set AntiAliasedLinesUseTex = false).
    pub Locked: bool,             // Marked as Locked by NewFrame() so attempt to modify the atlas will assert.

    // [Internal]
    // NB: Access texture data via GetTexData*() calls! Which will setup a default font for you.
    pub TexReady: bool,
    // Set when texture was built matching current font input
    pub TexPixelsUseColors: bool,
    // Tell whether our texture data is known to use colors (rather than just alpha channel), in order to help backend select a format.
    pub TexPixelsAlpha8: *mut c_uchar,
    // char*              TexPixelsAlpha8;    // 1 component per pixel, each component is unsigned 8-bit. Total size = TexWidth * TexHeight
    pub TexPixelsRGBA32: *mut c_uint,
    //  unsigned c_int*               TexPixelsRGBA32;    // 4 component per pixel, each component is unsigned 8-bit. Total size = TexWidth * TexHeight * 4
    pub TexWidth: c_int,
    // Texture width calculated during Build().
    pub TexHeight: c_int,
    // Texture height calculated during Build().
    pub TexUvScale: ImVec2,
    // = (1.0f/TexWidth, 1.0f/TexHeight)
    pub TexUvWhitePixel: ImVec2,
    // Texture coordinates to a white pixel
    pub Fonts: Vec<*mut ImFont>,
    // Hold all the fonts returned by AddFont*. Fonts[0] is the default font upon calling NewFrame(), use PushFont()/PopFont() to change the current font.
    pub CustomRects: Vec<ImFontAtlasCustomRect>,
    // Rectangles for packing custom texture data into the atlas.
    pub ConfigData: Vec<ImFontConfig>,
    // Configuration data
// ImVec4                      TexUvLines[IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 1];  // UVs for baked anti-aliased lines
    pub TexUvLines: [ImVec4; IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 1],

    // [Internal] Font builder
    pub FontBuilderIO: *const ImFontBuilderIO,
    //    const ImFontBuilderIO*      FontBuilderIO;      // Opaque interface to a font builder (default to stb_truetype, can be changed to use FreeType by defining IMGUI_ENABLE_FREETYPE).
    pub FontBuilderFlags: c_uint, // unsigned c_int                FontBuilderFlags;   // Shared flags (for all fonts) for custom font builder. THIS IS BUILD IMPLEMENTATION DEPENDENT. Per-font override is also available in ImFontConfig.

    // [Internal] Packing data
    pub PackIdMouseCursors: c_int,
    // Custom texture rectangle ID for white pixel and mouse cursors
    pub PackIdLines: c_int,        // Custom texture rectangle ID for baked anti-aliased lines

    // [Obsolete]
    //typedef ImFontAtlasCustomRect    CustomRect;         // OBSOLETED in 1.72+
    //typedef ImFontGlyphRangesBuilder GlyphRangesBuilder; // OBSOLETED in 1.67+
}

impl ImFontAtlas {
    // ImFontAtlas();


    // ~ImFontAtlas();


    // ImFont*           AddFont(const ImFontConfig* font_cfg);
    pub fn AddFont(&mut self, font_cfg: *const ImFontConfig) -> *mut ImFont {
        todo!()
    }


    // ImFont*           AddFontDefault(const ImFontConfig* font_cfg = NULL);
    pub fn AddFontDefault(&mut self, font_cfg: *const ImFontConfig) -> *mut ImFont {
        todo!()
    }


    // ImFont*           AddFontFromFileTTF(const char* filename, size_pixels: c_float, const ImFontConfig* font_cfg = NULL, const ImWchar* glyph_ranges = NULL);
    pub fn AddFontFromFileTTF(&mut self, filename: *const c_char, size_pixels: c_float, font_cfg: *const ImFontConfig, glyph_ranges: *const ImWchar) -> *mut ImFont {
        todo!()
    }


    // ImFont*           AddFontFromMemoryTTF(void* font_data, font_size: c_int, size_pixels: c_float, const ImFontConfig* font_cfg = NULL, const ImWchar* glyph_ranges = NULL); // Note: Transfer ownership of 'ttf_data' to ImFontAtlas! Will be deleted after destruction of the atlas. Set font_cfg.FontDataOwnedByAtlas=false to keep ownership of your data and it won't be freed.
    pub fn AddFontFromMemoryTTF(&mut self, font_data: *mut c_void, font_size: c_int, size_pixels: c_float, font_cfg: *const ImFontConfig, glyph_ranges: *const ImWchar) -> *mut ImFont {
        todo!()
    }


    // ImFont*           AddFontFromMemoryCompressedTTF(const void* compressed_font_data, compressed_font_size: c_int, size_pixels: c_float, const ImFontConfig* font_cfg = NULL, const ImWchar* glyph_ranges = NULL); // 'compressed_font_data' still owned by caller. Compress with binary_to_compressed_c.cpp.
    pub fn AddFontFromMemoryCompressedTTF(&mut self, compressed_font_data: *const c_void, compressed_font_size: c_int, size_pixels: c_float, font_cfg: *const ImFontConfig, glyph_ranges: *const ImWchar) -> *mut ImFont {
        todo!()
    }


    // ImFont*           AddFontFromMemoryCompressedBase85TTF(const char* compressed_font_data_base85, size_pixels: c_float, const ImFontConfig* font_cfg = NULL, const ImWchar* glyph_ranges = NULL);              // 'compressed_font_data_base85' still owned by caller. Compress with binary_to_compressed_c.cpp with -base85 parameter.
    pub fn AddFontFromMemoryCompressedBase85TTF(&mut self, compressed_font_data_base85: *const c_char, size_pixels: c_float, font_cfg: *const ImFontConfig, glyph_ranges: *const ImWchar) -> *mut ImFont {
        todo!()
    }


    // void              ClearInputData();           // Clear input data (all ImFontConfig structures including sizes, TTF data, glyph ranges, etc.) = all the data used to build the texture and fonts.
    pub fn ClearInputData(&mut self) {
        todo!()
    }


    // void              ClearTexData();             // Clear output texture data (CPU side). Saves RAM once the texture has been copied to graphics memory.
    pub fn ClearTexData(&mut self) {
        todo!()
    }


    // void              ClearFonts();               // Clear output font data (glyphs storage, UV coordinates).
    pub fn ClearFonts(&mut self) {
        todo!()
    }

    // void              Clear();                    // Clear all input and output.
    pub fn Clear(&mut self) {
        todo!()
    }

    // Build atlas, retrieve pixel data.
    // User is in charge of copying the pixels into graphics memory (e.g. create a texture with your engine). Then store your texture handle with SetTexID().
    // The pitch is always = Width * BytesPerPixels (1 or 4)
    // Building in RGBA32 format is provided for convenience and compatibility, but note that unless you manually manipulate or copy color data into
    // the texture (e.g. when using the AddCustomRect*** api), then the RGB pixels emitted will always be white (~75% of memory/bandwidth waste.
    // bool              Build();                    // Build pixels data. This is called automatically for you by the GetTexData*** functions.
    pub fn Build(&mut self) -> bool {
        todo!()
    }


    // void              GetTexDataAsAlpha8(unsigned char** out_pixels, c_int* out_width, c_int* out_height, c_int* out_bytes_per_pixel = NULL);  // 1 byte per-pixel
    pub fn GetTexDataAsAlpha8(&mut self, out_pixels: *mut *mut c_uchar, out_width: *mut c_int, out_height: *mut c_int, out_bytes_per_pixel: *mut c_int) {
        todo!()
    }


    // void              GetTexDataAsRGBA32(unsigned char** out_pixels, c_int* out_width, c_int* out_height, c_int* out_bytes_per_pixel = NULL);  // 4 bytes-per-pixel
    pub fn GetTexAdataAsRGBA32(&mut self, out_pixels: *mut *mut c_uchar, out_width: *mut c_int, out_height: *mut c_int, out_bytes_per_pixel: *mut c_int) {
        todo!()
    }


    // bool                        IsBuilt() const             { return Fonts.Size > 0 && TexReady; } // Bit ambiguous: used to detect when user didn't built texture but effectively we should check TexID != 0 except that would be backend dependent...
    pub fn IsBuilt(&mut self) -> bool {
        self.Fonts.len() > 0 && self.TexReady
    }


    // void                        SetTexID(ImTextureID id)    { TexID = id; }
    pub fn SetTexID(&mut self, id: ImTextureID) {
        self.TexID = id
    }

    //-------------------------------------------
    // Glyph Ranges
    //-------------------------------------------

    // Helpers to retrieve list of common Unicode ranges (2 value per range, values are inclusive, zero-terminated list)
    // NB: Make sure that your string are UTF-8 and NOT in your local code page. In C++11, you can create UTF-8 string literal using the u8"Hello world" syntax. See FAQ for details.
    // NB: Consider using ImFontGlyphRangesBuilder to build glyph ranges from textual data.
    // const ImWchar*    GetGlyphRangesDefault();                // Basic Latin, Extended Latin
    pub fn GetGlyphRangesDefault(&mut self) -> *const ImWchar {
        todo!()
    }


    // const ImWchar*    GetGlyphRangesKorean();                 // Default + Korean characters
    pub fn GetGlyphRangesKorean(&mut self) -> *const ImWchar {
        todo!()
    }


    // const ImWchar*    GetGlyphRangesJapanese();               // Default + Hiragana, Katakana, Half-Width, Selection of 2999 Ideographs
    pub fn GetGlyphRangesJapanese(&mut self) -> *const ImWchar {
        todo!()
    }

    // const ImWchar*    GetGlyphRangesChineseFull();            // Default + Half-Width + Japanese Hiragana/Katakana + full set of about 21000 CJK Unified Ideographs
    pub fn GetGlyphRangesChineseFull(&mut self) -> *const ImWchar {
        todo!()
    }

    // const ImWchar*    GetGlyphRangesChineseSimplifiedCommon();// Default + Half-Width + Japanese Hiragana/Katakana + set of 2500 CJK Unified Ideographs for common simplified Chinese
    pub fn GetGlyphRangesChineseSimplifiedCommon(&mut self) -> *const ImWchar {
        todo!()
    }

    // const ImWchar*    GetGlyphRangesCyrillic();               // Default + about 400 Cyrillic characters
    pub fn GetGlyphRangesCyrillic(&mut self) -> *const ImWchar {
        todo!()
    }

    // const ImWchar*    GetGlyphRangesThai();                   // Default + Thai characters
    pub fn GetGlyphRangesThai(&mut self) -> *const ImWchar {
        todo!()
    }

    // const ImWchar*    GetGlyphRangesVietnamese();             // Default + Vietnamese characters
    pub fn GetGlyphRangesVietnamese(&mut self) -> *const ImWchar {
        todo!()
    }

    //-------------------------------------------
    // [BETA] Custom Rectangles/Glyphs API
    //-------------------------------------------

    // You can request arbitrary rectangles to be packed into the atlas, for your own purposes.
    // - After calling Build(), you can query the rectangle position and render your pixels.
    // - If you render colored output, set 'atlas.TexPixelsUseColors = true' as this may help some backends decide of prefered texture format.
    // - You can also request your rectangles to be mapped as font glyph (given a font + Unicode point),
    //   so you can render e.g. custom colorful icons and use them as regular glyphs.
    // - Read docs/FONTS.md for more details about using colorful icons.
    // - Note: this API may be redesigned later in order to support multi-monitor varying DPI settings.
    // c_int               AddCustomRectRegular(width: c_int, height: c_int);
    pub fn AddCustomRectRegular(&mut self, width: c_int, height: c_int) -> c_int {
        todo!()
    }


    // c_int               AddCustomRectFontGlyph(font: *mut ImFont, ImWchar id, width: c_int, height: c_int, advance_x: c_float, const ImVec2& offset = ImVec2::new2(0, 0));
    pub fn AddCustomRectFontGlyph(&mut self, font: *mut ImFont, id: ImWchar, width: c_int, height: c_int, advance_x: c_float, offset: &ImVec2) -> c_int {
        todo!()
    }


    // ImFontAtlasCustomRect*      GetCustomRectByIndex(index: c_int) { IM_ASSERT(index >= 0); return &CustomRects[index]; }
    pub fn GetCustomRectByIndex(&mut self, index: c_int) {
        self.CustomRects[index]
    }

    // [Internal]
    // void              CalcCustomRectUV(const ImFontAtlasCustomRect* rect, out_uv_min: *mut ImVec2, out_uv_max: *mut ImVec2) const;
    pub fn CalcUstomRectUV(&mut self, rect: *const ImFontAtlasCustomRect, out_uv_min: *mut ImVec2, out_uv_max: *mut ImVec2) {
        todo!()
    }


    // bool              GetMouseCursorTexData(ImGuiMouseCursor cursor, out_offset: *mut ImVec2, out_size: *mut ImVec2, ImVec2 out_uv_border[2], ImVec2 out_uv_fill[2]);
    pub fn GetMouseCursorTexData(&mut self, cursor: ImGuiMouseCursor, out_offset: *mut ImVec2, out_size: *mut ImVec2, out_uv_border: [ImVec2; 2], out_uv_fill: [ImVec2; 2]) {
        todo!()
    }
}
