#![allow(non_snake_case)]

use std::ptr::{null, null_mut};
use libc::{c_char, c_float, c_int, c_uchar, c_uint, c_ushort, c_void, size_t};
use crate::color::IM_COL32;
use crate::file_ops::ImFileLoadToMemory;
use crate::font::ImFont;
use crate::font_atlas_custom_rect::ImFontAtlasCustomRect;
use crate::font_atlas_default_tex_data::{FONT_ATLAS_DEFAULT_TEX_CURSOR_DATA, FONT_ATLAS_DEFAULT_TEX_DATA_W};
use crate::font_atlas_flags::{ImFontAtlasFlags, ImFontAtlasFlags_NoMouseCursors};
use crate::font_atlas_ops::ImFontAtlasGetBuilderForStbTruetype;
use crate::font_config::ImFontConfig;
use crate::font_glyph::ImFontGlyph;
use crate::font_ops::{Decode85, UnpackAccumulativeOffsetsIntoRanges};
use crate::mouse_cursor::{ImGuiMouseCursor, ImGuiMouseCursor_COUNT, ImGuiMouseCursor_None};
use crate::stb_ops::{stb_decompress, stb_decompress_length};
use crate::string_ops::{ImFormatString, str_to_const_c_char_ptr};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::type_defs::{ImTextureID, ImWchar};
use crate::utils::is_not_null;

