use crate::types::DimgWchar;
use crate::font::{DimgFont, DimgFontConfig, ImFontBuilderIO};
use crate::input::DimgMouseCursor;
use crate::rect::DimgRect;
use crate::texture::DimgTextureId;
use crate::vec_nd::{DimgVec2D, DimgVec4};

// Load and rasterize multiple TTF/OTF fonts into a same texture. The font atlas will build a single texture holding:
//  - One or more fonts.
//  - Custom graphics data needed to render the shapes needed by Dear ImGui.
//  - Mouse cursor shapes for software cursor rendering (unless setting 'flags |= NoMouseCursors' in the font atlas).
// It is the user-code responsibility to setup/build the atlas, then upload the pixel data into a texture accessible by your graphics api.
//  - Optionally, call any of the add_font*** functions. If you don't call any, the default font embedded in the code will be loaded for you.
//  - Call GetTexDataAsAlpha8() or GetTexDataAsRGBA32() to build and retrieve pixels data.
//  - Upload the pixels data into a texture within your graphics system (see imgui_impl_xxxx.cpp examples)
//  - Call set_tex_id(my_tex_id); and pass the pointer/identifier to your texture in a format natural to your graphics API.
//    This value will be passed back to you during rendering to identify the texture. Read FAQ entry about ImTextureID for more details.
// Common pitfalls:
// - If you pass a 'glyph_ranges' array to add_font*** functions, you need to make sure that your array persist up until the
//   atlas is build (when calling GetTexData*** or build()). We only copy the pointer, not the data.
// - Important: By default, add_font_from_memory_ttf() takes ownership of the data. Even though we are not writing to it, we will free the pointer on destruction.
//   You can set font_cfg->font_data_owned_by_atlas=false to keep ownership of your data and it won't be freed,
// - Even though many functions are suffixed with "TTF", OTF data is supported just as well.
// - This is an old API and it is currently awkward for those and and various other reasons! We will address them in the future!
#[derive(Clone, Debug, Default)]
pub struct DimgFontAtlas {
    //-------------------------------------------
    // Members
    //-------------------------------------------

    pub flags: ImFontAtlasFlags,
    // ImFontAtlasFlags            flags;              // build flags (see )
    pub tex_id: DimgTextureId,
    // ImTextureID                 tex_id;              // User data to refer to the texture once it has been uploaded to user's graphic systems. It is passed back to you during rendering via the ImDrawCmd structure.
    pub tex_desired_width: i32,
    // Texture width desired by user before build(). Must be a power-of-two. If have many glyphs your graphics API have texture size restrictions you may want to increase texture width to decrease height.
    pub tex_glyph_padding: i32,
    // Padding between glyphs within texture in pixels. Defaults to 1. If your rendering method doesn't rely on bilinear filtering you may set this to 0 (will also need to set AntiAliasedLinesUseTex = false).
    pub locked: bool,             // Marked as locked by ImGui::NewFrame() so attempt to modify the atlas will assert.

    // [Internal]
    // NB: Access texture data via GetTexData*() calls! Which will setup a default font for you.
    pub tex_ready: bool,
    // Set when texture was built matching current font input
    pub tex_pixels_use_colors: bool,
    // Tell whether our texture data is known to use colors (rather than just alpha channel), in order to help backend select a format.
    pub tex_pixels_alpha8: Vec<u8>,
    // unsigned char*              tex_pixels_alpha8;    // 1 component per pixel, each component is unsigned 8-bit. Total size = tex_width * tex_height
    pub tex_pixels_rgba32: Vec<u32>,
    // unsigned int*               tex_pixels_rgba32;    // 4 component per pixel, each component is unsigned 8-bit. Total size = tex_width * tex_height * 4
    pub tex_width: i32,
    // Texture width calculated during build().
    pub tex_height: i32,
    // Texture height calculated during build().
    pub tex_uv_scale: DimgVec2D,
    // = (1.0/tex_width, 1.0/tex_height)
    pub tex_uv_white_pixel: DimgVec2D,
    // Texture coordinates to a white pixel
    pub fonts: Vec<DimgFont>,
    // ImVector<ImFont*>           fonts;              // Hold all the fonts returned by add_font*. fonts[0] is the default font upon calling ImGui::NewFrame(), use ImGui::PushFont()/PopFont() to change the current font.
    // ImVector<ImFontAtlasCustomRect> custom_rects;    // Rectangles for packing custom texture data into the atlas.
    pub custom_rects: Vec<ImFontAtlasCustomRect>,
    // ImVector<ImFontConfig>      config_data;         // Configuration data
    pub config_data: Vec<DimgFontConfig>,
    // ImVec4                      tex_uv_lines[IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 1];  // UVs for baked anti-aliased lines
    pub tex_uv_lines: Vec<DimgVec4>,
    // [Internal] font builder
    // const ImFontBuilderIO*      font_builder_io;      // Opaque interface to a font builder (default to stb_truetype, can be changed to use FreeType by defining IMGUI_ENABLE_FREETYPE).
    pub font_builder_io: ImFontBuilderIO,
    // unsigned pub font_builder_flags: i32, // Shared flags (for all fonts) for custom font builder. THIS IS BUILD IMPLEMENTATION DEPENDENT. Per-font override is also available in ImFontConfig.
    pub font_builder_flags: i32,

    // [Internal] Packing data
    // int                         pack_id_mouse_cursors; // Custom texture rectangle id for white pixel and mouse cursors
    pub pack_id_mouse_cursors: i32,
    pub pack_id_lines: i32,      // Custom texture rectangle id for baked anti-aliased lines

    // [Obsolete]
    //typedef ImFontAtlasCustomRect    CustomRect;         // OBSOLETED in 1.72+
    //typedef ImFontGlyphRangesBuilder GlyphRangesBuilder; // OBSOLETED in 1.67+
}

impl DimgFontAtlas {
    //  ImFontAtlas();
    //      ~ImFontAtlas();
    //      ImFont*           add_font(const ImFontConfig* font_cfg);
    pub fn add_font(&mut self, font_cfg: &DimgFontConfig) -> DimgFont {
        todo!()
    }
    //      ImFont*           add_font_default(const ImFontConfig* font_cfg = NULL);
    pub fn add_font_default(&mut self, font_cfg: &DimgFontConfig) -> DimgFont {
        todo!()
    }
    //      ImFont*           AddFontFromFileTTF(const char* filename, float size_pixels, const ImFontConfig* font_cfg = NULL, const ImWchar* glyph_ranges = NULL);
    pub fn add_font_file_ttf(&mut self, filename: &String, size_pixels: f32, font_cfg: &DimgFontConfig, glyph_ranges: &[DimgWchar]) -> DimgFont {
        todo!()
    }
    //      ImFont*           add_font_from_memory_ttf(void* font_data, int font_size, float size_pixels, const ImFontConfig* font_cfg = NULL, const ImWchar* glyph_ranges = NULL); // Note: Transfer ownership of 'ttf_data' to ImFontAtlas! Will be deleted after destruction of the atlas. Set font_cfg->font_data_owned_by_atlas=false to keep ownership of your data and it won't be freed.
    pub fn add_font_from_memory_ttf(&mut self, font_data: &Vec<u8>, font_size: i32, size_pixels: f32, font_cfg: &DimgFontConfig, glyph_ranges: &[DimgWchar]) -> DimgFont {
        todo!()
    }
    //      ImFont*           add_font_from_memory_compressed_ttf(const void* compressed_font_data, int compressed_font_size, float size_pixels, const ImFontConfig* font_cfg = NULL, const ImWchar* glyph_ranges = NULL); // 'compressed_font_data' still owned by caller. Compress with binary_to_compressed_c.cpp.
    pub fn add_font_from_memory_compressed_ttf(&mut self, compressed_font_data: &Vec<u8>, compressed_font_size: usize, size_pixels: f32, font_config: &DimgFontConfig, glyph_ranges: &Vec<DimgWchar>) -> DimgFont {
        todo!()
    }
    //      ImFont*           add_font_from_memory_compressed_base85ttf(const char* compressed_font_data_base85, float size_pixels, const ImFontConfig* font_cfg = NULL, const ImWchar* glyph_ranges = NULL);              // 'compressed_font_data_base85' still owned by caller. Compress with binary_to_compressed_c.cpp with -base85 parameter.
    pub fn add_font_from_memory_compressed_base85ttf(&mut self, compressed_font_data_base85: &String, size_pixels: f32, font_cfg: &DimgFontConfig, glyph_ranges: &Vec<DimgWchar>) -> DimgFont {
        todo!()
    }
    //      void              clear_input_data();           // clear input data (all ImFontConfig structures including sizes, TTF data, glyph ranges, etc.) = all the data used to build the texture and fonts.
    pub fn clear_input_data(&mut self) {
        todo!()
    }
    //      void              clear_tex_data();             // clear output texture data (CPU side). Saves RAM once the texture has been copied to graphics memory.
    pub fn clear_tex_data(&mut self) {
        todo!()
    }
    //      void              clear_fonts();               // clear output font data (glyphs storage, UV coordinates).
    pub fn clear_fonts(&mut self) {
        todo!()
    }
    //      void              clear();                    // clear all input and output.
    pub fn clear(&mut self) {
        todo!()
    }
    //
    //     // build atlas, retrieve pixel data.
    //     // User is in charge of copying the pixels into graphics memory (e.g. create a texture with your engine). Then store your texture handle with set_tex_id().
    //     // The pitch is always = width * BytesPerPixels (1 or 4)
    //     // Building in RGBA32 format is provided for convenience and compatibility, but note that unless you manually manipulate or copy color data into
    //     // the texture (e.g. when using the AddCustomRect*** api), then the RGB pixels emitted will always be white (~75% of memory/bandwidth waste.
    //      bool              build();                    // build pixels data. This is called automatically for you by the GetTexData*** functions.
    pub fn build(&mut self) {
        todo!()
    }
    //      void              GetTexDataAsAlpha8(unsigned char** out_pixels, int* out_width, int* out_height, int* out_bytes_per_pixel = NULL);  // 1 byte per-pixel
    pub fn get_text_data_as_alpha8(&mut self, out_pixels: &Vec<Vec<u8>>, out_width: &mut i32, out_height: &mut i32, out_bytes_per_pixel: &mut i32) {
        todo!()
    }
    //      void              GetTexDataAsRGBA32(unsigned char** out_pixels, int* out_width, int* out_height, int* out_bytes_per_pixel = NULL);  // 4 bytes-per-pixel
    pub fn get_text_data_as_rgba32(&mut self, out_pixels: &Vec<Vec<u8>>, out_width: &mut i32, out_height: &mut i32, out_bytes_per_pixel: &mut i32) {
        todo!()
    }
    //     bool                        is_built() const             { return fonts.size > 0 && tex_ready; } // Bit ambiguous: used to detect when user didn't built texture but effectively we should check tex_id != 0 except that would be backend dependent...
    pub fn is_built(&self) -> bool {
        self.fonts.len() > 0 && self.tex_ready
    }
    //     void                        set_tex_id(ImTextureID id)    { tex_id = id; }
    pub fn set_tex_id(&mut self, id: DimgTextureId) {
        self.tex_id = id
    }
    //
    //     //-------------------------------------------
    //     // Glyph Ranges
    //     //-------------------------------------------
    //
    //     // Helpers to retrieve list of common Unicode ranges (2 value per range, values are inclusive, zero-terminated list)
    //     // NB: Make sure that your string are UTF-8 and NOT in your local code page. In C++11, you can create UTF-8 string literal using the u8"Hello world" syntax. See FAQ for details.
    //     // NB: Consider using ImFontGlyphRangesBuilder to build glyph ranges from textual data.
    //      const ImWchar*    get_glyph_ranges_default();                // Basic Latin, Extended Latin
    pub fn get_glyph_ranges_default(&self) -> Vec<DimgWchar> {
        todo!()
    }
    //      const ImWchar*    get_glyph_ranges_korean();                 // Default + Korean characters
    pub fn get_glyph_ranges_korean(&self) -> Vec<DimgWchar> {
        todo!()
    }
    //      const ImWchar*    get_glyph_ranges_japanese();               // Default + Hiragana, Katakana, Half-width, Selection of 2999 Ideographs
    pub fn get_glyph_ranges_japanese(&self) -> Vec<DimgWchar> {
        todo!()
    }
    //      const ImWchar*    get_glyph_ranges_chinese_full();            // Default + Half-width + Japanese Hiragana/Katakana + full set of about 21000 CJK Unified Ideographs
    pub fn get_glyph_ranges_chinese_full(&self) -> Vec<DimgWchar> {
        todo!()
    }
    //      const ImWchar*    get_glyph_ranges_chinese_simplified_common();// Default + Half-width + Japanese Hiragana/Katakana + set of 2500 CJK Unified Ideographs for common simplified Chinese
    pub fn get_glyph_ranges_chinese_simplified_common(&self) -> Vec<DimgWchar> {
        todo!()
    }
    //      const ImWchar*    get_glyph_ranges_cyrillic();               // Default + about 400 Cyrillic characters
    pub fn get_glyph_ranges_cyrillic(&self) -> Vec<DimgWchar> {
        todo!()
    }
    //      const ImWchar*    get_glyph_ranges_thai();                   // Default + Thai characters
    pub fn get_glyph_ranges_thai(&self) -> Vec<DimgWchar> {
        todo!()
    }
    //      const ImWchar*    get_glyph_ranges_vietnamese();             // Default + Vietnamese characters
    pub fn get_glyph_ranges_vietnamese(&self) -> Vec<DimgWchar> {
        todo!()
    }
    //
    //     //-------------------------------------------
    //     // [BETA] Custom Rectangles/glyphs API
    //     //-------------------------------------------
    //
    //     // You can request arbitrary rectangles to be packed into the atlas, for your own purposes.
    //     // - After calling build(), you can query the rectangle position and render your pixels.
    //     // - If you render colored output, set 'atlas->tex_pixels_use_colors = true' as this may help some backends decide of prefered texture format.
    //     // - You can also request your rectangles to be mapped as font glyph (given a font + Unicode point),
    //     //   so you can render e.g. custom colorful icons and use them as regular glyphs.
    //     // - Read docs/FONTS.md for more details about using colorful icons.
    //     // - Note: this API may be redesigned later in order to support multi-monitor varying DPI settings.
    //      int               AddCustomRectRegular(int width, int height);
    pub fn add_custom_regular(&mut self, width: i32, height: i32) -> i32 {
        todo!()
    }
    //      int               add_custom_rect_font_glyph(ImFont* font, ImWchar id, int width, int height, float advance_x, const ImVec2& offset = ImVec2(0, 0));
    pub fn add_custom_rect_font_glyph(&mut self, font: &DimgFont, id: DimgWchar, width: i32, height: i32, advance_x: f32, offset: &DimgVec2D) -> i32 {
        todo!()
    }
    //     ImFontAtlasCustomRect*      get_custom_rect_by_index(int index) { IM_ASSERT(index >= 0); return &custom_rects[index]; }
    pub fn get_custom_rect_by_index(&mut self, index: i32) -> Result<ImFontAtlasCustomRect, String> {
        if index >= 0 {
            Ok(self.custom_rects[index])
        }
        Err(format!("Invalid index arg: {}", index))
    }
    //
    //     // [Internal]
    //      void              calc_custom_rect_uv(const ImFontAtlasCustomRect* rect, ImVec2* out_uv_min, ImVec2* out_uv_max) const;
    pub fn calc_custom_rect_uv(&mut self, rect: &DimgRect, out_uv_min: &DimgVec2D, out_uv_max: &DimgVec2D) {
        todo!()
    }
    //      bool              get_mouse_cursor_tex_data(ImGuiMouseCursor cursor, ImVec2* out_offset, ImVec2* out_size, ImVec2 out_uv_border[2], ImVec2 out_uv_fill[2]);
    pub fn get_mouse_cursor_tex_data(&mut self, cursor: DimgMouseCursor, out_offset: &mut DimgVec2D, out_size: &mut DimgVec2D, out_uv_border: &mut [DimgVec2D; 2], out_uv_fill: &mut [DimgVec2D; 2]) -> bool {
        todo!()
    }
}