// Load and rasterize multiple TTF/OTF fonts into a same texture. The font atlas will build a single texture holding:
//  - One or more fonts.
//  - Custom graphics data needed to render the shapes needed by Dear ImGui.
//  - Mouse cursor shapes for software cursor rendering (unless setting 'Flags |= ImFontAtlasFlags_NoMouseCursors' in the font atlas).
// It is the user-code responsibility to setup/build the atlas, then upload the pixel data into a texture accessible by your graphics api.
//  - Optionally, call any of the AddFont*** functions. If you don't call any, the default font embedded in the code will be loaded for you.
//  - Call GetTexDataAsAlpha8() or GetTexDataAsRGBA32() to build and retrieve pixels data.
//  - Upload the pixels data into a texture within your graphics system (see imgui_impl_xxxx.cpp examples)
//  - Call SetTexID(my_tex_id); and pass the pointer/identifier to your texture in a format natural to your graphics API.
//    This value will be passed back to you during rendering to identify the texture. Read FAQ entry about for: ImTextureID more details.
// Common pitfalls:
// - If you pass a 'glyph_ranges' array to AddFont*** functions, you need to make sure that your array persist up until the
//   atlas is build (when calling GetTexData*** or Build()). We only copy the pointer, not the data.
// - Important: By default, AddFontFromMemoryTTF() takes ownership of the data. Even though we are not writing to it, we will free the pointer on destruction.
//   You can set font_cfg->FontDataOwnedByAtlas=false to keep ownership of your data and it won't be freed,
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
    pub TexDesiredWidth: size_t,
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
    pub TexWidth: size_t,
    // Texture width calculated during Build().
    pub TexHeight: size_t,
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
    pub fn new() -> Self {
        let mut out = Self::default();
        out.TexGlyphPadding = 1;
        out.PackIdMouseCursors = -1;
        out.PackIdLines = -1;
        out
    }

    // ~ImFontAtlas();


    // ImFont*           AddFont(const ImFontConfig* font_cfg);
    pub unsafe fn AddFont(&mut self, font_cfg: *const ImFontConfig) -> *mut ImFont {
        // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");
        // IM_ASSERT(font_cfg->FontData != NULL && font_cfg->FontDataSize > 0);
        // IM_ASSERT(font_cfg.SizePixels > 0);

        // Create new font
        if !font_cfg.MergeMode {
            self.Fonts.push(&mut ImFont::default());
        }
        else {}
        // IM_ASSERT(!Fonts.empty() && "Cannot use MergeMode for the first font"); // When using MergeMode make sure that a font has already been added before. You can use GetIO().Fonts.AddFontDefault() to add the default imgui font.

        self.ConfigData.push(*font_cfg);
        let mut new_font_cfg = self.ConfigData.last_mut().unwrap();
        if new_font_cfg.DstFont == null_mut() {
            new_font_cfg.DstFont = *self.Fonts.last_mut().unwrap();
        }
        if !new_font_cfg.FontDataOwnedByAtlas
        {
            new_font_cfg.FontData = libc::malloc(new_font_cfg.FontDataSize);
            new_font_cfg.FontDataOwnedByAtlas = true;
            libc::memcpy(new_font_cfg.FontData, font_cfg.FontData, new_font_cfg.FontDataSize);
        }

        if new_font_cfg.DstFont.EllipsisChar == -1 {
            new_font_cfg.DstFont . EllipsisChar = font_cfg . EllipsisChar;
        }

        // Invalidate texture
        self.TexReady = false;
        self.ClearTexData();
        return new_font_cfg.DstFont;
    }


    // ImFont*           AddFontDefault(const ImFontConfig* font_cfg = NULL);
    pub unsafe fn AddFontDefault(&mut self, font_cfg_template: *const ImFontConfig) -> *mut ImFont {
        let mut font_cfg = if font_cfg_template { *font_cfg_template } else { ImFontConfig::default() };
        if !font_cfg_template {
            font_cfg.OversampleH = 1;
            font_cfg.OversampleV = 1;
            font_cfg.PixelSnapH = true;
        }
        if font_cfg.SizePixels <= 0 {
            font_cfg.SizePixels = 13.0 * 1;
        }
        if font_cfg.Name[0] == '\0' as c_char {
            // ImFormatString(font_cfg.Name, font_cfg.Name.len(), "ProggyClean.ttf, %dpx", font_cfg.SizePixels);
        }
        font_cfg.EllipsisChar = 0x0085;
        font_cfg.GlyphOffset.y = 1 * IM_FLOOR(font_cfg.SizePixels / 13.00);  // Add +1 offset per 13 units

        let mut ttf_compressed_base85: *const c_char = self.GetDefaultCompressedFontDataTTFBase85();
        let glyph_ranges: *const ImWchar = if font_cfg.GlyphRanges != null_mut() { font_cfg.GlyphRanges } else { self.GetGlyphRangesDefault() };
        let mut font = self.AddFontFromMemoryCompressedBase85TTF(ttf_compressed_base85, font_cfg.SizePixels as c_float, &font_cfg, glyph_ranges);
        return font;
    }


    // ImFont*           AddFontFromFileTTF(const char* filename, c_float size_pixels, const ImFontConfig* font_cfg = NULL, const glyph_ranges: *mut ImWchar = NULL);
    pub unsafe fn AddFontFromFileTTF(&mut self, filename: *const c_char, size_pixels: c_float, font_cfg: *const ImFontConfig, glyph_ranges: *const ImWchar) -> *mut ImFont {
        // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");
        let mut data_size: size_t = 0;
        let data: *mut c_void = ImFileLoadToMemory(filename, str_to_const_c_char_ptr("rb"), &mut data_size, 0);
        if !data
        {
            // IM_ASSERT_USER_ERROR(0, "Could not load font file!");
            return null_mut();
        }
        let font_cfg = if is_not_null(font_cfg_template) { *font_cfg_template } else {ImFontConfig::default()};
        if font_cfg.Name[0] == '\0' as c_char
        {
            // Store a short copy of filename into into the font name for convenience
            let mut p: *const c_char = null();
            // for (p = filename + strlen(filename); p > filename && p[-1] != '/' && p[-1] != '\\'; p--)
            while p > filename && p[-1] != '/' as c_char && p[-1] != '\\' as c_char
            {
                // ImFormatString(font_cfg.Name, IM_ARRAYSIZE(font_cfg.Name), "%s, %.0px", p, size_pixels);
                p = filename + libc:strlen(filename)
            }
        }
        return self.AddFontFromMemoryTTF(data, data_size, size_pixels, &font_cfg, glyph_ranges);
    }


    // ImFont*           AddFontFromMemoryTTF(void* font_data, font_size: c_int, c_float size_pixels, const ImFontConfig* font_cfg = NULL, const glyph_ranges: *mut ImWchar = NULL); // Note: Transfer ownership of 'ttf_data' to ImFontAtlas! Will be deleted after destruction of the atlas. Set font_cfg->FontDataOwnedByAtlas=false to keep ownership of your data and it won't be freed.
    pub unsafe fn AddFontFromMemoryTTF(&mut self, font_data: *mut c_void, font_size: size_t, size_pixels: c_float, font_cfg: *const ImFontConfig, glyph_ranges: *const ImWchar) -> *mut ImFont {
        // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");
        let mut font_cfg = if is_not_null(font_cfg_template) { *font_cfg_template } else { ImFontConfig::default() };
        // IM_ASSERT(font_cfg.FontData == NULL);
        font_cfg.FontData = ttf_data;
        font_cfg.FontDataSize = ttf_size;
        font_cfg.SizePixels = if size_pixels > 0.0 { size_pixels } else { font_cfg.SizePixels } as size_t;
        if glyph_ranges {
            font_cfg.GlyphRanges = glyph_ranges;
        }
        return self.AddFont(&font_cfg);
    }


    // ImFont*           AddFontFromMemoryCompressedTTF(const void* compressed_font_data, compressed_font_size: c_int, c_float size_pixels, const ImFontConfig* font_cfg = NULL, const glyph_ranges: *mut ImWchar = NULL); // 'compressed_font_data' still owned by caller. Compress with binary_to_compressed_c.cpp.
    pub unsafe fn AddFontFromMemoryCompressedTTF(&mut self, compressed_ttf_data: *const c_void, compressed_ttf_size: size_t, size_pixels: c_float, font_cfg: *const ImFontConfig, glyph_ranges: *const ImWchar) -> *mut ImFont {
        let mut buf_decompressed_size: size_t = stb_decompress_length(compressed_ttf_data) as size_t;
        let mut buf_decompressed_data: *mut c_uchar = libc::malloc(buf_decompressed_size);
        stb_decompress(buf_decompressed_data, compressed_ttf_data, compressed_ttf_size);

        let mut font_cfg = if is_not_null(font_cfg_template) { *font_cfg_template } else { ImFontConfig::default() };
        // IM_ASSERT(font_cfg.FontData == NULL);
        font_cfg.FontDataOwnedByAtlas = true;
        return self.AddFontFromMemoryTTF(buf_decompressed_data, buf_decompressed_size, size_pixels, &font_cfg, glyph_ranges);
    }


    // ImFont*           AddFontFromMemoryCompressedBase85TTF(const char* compressed_font_data_base85, c_float size_pixels, const ImFontConfig* font_cfg = NULL, const glyph_ranges: *mut ImWchar = NULL);              // 'compressed_font_data_base85' still owned by caller. Compress with binary_to_compressed_c.cpp with -base85 parameter.
    pub unsafe fn AddFontFromMemoryCompressedBase85TTF(&mut self, compressed_ttf_data_base85: *const c_char, size_pixels: c_float, font_cfg: *const ImFontConfig, glyph_ranges: *const ImWchar) -> *mut ImFont {
        let compressed_ttf_size: size_t = ((libc::strlen(compressed_ttf_data_base85) + 4) / 5) * 4;
        let mut compressed_ttf: *mut c_void = libc::malloc(compressed_ttf_size);
        Decode85(compressed_ttf_data_base85, compressed_tt0);
        let mut font = self.AddFontFromMemoryCompressedTTF(compressed_ttf, compressed_ttf_size, size_pixels, font_cfg, glyph_ranges);
        libc::free(compressed_tt0);
        return font;
    }


    // void              ClearInputData();           // Clear input data (all ImFontConfig structures including sizes, TTF data, glyph ranges, etc.) = all the data used to build the texture and fonts.
    pub fn ClearInputData(&mut self) {
        // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");
        // for (let i: c_int = 0; i < ConfigData.Size; i++)
        for i in 0 .. self.ConfigData.len()
        {
            if is_not_null(self.ConfigData[i].FontData) && self.ConfigData[i].FontDataOwnedByAtlas {
                IM_FREE(self.ConfigData[i].FontData);
                self.ConfigData[i].FontData = null_mut();
            }
        }

        // When clearing this we lose access to the font name and other information used to build the font.
        // for (let i: c_int = 0; i < Fonts.Size; i++)
        for i in 0 .. self.Fonts.len()
        {
            if self.Fonts[i].ConfigData >= self.ConfigData.as_ptr() && self.Fonts[i].ConfigData < self.ConfigData.Data + self.ConfigData.len()
            {
                self.Fonts[i].ConfigData = null_mut();
                self.Fonts[i].ConfigDataCount = 0;
            }
        }
        self.ConfigData.clear();
        self.CustomRects.clear();
        self.PackIdMouseCursors = -1;
        self.PackIdLines = -1;
        // Important: we leave TexReady untouched

    }


    // void              ClearTexData();             // Clear output texture data (CPU side). Saves RAM once the texture has been copied to graphics memory.
    pub fn ClearTexData(&mut self) {

         // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");
        if self.TexPixelsAlpha8 {
            IM_FREE(self.TexPixelsAlpha8);
        }
        if self.TexPixelsRGBA32 {
            IM_FREE(self.TexPixelsRGBA32);
        }
        self.TexPixelsAlpha8= null_mut();
        self.TexPixelsRGBA32= null_mut();
        self.TexPixelsUseColors = false;
        // Important: we leave TexReady untouched
    }


    // void              ClearFonts();               // Clear output font data (glyphs storage, UV coordinates).
    pub fn ClearFonts(&mut self) {
        // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");
        self.Fonts.clear_delete();
        self.TexReady = false;

    }

    // void              Clear();                    // Clear all input and output.
    pub fn Clear(&mut self) {
        self.ClearInputData();
        self.ClearTexData();
        self.ClearFonts();
    }

    // Build atlas, retrieve pixel data.
    // User is in charge of copying the pixels into graphics memory (e.g. create a texture with your engine). Then store your texture handle with SetTexID().
    // The pitch is always = Width * BytesPerPixels (1 or 4)
    // Building in RGBA32 format is provided for convenience and compatibility, but note that unless you manually manipulate or copy color data into
    // the texture (e.g. when using the AddCustomRect*** api), then the RGB pixels emitted will always be white (~75% of memory/bandwidth waste.
    // bool              Build();                    // Build pixels data. This is called automatically for you by the GetTexData*** functions.
    pub unsafe fn Build(&mut self) -> bool {
        // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");

        // Default font is none are specified
        if ConfigData.Size == 0 {
            self.AddFontDefault(null());
        }

        // Select builder
        // - Note that we do not reassign to atlas->FontBuilderIO, since it is likely to point to static data which
        //   may mess with some hot-reloading schemes. If you need to assign to this (for dynamic selection) AND are
        //   using a hot-reloading scheme that messes up static data, store your own instance of ImFontBuilderIO somewhere
        //   and point to it instead of pointing directly to return value of the GetBuilderXXX functions.
        let mut builder_io: *const ImFontBuilderIO = self.FontBuilderIO;
        if builder_io == null_mut()
        {
// #ifdef IMGUI_ENABLE_FREETYPE
            builder_io = GetBuilderForFreeType();
// #elif defined(IMGUI_ENABLE_STB_TRUETYPE)
            builder_io = ImFontAtlasGetBuilderForStbTruetype();
// #else
            // IM_ASSERT(0); // Invalid Build function
// #endif
        }

        // Build
        return builder_io.FontBuilder_Build(this);
    }


    // void              GetTexDataAsAlpha8(unsigned char** out_pixels, c_int* out_width, c_int* out_height, c_int* out_bytes_per_pixel = NULL);  // 1 byte per-pixel
    pub unsafe fn GetTexDataAsAlpha8(&mut self, out_pixels: *mut *mut c_uchar, out_width: *mut size_t, out_height: *mut size_t, out_bytes_per_pixel: *mut size_t) {
        // Build atlas on demand
        if self.TexPixelsAlpha8 == null_mut() {
            self.Build();
        }

        *out_pixels = self.TexPixelsAlpha8;
        if out_width { *out_width = self.TexWidth };
        if out_height { *out_height = self.TexHeight };
        if out_bytes_per_pixel { *out_bytes_per_pixel = 1 };
    }


    // void              GetTexDataAsRGBA32(unsigned char** out_pixels, c_int* out_width, c_int* out_height, c_int* out_bytes_per_pixel = NULL);  // 4 bytes-per-pixel
    pub unsafe fn GetTexAdataAsRGBA32(&mut self, out_pixels: *mut *mut c_uint, out_width: *mut size_t, out_height: *mut size_t, out_bytes_per_pixel: *mut size_t) {
        // Convert to RGBA32 format on demand
        // Although it is likely to be the most commonly used format, our font rendering is 1 channel / 8 bpp
        if !self.TexPixelsRGBA32
        {
            let mut pixels: *mut c_uchar= null_mut();
            self.GetTexDataAsAlpha8(&mut pixels, null_mut(), null_mut(), null_mut());
            if pixels
            {
                self.TexPixelsRGBA32 = libc::malloc(self.TexWidth * self.TexHeight * 4);
                let mut src: *mut c_uchar = pixels;
                let mut dst: *mut c_uint = self.TexPixelsRGBA32;
                // for (let n: c_int = TexWidth * TexHeight; n > 0; n--)
                for n in self.TexWidth + self.TexHeight > 0
                {
                    *dst = IM_COL32(255, 255, 255, (*src) as u32);
                    dst += 1;
                    src += 1;
                }
            }
        }

        *out_pixels = self.TexPixelsRGBA32;
        if out_width { *out_width = self.TexWidth };
        if out_height { *out_height = self.TexHeight; }
        if out_bytes_per_pixel { *out_bytes_per_pixel = 4 };
    }


    // bool                        IsBuilt() const             { return Fonts.Size > 0 && TexReady; } // Bit ambiguous: used to detect when user didn't built texture but effectively we should check TexID != 0 except that would be backend dependent...
    pub fn IsBuilt(&mut self) -> bool {
        self.Fonts.len() > 0 && self.TexReady
    }


    // void                        SetTexID(id: ImTextureID)    { TexID = id; }
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
        // static const ImWchar ranges[] =
        //     {
        //         0x0020, 0x00FF, // Basic Latin + Latin Supplement
        //         0,
        //     };
        let ranges: [ImWchar;3] = [0x0020,0x00FF,0];
        return &ranges[0];
    }


    // const ImWchar*    GetGlyphRangesKorean();                 // Default + Korean characters
    pub fn GetGlyphRangesKorean(&mut self) -> *const ImWchar {
        let ranges: [ImWchar; 9] = [
            0x0020, 0x00FF, // Basic Latin + Latin Supplement
            0x3131, 0x3163, // Korean alphabets
            0xAC00, 0xD7A3, // Korean characters
            0xFFFD, 0xFFFD, // Invalid
            0,
        ];
        return &ranges[0];
    }


    // const ImWchar*    GetGlyphRangesJapanese();               // Default + Hiragana, Katakana, Half-Width, Selection of 2999 Ideographs
    pub fn GetGlyphRangesJapanese(&mut self) -> *const ImWchar {
        todo!()
    }

    // const ImWchar*    GetGlyphRangesChineseFull();            // Default + Half-Width + Japanese Hiragana/Katakana + full set of about 21000 CJK Unified Ideographs
    pub fn GetGlyphRangesChineseFull(&mut self) -> *const ImWchar {
        let ranges: [ImWchar;15] =
            [                0x0020, 0x00FF, // Basic Latin + Latin Supplement
                0x2000, 0x206F, // General Punctuation
                0x3000, 0x30FF, // CJK Symbols and Punctuations, Hiragana, Katakana
                0x31F0, 0x31FF, // Katakana Phonetic Extensions
                0xFF00, 0xFFEF, // Half-width characters
                0xFFFD, 0xFFFD, // Invalid
                0x4e00, 0x9FAF, // CJK Ideograms
                0,
            ];
        return &ranges[0];

    }

    // const ImWchar*    GetGlyphRangesChineseSimplifiedCommon();// Default + Half-Width + Japanese Hiragana/Katakana + set of 2500 CJK Unified Ideographs for common simplified Chinese
    pub unsafe fn GetGlyphRangesChineseSimplifiedCommon(&mut self) -> *const ImWchar {

        // Store 2500 regularly used characters for Simplified Chinese.
        // Sourced from https://zh.wiktionary.org/wiki/%E9%99%84%E5%BD%95:%E7%8E%B0%E4%BB%A3%E6%B1%89%E8%AF%AD%E5%B8%B8%E7%94%A8%E5%AD%97%E8%A1%A8
        // This table covers 97.97% of all characters used during the month in July, 1987.
        // You can use ImFontGlyphRangesBuilder to create your own ranges derived from this, by merging existing ranges or adding new characters.
        // (Stored as accumulative offsets from the initial unicode codepoint 0x4E00. This encoding is designed to helps us compact the source code size.)
        let accumulative_offsets_from_0x4E00: [c_ushort; 2500] = [
            0, 1, 2, 4, 1, 1, 1, 1, 2, 1, 3, 2, 1, 2, 2, 1, 1, 1, 1, 1, 5, 2, 1, 2, 3, 3, 3, 2, 2, 4, 1, 1, 1, 2, 1, 5, 2, 3, 1, 2, 1, 2, 1, 1, 2, 1, 1, 2, 2, 1, 4, 1, 1, 1, 1, 5, 10, 1, 2, 19, 2, 1, 2, 1, 2, 1, 2, 1, 2,
            1, 5, 1, 6, 3, 2, 1, 2, 2, 1, 1, 1, 4, 8, 5, 1, 1, 4, 1, 1, 3, 1, 2, 1, 5, 1, 2, 1, 1, 1, 10, 1, 1, 5, 2, 4, 6, 1, 4, 2, 2, 2, 12, 2, 1, 1, 6, 1, 1, 1, 4, 1, 1, 4, 6, 5, 1, 4, 2, 2, 4, 10, 7, 1, 1, 4, 2, 4,
            2, 1, 4, 3, 6, 10, 12, 5, 7, 2, 14, 2, 9, 1, 1, 6, 7, 10, 4, 7, 13, 1, 5, 4, 8, 4, 1, 1, 2, 28, 5, 6, 1, 1, 5, 2, 5, 20, 2, 2, 9, 8, 11, 2, 9, 17, 1, 8, 6, 8, 27, 4, 6, 9, 20, 11, 27, 6, 68, 2, 2, 1, 1,
            1, 2, 1, 2, 2, 7, 6, 11, 3, 3, 1, 1, 3, 1, 2, 1, 1, 1, 1, 1, 3, 1, 1, 8, 3, 4, 1, 5, 7, 2, 1, 4, 4, 8, 4, 2, 1, 2, 1, 1, 4, 5, 6, 3, 6, 2, 12, 3, 1, 3, 9, 2, 4, 3, 4, 1, 5, 3, 3, 1, 3, 7, 1, 5, 1, 1, 1, 1, 2,
            3, 4, 5, 2, 3, 2, 6, 1, 1, 2, 1, 7, 1, 7, 3, 4, 5, 15, 2, 2, 1, 5, 3, 22, 19, 2, 1, 1, 1, 1, 2, 5, 1, 1, 1, 6, 1, 1, 12, 8, 2, 9, 18, 22, 4, 1, 1, 5, 1, 16, 1, 2, 7, 10, 15, 1, 1, 6, 2, 4, 1, 2, 4, 1, 6,
            1, 1, 3, 2, 4, 1, 6, 4, 5, 1, 2, 1, 1, 2, 1, 10, 3, 1, 3, 2, 1, 9, 3, 2, 5, 7, 2, 19, 4, 3, 6, 1, 1, 1, 1, 1, 4, 3, 2, 1, 1, 1, 2, 5, 3, 1, 1, 1, 2, 2, 1, 1, 2, 1, 1, 2, 1, 3, 1, 1, 1, 3, 7, 1, 4, 1, 1, 2, 1,
            1, 2, 1, 2, 4, 4, 3, 8, 1, 1, 1, 2, 1, 3, 5, 1, 3, 1, 3, 4, 6, 2, 2, 14, 4, 6, 6, 11, 9, 1, 15, 3, 1, 28, 5, 2, 5, 5, 3, 1, 3, 4, 5, 4, 6, 14, 3, 2, 3, 5, 21, 2, 7, 20, 10, 1, 2, 19, 2, 4, 28, 28, 2, 3,
            2, 1, 14, 4, 1, 26, 28, 42, 12, 40, 3, 52, 79, 5, 14, 17, 3, 2, 2, 11, 3, 4, 6, 3, 1, 8, 2, 23, 4, 5, 8, 10, 4, 2, 7, 3, 5, 1, 1, 6, 3, 1, 2, 2, 2, 5, 28, 1, 1, 7, 7, 20, 5, 3, 29, 3, 17, 26, 1, 8, 4,
            27, 3, 6, 11, 23, 5, 3, 4, 6, 13, 24, 16, 6, 5, 10, 25, 35, 7, 3, 2, 3, 3, 14, 3, 6, 2, 6, 1, 4, 2, 3, 8, 2, 1, 1, 3, 3, 3, 4, 1, 1, 13, 2, 2, 4, 5, 2, 1, 14, 14, 1, 2, 2, 1, 4, 5, 2, 3, 1, 14, 3, 12,
            3, 17, 2, 16, 5, 1, 2, 1, 8, 9, 3, 19, 4, 2, 2, 4, 17, 25, 21, 20, 28, 75, 1, 10, 29, 103, 4, 1, 2, 1, 1, 4, 2, 4, 1, 2, 3, 24, 2, 2, 2, 1, 1, 2, 1, 3, 8, 1, 1, 1, 2, 1, 1, 3, 1, 1, 1, 6, 1, 5, 3, 1, 1,
            1, 3, 4, 1, 1, 5, 2, 1, 5, 6, 13, 9, 16, 1, 1, 1, 1, 3, 2, 3, 2, 4, 5, 2, 5, 2, 2, 3, 7, 13, 7, 2, 2, 1, 1, 1, 1, 2, 3, 3, 2, 1, 6, 4, 9, 2, 1, 14, 2, 14, 2, 1, 18, 3, 4, 14, 4, 11, 41, 15, 23, 15, 23,
            176, 1, 3, 4, 1, 1, 1, 1, 5, 3, 1, 2, 3, 7, 3, 1, 1, 2, 1, 2, 4, 4, 6, 2, 4, 1, 9, 7, 1, 10, 5, 8, 16, 29, 1, 1, 2, 2, 3, 1, 3, 5, 2, 4, 5, 4, 1, 1, 2, 2, 3, 3, 7, 1, 6, 10, 1, 17, 1, 44, 4, 6, 2, 1, 1, 6,
            5, 4, 2, 10, 1, 6, 9, 2, 8, 1, 24, 1, 2, 13, 7, 8, 8, 2, 1, 4, 1, 3, 1, 3, 3, 5, 2, 5, 10, 9, 4, 9, 12, 2, 1, 6, 1, 10, 1, 1, 7, 7, 4, 10, 8, 3, 1, 13, 4, 3, 1, 6, 1, 3, 5, 2, 1, 2, 17, 16, 5, 2, 16, 6,
            1, 4, 2, 1, 3, 3, 6, 8, 5, 11, 11, 1, 3, 3, 2, 4, 6, 10, 9, 5, 7, 4, 7, 4, 7, 1, 1, 4, 2, 1, 3, 6, 8, 7, 1, 6, 11, 5, 5, 3, 24, 9, 4, 2, 7, 13, 5, 1, 8, 82, 16, 61, 1, 1, 1, 4, 2, 2, 16, 10, 3, 8, 1, 1,
            6, 4, 2, 1, 3, 1, 1, 1, 4, 3, 8, 4, 2, 2, 1, 1, 1, 1, 1, 6, 3, 5, 1, 1, 4, 6, 9, 2, 1, 1, 1, 2, 1, 7, 2, 1, 6, 1, 5, 4, 4, 3, 1, 8, 1, 3, 3, 1, 3, 2, 2, 2, 2, 3, 1, 6, 1, 2, 1, 2, 1, 3, 7, 1, 8, 2, 1, 2, 1, 5,
            2, 5, 3, 5, 10, 1, 2, 1, 1, 3, 2, 5, 11, 3, 9, 3, 5, 1, 1, 5, 9, 1, 2, 1, 5, 7, 9, 9, 8, 1, 3, 3, 3, 6, 8, 2, 3, 2, 1, 1, 32, 6, 1, 2, 15, 9, 3, 7, 13, 1, 3, 10, 13, 2, 14, 1, 13, 10, 2, 1, 3, 10, 4, 15,
            2, 15, 15, 10, 1, 3, 9, 6, 9, 32, 25, 26, 47, 7, 3, 2, 3, 1, 6, 3, 4, 3, 2, 8, 5, 4, 1, 9, 4, 2, 2, 19, 10, 6, 2, 3, 8, 1, 2, 2, 4, 2, 1, 9, 4, 4, 4, 6, 4, 8, 9, 2, 3, 1, 1, 1, 1, 3, 5, 5, 1, 3, 8, 4, 6,
            2, 1, 4, 12, 1, 5, 3, 7, 13, 2, 5, 8, 1, 6, 1, 2, 5, 14, 6, 1, 5, 2, 4, 8, 15, 5, 1, 23, 6, 62, 2, 10, 1, 1, 8, 1, 2, 2, 10, 4, 2, 2, 9, 2, 1, 1, 3, 2, 3, 1, 5, 3, 3, 2, 1, 3, 8, 1, 1, 1, 11, 3, 1, 1, 4,
            3, 7, 1, 14, 1, 2, 3, 12, 5, 2, 5, 1, 6, 7, 5, 7, 14, 11, 1, 3, 1, 8, 9, 12, 2, 1, 11, 8, 4, 4, 2, 6, 10, 9, 13, 1, 1, 3, 1, 5, 1, 3, 2, 4, 4, 1, 18, 2, 3, 14, 11, 4, 29, 4, 2, 7, 1, 3, 13, 9, 2, 2, 5,
            3, 5, 20, 7, 16, 8, 5, 72, 34, 6, 4, 22, 12, 12, 28, 45, 36, 9, 7, 39, 9, 191, 1, 1, 1, 4, 11, 8, 4, 9, 2, 3, 22, 1, 1, 1, 1, 4, 17, 1, 7, 7, 1, 11, 31, 10, 2, 4, 8, 2, 3, 2, 1, 4, 2, 16, 4, 32, 2,
            3, 19, 13, 4, 9, 1, 5, 2, 14, 8, 1, 1, 3, 6, 19, 6, 5, 1, 16, 6, 2, 10, 8, 5, 1, 2, 3, 1, 5, 5, 1, 11, 6, 6, 1, 3, 3, 2, 6, 3, 8, 1, 1, 4, 10, 7, 5, 7, 7, 5, 8, 9, 2, 1, 3, 4, 1, 1, 3, 1, 3, 3, 2, 6, 16,
            1, 4, 6, 3, 1, 10, 6, 1, 3, 15, 2, 9, 2, 10, 25, 13, 9, 16, 6, 2, 2, 10, 11, 4, 3, 9, 1, 2, 6, 6, 5, 4, 30, 40, 1, 10, 7, 12, 14, 33, 6, 3, 6, 7, 3, 1, 3, 1, 11, 14, 4, 9, 5, 12, 11, 49, 18, 51, 31,
            140, 31, 2, 2, 1, 5, 1, 8, 1, 10, 1, 4, 4, 3, 24, 1, 10, 1, 3, 6, 6, 16, 3, 4, 5, 2, 1, 4, 2, 57, 10, 6, 22, 2, 22, 3, 7, 22, 6, 10, 11, 36, 18, 16, 33, 36, 2, 5, 5, 1, 1, 1, 4, 10, 1, 4, 13, 2, 7,
            5, 2, 9, 3, 4, 1, 7, 43, 3, 7, 3, 9, 14, 7, 9, 1, 11, 1, 1, 3, 7, 4, 18, 13, 1, 14, 1, 3, 6, 10, 73, 2, 2, 30, 6, 1, 11, 18, 19, 13, 22, 3, 46, 42, 37, 89, 7, 3, 16, 34, 2, 2, 3, 9, 1, 7, 1, 1, 1, 2,
            2, 4, 10, 7, 3, 10, 3, 9, 5, 28, 9, 2, 6, 13, 7, 3, 1, 3, 10, 2, 7, 2, 11, 3, 6, 21, 54, 85, 2, 1, 4, 2, 2, 1, 39, 3, 21, 2, 2, 5, 1, 1, 1, 4, 1, 1, 3, 4, 15, 1, 3, 2, 4, 4, 2, 3, 8, 2, 20, 1, 8, 7, 13,
            4, 1, 26, 6, 2, 9, 34, 4, 21, 52, 10, 4, 4, 1, 5, 12, 2, 11, 1, 7, 2, 30, 12, 44, 2, 30, 1, 1, 3, 6, 16, 9, 17, 39, 82, 2, 2, 24, 7, 1, 7, 3, 16, 9, 14, 44, 2, 1, 2, 1, 2, 3, 5, 2, 4, 1, 6, 7, 5, 3,
            2, 6, 1, 11, 5, 11, 2, 1, 18, 19, 8, 1, 3, 24, 29, 2, 1, 3, 5, 2, 2, 1, 13, 6, 5, 1, 46, 11, 3, 5, 1, 1, 5, 8, 2, 10, 6, 12, 6, 3, 7, 11, 2, 4, 16, 13, 2, 5, 1, 1, 2, 2, 5, 2, 28, 5, 2, 23, 10, 8, 4,
            4, 22, 39, 95, 38, 8, 14, 9, 5, 1, 13, 5, 4, 3, 13, 12, 11, 1, 9, 1, 27, 37, 2, 5, 4, 4, 63, 211, 95, 2, 2, 2, 1, 3, 5, 2, 1, 1, 2, 2, 1, 1, 1, 3, 2, 4, 1, 2, 1, 1, 5, 2, 2, 1, 1, 2, 3, 1, 3, 1, 1, 1,
            3, 1, 4, 2, 1, 3, 6, 1, 1, 3, 7, 15, 5, 3, 2, 5, 3, 9, 11, 4, 2, 22, 1, 6, 3, 8, 7, 1, 4, 28, 4, 16, 3, 3, 25, 4, 4, 27, 27, 1, 4, 1, 2, 2, 7, 1, 3, 5, 2, 28, 8, 2, 14, 1, 8, 6, 16, 25, 3, 3, 3, 14, 3,
            3, 1, 1, 2, 1, 4, 6, 3, 8, 4, 1, 1, 1, 2, 3, 6, 10, 6, 2, 3, 18, 3, 2, 5, 5, 4, 3, 1, 5, 2, 5, 4, 23, 7, 6, 12, 6, 4, 17, 11, 9, 5, 1, 1, 10, 5, 12, 1, 1, 11, 26, 33, 7, 3, 6, 1, 17, 7, 1, 5, 12, 1, 11,
            2, 4, 1, 8, 14, 17, 23, 1, 2, 1, 7, 8, 16, 11, 9, 6, 5, 2, 6, 4, 16, 2, 8, 14, 1, 11, 8, 9, 1, 1, 1, 9, 25, 4, 11, 19, 7, 2, 15, 2, 12, 8, 52, 7, 5, 19, 2, 16, 4, 36, 8, 1, 16, 8, 24, 26, 4, 6, 2, 9,
            5, 4, 36, 3, 28, 12, 25, 15, 37, 27, 17, 12, 59, 38, 5, 32, 127, 1, 2, 9, 17, 14, 4, 1, 2, 1, 1, 8, 11, 50, 4, 14, 2, 19, 16, 4, 17, 5, 4, 5, 26, 12, 45, 2, 23, 45, 104, 30, 12, 8, 3, 10, 2, 2,
            3, 3, 1, 4, 20, 7, 2, 9, 6, 15, 2, 20, 1, 3, 16, 4, 11, 15, 6, 134, 2, 5, 59, 1, 2, 2, 2, 1, 9, 17, 3, 26, 137, 10, 211, 59, 1, 2, 4, 1, 4, 1, 1, 1, 2, 6, 2, 3, 1, 1, 2, 3, 2, 3, 1, 3, 4, 4, 2, 3, 3,
            1, 4, 3, 1, 7, 2, 2, 3, 1, 2, 1, 3, 3, 3, 2, 2, 3, 2, 1, 3, 14, 6, 1, 3, 2, 9, 6, 15, 27, 9, 34, 145, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 2, 2, 3, 1, 2, 1, 1, 1, 2, 3, 5, 8, 3, 5, 2, 4, 1, 3, 2, 2, 2, 12,
            4, 1, 1, 1, 10, 4, 5, 1, 20, 4, 16, 1, 15, 9, 5, 12, 2, 9, 2, 5, 4, 2, 26, 19, 7, 1, 26, 4, 30, 12, 15, 42, 1, 6, 8, 172, 1, 1, 4, 2, 1, 1, 11, 2, 2, 4, 2, 1, 2, 1, 10, 8, 1, 2, 1, 4, 5, 1, 2, 5, 1, 8,
            4, 1, 3, 4, 2, 1, 6, 2, 1, 3, 4, 1, 2, 1, 1, 1, 1, 12, 5, 7, 2, 4, 3, 1, 1, 1, 3, 3, 6, 1, 2, 2, 3, 3, 3, 2, 1, 2, 12, 14, 11, 6, 6, 4, 12, 2, 8, 1, 7, 10, 1, 35, 7, 4, 13, 15, 4, 3, 23, 21, 28, 52, 5,
            26, 5, 6, 1, 7, 10, 2, 7, 53, 3, 2, 1, 1, 1, 2, 163, 532, 1, 10, 11, 1, 3, 3, 4, 8, 2, 8, 6, 2, 2, 23, 22, 4, 2, 2, 4, 2, 1, 3, 1, 3, 3, 5, 9, 8, 2, 1, 2, 8, 1, 10, 2, 12, 21, 20, 15, 105, 2, 3, 1, 1,
            3, 2, 3, 1, 1, 2, 5, 1, 4, 15, 11, 19, 1, 1, 1, 1, 5, 4, 5, 1, 1, 2, 5, 3, 5, 12, 1, 2, 5, 1, 11, 1, 1, 15, 9, 1, 4, 5, 3, 26, 8, 2, 1, 3, 1, 1, 15, 19, 2, 12, 1, 2, 5, 2, 7, 2, 19, 2, 20, 6, 26, 7, 5,
            2, 2, 7, 34, 21, 13, 70, 2, 128, 1, 1, 2, 1, 1, 2, 1, 1, 3, 2, 2, 2, 15, 1, 4, 1, 3, 4, 42, 10, 6, 1, 49, 85, 8, 1, 2, 1, 1, 4, 4, 2, 3, 6, 1, 5, 7, 4, 3, 211, 4, 1, 2, 1, 2, 5, 1, 2, 4, 2, 2, 6, 5, 6,
            10, 3, 4, 48, 100, 6, 2, 16, 296, 5, 27, 387, 2, 2, 3, 7, 16, 8, 5, 38, 15, 39, 21, 9, 10, 3, 7, 59, 13, 27, 21, 47, 5, 21, 6
        ];
        let base_ranges: [ImWchar; 12] = // not zero-terminated
            [
                0x0020, 0x00FF, // Basic Latin + Latin Supplement
                0x2000, 0x206F, // General Punctuation
                0x3000, 0x30FF, // CJK Symbols and Punctuations, Hiragana, Katakana
                0x31F0, 0x31FF, // Katakana Phonetic Extensions
                0xFF00, 0xFFEF, // Half-width characters
                0xFFFD, 0xFFFD  // Invalid
            ];
        let full_ranges: [ImWchar; 12 + 2500 * 2 + 1] = [0; 5013];
        if !full_ranges[0] {
            libc::memcpy(full_ranges.as_mut_ptr(), base_ranges.as_ptr(), base_ranges.len());
            UnpackAccumulativeOffsetsIntoRanges(0x4E00, accumulative_offsets_from_0x4E00.as_ptr(), accumulative_offsets_from_0x4E00.len(), full_ranges + base_ranges.len());
        }
        return &full_ranges[0];
    }

    // const ImWchar*    GetGlyphRangesCyrillic();               // Default + about 400 Cyrillic characters
    pub fn GetGlyphRangesCyrillic(&mut self) -> *const ImWchar {
        let ranges: [ImWchar; 9] = [
            0x0020, 0x00FF, // Basic Latin + Latin Supplement
            0x0400, 0x052F, // Cyrillic + Cyrillic Supplement
            0x2DE0, 0x2DFF, // Cyrillic Extended-A
            0xA640, 0xA69F, // Cyrillic Extended-B
            0,
        ];
        return &ranges[0];
    }

    // const ImWchar*    GetGlyphRangesThai();                   // Default + Thai characters
    pub fn GetGlyphRangesThai(&mut self) -> *const ImWchar {
        let ranges: [ImWchar; 7] = [
            0x0020, 0x00FF, // Basic Latin
            0x2010, 0x205E, // Punctuations
            0x0E00, 0x0E7F, // Thai
            0,
        ];
        return &ranges[0];
    }

    // const ImWchar*    GetGlyphRangesVietnamese();             // Default + Vietnamese characters
    pub fn GetGlyphRangesVietnamese(&mut self) -> *const ImWchar {

        let ranges: [ImWchar;17] =
            [
                0x0020, 0x00FF, // Basic Latin
                0x0102, 0x0103,
                0x0110, 0x0111,
                0x0128, 0x0129,
                0x0168, 0x0169,
                0x01A0, 0x01A1,
                0x01AF, 0x01B0,
                0x1EA0, 0x1EF9,
                0,
            ];
        return &ranges[0];
    }

    //-------------------------------------------
    // [BETA] Custom Rectangles/Glyphs API
    //-------------------------------------------

    // You can request arbitrary rectangles to be packed into the atlas, for your own purposes.
    // - After calling Build(), you can query the rectangle position and render your pixels.
    // - If you render colored output, set 'atlas->TexPixelsUseColors = true' as this may help some backends decide of prefered texture format.
    // - You can also request your rectangles to be mapped as font glyph (given a font + Unicode point),
    //   so you can render e.g. custom colorful icons and use them as regular glyphs.
    // - Read docs/FONTS.md for more details about using colorful icons.
    // - Note: this API may be redesigned later in order to support multi-monitor varying DPI settings.
    // c_int               AddCustomRectRegular(width: c_int, height: c_int);
    pub fn  AddCustomRectRegular(&mut self, width: size_t, height: size_t) -> c_int {
        // IM_ASSERT(width > 0 && width <= 0xFFF0);
        // IM_ASSERT(height > 0 && height <= 0xFFF0);
        let mut r = ImFontAtlasCustomRect::default();
        r.Width = width;
        r.Height = height;
        self.CustomRects.push(r);
        return self.CustomRects.Size - 1; // Return index
    }


    // c_int               AddCustomRectFontGlyph(font: *mut ImFont, ImWchar id, width: c_int, height: c_int, c_float advance_x, const offset: &mut ImVec2 = ImVec2::new(0, 0));
    pub fn AddCustomRectFontGlyph(&mut self, font: *mut ImFont, id: ImWchar, width: size_t, height: size_t, advance_x: c_float, offset: &ImVec2) -> size_t {

        // #ifdef IMGUI_USE_WCHAR32
        // IM_ASSERT(id <= IM_UNICODE_CODEPOINT_MAX);
// #endif
        // IM_ASSERT(font != NULL);
        // IM_ASSERT(width > 0 && width <= 0xFFF0);
        // IM_ASSERT(height > 0 && height <= 0xFFF0);
        // ImFontAtlasCustomRect r;
        let mut r = ImFontAtlasCustomRect::default();
        r.Width = width;
        r.Height = height;
        r.GlyphID = id;
        r.GlyphAdvanceX = advance_x;
        r.GlyphOffset = offset.clone();
        r.Font = font;
        self.CustomRects.push(r);
        return self.CustomRects.len() - 1; // Return index
    }


    // ImFontAtlasCustomRect*      GetCustomRectByIndex(index: c_int) { IM_ASSERT(index >= 0); return &CustomRects[index]; }
    pub fn GetCustomRectByIndex(&mut self, index: c_int) -> *mut ImFontAtlasCustomRect {
        self.CustomRects[index]
    }

    // [Internal]
    // void              CalcCustomRectUV(const ImFontAtlasCustomRect* rect, ImVec2* out_uv_min, ImVec2* out_uv_max) const;
    pub unsafe fn CalcUstomRectUV(&mut self, rect: *const ImFontAtlasCustomRect, out_uv_min: *mut ImVec2, out_uv_max: *mut ImVec2) {
        // IM_ASSERT(TexWidth > 0 && TexHeight > 0);   // Font atlas needs to be built before we can calculate UV coordinates
        // IM_ASSERT(rect.IsPacked());                // Make sure the rectangle has been packed
        *out_uv_min = ImVec2::from_floats(rect.X * self.TexUvScale.x, rect.Y * self.TexUvScale.y);
        *out_uv_max = ImVec2::from_floats((rect.X + rect.Width) * self.TexUvScale.x, (rect.Y + rect.Height) * self.TexUvScale.y);
    }


    // bool              GetMouseCursorTexData(ImGuiMouseCursor cursor, ImVec2* out_offset, ImVec2* out_size, out_uv_border: ImVec2[2], out_uv_fill: ImVec2[2]);
    pub unsafe fn GetMouseCursorTexData(&mut self, cursor: ImGuiMouseCursor, out_offset: *mut ImVec2, out_size: *mut ImVec2, mut out_uv_border: [ImVec2; 2], mut out_uv_fill: [ImVec2; 2]) -> bool {
        if cursor_type <= ImGuiMouseCursor_None || cursor_type >= ImGuiMouseCursor_COUNT {
            return false;
        }
        if self.Flags & ImFontAtlasFlags_NoMouseCursors {
            return false;
        }

        // IM_ASSERT(PackIdMouseCursors != -1);
        let mut r = self.GetCustomRectByIndex(self.PackIdMouseCursors);
        let mut pos: ImVec2 = FONT_ATLAS_DEFAULT_TEX_CURSOR_DATA[cursor_type][0] + ImVec2::from_floats(r.X as c_float, r.Y as c_float);
        let size: ImVec2 = FONT_ATLAS_DEFAULT_TEX_CURSOR_DATA[cursor_type][1];
        *out_size = size;
        *out_offset = FONT_ATLAS_DEFAULT_TEX_CURSOR_DATA[cursor_type][2];
        out_uv_border[0] = (pos) * self.TexUvScale;
        out_uv_border[1] = (pos + size) * self.TexUvScale;
        pos.x += FONT_ATLAS_DEFAULT_TEX_DATA_W + 1;
        out_uv_fill[0] = (pos) * self.TexUvScale;
        out_uv_fill[1] = (pos + size) * self.TexUvScale;
        return true;
    }
}