// flags for ImFontAtlas build
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ImFontAtlasFlags {
    None = 0,
    NoPowerOfTwoHeight = 1 << 0,
    // Don't round the height to next power of two
    NoMouseCursors = 1 << 1,
    // Don't build software mouse cursors into the atlas (save a little texture memory)
    NoBakedLines = 1 << 2,    // Don't build thick line textures into the atlas (save a little texture memory, allow support for point/nearest filtering). The AntiAliasedLinesUseTex features uses them, otherwise they will be rendered using polygons (more expensive for CPU/GPU).
}

// See ImFontAtlas::AddCustomRectXXX functions.
#[derive(Default, Debug, Clone)]
pub struct ImFontAtlasCustomRect {
    // unsigned short  width, height;  // Input    // Desired rectangle dimension
    pub width: u16,
    pub height: u16,
    // unsigned short  x, Y;           // Output   // Packed position in Atlas
    pub x: u16,
    pub Y: u16,
    // unsigned pub glyph_id: i32,      // Input    // For custom font glyphs only (id < 0x110000)
    pub glyph_id: u32,
    pub glyph_advance_x: f32,
    // Input    // For custom font glyphs only: glyph xadvance
    pub glyph_offset: DimgVec2D,
    // Input    // For custom font glyphs only: glyph display offset
    pub font: DimgFont, // ImFont*         font;           // Input    // For custom font glyphs only: target font
}

impl ImFontAtlasCustomRect {
    // ImFontAtlasCustomRect()         { width = height = 0; x = Y = 0xFFFF; glyph_id = 0; glyph_advance_x = 0.0; glyph_offset = ImVec2(0, 0); font = NULL;
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            x: 0xFFFF,
            Y: 0xFFFF,
            glyph_id: 0,
            glyph_advance_x: 0.0,
            glyph_offset: Default::default(),
            font: Default::default(),
        }
    }
    //     bool is_packed() const           { return x != 0xFFFF; }
    pub fn is_packed(&self) -> bool {
        self.x != 0xFFFF
    }
}
