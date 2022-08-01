use crate::font::{DimgFontConfig, Font, ImFontBuilderIO};
use crate::input::MouseCursor;
use crate::rect::Rect;
use crate::texture::TextureId;
use crate::types::DimgWchar;
use crate::vectors::two_d::Vector2D;
use crate::vectors::Vector4D;

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
pub struct FontAtlas {
    //-------------------------------------------
    // Members
    //-------------------------------------------
    pub flags: ImFontAtlasFlags,
    // ImFontAtlasFlags            flags;              // build flags (see )
    pub tex_id: TextureId,
    // ImTextureID                 tex_id;              // User data to refer to the texture once it has been uploaded to user's graphic systems. It is passed back to you during rendering via the ImDrawCmd structure.
    pub tex_desired_width: i32,
    // Texture width desired by user before build(). Must be a power-of-two. If have many glyphs your graphics API have texture size restrictions you may want to increase texture width to decrease height.
    pub tex_glyph_padding: i32,
    // Padding between glyphs within texture in pixels. Defaults to 1. If your rendering method doesn't rely on bilinear filtering you may set this to 0 (will also need to set AntiAliasedLinesUseTex = false).
    pub locked: bool, // Marked as locked by ImGui::NewFrame() so attempt to modify the atlas will assert.

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
    pub tex_uv_scale: Vector2D,
    // = (1.0/tex_width, 1.0/tex_height)
    pub tex_uv_white_pixel: Vector2D,
    // Texture coordinates to a white pixel
    pub fonts: Vec<Font>,
    // ImVector<ImFont*>           fonts;              // Hold all the fonts returned by add_font*. fonts[0] is the default font upon calling ImGui::NewFrame(), use ImGui::PushFont()/PopFont() to change the current font.
    // ImVector<ImFontAtlasCustomRect> custom_rects;    // Rectangles for packing custom texture data into the atlas.
    pub custom_rects: Vec<ImFontAtlasCustomRect>,
    // ImVector<ImFontConfig>      config_data;         // Configuration data
    pub config_data: Vec<DimgFontConfig>,
    // Vector4D                      tex_uv_lines[IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 1];  // UVs for baked anti-aliased lines
    pub tex_uv_lines: Vec<Vector4D>,
    // [Internal] font builder
    // const ImFontBuilderIO*      font_builder_io;      // Opaque interface to a font builder (default to stb_truetype, can be changed to use FreeType by defining IMGUI_ENABLE_FREETYPE).
    pub font_builder_io: ImFontBuilderIO,
    // unsigned pub font_builder_flags: i32, // Shared flags (for all fonts) for custom font builder. THIS IS BUILD IMPLEMENTATION DEPENDENT. Per-font override is also available in ImFontConfig.
    pub font_builder_flags: i32,

    // [Internal] Packing data
    // int                         pack_id_mouse_cursors; // Custom texture rectangle id for white pixel and mouse cursors
    pub pack_id_mouse_cursors: i32,
    pub pack_id_lines: i32, // Custom texture rectangle id for baked anti-aliased lines

                            // [Obsolete]
                            //typedef ImFontAtlasCustomRect    CustomRect;         // OBSOLETED in 1.72+
                            //typedef ImFontGlyphRangesBuilder GlyphRangesBuilder; // OBSOLETED in 1.67+
}

impl FontAtlas {
    //  ImFontAtlas();
    //      ~ImFontAtlas();
    //      ImFont*           add_font(const ImFontConfig* font_cfg);
    pub fn add_font(&mut self, font_cfg: &DimgFontConfig) -> Font {
        todo!()
    }
    //      ImFont*           add_font_default(const ImFontConfig* font_cfg = None);
    pub fn add_font_default(&mut self, font_cfg: &DimgFontConfig) -> Font {
        todo!()
    }
    //      ImFont*           AddFontFromFileTTF(const char* filename, float size_pixels, const ImFontConfig* font_cfg = None, const ImWchar* glyph_ranges = None);
    pub fn add_font_file_ttf(
        &mut self,
        filename: &String,
        size_pixels: f32,
        font_cfg: &DimgFontConfig,
        glyph_ranges: &[DimgWchar],
    ) -> Font {
        todo!()
    }
    //      ImFont*           add_font_from_memory_ttf(void* font_data, int font_size, float size_pixels, const ImFontConfig* font_cfg = None, const ImWchar* glyph_ranges = None); // Note: Transfer ownership of 'ttf_data' to ImFontAtlas! Will be deleted after destruction of the atlas. Set font_cfg->font_data_owned_by_atlas=false to keep ownership of your data and it won't be freed.
    pub fn add_font_from_memory_ttf(
        &mut self,
        font_data: &Vec<u8>,
        font_size: i32,
        size_pixels: f32,
        font_cfg: &DimgFontConfig,
        glyph_ranges: &[DimgWchar],
    ) -> Font {
        todo!()
    }
    //      ImFont*           add_font_from_memory_compressed_ttf(const void* compressed_font_data, int compressed_font_size, float size_pixels, const ImFontConfig* font_cfg = None, const ImWchar* glyph_ranges = None); // 'compressed_font_data' still owned by caller. Compress with binary_to_compressed_c.cpp.
    pub fn add_font_from_memory_compressed_ttf(
        &mut self,
        compressed_font_data: &Vec<u8>,
        compressed_font_size: usize,
        size_pixels: f32,
        font_config: &DimgFontConfig,
        glyph_ranges: &Vec<DimgWchar>,
    ) -> Font {
        todo!()
    }
    //      ImFont*           add_font_from_memory_compressed_base85ttf(const char* compressed_font_data_base85, float size_pixels, const ImFontConfig* font_cfg = None, const ImWchar* glyph_ranges = None);              // 'compressed_font_data_base85' still owned by caller. Compress with binary_to_compressed_c.cpp with -base85 parameter.
    pub fn add_font_from_memory_compressed_base85ttf(
        &mut self,
        compressed_font_data_base85: &String,
        size_pixels: f32,
        font_cfg: &DimgFontConfig,
        glyph_ranges: &Vec<DimgWchar>,
    ) -> Font {
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
    //      void              GetTexDataAsAlpha8(unsigned char** out_pixels, int* out_width, int* out_height, int* out_bytes_per_pixel = None);  // 1 byte per-pixel
    pub fn get_text_data_as_alpha8(
        &mut self,
        out_pixels: &Vec<Vec<u8>>,
        out_width: &mut i32,
        out_height: &mut i32,
        out_bytes_per_pixel: &mut i32,
    ) {
        todo!()
    }
    //      void              GetTexDataAsRGBA32(unsigned char** out_pixels, int* out_width, int* out_height, int* out_bytes_per_pixel = None);  // 4 bytes-per-pixel
    pub fn get_text_data_as_rgba32(
        &mut self,
        out_pixels: &Vec<Vec<u8>>,
        out_width: &mut i32,
        out_height: &mut i32,
        out_bytes_per_pixel: &mut i32,
    ) {
        todo!()
    }
    //     bool                        is_built() const             { return fonts.size > 0 && tex_ready; } // Bit ambiguous: used to detect when user didn't built texture but effectively we should check tex_id != 0 except that would be backend dependent...
    pub fn is_built(&self) -> bool {
        self.fonts.len() > 0 && self.tex_ready
    }
    //     void                        set_tex_id(ImTextureID id)    { tex_id = id; }
    pub fn set_tex_id(&mut self, id: TextureId) {
        self.tex_id = id
    }
    //
    //     //-------------------------------------------
    //     // Glyph ranges
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
    //      int               add_custom_rect_font_glyph(ImFont* font, ImWchar id, int width, int height, float advance_x, const Vector2D& offset = Vector2D(0, 0));
    pub fn add_custom_rect_font_glyph(
        &mut self,
        font: &Font,
        id: DimgWchar,
        width: i32,
        height: i32,
        advance_x: f32,
        offset: &Vector2D,
    ) -> i32 {
        todo!()
    }
    //     ImFontAtlasCustomRect*      get_custom_rect_by_index(int index) { IM_ASSERT(index >= 0); return &custom_rects[index]; }
    pub fn get_custom_rect_by_index(
        &mut self,
        index: i32,
    ) -> Result<ImFontAtlasCustomRect, String> {
        if index >= 0 {
            Ok(self.custom_rects[index])
        }
        Err(format!("Invalid index arg: {}", index))
    }
    //
    //     // [Internal]
    //      void              calc_custom_rect_uv(const ImFontAtlasCustomRect* rect, Vector2D* out_uv_min, Vector2D* out_uv_max) const;
    pub fn calc_custom_rect_uv(
        &mut self,
        rect: &Rect,
        out_uv_min: &Vector2D,
        out_uv_max: &Vector2D,
    ) {
        todo!()
    }
    //      bool              get_mouse_cursor_tex_data(ImGuiMouseCursor cursor, Vector2D* out_offset, Vector2D* out_size, Vector2D out_uv_border[2], Vector2D out_uv_fill[2]);
    pub fn get_mouse_cursor_tex_data(
        &mut self,
        cursor: MouseCursor,
        out_offset: &mut Vector2D,
        out_size: &mut Vector2D,
        out_uv_border: &mut [Vector2D; 2],
        out_uv_fill: &mut [Vector2D; 2],
    ) -> bool {
        todo!()
    }
}

// flags for ImFontAtlas build
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ImFontAtlasFlags {
    None = 0,
    NoPowerOfTwoHeight,
    // Don't round the height to next power of two
    NoMouseCursors,
    // Don't build software mouse cursors into the atlas (save a little texture memory)
    NoBakedLines, // Don't build thick line textures into the atlas (save a little texture memory, allow support for point/nearest filtering). The AntiAliasedLinesUseTex features uses them, otherwise they will be rendered using polygons (more expensive for CPU/GPU).
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
    pub glyph_offset: Vector2D,
    // Input    // For custom font glyphs only: glyph display offset
    pub font: Font, // ImFont*         font;           // Input    // For custom font glyphs only: target font
}

impl ImFontAtlasCustomRect {
    // ImFontAtlasCustomRect()         { width = height = 0; x = Y = 0xFFFF; glyph_id = 0; glyph_advance_x = 0.0; glyph_offset = Vector2D(0, 0); font = None;
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


// A work of art lies ahead! (. = white layer, x = black layer, others are blank)
// The 2x2 white texels on the top left are the ones we'll use everywhere in Dear ImGui to render filled shapes.
// (This is used when io.mouse_draw_cursor = true)
let FONT_ATLAS_DEFAULT_TEX_DATA_W = 122; // Actual texture will be 2 times that + 1 spacing.
let FONT_ATLAS_DEFAULT_TEX_DATA_H = 27;
static const char FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS[FONT_ATLAS_DEFAULT_TEX_DATA_W * FONT_ATLAS_DEFAULT_TEX_DATA_H + 1] =
{
    "..-         -XXXXXXX-    x    -           x           -XXXXXXX          -          XXXXXXX-     XX          - XX       XX "
    "..-         -x.....x-   x.x   -          x.x          -x.....x          -          x.....x-    x..x         -x..x     x..x"
    "---         -XXX.XXX-  x...x  -         x...x         -x....x           -           x....x-    x..x         -x...x   x...x"
    "x           -  x.x  - x.....x -        x.....x        -x...x            -            x...x-    x..x         - x...x x...x "
    "XX          -  x.x  -x.......x-       x.......x       -x..x.x           -           x.x..x-    x..x         -  x...x...x  "
    "x.x         -  x.x  -XXXX.XXXX-       XXXX.XXXX       -x.x x.x          -          x.x x.x-    x..XXX       -   x.....x   "
    "x..x        -  x.x  -   x.x   -          x.x          -XX   x.x         -         x.x   XX-    x..x..XXX    -    x...x    "
    "x...x       -  x.x  -   x.x   -    XX    x.x    XX    -      x.x        -        x.x      -    x..x..x..XX  -     x.x     "
    "x....x      -  x.x  -   x.x   -   x.x    x.x    x.x   -       x.x       -       x.x       -    x..x..x..x.x -    x...x    "
    "x.....x     -  x.x  -   x.x   -  x..x    x.x    x..x  -        x.x      -      x.x        -XXX x..x..x..x..x-   x.....x   "
    "x......x    -  x.x  -   x.x   - x...XXXXXX.XXXXXX...x -         x.x   XX-XX   x.x         -x..XX........x..x-  x...x...x  "
    "x.......x   -  x.x  -   x.x   -x.....................x-          x.x x.x-x.x x.x          -x...x...........x- x...x x...x "
    "x........x  -  x.x  -   x.x   - x...XXXXXX.XXXXXX...x -           x.x..x-x..x.x           - x..............x-x...x   x...x"
    "x.........x -XXX.XXX-   x.x   -  x..x    x.x    x..x  -            x...x-x...x            -  x.............x-x..x     x..x"
    "x..........x-x.....x-   x.x   -   x.x    x.x    x.x   -           x....x-x....x           -  x.............x- XX       XX "
    "x......XXXXX-XXXXXXX-   x.x   -    XX    x.x    XX    -          x.....x-x.....x          -   x............x--------------"
    "x...x..x    ---------   x.x   -          x.x          -          XXXXXXX-XXXXXXX          -   x...........x -             "
    "x..x x..x   -       -XXXX.XXXX-       XXXX.XXXX       -------------------------------------    x..........x -             "
    "x.x  x..x   -       -x.......x-       x.......x       -    XX           XX    -           -    x..........x -             "
    "XX    x..x  -       - x.....x -        x.....x        -   x.x           x.x   -           -     x........x  -             "
    "      x..x  -       -  x...x  -         x...x         -  x..x           x..x  -           -     x........x  -             "
    "       XX   -       -   x.x   -          x.x          - x...XXXXXXXXXXXXX...x -           -     XXXXXXXXXX  -             "
    "-------------       -    x    -           x           -x.....................x-           -------------------             "
    "                    ----------------------------------- x...XXXXXXXXXXXXX...x -                                           "
    "                                                      -  x..x           x..x  -                                           "
    "                                                      -   x.x           x.x   -                                           "
    "                                                      -    XX           XX    -                                           "
};

static const Vector2D FONT_ATLAS_DEFAULT_TEX_CURSOR_DATA[ImGuiMouseCursor_COUNT][3] =
{
    // pos ........ size ......... Offset ......
    { Vector2D::new( 0,3), Vector2D::new(12,19), Vector2D::new( 0, 0) }, // ImGuiMouseCursor_Arrow
    { Vector2D::new(13,0), Vector2D::new( 7,16), Vector2D::new( 1, 8) }, // ImGuiMouseCursor_TextInput
    { Vector2D::new(31,0), Vector2D::new(23,23), Vector2D::new(11,11) }, // ImGuiMouseCursor_ResizeAll
    { Vector2D::new(21,0), Vector2D::new( 9,23), Vector2D::new( 4,11) }, // ImGuiMouseCursor_ResizeNS
    { Vector2D::new(55,18),Vector2D::new(23, 9), Vector2D::new(11, 4) }, // ImGuiMouseCursor_ResizeEW
    { Vector2D::new(73,0), Vector2D::new(17,17), Vector2D::new( 8, 8) }, // ImGuiMouseCursor_ResizeNESW
    { Vector2D::new(55,0), Vector2D::new(17,17), Vector2D::new( 8, 8) }, // ImGuiMouseCursor_ResizeNWSE
    { Vector2D::new(91,0), Vector2D::new(17,22), Vector2D::new( 5, 0) }, // ImGuiMouseCursor_Hand
    { Vector2D::new(109,0),Vector2D::new(13,15), Vector2D::new( 6, 7) }, // ImGuiMouseCursor_NotAllowed
};

ImFontAtlas::ImFontAtlas()
{
    memset(this, 0, sizeof(*this));
    TexGlyphPadding = 1;
    PackIdMouseCursors = PackIdLines = -1;
}

ImFontAtlas::~ImFontAtlas()
{
    // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");
    Clear();
}

void    ImFontAtlas::ClearInputData()
{
    // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");
    for (int i = 0; i < ConfigData.size; i += 1)
        if (ConfigData[i].FontData && ConfigData[i].FontDataOwnedByAtlas)
        {
            IM_FREE(ConfigData[i].FontData);
            ConfigData[i].FontData = None;
        }

    // When clearing this we lose access to the font name and other information used to build the font.
    for (int i = 0; i < Fonts.size; i += 1)
        if (Fonts[i].ConfigData >= ConfigData.data && Fonts[i].ConfigData < ConfigData.data + ConfigData.size)
        {
            Fonts[i].ConfigData = None;
            Fonts[i].ConfigDataCount = 0;
        }
    ConfigData.clear();
    CustomRects.clear();
    PackIdMouseCursors = PackIdLines = -1;
    // Important: we leave tex_ready untouched
}

void    ImFontAtlas::ClearTexData()
{
    // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");
    if (TexPixelsAlpha8)
        IM_FREE(TexPixelsAlpha8);
    if (TexPixelsRGBA32)
        IM_FREE(TexPixelsRGBA32);
    TexPixelsAlpha8 = None;
    TexPixelsRGBA32 = None;
    TexPixelsUseColors = false;
    // Important: we leave tex_ready untouched
}

void    ImFontAtlas::ClearFonts()
{
    // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");
    Fonts.clear_delete();
    TexReady = false;
}

void    ImFontAtlas::Clear()
{
    ClearInputData();
    ClearTexData();
    ClearFonts();
}

void    ImFontAtlas::GetTexDataAsAlpha8(unsigned char** out_pixels, int* out_width, int* out_height, int* out_bytes_per_pixel)
{
    // build atlas on demand
    if (TexPixelsAlpha8 == None)
        Build();

    *out_pixels = TexPixelsAlpha8;
    if (out_width) *out_width = TexWidth;
    if (out_height) *out_height = TexHeight;
    if (out_bytes_per_pixel) *out_bytes_per_pixel = 1;
}

void    ImFontAtlas::GetTexDataAsRGBA32(unsigned char** out_pixels, int* out_width, int* out_height, int* out_bytes_per_pixel)
{
    // Convert to RGBA32 format on demand
    // Although it is likely to be the most commonly used format, our font rendering is 1 channel / 8 bpp
    if (!TexPixelsRGBA32)
    {
        unsigned char* pixels = None;
        GetTexDataAsAlpha8(&pixels, None, None);
        if (pixels)
        {
            TexPixelsRGBA32 = (unsigned int*)IM_ALLOC(TexWidth * TexHeight * 4);
            const unsigned char* src = pixels;
            unsigned int* dst = TexPixelsRGBA32;
            for (int n = TexWidth * TexHeight; n > 0; n--)
                *dst += 1 = IM_COL32(255, 255, 255, (unsigned int)(*src += 1));
        }
    }

    *out_pixels = (unsigned char*)TexPixelsRGBA32;
    if (out_width) *out_width = TexWidth;
    if (out_height) *out_height = TexHeight;
    if (out_bytes_per_pixel) *out_bytes_per_pixel = 4;
}

ImFont* ImFontAtlas::AddFont(const ImFontConfig* font_cfg)
{
    // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");
    // IM_ASSERT(font_cfg.FontData != None && font_cfg.FontDataSize > 0);
    // IM_ASSERT(font_cfg.sizePixels > 0.0);

    // Create new font
    if (!font_cfg.MergeMode)
        Fonts.push_back(IM_NEW(ImFont));
    else
        // IM_ASSERT(!Fonts.empty() && "Cannot use merge_mode for the first font"); // When using merge_mode make sure that a font has already been added before. You can use ImGui::GetIO().fonts->add_font_default() to add the default imgui font.

    ConfigData.push_back(*font_cfg);
    ImFontConfig& new_font_cfg = ConfigData.back();
    if (new_font_cfg.DstFont == None)
        new_font_cfg.DstFont = Fonts.back();
    if (!new_font_cfg.FontDataOwnedByAtlas)
    {
        new_font_cfg.FontData = IM_ALLOC(new_font_cfg.FontDataSize);
        new_font_cfg.FontDataOwnedByAtlas = true;
        memcpy(new_font_cfg.FontData, font_cfg.FontData, new_font_cfg.FontDataSize);
    }

    if (new_font_cfg.DstFont.EllipsisChar == (ImWchar)-1)
        new_font_cfg.DstFont.EllipsisChar = font_cfg.EllipsisChar;

    // Invalidate texture
    TexReady = false;
    ClearTexData();
    return new_font_cfg.DstFont;
}

// Default font TTF is compressed with stb_compress then base85 encoded (see misc/fonts/binary_to_compressed_c.cpp for encoder)
static unsigned int stb_decompress_length(const unsigned char* input);
static unsigned int stb_decompress(unsigned char* output, const unsigned char* input, unsigned int length);
static const char*  GetDefaultCompressedFontDataTTFBase85();
static unsigned int Decode85Byte(char c)                                    { return c >= '\\' ? c-36 : c-35; }
static void         Decode85(const unsigned char* src, unsigned char* dst)
{
    while (*src)
    {
        unsigned int tmp = Decode85Byte(src[0]) + 85 * (Decode85Byte(src[1]) + 85 * (Decode85Byte(src[2]) + 85 * (Decode85Byte(src[3]) + 85 * Decode85Byte(src[4]))));
        dst[0] = ((tmp >> 0) & 0xFF); dst[1] = ((tmp >> 8) & 0xFF); dst[2] = ((tmp >> 16) & 0xFF); dst[3] = ((tmp >> 24) & 0xFF);   // We can't assume little-endianness.
        src += 5;
        dst += 4;
    }
}

// Load embedded ProggyClean.ttf at size 13, disable oversampling
ImFont* ImFontAtlas::AddFontDefault(const ImFontConfig* font_cfg_template)
{
    ImFontConfig font_cfg = font_cfg_template ? *font_cfg_template : ImFontConfig();
    if (!font_cfg_template)
    {
        font_cfg.OversampleH = font_cfg.OversampleV = 1;
        font_cfg.PixelSnapH = true;
    }
    if (font_cfg.sizePixels <= 0.0)
        font_cfg.sizePixels = 13.0 * 1.0;
    if (font_cfg.name[0] == '\0')
        ImFormatString(font_cfg.name, IM_ARRAYSIZE(font_cfg.name), "ProggyClean.ttf, %dpx", font_cfg.sizePixels);
    font_cfg.EllipsisChar = (ImWchar)0x0085;
    font_cfg.GlyphOffset.y = 1.0 * f32::floor(font_cfg.sizePixels / 13.0);  // Add +1 offset per 13 units

    const char* ttf_compressed_base85 = GetDefaultCompressedFontDataTTFBase85();
    const ImWchar* glyph_ranges = font_cfg.GlyphRanges != None ? font_cfg.GlyphRanges : GetGlyphRangesDefault();
    ImFont* font = AddFontFromMemoryCompressedBase85TTF(ttf_compressed_base85, font_cfg.sizePixels, &font_cfg, glyph_ranges);
    return font;
}

ImFont* ImFontAtlas::AddFontFromFileTTF(const char* filename, float size_pixels, const ImFontConfig* font_cfg_template, const ImWchar* glyph_ranges)
{
    // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");
    size_t data_size = 0;
    void* data = ImFileLoadToMemory(filename, "rb", &data_size, 0);
    if (!data)
    {
        // IM_ASSERT_USER_ERROR(0, "Could not load font file!");
        return None;
    }
    ImFontConfig font_cfg = font_cfg_template ? *font_cfg_template : ImFontConfig();
    if (font_cfg.name[0] == '\0')
    {
        // Store a short copy of filename into into the font name for convenience
        const char* p;
        for (p = filename + strlen(filename); p > filename && p[-1] != '/' && p[-1] != '\\'; p--) {}
        ImFormatString(font_cfg.name, IM_ARRAYSIZE(font_cfg.name), "%s, %.0px", p, size_pixels);
    }
    return AddFontFromMemoryTTF(data, data_size, size_pixels, &font_cfg, glyph_ranges);
}

// NB: Transfer ownership of 'ttf_data' to ImFontAtlas, unless font_cfg_template->font_data_owned_by_atlas == false. Owned TTF buffer will be deleted after build().
ImFont* ImFontAtlas::AddFontFromMemoryTTF(void* ttf_data, int ttf_size, float size_pixels, const ImFontConfig* font_cfg_template, const ImWchar* glyph_ranges)
{
    // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");
    ImFontConfig font_cfg = font_cfg_template ? *font_cfg_template : ImFontConfig();
    // IM_ASSERT(font_cfg.FontData == None);
    font_cfg.FontData = ttf_data;
    font_cfg.FontDataSize = ttf_size;
    font_cfg.sizePixels = size_pixels > 0.0 ? size_pixels : font_cfg.sizePixels;
    if (glyph_ranges)
        font_cfg.GlyphRanges = glyph_ranges;
    return AddFont(&font_cfg);
}

ImFont* ImFontAtlas::AddFontFromMemoryCompressedTTF(const void* compressed_ttf_data, int compressed_ttf_size, float size_pixels, const ImFontConfig* font_cfg_template, const ImWchar* glyph_ranges)
{
    const unsigned int buf_decompressed_size = stb_decompress_length((const unsigned char*)compressed_ttf_data);
    unsigned char* buf_decompressed_data = (unsigned char*)IM_ALLOC(buf_decompressed_size);
    stb_decompress(buf_decompressed_data, (const unsigned char*)compressed_ttf_data, (unsigned int)compressed_ttf_size);

    ImFontConfig font_cfg = font_cfg_template ? *font_cfg_template : ImFontConfig();
    // IM_ASSERT(font_cfg.FontData == None);
    font_cfg.FontDataOwnedByAtlas = true;
    return AddFontFromMemoryTTF(buf_decompressed_data, buf_decompressed_size, size_pixels, &font_cfg, glyph_ranges);
}

ImFont* ImFontAtlas::AddFontFromMemoryCompressedBase85TTF(const char* compressed_ttf_data_base85, float size_pixels, const ImFontConfig* font_cfg, const ImWchar* glyph_ranges)
{
    int compressed_ttf_size = ((strlen(compressed_ttf_data_base85) + 4) / 5) * 4;
    void* compressed_ttf = IM_ALLOC(compressed_ttf_size);
    Decode85((const unsigned char*)compressed_ttf_data_base85, (unsigned char*)compressed_ttf);
    ImFont* font = AddFontFromMemoryCompressedTTF(compressed_ttf, compressed_ttf_size, size_pixels, font_cfg, glyph_ranges);
    IM_FREE(compressed_ttf);
    return font;
}

int ImFontAtlas::AddCustomRectRegular(int width, int height)
{
    // IM_ASSERT(width > 0 && width <= 0xFFFF);
    // IM_ASSERT(height > 0 && height <= 0xFFFF);
    ImFontAtlasCustomRect r;
    r.Width = (unsigned short)width;
    r.Height = (unsigned short)height;
    CustomRects.push_back(r);
    return CustomRects.size - 1; // Return index
}

int ImFontAtlas::AddCustomRectFontGlyph(ImFont* font, ImWchar id, int width, int height, float advance_x, const Vector2D& offset)
{
#ifdef IMGUI_USE_WCHAR32
    // IM_ASSERT(id <= IM_UNICODE_CODEPOINT_MAX);

    // IM_ASSERT(font != None);
    // IM_ASSERT(width > 0 && width <= 0xFFFF);
    // IM_ASSERT(height > 0 && height <= 0xFFFF);
    ImFontAtlasCustomRect r;
    r.Width = (unsigned short)width;
    r.Height = (unsigned short)height;
    r.GlyphID = id;
    r.GlyphAdvanceX = advance_x;
    r.GlyphOffset = offset;
    r.font = font;
    CustomRects.push_back(r);
    return CustomRects.size - 1; // Return index
}

void ImFontAtlas::CalcCustomRectUV(const ImFontAtlasCustomRect* rect, Vector2D* out_uv_min, Vector2D* out_uv_max) const
{
    // IM_ASSERT(TexWidth > 0 && TexHeight > 0);   // font atlas needs to be built before we can calculate UV coordinates
    // IM_ASSERT(rect.IsPacked());                // Make sure the rectangle has been packed
    *out_uv_min = Vector2D::new((float)rect.X * TexUvScale.x, rect.Y * TexUvScale.y);
    *out_uv_max = Vector2D::new((float)(rect.X + rect.Width) * TexUvScale.x, (rect.Y + rect.Height) * TexUvScale.y);
}

bool ImFontAtlas::GetMouseCursorTexData(ImGuiMouseCursor cursor_type, Vector2D* out_offset, Vector2D* out_size, Vector2D out_uv_border[2], Vector2D out_uv_fill[2])
{
    if (cursor_type <= MouseCursor::None || cursor_type >= ImGuiMouseCursor_COUNT)
        return false;
    if (Flags & ImFontAtlasFlags_NoMouseCursors)
        return false;

    // IM_ASSERT(PackIdMouseCursors != -1);
    ImFontAtlasCustomRect* r = GetCustomRectByIndex(PackIdMouseCursors);
    Vector2D pos = FONT_ATLAS_DEFAULT_TEX_CURSOR_DATA[cursor_type][0] + Vector2D::new((float)r.X, r.Y);
    Vector2D size = FONT_ATLAS_DEFAULT_TEX_CURSOR_DATA[cursor_type][1];
    *out_size = size;
    *out_offset = FONT_ATLAS_DEFAULT_TEX_CURSOR_DATA[cursor_type][2];
    out_uv_border[0] = (pos) * TexUvScale;
    out_uv_border[1] = (pos + size) * TexUvScale;
    pos.x += FONT_ATLAS_DEFAULT_TEX_DATA_W + 1;
    out_uv_fill[0] = (pos) * TexUvScale;
    out_uv_fill[1] = (pos + size) * TexUvScale;
    return true;
}

bool    ImFontAtlas::Build()
{
    // IM_ASSERT(!Locked && "Cannot modify a locked ImFontAtlas between NewFrame() and EndFrame/Render()!");

    // Default font is none are specified
    if (ConfigData.size == 0)
        AddFontDefault();

    // Select builder
    // - Note that we do not reassign to atlas->font_builder_io, since it is likely to point to static data which
    //   may mess with some hot-reloading schemes. If you need to assign to this (for dynamic selection) AND are
    //   using a hot-reloading scheme that messes up static data, store your own instance of ImFontBuilderIO somewhere
    //   and point to it instead of pointing directly to return value of the GetBuilderXXX functions.
    const ImFontBuilderIO* builder_io = FontBuilderIO;
    if (builder_io == None)
    {
#ifdef IMGUI_ENABLE_FREETYPE
        builder_io = ImGuiFreeType::GetBuilderForFreeType();
#elif defined(IMGUI_ENABLE_STB_TRUETYPE)
        builder_io = ImFontAtlasGetBuilderForStbTruetype();
#else
        // IM_ASSERT(0); // Invalid build function

    }

    // build
    return builder_io.FontBuilder_Build(this);
}

void    ImFontAtlasBuildMultiplyCalcLookupTable(unsigned char out_table[256], float in_brighten_factor)
{
    for (unsigned int i = 0; i < 256; i += 1)
    {
        unsigned int value = (unsigned int)(i * in_brighten_factor);
        out_table[i] = value > 255 ? 255 : (value & 0xFF);
    }
}

void    ImFontAtlasBuildMultiplyRectAlpha8(const unsigned char table[256], unsigned char* pixels, int x, int y, int w, int h, int stride)
{
    unsigned char* data = pixels + x + y * stride;
    for (int j = h; j > 0; j--, data += stride)
        for (int i = 0; i < w; i += 1)
            data[i] = table[data[i]];
}

#ifdef IMGUI_ENABLE_STB_TRUETYPE
// Temporary data for one source font (multiple source fonts can be merged into one destination ImFont)
// (C++03 doesn't allow instancing ImVector<> with function-local types so we declare the type here.)
struct ImFontBuildSrcData
{
    stbtt_fontinfo      FontInfo;
    stbtt_pack_range    PackRange;          // Hold the list of codepoints to pack (essentially points to Codepoints.data)
    stbrp_rect*         Rects;              // Rectangle to pack. We first fill in their size and the packer will give us their position.
    stbtt_packedchar*   PackedChars;        // Output glyphs
    const ImWchar*      SrcRanges;          // ranges as requested by user (user is allowed to request too much, e.g. 0x0020..0xFFFF)
    int                 DstIndex;           // index into atlas->fonts[] and dst_tmp_array[]
    int                 GlyphsHighest;      // Highest requested codepoint
    int                 GlyphsCount;        // Glyph count (excluding missing glyphs and glyphs already set by an earlier source font)
    ImBitVector         GlyphsSet;          // Glyph bit map (random access, 1-bit per codepoint. This will be a maximum of 8KB)
    ImVector<int>       GlyphsList;         // Glyph codepoints list (flattened version of GlyphsMap)
};

// Temporary data for one destination ImFont* (multiple source fonts can be merged into one destination ImFont)
struct ImFontBuildDstData
{
    int                 SrcCount;           // Number of source fonts targeting this destination font.
    int                 GlyphsHighest;
    int                 GlyphsCount;
    ImBitVector         GlyphsSet;          // This is used to resolve collision when multiple sources are merged into a same destination font.
};



static bool ImFontAtlasBuildWithStbTruetype(ImFontAtlas* atlas)
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
    ImVector<ImFontBuildSrcData> src_tmp_array;
    ImVector<ImFontBuildDstData> dst_tmp_array;
    src_tmp_array.resize(atlas.ConfigData.size);
    dst_tmp_array.resize(atlas.Fonts.size);
    memset(src_tmp_array.data, 0, src_tmp_array.size_in_bytes());
    memset(dst_tmp_array.data, 0, dst_tmp_array.size_in_bytes());

    // 1. Initialize font loading structure, check font data validity
    for (int src_i = 0; src_i < atlas.ConfigData.size; src_i += 1)
    {
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        ImFontConfig& cfg = atlas.ConfigData[src_i];
        // IM_ASSERT(cfg.DstFont && (!cfg.DstFont.IsLoaded() || cfg.DstFont.container_atlas == atlas));

        // Find index from cfg.dst_font (we allow the user to set cfg.dst_font. Also it makes casual debugging nicer than when storing indices)
        src_tmp.DstIndex = -1;
        for (int output_i = 0; output_i < atlas.Fonts.size && src_tmp.DstIndex == -1; output_i += 1)
            if (cfg.DstFont == atlas.Fonts[output_i])
                src_tmp.DstIndex = output_i;
        if (src_tmp.DstIndex == -1)
        {
            // IM_ASSERT(src_tmp.DstIndex != -1); // cfg.dst_font not pointing within atlas->fonts[] array?
            return false;
        }
        // Initialize helper structure for font loading and verify that the TTF/OTF data is correct
        let font_offset = stbtt_GetFontOffsetForIndex((unsigned char*)cfg.FontData, cfg.FontNo);
        // IM_ASSERT(font_offset >= 0 && "font_data is incorrect, or font_no cannot be found.");
        if (!stbtt_InitFont(&src_tmp.FontInfo, (unsigned char*)cfg.FontData, font_offset))
            return false;

        // Measure highest codepoints
        ImFontBuildDstData& dst_tmp = dst_tmp_array[src_tmp.DstIndex];
        src_tmp.SrcRanges = cfg.GlyphRanges ? cfg.GlyphRanges : atlas.GetGlyphRangesDefault();
        for (const ImWchar* src_range = src_tmp.SrcRanges; src_range[0] && src_range[1]; src_range += 2)
            src_tmp.GlyphsHighest = ImMax(src_tmp.GlyphsHighest, src_range[1]);
        dst_tmp.SrcCount += 1;
        dst_tmp.GlyphsHighest = ImMax(dst_tmp.GlyphsHighest, src_tmp.GlyphsHighest);
    }

    // 2. For every requested codepoint, check for their presence in the font data, and handle redundancy or overlaps between source fonts to avoid unused glyphs.
    int total_glyphs_count = 0;
    for (int src_i = 0; src_i < src_tmp_array.size; src_i += 1)
    {
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        ImFontBuildDstData& dst_tmp = dst_tmp_array[src_tmp.DstIndex];
        src_tmp.GlyphsSet.Create(src_tmp.GlyphsHighest + 1);
        if (dst_tmp.GlyphsSet.Storage.empty())
            dst_tmp.GlyphsSet.Create(dst_tmp.GlyphsHighest + 1);

        for (const ImWchar* src_range = src_tmp.SrcRanges; src_range[0] && src_range[1]; src_range += 2)
            for (unsigned int codepoint = src_range[0]; codepoint <= src_range[1]; codepoint += 1)
            {
                if (dst_tmp.GlyphsSet.TestBit(codepoint))    // Don't overwrite existing glyphs. We could make this an option for merge_mode (e.g. MergeOverwrite==true)
                    continue;
                if (!stbtt_FindGlyphIndex(&src_tmp.FontInfo, codepoint))    // It is actually in the font?
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
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        src_tmp.GlyphsList.reserve(src_tmp.GlyphsCount);
        UnpackBitVectorToFlatIndexList(&src_tmp.GlyphsSet, &src_tmp.GlyphsList);
        src_tmp.GlyphsSet.Clear();
        // IM_ASSERT(src_tmp.GlyphsList.size == src_tmp.GlyphsCount);
    }
    for (int dst_i = 0; dst_i < dst_tmp_array.size; dst_i += 1)
        dst_tmp_array[dst_i].GlyphsSet.Clear();
    dst_tmp_array.clear();

    // Allocate packing character data and flag packed characters buffer as non-packed (x0=y0=x1=y1=0)
    // (We technically don't need to zero-clear buf_rects, but let's do it for the sake of sanity)
    ImVector<stbrp_rect> buf_rects;
    ImVector<stbtt_packedchar> buf_packedchars;
    buf_rects.resize(total_glyphs_count);
    buf_packedchars.resize(total_glyphs_count);
    memset(buf_rects.data, 0, buf_rects.size_in_bytes());
    memset(buf_packedchars.data, 0, buf_packedchars.size_in_bytes());

    // 4. Gather glyphs sizes so we can pack them in our virtual canvas.
    int total_surface = 0;
    int buf_rects_out_n = 0;
    int buf_packedchars_out_n = 0;
    for (int src_i = 0; src_i < src_tmp_array.size; src_i += 1)
    {
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        if (src_tmp.GlyphsCount == 0)
            continue;

        src_tmp.Rects = &buf_rects[buf_rects_out_n];
        src_tmp.PackedChars = &buf_packedchars[buf_packedchars_out_n];
        buf_rects_out_n += src_tmp.GlyphsCount;
        buf_packedchars_out_n += src_tmp.GlyphsCount;

        // Convert our ranges in the format stb_truetype wants
        ImFontConfig& cfg = atlas.ConfigData[src_i];
        src_tmp.PackRange.font_size = cfg.sizePixels;
        src_tmp.PackRange.first_unicode_codepoint_in_range = 0;
        src_tmp.PackRange.array_of_unicode_codepoints = src_tmp.GlyphsList.data;
        src_tmp.PackRange.num_chars = src_tmp.GlyphsList.size;
        src_tmp.PackRange.chardata_for_range = src_tmp.PackedChars;
        src_tmp.PackRange.h_oversample = (unsigned char)cfg.OversampleH;
        src_tmp.PackRange.v_oversample = (unsigned char)cfg.OversampleV;

        // Gather the sizes of all rectangles we will need to pack (this loop is based on stbtt_PackFontRangesGatherRects)
        let scale = (cfg.sizePixels > 0) ? stbtt_ScaleForPixelHeight(&src_tmp.FontInfo, cfg.sizePixels) : stbtt_ScaleForMappingEmToPixels(&src_tmp.FontInfo, -cfg.sizePixels);
        let padding = atlas.TexGlyphPadding;
        for (int glyph_i = 0; glyph_i < src_tmp.GlyphsList.size; glyph_i += 1)
        {
            int x0, y0, x1, y1;
            let glyph_index_in_font = stbtt_FindGlyphIndex(&src_tmp.FontInfo, src_tmp.GlyphsList[glyph_i]);
            // IM_ASSERT(glyph_index_in_font != 0);
            stbtt_GetGlyphBitmapBoxSubpixel(&src_tmp.FontInfo, glyph_index_in_font, scale * cfg.OversampleH, scale * cfg.OversampleV, 0, 0, &x0, &y0, &x1, &y1);
            src_tmp.Rects[glyph_i].w = (stbrp_coord)(x1 - x0 + padding + cfg.OversampleH - 1);
            src_tmp.Rects[glyph_i].h = (stbrp_coord)(y1 - y0 + padding + cfg.OversampleV - 1);
            total_surface += src_tmp.Rects[glyph_i].w * src_tmp.Rects[glyph_i].h;
        }
    }

    // We need a width for the skyline algorithm, any width!
    // The exact width doesn't really matter much, but some API/GPU have texture size limitations and increasing width can decrease height.
    // User can override tex_desired_width and tex_glyph_padding if they wish, otherwise we use a simple heuristic to select the width based on expected surface.
    let surface_sqrt = ImSqrt((float)total_surface) + 1;
    atlas.TexHeight = 0;
    if (atlas.TexDesiredWidth > 0)
        atlas.TexWidth = atlas.TexDesiredWidth;
    else
        atlas.TexWidth = (surface_sqrt >= 4096 * 0.7) ? 4096 : (surface_sqrt >= 2048 * 0.7) ? 2048 : (surface_sqrt >= 1024 * 0.7) ? 1024 : 512;

    // 5. Start packing
    // Pack our extra data rectangles first, so it will be on the upper-left corner of our texture (UV will have small values).
    let TEX_HEIGHT_MAX = 1024 * 32;
    stbtt_pack_context spc = {};
    stbtt_PackBegin(&spc, None, atlas.TexWidth, TEX_HEIGHT_MAX, 0, atlas.TexGlyphPadding, None);
    ImFontAtlasBuildPackCustomRects(atlas, spc.pack_info);

    // 6. Pack each source font. No rendering yet, we are working with rectangles in an infinitely tall texture at this point.
    for (int src_i = 0; src_i < src_tmp_array.size; src_i += 1)
    {
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        if (src_tmp.GlyphsCount == 0)
            continue;

        stbrp_pack_rects((stbrp_context*)spc.pack_info, src_tmp.Rects, src_tmp.GlyphsCount);

        // Extend texture height and mark missing glyphs as non-packed so we won't render them.
        // FIXME: We are not handling packing failure here (would happen if we got off TEX_HEIGHT_MAX or if a single if larger than tex_width?)
        for (int glyph_i = 0; glyph_i < src_tmp.GlyphsCount; glyph_i += 1)
            if (src_tmp.Rects[glyph_i].was_packed)
                atlas.TexHeight = ImMax(atlas.TexHeight, src_tmp.Rects[glyph_i].y + src_tmp.Rects[glyph_i].h);
    }

    // 7. Allocate texture
    atlas.TexHeight = (atlas.flags & ImFontAtlasFlags_NoPowerOfTwoHeight) ? (atlas.TexHeight + 1) : ImUpperPowerOfTwo(atlas.TexHeight);
    atlas.TexUvScale = Vector2D::new(1.0 / atlas.TexWidth, 1.0 / atlas.TexHeight);
    atlas.TexPixelsAlpha8 = (unsigned char*)IM_ALLOC(atlas.TexWidth * atlas.TexHeight);
    memset(atlas.TexPixelsAlpha8, 0, atlas.TexWidth * atlas.TexHeight);
    spc.pixels = atlas.TexPixelsAlpha8;
    spc.height = atlas.TexHeight;

    // 8. Render/rasterize font characters into the texture
    for (int src_i = 0; src_i < src_tmp_array.size; src_i += 1)
    {
        ImFontConfig& cfg = atlas.ConfigData[src_i];
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        if (src_tmp.GlyphsCount == 0)
            continue;

        stbtt_PackFontRangesRenderIntoRects(&spc, &src_tmp.FontInfo, &src_tmp.PackRange, 1, src_tmp.Rects);

        // Apply multiply operator
        if (cfg.RasterizerMultiply != 1.0)
        {
            unsigned char multiply_table[256];
            ImFontAtlasBuildMultiplyCalcLookupTable(multiply_table, cfg.RasterizerMultiply);
            stbrp_rect* r = &src_tmp.Rects[0];
            for (int glyph_i = 0; glyph_i < src_tmp.GlyphsCount; glyph_i += 1, r += 1)
                if (r.was_packed)
                    ImFontAtlasBuildMultiplyRectAlpha8(multiply_table, atlas.TexPixelsAlpha8, r.x, r.y, r.w, r.h, atlas.TexWidth * 1);
        }
        src_tmp.Rects = None;
    }

    // End packing
    stbtt_PackEnd(&spc);
    buf_rects.clear();

    // 9. Setup ImFont and glyphs for runtime
    for (int src_i = 0; src_i < src_tmp_array.size; src_i += 1)
    {
        ImFontBuildSrcData& src_tmp = src_tmp_array[src_i];
        if (src_tmp.GlyphsCount == 0)
            continue;

        // When merging fonts with merge_mode=true:
        // - We can have multiple input fonts writing into a same destination font.
        // - dst_font->config_data is != from cfg which is our source configuration.
        ImFontConfig& cfg = atlas.ConfigData[src_i];
        ImFont* dst_font = cfg.DstFont;

        let font_scale = stbtt_ScaleForPixelHeight(&src_tmp.FontInfo, cfg.sizePixels);
        int unscaled_ascent, unscaled_descent, unscaled_line_gap;
        stbtt_GetFontVMetrics(&src_tmp.FontInfo, &unscaled_ascent, &unscaled_descent, &unscaled_line_gap);

        let ascent = f32::floor(unscaled_ascent * font_scale + ((unscaled_ascent > 0.0) ? +1 : -1));
        let descent = f32::floor(unscaled_descent * font_scale + ((unscaled_descent > 0.0) ? +1 : -1));
        ImFontAtlasBuildSetupFont(atlas, dst_font, &cfg, ascent, descent);
        let font_off_x = cfg.GlyphOffset.x;
        let font_off_y = cfg.GlyphOffset.y + IM_ROUND(dst_font.Ascent);

        for (int glyph_i = 0; glyph_i < src_tmp.GlyphsCount; glyph_i += 1)
        {
            // Register glyph
            let codepoint = src_tmp.GlyphsList[glyph_i];
            const stbtt_packedchar& pc = src_tmp.PackedChars[glyph_i];
            stbtt_aligned_quad q;
            let unused_x =  0.0, unused_y = 0.0;
            stbtt_GetPackedQuad(src_tmp.PackedChars, atlas.TexWidth, atlas.TexHeight, glyph_i, &unused_x, &unused_y, &q, 0);
            dst_font.AddGlyph(&cfg, (ImWchar)codepoint, q.x0 + font_off_x, q.y0 + font_off_y, q.x1 + font_off_x, q.y1 + font_off_y, q.s0, q.t0, q.s1, q.t1, pc.xadvance);
        }
    }

    // Cleanup
    src_tmp_array.clear_destruct();

    ImFontAtlasBuildFinish(atlas);
    return true;
}

const ImFontBuilderIO* ImFontAtlasGetBuilderForStbTruetype()
{
    static ImFontBuilderIO io;
    io.FontBuilder_Build = ImFontAtlasBuildWithStbTruetype;
    return &io;
}

 // IMGUI_ENABLE_STB_TRUETYPE

void ImFontAtlasBuildSetupFont(ImFontAtlas* atlas, ImFont* font, ImFontConfig* font_config, float ascent, float descent)
{
    if (!font_config.MergeMode)
    {
        font.ClearOutputData();
        font.font_size = font_config.sizePixels;
        font.ConfigData = font_config;
        font.ConfigDataCount = 0;
        font.container_atlas = atlas;
        font.Ascent = ascent;
        font.Descent = descent;
    }
    font.ConfigDataCount += 1;
}

void ImFontAtlasBuildPackCustomRects(ImFontAtlas* atlas, void* stbrp_context_opaque)
{
    stbrp_context* pack_context = (stbrp_context*)stbrp_context_opaque;
    // IM_ASSERT(pack_context != None);

    ImVector<ImFontAtlasCustomRect>& user_rects = atlas.CustomRects;
    // IM_ASSERT(user_rects.size >= 1); // We expect at least the default custom rects to be registered, else something went wrong.

    ImVector<stbrp_rect> pack_rects;
    pack_rects.resize(user_rects.size);
    memset(pack_rects.data, 0, pack_rects.size_in_bytes());
    for (int i = 0; i < user_rects.size; i += 1)
    {
        pack_rects[i].w = user_rects[i].Width;
        pack_rects[i].h = user_rects[i].Height;
    }
    stbrp_pack_rects(pack_context, &pack_rects[0], pack_rects.size);
    for (int i = 0; i < pack_rects.size; i += 1)
        if (pack_rects[i].was_packed)
        {
            user_rects[i].X = (unsigned short)pack_rects[i].x;
            user_rects[i].Y = (unsigned short)pack_rects[i].y;
            // IM_ASSERT(pack_rects[i].w == user_rects[i].Width && pack_rects[i].h == user_rects[i].Height);
            atlas.TexHeight = ImMax(atlas.TexHeight, pack_rects[i].y + pack_rects[i].h);
        }
}

void ImFontAtlasBuildRender8bppRectFromString(ImFontAtlas* atlas, int x, int y, int w, int h, const char* in_str, char in_marker_char, unsigned char in_marker_pixel_value)
{
    // IM_ASSERT(x >= 0 && x + w <= atlas.TexWidth);
    // IM_ASSERT(y >= 0 && y + h <= atlas.TexHeight);
    unsigned char* out_pixel = atlas.TexPixelsAlpha8 + x + (y * atlas.TexWidth);
    for (int off_y = 0; off_y < h; off_y += 1, out_pixel += atlas.TexWidth, in_str += w)
        for (int off_x = 0; off_x < w; off_x += 1)
            out_pixel[off_x] = (in_str[off_x] == in_marker_char) ? in_marker_pixel_value : 0x00;
}

void ImFontAtlasBuildRender32bppRectFromString(ImFontAtlas* atlas, int x, int y, int w, int h, const char* in_str, char in_marker_char, unsigned int in_marker_pixel_value)
{
    // IM_ASSERT(x >= 0 && x + w <= atlas.TexWidth);
    // IM_ASSERT(y >= 0 && y + h <= atlas.TexHeight);
    unsigned int* out_pixel = atlas.TexPixelsRGBA32 + x + (y * atlas.TexWidth);
    for (int off_y = 0; off_y < h; off_y += 1, out_pixel += atlas.TexWidth, in_str += w)
        for (int off_x = 0; off_x < w; off_x += 1)
            out_pixel[off_x] = (in_str[off_x] == in_marker_char) ? in_marker_pixel_value : IM_COL32_BLACK_TRANS;
}

static void ImFontAtlasBuildRenderDefaultTexData(ImFontAtlas* atlas)
{
    ImFontAtlasCustomRect* r = atlas.GetCustomRectByIndex(atlas.PackIdMouseCursors);
    // IM_ASSERT(r.IsPacked());

    let w = atlas.TexWidth;
    if (!(atlas.flags & ImFontAtlasFlags_NoMouseCursors))
    {
        // Render/copy pixels
        // IM_ASSERT(r.Width == FONT_ATLAS_DEFAULT_TEX_DATA_W * 2 + 1 && r.Height == FONT_ATLAS_DEFAULT_TEX_DATA_H);
        let x_for_white = r.X;
        let x_for_black = r.X + FONT_ATLAS_DEFAULT_TEX_DATA_W + 1;
        if (atlas.TexPixelsAlpha8 != None)
        {
            ImFontAtlasBuildRender8bppRectFromString(atlas, x_for_white, r.Y, FONT_ATLAS_DEFAULT_TEX_DATA_W, FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS, '.', 0xFF);
            ImFontAtlasBuildRender8bppRectFromString(atlas, x_for_black, r.Y, FONT_ATLAS_DEFAULT_TEX_DATA_W, FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS, 'X', 0xFF);
        }
        else
        {
            ImFontAtlasBuildRender32bppRectFromString(atlas, x_for_white, r.Y, FONT_ATLAS_DEFAULT_TEX_DATA_W, FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS, '.', IM_COL32_WHITE);
            ImFontAtlasBuildRender32bppRectFromString(atlas, x_for_black, r.Y, FONT_ATLAS_DEFAULT_TEX_DATA_W, FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS, 'X', IM_COL32_WHITE);
        }
    }
    else
    {
        // Render 4 white pixels
        // IM_ASSERT(r.Width == 2 && r.Height == 2);
        let offset = r.X + r.Y * w;
        if (atlas.TexPixelsAlpha8 != None)
        {
            atlas.TexPixelsAlpha8[offset] = atlas.TexPixelsAlpha8[offset + 1] = atlas.TexPixelsAlpha8[offset + w] = atlas.TexPixelsAlpha8[offset + w + 1] = 0xFF;
        }
        else
        {
            atlas.TexPixelsRGBA32[offset] = atlas.TexPixelsRGBA32[offset + 1] = atlas.TexPixelsRGBA32[offset + w] = atlas.TexPixelsRGBA32[offset + w + 1] = IM_COL32_WHITE;
        }
    }
    atlas.TexUvWhitePixel = Vector2D::new((r.X + 0.5) * atlas.TexUvScale.x, (r.Y + 0.5) * atlas.TexUvScale.y);
}

static void ImFontAtlasBuildRenderLinesTexData(ImFontAtlas* atlas)
{
    if (atlas.flags & FontAtlasFlags::NoBakedLines)
        return;

    // This generates a triangular shape in the texture, with the various line widths stacked on top of each other to allow interpolation between them
    ImFontAtlasCustomRect* r = atlas.GetCustomRectByIndex(atlas.PackIdLines);
    // IM_ASSERT(r.IsPacked());
    for (unsigned int n = 0; n < IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 1; n += 1) // +1 because of the zero-width row
    {
        // Each line consists of at least two empty pixels at the ends, with a line of solid pixels in the middle
        unsigned int y = n;
        unsigned int line_width = n;
        unsigned int pad_left = (r.Width - line_width) / 2;
        unsigned int pad_right = r.Width - (pad_left + line_width);

        // Write each slice
        // IM_ASSERT(pad_left + line_width + pad_right == r.Width && y < r.Height); // Make sure we're inside the texture bounds before we start writing pixels
        if (atlas.TexPixelsAlpha8 != None)
        {
            unsigned char* write_ptr = &atlas.TexPixelsAlpha8[r.X + ((r.Y + y) * atlas.TexWidth)];
            for (unsigned int i = 0; i < pad_left; i += 1)
                *(write_ptr + i) = 0x00;

            for (unsigned int i = 0; i < line_width; i += 1)
                *(write_ptr + pad_left + i) = 0xFF;

            for (unsigned int i = 0; i < pad_right; i += 1)
                *(write_ptr + pad_left + line_width + i) = 0x00;
        }
        else
        {
            unsigned int* write_ptr = &atlas.TexPixelsRGBA32[r.X + ((r.Y + y) * atlas.TexWidth)];
            for (unsigned int i = 0; i < pad_left; i += 1)
                *(write_ptr + i) = IM_COL32(255, 255, 255, 0);

            for (unsigned int i = 0; i < line_width; i += 1)
                *(write_ptr + pad_left + i) = IM_COL32_WHITE;

            for (unsigned int i = 0; i < pad_right; i += 1)
                *(write_ptr + pad_left + line_width + i) = IM_COL32(255, 255, 255, 0);
        }

        // Calculate UVs for this line
        Vector2D uv0 = Vector2D::new((float)(r.X + pad_left - 1), (r.Y + y)) * atlas.TexUvScale;
        Vector2D uv1 = Vector2D::new((float)(r.X + pad_left + line_width + 1), (r.Y + y + 1)) * atlas.TexUvScale;
        let half_v =  (uv0.y + uv1.y) * 0.5; // Calculate a constant V in the middle of the row to avoid sampling artifacts
        atlas.TexUvLines[n] = Vector4D(uv0.x, half_v, uv1.x, half_v);
    }
}

// Note: this is called / shared by both the stb_truetype and the FreeType builder
void ImFontAtlasBuildInit(ImFontAtlas* atlas)
{
    // Register texture region for mouse cursors or standard white pixels
    if (atlas.PackIdMouseCursors < 0)
    {
        if (!(atlas.flags & ImFontAtlasFlags_NoMouseCursors))
            atlas.PackIdMouseCursors = atlas.AddCustomRectRegular(FONT_ATLAS_DEFAULT_TEX_DATA_W * 2 + 1, FONT_ATLAS_DEFAULT_TEX_DATA_H);
        else
            atlas.PackIdMouseCursors = atlas.AddCustomRectRegular(2, 2);
    }

    // Register texture region for thick lines
    // The +2 here is to give space for the end caps, whilst height +1 is to accommodate the fact we have a zero-width row
    if (atlas.PackIdLines < 0)
    {
        if (!(atlas.flags & FontAtlasFlags::NoBakedLines))
            atlas.PackIdLines = atlas.AddCustomRectRegular(IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 2, IM_DRAWLIST_TEX_LINES_WIDTH_MAX + 1);
    }
}

// This is called/shared by both the stb_truetype and the FreeType builder.
void ImFontAtlasBuildFinish(ImFontAtlas* atlas)
{
    // Render into our custom data blocks
    // IM_ASSERT(atlas.TexPixelsAlpha8 != None || atlas.TexPixelsRGBA32 != None);
    ImFontAtlasBuildRenderDefaultTexData(atlas);
    ImFontAtlasBuildRenderLinesTexData(atlas);

    // Register custom rectangle glyphs
    for (int i = 0; i < atlas.CustomRects.size; i += 1)
    {
        const ImFontAtlasCustomRect* r = &atlas.CustomRects[i];
        if (r.Font == None || r.GlyphID == 0)
            continue;

        // Will ignore ImFontConfig settings: glyph_min_advance_x, GlyphMinAdvanceY, glyph_extra_spacing, pixel_snap_h
        // IM_ASSERT(r.Font.container_atlas == atlas);
        Vector2D uv0, uv1;
        atlas.CalcCustomRectUV(r, &uv0, &uv1);
        r.Font.AddGlyph(None, (ImWchar)r.GlyphID, r.GlyphOffset.x, r.GlyphOffset.y, r.GlyphOffset.x + r.Width, r.GlyphOffset.y + r.Height, uv0.x, uv0.y, uv1.x, uv1.y, r.GlyphAdvanceX);
    }

    // build all fonts lookup tables
    for (int i = 0; i < atlas.Fonts.size; i += 1)
        if (atlas.Fonts[i].DirtyLookupTables)
            atlas.Fonts[i].BuildLookupTable();

    atlas.TexReady = true;
}

// Retrieve list of range (2 int per range, values are inclusive)
const ImWchar*   ImFontAtlas::GetGlyphRangesDefault()
{
    static const ImWchar ranges[] =
    {
        0x0020, 0x00FF, // Basic Latin + Latin Supplement
        0,
    };
    return &ranges[0];
}

const ImWchar*  ImFontAtlas::GetGlyphRangesKorean()
{
    static const ImWchar ranges[] =
    {
        0x0020, 0x00FF, // Basic Latin + Latin Supplement
        0x3131, 0x3163, // Korean alphabets
        0xAC00, 0xD7A3, // Korean characters
        0xFFFD, 0xFFFD, // Invalid
        0,
    };
    return &ranges[0];
}

const ImWchar*  ImFontAtlas::GetGlyphRangesChineseFull()
{
    static const ImWchar ranges[] =
    {
        0x0020, 0x00FF, // Basic Latin + Latin Supplement
        0x2000, 0x206F, // General Punctuation
        0x3000, 0x30FF, // CJK Symbols and Punctuations, Hiragana, Katakana
        0x31F0, 0x31FF, // Katakana Phonetic Extensions
        0xFF00, 0xFFEF, // Half-width characters
        0xFFFD, 0xFFFD, // Invalid
        0x4e00, 0x9FAF, // CJK Ideograms
        0,
    };
    return &ranges[0];
}


const ImWchar*  ImFontAtlas::GetGlyphRangesChineseSimplifiedCommon()
{
    // Store 2500 regularly used characters for Simplified Chinese.
    // Sourced from https://zh.wiktionary.org/wiki/%E9%99%84%E5%BD%95:%E7%8E%B0%E4%BB%A3%E6%B1%89%E8%AF%AD%E5%B8%B8%E7%94%A8%E5%AD%97%E8%A1%A8
    // This table covers 97.97% of all characters used during the month in July, 1987.
    // You can use ImFontGlyphRangesBuilder to create your own ranges derived from this, by merging existing ranges or adding new characters.
    // (Stored as accumulative offsets from the initial unicode codepoint 0x4E00. This encoding is designed to helps us compact the source code size.)
    static const short accumulative_offsets_from_0x4E00[] =
    {
        0,1,2,4,1,1,1,1,2,1,3,2,1,2,2,1,1,1,1,1,5,2,1,2,3,3,3,2,2,4,1,1,1,2,1,5,2,3,1,2,1,2,1,1,2,1,1,2,2,1,4,1,1,1,1,5,10,1,2,19,2,1,2,1,2,1,2,1,2,
        1,5,1,6,3,2,1,2,2,1,1,1,4,8,5,1,1,4,1,1,3,1,2,1,5,1,2,1,1,1,10,1,1,5,2,4,6,1,4,2,2,2,12,2,1,1,6,1,1,1,4,1,1,4,6,5,1,4,2,2,4,10,7,1,1,4,2,4,
        2,1,4,3,6,10,12,5,7,2,14,2,9,1,1,6,7,10,4,7,13,1,5,4,8,4,1,1,2,28,5,6,1,1,5,2,5,20,2,2,9,8,11,2,9,17,1,8,6,8,27,4,6,9,20,11,27,6,68,2,2,1,1,
        1,2,1,2,2,7,6,11,3,3,1,1,3,1,2,1,1,1,1,1,3,1,1,8,3,4,1,5,7,2,1,4,4,8,4,2,1,2,1,1,4,5,6,3,6,2,12,3,1,3,9,2,4,3,4,1,5,3,3,1,3,7,1,5,1,1,1,1,2,
        3,4,5,2,3,2,6,1,1,2,1,7,1,7,3,4,5,15,2,2,1,5,3,22,19,2,1,1,1,1,2,5,1,1,1,6,1,1,12,8,2,9,18,22,4,1,1,5,1,16,1,2,7,10,15,1,1,6,2,4,1,2,4,1,6,
        1,1,3,2,4,1,6,4,5,1,2,1,1,2,1,10,3,1,3,2,1,9,3,2,5,7,2,19,4,3,6,1,1,1,1,1,4,3,2,1,1,1,2,5,3,1,1,1,2,2,1,1,2,1,1,2,1,3,1,1,1,3,7,1,4,1,1,2,1,
        1,2,1,2,4,4,3,8,1,1,1,2,1,3,5,1,3,1,3,4,6,2,2,14,4,6,6,11,9,1,15,3,1,28,5,2,5,5,3,1,3,4,5,4,6,14,3,2,3,5,21,2,7,20,10,1,2,19,2,4,28,28,2,3,
        2,1,14,4,1,26,28,42,12,40,3,52,79,5,14,17,3,2,2,11,3,4,6,3,1,8,2,23,4,5,8,10,4,2,7,3,5,1,1,6,3,1,2,2,2,5,28,1,1,7,7,20,5,3,29,3,17,26,1,8,4,
        27,3,6,11,23,5,3,4,6,13,24,16,6,5,10,25,35,7,3,2,3,3,14,3,6,2,6,1,4,2,3,8,2,1,1,3,3,3,4,1,1,13,2,2,4,5,2,1,14,14,1,2,2,1,4,5,2,3,1,14,3,12,
        3,17,2,16,5,1,2,1,8,9,3,19,4,2,2,4,17,25,21,20,28,75,1,10,29,103,4,1,2,1,1,4,2,4,1,2,3,24,2,2,2,1,1,2,1,3,8,1,1,1,2,1,1,3,1,1,1,6,1,5,3,1,1,
        1,3,4,1,1,5,2,1,5,6,13,9,16,1,1,1,1,3,2,3,2,4,5,2,5,2,2,3,7,13,7,2,2,1,1,1,1,2,3,3,2,1,6,4,9,2,1,14,2,14,2,1,18,3,4,14,4,11,41,15,23,15,23,
        176,1,3,4,1,1,1,1,5,3,1,2,3,7,3,1,1,2,1,2,4,4,6,2,4,1,9,7,1,10,5,8,16,29,1,1,2,2,3,1,3,5,2,4,5,4,1,1,2,2,3,3,7,1,6,10,1,17,1,44,4,6,2,1,1,6,
        5,4,2,10,1,6,9,2,8,1,24,1,2,13,7,8,8,2,1,4,1,3,1,3,3,5,2,5,10,9,4,9,12,2,1,6,1,10,1,1,7,7,4,10,8,3,1,13,4,3,1,6,1,3,5,2,1,2,17,16,5,2,16,6,
        1,4,2,1,3,3,6,8,5,11,11,1,3,3,2,4,6,10,9,5,7,4,7,4,7,1,1,4,2,1,3,6,8,7,1,6,11,5,5,3,24,9,4,2,7,13,5,1,8,82,16,61,1,1,1,4,2,2,16,10,3,8,1,1,
        6,4,2,1,3,1,1,1,4,3,8,4,2,2,1,1,1,1,1,6,3,5,1,1,4,6,9,2,1,1,1,2,1,7,2,1,6,1,5,4,4,3,1,8,1,3,3,1,3,2,2,2,2,3,1,6,1,2,1,2,1,3,7,1,8,2,1,2,1,5,
        2,5,3,5,10,1,2,1,1,3,2,5,11,3,9,3,5,1,1,5,9,1,2,1,5,7,9,9,8,1,3,3,3,6,8,2,3,2,1,1,32,6,1,2,15,9,3,7,13,1,3,10,13,2,14,1,13,10,2,1,3,10,4,15,
        2,15,15,10,1,3,9,6,9,32,25,26,47,7,3,2,3,1,6,3,4,3,2,8,5,4,1,9,4,2,2,19,10,6,2,3,8,1,2,2,4,2,1,9,4,4,4,6,4,8,9,2,3,1,1,1,1,3,5,5,1,3,8,4,6,
        2,1,4,12,1,5,3,7,13,2,5,8,1,6,1,2,5,14,6,1,5,2,4,8,15,5,1,23,6,62,2,10,1,1,8,1,2,2,10,4,2,2,9,2,1,1,3,2,3,1,5,3,3,2,1,3,8,1,1,1,11,3,1,1,4,
        3,7,1,14,1,2,3,12,5,2,5,1,6,7,5,7,14,11,1,3,1,8,9,12,2,1,11,8,4,4,2,6,10,9,13,1,1,3,1,5,1,3,2,4,4,1,18,2,3,14,11,4,29,4,2,7,1,3,13,9,2,2,5,
        3,5,20,7,16,8,5,72,34,6,4,22,12,12,28,45,36,9,7,39,9,191,1,1,1,4,11,8,4,9,2,3,22,1,1,1,1,4,17,1,7,7,1,11,31,10,2,4,8,2,3,2,1,4,2,16,4,32,2,
        3,19,13,4,9,1,5,2,14,8,1,1,3,6,19,6,5,1,16,6,2,10,8,5,1,2,3,1,5,5,1,11,6,6,1,3,3,2,6,3,8,1,1,4,10,7,5,7,7,5,8,9,2,1,3,4,1,1,3,1,3,3,2,6,16,
        1,4,6,3,1,10,6,1,3,15,2,9,2,10,25,13,9,16,6,2,2,10,11,4,3,9,1,2,6,6,5,4,30,40,1,10,7,12,14,33,6,3,6,7,3,1,3,1,11,14,4,9,5,12,11,49,18,51,31,
        140,31,2,2,1,5,1,8,1,10,1,4,4,3,24,1,10,1,3,6,6,16,3,4,5,2,1,4,2,57,10,6,22,2,22,3,7,22,6,10,11,36,18,16,33,36,2,5,5,1,1,1,4,10,1,4,13,2,7,
        5,2,9,3,4,1,7,43,3,7,3,9,14,7,9,1,11,1,1,3,7,4,18,13,1,14,1,3,6,10,73,2,2,30,6,1,11,18,19,13,22,3,46,42,37,89,7,3,16,34,2,2,3,9,1,7,1,1,1,2,
        2,4,10,7,3,10,3,9,5,28,9,2,6,13,7,3,1,3,10,2,7,2,11,3,6,21,54,85,2,1,4,2,2,1,39,3,21,2,2,5,1,1,1,4,1,1,3,4,15,1,3,2,4,4,2,3,8,2,20,1,8,7,13,
        4,1,26,6,2,9,34,4,21,52,10,4,4,1,5,12,2,11,1,7,2,30,12,44,2,30,1,1,3,6,16,9,17,39,82,2,2,24,7,1,7,3,16,9,14,44,2,1,2,1,2,3,5,2,4,1,6,7,5,3,
        2,6,1,11,5,11,2,1,18,19,8,1,3,24,29,2,1,3,5,2,2,1,13,6,5,1,46,11,3,5,1,1,5,8,2,10,6,12,6,3,7,11,2,4,16,13,2,5,1,1,2,2,5,2,28,5,2,23,10,8,4,
        4,22,39,95,38,8,14,9,5,1,13,5,4,3,13,12,11,1,9,1,27,37,2,5,4,4,63,211,95,2,2,2,1,3,5,2,1,1,2,2,1,1,1,3,2,4,1,2,1,1,5,2,2,1,1,2,3,1,3,1,1,1,
        3,1,4,2,1,3,6,1,1,3,7,15,5,3,2,5,3,9,11,4,2,22,1,6,3,8,7,1,4,28,4,16,3,3,25,4,4,27,27,1,4,1,2,2,7,1,3,5,2,28,8,2,14,1,8,6,16,25,3,3,3,14,3,
        3,1,1,2,1,4,6,3,8,4,1,1,1,2,3,6,10,6,2,3,18,3,2,5,5,4,3,1,5,2,5,4,23,7,6,12,6,4,17,11,9,5,1,1,10,5,12,1,1,11,26,33,7,3,6,1,17,7,1,5,12,1,11,
        2,4,1,8,14,17,23,1,2,1,7,8,16,11,9,6,5,2,6,4,16,2,8,14,1,11,8,9,1,1,1,9,25,4,11,19,7,2,15,2,12,8,52,7,5,19,2,16,4,36,8,1,16,8,24,26,4,6,2,9,
        5,4,36,3,28,12,25,15,37,27,17,12,59,38,5,32,127,1,2,9,17,14,4,1,2,1,1,8,11,50,4,14,2,19,16,4,17,5,4,5,26,12,45,2,23,45,104,30,12,8,3,10,2,2,
        3,3,1,4,20,7,2,9,6,15,2,20,1,3,16,4,11,15,6,134,2,5,59,1,2,2,2,1,9,17,3,26,137,10,211,59,1,2,4,1,4,1,1,1,2,6,2,3,1,1,2,3,2,3,1,3,4,4,2,3,3,
        1,4,3,1,7,2,2,3,1,2,1,3,3,3,2,2,3,2,1,3,14,6,1,3,2,9,6,15,27,9,34,145,1,1,2,1,1,1,1,2,1,1,1,1,2,2,2,3,1,2,1,1,1,2,3,5,8,3,5,2,4,1,3,2,2,2,12,
        4,1,1,1,10,4,5,1,20,4,16,1,15,9,5,12,2,9,2,5,4,2,26,19,7,1,26,4,30,12,15,42,1,6,8,172,1,1,4,2,1,1,11,2,2,4,2,1,2,1,10,8,1,2,1,4,5,1,2,5,1,8,
        4,1,3,4,2,1,6,2,1,3,4,1,2,1,1,1,1,12,5,7,2,4,3,1,1,1,3,3,6,1,2,2,3,3,3,2,1,2,12,14,11,6,6,4,12,2,8,1,7,10,1,35,7,4,13,15,4,3,23,21,28,52,5,
        26,5,6,1,7,10,2,7,53,3,2,1,1,1,2,163,532,1,10,11,1,3,3,4,8,2,8,6,2,2,23,22,4,2,2,4,2,1,3,1,3,3,5,9,8,2,1,2,8,1,10,2,12,21,20,15,105,2,3,1,1,
        3,2,3,1,1,2,5,1,4,15,11,19,1,1,1,1,5,4,5,1,1,2,5,3,5,12,1,2,5,1,11,1,1,15,9,1,4,5,3,26,8,2,1,3,1,1,15,19,2,12,1,2,5,2,7,2,19,2,20,6,26,7,5,
        2,2,7,34,21,13,70,2,128,1,1,2,1,1,2,1,1,3,2,2,2,15,1,4,1,3,4,42,10,6,1,49,85,8,1,2,1,1,4,4,2,3,6,1,5,7,4,3,211,4,1,2,1,2,5,1,2,4,2,2,6,5,6,
        10,3,4,48,100,6,2,16,296,5,27,387,2,2,3,7,16,8,5,38,15,39,21,9,10,3,7,59,13,27,21,47,5,21,6
    };
    static ImWchar base_ranges[] = // not zero-terminated
    {
        0x0020, 0x00FF, // Basic Latin + Latin Supplement
        0x2000, 0x206F, // General Punctuation
        0x3000, 0x30FF, // CJK Symbols and Punctuations, Hiragana, Katakana
        0x31F0, 0x31FF, // Katakana Phonetic Extensions
        0xFF00, 0xFFEF, // Half-width characters
        0xFFFD, 0xFFFD  // Invalid
    };
    static ImWchar full_ranges[IM_ARRAYSIZE(base_ranges) + IM_ARRAYSIZE(accumulative_offsets_from_0x4E00) * 2 + 1] = { 0 };
    if (!full_ranges[0])
    {
        memcpy(full_ranges, base_ranges, sizeof(base_ranges));
        UnpackAccumulativeOffsetsIntoRanges(0x4E00, accumulative_offsets_from_0x4E00, IM_ARRAYSIZE(accumulative_offsets_from_0x4E00), full_ranges + IM_ARRAYSIZE(base_ranges));
    }
    return &full_ranges[0];
}

const ImWchar*  ImFontAtlas::GetGlyphRangesJapanese()
{
    // 2999 ideograms code points for Japanese
    // - 2136 Joyo (meaning "for regular use" or "for common use") Kanji code points
    // - 863 Jinmeiyo (meaning "for personal name") Kanji code points
    // - Sourced from the character information database of the Information-technology Promotion Agency, Japan
    //   - https://mojikiban.ipa.go.jp/mji/
    //   - Available under the terms of the Creative Commons Attribution-ShareAlike 2.1 Japan (CC BY-SA 2.1 JP).
    //     - https://creativecommons.org/licenses/by-sa/2.1/jp/deed.en
    //     - https://creativecommons.org/licenses/by-sa/2.1/jp/legalcode
    //   - You can generate this code by the script at:
    //     - https://github.com/vaiorabbit/everyday_use_kanji
    // - References:
    //   - List of Joyo Kanji
    //     - (Official list by the Agency for Cultural Affairs) https://www.bunka.go.jp/kokugo_nihongo/sisaku/joho/joho/kakuki/14/tosin02/index.html
    //     - (Wikipedia) https://en.wikipedia.org/wiki/List_of_j%C5%8Dy%C5%8D_kanji
    //   - List of Jinmeiyo Kanji
    //     - (Official list by the Ministry of Justice) http://www.moj.go.jp/MINJI/minji86.html
    //     - (Wikipedia) https://en.wikipedia.org/wiki/Jinmeiy%C5%8D_kanji
    // - Missing 1 Joyo Kanji: U+20B9F (Kun'yomi: Shikaru, On'yomi: Shitsu,shichi), see https://github.com/ocornut/imgui/pull/3627 for details.
    // You can use ImFontGlyphRangesBuilder to create your own ranges derived from this, by merging existing ranges or adding new characters.
    // (Stored as accumulative offsets from the initial unicode codepoint 0x4E00. This encoding is designed to helps us compact the source code size.)
    static const short accumulative_offsets_from_0x4E00[] =
    {
        0,1,2,4,1,1,1,1,2,1,3,3,2,2,1,5,3,5,7,5,6,1,2,1,7,2,6,3,1,8,1,1,4,1,1,18,2,11,2,6,2,1,2,1,5,1,2,1,3,1,2,1,2,3,3,1,1,2,3,1,1,1,12,7,9,1,4,5,1,
        1,2,1,10,1,1,9,2,2,4,5,6,9,3,1,1,1,1,9,3,18,5,2,2,2,2,1,6,3,7,1,1,1,1,2,2,4,2,1,23,2,10,4,3,5,2,4,10,2,4,13,1,6,1,9,3,1,1,6,6,7,6,3,1,2,11,3,
        2,2,3,2,15,2,2,5,4,3,6,4,1,2,5,2,12,16,6,13,9,13,2,1,1,7,16,4,7,1,19,1,5,1,2,2,7,7,8,2,6,5,4,9,18,7,4,5,9,13,11,8,15,2,1,1,1,2,1,2,2,1,2,2,8,
        2,9,3,3,1,1,4,4,1,1,1,4,9,1,4,3,5,5,2,7,5,3,4,8,2,1,13,2,3,3,1,14,1,1,4,5,1,3,6,1,5,2,1,1,3,3,3,3,1,1,2,7,6,6,7,1,4,7,6,1,1,1,1,1,12,3,3,9,5,
        2,6,1,5,6,1,2,3,18,2,4,14,4,1,3,6,1,1,6,3,5,5,3,2,2,2,2,12,3,1,4,2,3,2,3,11,1,7,4,1,2,1,3,17,1,9,1,24,1,1,4,2,2,4,1,2,7,1,1,1,3,1,2,2,4,15,1,
        1,2,1,1,2,1,5,2,5,20,2,5,9,1,10,8,7,6,1,1,1,1,1,1,6,2,1,2,8,1,1,1,1,5,1,1,3,1,1,1,1,3,1,1,12,4,1,3,1,1,1,1,1,10,3,1,7,5,13,1,2,3,4,6,1,1,30,
        2,9,9,1,15,38,11,3,1,8,24,7,1,9,8,10,2,1,9,31,2,13,6,2,9,4,49,5,2,15,2,1,10,2,1,1,1,2,2,6,15,30,35,3,14,18,8,1,16,10,28,12,19,45,38,1,3,2,3,
        13,2,1,7,3,6,5,3,4,3,1,5,7,8,1,5,3,18,5,3,6,1,21,4,24,9,24,40,3,14,3,21,3,2,1,2,4,2,3,1,15,15,6,5,1,1,3,1,5,6,1,9,7,3,3,2,1,4,3,8,21,5,16,4,
        5,2,10,11,11,3,6,3,2,9,3,6,13,1,2,1,1,1,1,11,12,6,6,1,4,2,6,5,2,1,1,3,3,6,13,3,1,1,5,1,2,3,3,14,2,1,2,2,2,5,1,9,5,1,1,6,12,3,12,3,4,13,2,14,
        2,8,1,17,5,1,16,4,2,2,21,8,9,6,23,20,12,25,19,9,38,8,3,21,40,25,33,13,4,3,1,4,1,2,4,1,2,5,26,2,1,1,2,1,3,6,2,1,1,1,1,1,1,2,3,1,1,1,9,2,3,1,1,
        1,3,6,3,2,1,1,6,6,1,8,2,2,2,1,4,1,2,3,2,7,3,2,4,1,2,1,2,2,1,1,1,1,1,3,1,2,5,4,10,9,4,9,1,1,1,1,1,1,5,3,2,1,6,4,9,6,1,10,2,31,17,8,3,7,5,40,1,
        7,7,1,6,5,2,10,7,8,4,15,39,25,6,28,47,18,10,7,1,3,1,1,2,1,1,1,3,3,3,1,1,1,3,4,2,1,4,1,3,6,10,7,8,6,2,2,1,3,3,2,5,8,7,9,12,2,15,1,1,4,1,2,1,1,
        1,3,2,1,3,3,5,6,2,3,2,10,1,4,2,8,1,1,1,11,6,1,21,4,16,3,1,3,1,4,2,3,6,5,1,3,1,1,3,3,4,6,1,1,10,4,2,7,10,4,7,4,2,9,4,3,1,1,1,4,1,8,3,4,1,3,1,
        6,1,4,2,1,4,7,2,1,8,1,4,5,1,1,2,2,4,6,2,7,1,10,1,1,3,4,11,10,8,21,4,6,1,3,5,2,1,2,28,5,5,2,3,13,1,2,3,1,4,2,1,5,20,3,8,11,1,3,3,3,1,8,10,9,2,
        10,9,2,3,1,1,2,4,1,8,3,6,1,7,8,6,11,1,4,29,8,4,3,1,2,7,13,1,4,1,6,2,6,12,12,2,20,3,2,3,6,4,8,9,2,7,34,5,1,18,6,1,1,4,4,5,7,9,1,2,2,4,3,4,1,7,
        2,2,2,6,2,3,25,5,3,6,1,4,6,7,4,2,1,4,2,13,6,4,4,3,1,5,3,4,4,3,2,1,1,4,1,2,1,1,3,1,11,1,6,3,1,7,3,6,2,8,8,6,9,3,4,11,3,2,10,12,2,5,11,1,6,4,5,
        3,1,8,5,4,6,6,3,5,1,1,3,2,1,2,2,6,17,12,1,10,1,6,12,1,6,6,19,9,6,16,1,13,4,4,15,7,17,6,11,9,15,12,6,7,2,1,2,2,15,9,3,21,4,6,49,18,7,3,2,3,1,
        6,8,2,2,6,2,9,1,3,6,4,4,1,2,16,2,5,2,1,6,2,3,5,3,1,2,5,1,2,1,9,3,1,8,6,4,8,11,3,1,1,1,1,3,1,13,8,4,1,3,2,2,1,4,1,11,1,5,2,1,5,2,5,8,6,1,1,7,
        4,3,8,3,2,7,2,1,5,1,5,2,4,7,6,2,8,5,1,11,4,5,3,6,18,1,2,13,3,3,1,21,1,1,4,1,4,1,1,1,8,1,2,2,7,1,2,4,2,2,9,2,1,1,1,4,3,6,3,12,5,1,1,1,5,6,3,2,
        4,8,2,2,4,2,7,1,8,9,5,2,3,2,1,3,2,13,7,14,6,5,1,1,2,1,4,2,23,2,1,1,6,3,1,4,1,15,3,1,7,3,9,14,1,3,1,4,1,1,5,8,1,3,8,3,8,15,11,4,14,4,4,2,5,5,
        1,7,1,6,14,7,7,8,5,15,4,8,6,5,6,2,1,13,1,20,15,11,9,2,5,6,2,11,2,6,2,5,1,5,8,4,13,19,25,4,1,1,11,1,34,2,5,9,14,6,2,2,6,1,1,14,1,3,14,13,1,6,
        12,21,14,14,6,32,17,8,32,9,28,1,2,4,11,8,3,1,14,2,5,15,1,1,1,1,3,6,4,1,3,4,11,3,1,1,11,30,1,5,1,4,1,5,8,1,1,3,2,4,3,17,35,2,6,12,17,3,1,6,2,
        1,1,12,2,7,3,3,2,1,16,2,8,3,6,5,4,7,3,3,8,1,9,8,5,1,2,1,3,2,8,1,2,9,12,1,1,2,3,8,3,24,12,4,3,7,5,8,3,3,3,3,3,3,1,23,10,3,1,2,2,6,3,1,16,1,16,
        22,3,10,4,11,6,9,7,7,3,6,2,2,2,4,10,2,1,1,2,8,7,1,6,4,1,3,3,3,5,10,12,12,2,3,12,8,15,1,1,16,6,6,1,5,9,11,4,11,4,2,6,12,1,17,5,13,1,4,9,5,1,11,
        2,1,8,1,5,7,28,8,3,5,10,2,17,3,38,22,1,2,18,12,10,4,38,18,1,4,44,19,4,1,8,4,1,12,1,4,31,12,1,14,7,75,7,5,10,6,6,13,3,2,11,11,3,2,5,28,15,6,18,
        18,5,6,4,3,16,1,7,18,7,36,3,5,3,1,7,1,9,1,10,7,2,4,2,6,2,9,7,4,3,32,12,3,7,10,2,23,16,3,1,12,3,31,4,11,1,3,8,9,5,1,30,15,6,12,3,2,2,11,19,9,
        14,2,6,2,3,19,13,17,5,3,3,25,3,14,1,1,1,36,1,3,2,19,3,13,36,9,13,31,6,4,16,34,2,5,4,2,3,3,5,1,1,1,4,3,1,17,3,2,3,5,3,1,3,2,3,5,6,3,12,11,1,3,
        1,2,26,7,12,7,2,14,3,3,7,7,11,25,25,28,16,4,36,1,2,1,6,2,1,9,3,27,17,4,3,4,13,4,1,3,2,2,1,10,4,2,4,6,3,8,2,1,18,1,1,24,2,2,4,33,2,3,63,7,1,6,
        40,7,3,4,4,2,4,15,18,1,16,1,1,11,2,41,14,1,3,18,13,3,2,4,16,2,17,7,15,24,7,18,13,44,2,2,3,6,1,1,7,5,1,7,1,4,3,3,5,10,8,2,3,1,8,1,1,27,4,2,1,
        12,1,2,1,10,6,1,6,7,5,2,3,7,11,5,11,3,6,6,2,3,15,4,9,1,1,2,1,2,11,2,8,12,8,5,4,2,3,1,5,2,2,1,14,1,12,11,4,1,11,17,17,4,3,2,5,5,7,3,1,5,9,9,8,
        2,5,6,6,13,13,2,1,2,6,1,2,2,49,4,9,1,2,10,16,7,8,4,3,2,23,4,58,3,29,1,14,19,19,11,11,2,7,5,1,3,4,6,2,18,5,12,12,17,17,3,3,2,4,1,6,2,3,4,3,1,
        1,1,1,5,1,1,9,1,3,1,3,6,1,8,1,1,2,6,4,14,3,1,4,11,4,1,3,32,1,2,4,13,4,1,2,4,2,1,3,1,11,1,4,2,1,4,4,6,3,5,1,6,5,7,6,3,23,3,5,3,5,3,3,13,3,9,10,
        1,12,10,2,3,18,13,7,160,52,4,2,2,3,2,14,5,4,12,4,6,4,1,20,4,11,6,2,12,27,1,4,1,2,2,7,4,5,2,28,3,7,25,8,3,19,3,6,10,2,2,1,10,2,5,4,1,3,4,1,5,
        3,2,6,9,3,6,2,16,3,3,16,4,5,5,3,2,1,2,16,15,8,2,6,21,2,4,1,22,5,8,1,1,21,11,2,1,11,11,19,13,12,4,2,3,2,3,6,1,8,11,1,4,2,9,5,2,1,11,2,9,1,1,2,
        14,31,9,3,4,21,14,4,8,1,7,2,2,2,5,1,4,20,3,3,4,10,1,11,9,8,2,1,4,5,14,12,14,2,17,9,6,31,4,14,1,20,13,26,5,2,7,3,6,13,2,4,2,19,6,2,2,18,9,3,5,
        12,12,14,4,6,2,3,6,9,5,22,4,5,25,6,4,8,5,2,6,27,2,35,2,16,3,7,8,8,6,6,5,9,17,2,20,6,19,2,13,3,1,1,1,4,17,12,2,14,7,1,4,18,12,38,33,2,10,1,1,
        2,13,14,17,11,50,6,33,20,26,74,16,23,45,50,13,38,33,6,6,7,4,4,2,1,3,2,5,8,7,8,9,3,11,21,9,13,1,3,10,6,7,1,2,2,18,5,5,1,9,9,2,68,9,19,13,2,5,
        1,4,4,7,4,13,3,9,10,21,17,3,26,2,1,5,2,4,5,4,1,7,4,7,3,4,2,1,6,1,1,20,4,1,9,2,2,1,3,3,2,3,2,1,1,1,20,2,3,1,6,2,3,6,2,4,8,1,3,2,10,3,5,3,4,4,
        3,4,16,1,6,1,10,2,4,2,1,1,2,10,11,2,2,3,1,24,31,4,10,10,2,5,12,16,164,15,4,16,7,9,15,19,17,1,2,1,1,5,1,1,1,1,1,3,1,4,3,1,3,1,3,1,2,1,1,3,3,7,
        2,8,1,2,2,2,1,3,4,3,7,8,12,92,2,10,3,1,3,14,5,25,16,42,4,7,7,4,2,21,5,27,26,27,21,25,30,31,2,1,5,13,3,22,5,6,6,11,9,12,1,5,9,7,5,5,22,60,3,5,
        13,1,1,8,1,1,3,3,2,1,9,3,3,18,4,1,2,3,7,6,3,1,2,3,9,1,3,1,3,2,1,3,1,1,1,2,1,11,3,1,6,9,1,3,2,3,1,2,1,5,1,1,4,3,4,1,2,2,4,4,1,7,2,1,2,2,3,5,13,
        18,3,4,14,9,9,4,16,3,7,5,8,2,6,48,28,3,1,1,4,2,14,8,2,9,2,1,15,2,4,3,2,10,16,12,8,7,1,1,3,1,1,1,2,7,4,1,6,4,38,39,16,23,7,15,15,3,2,12,7,21,
        37,27,6,5,4,8,2,10,8,8,6,5,1,2,1,3,24,1,16,17,9,23,10,17,6,1,51,55,44,13,294,9,3,6,2,4,2,2,15,1,1,1,13,21,17,68,14,8,9,4,1,4,9,3,11,7,1,1,1,
        5,6,3,2,1,1,1,2,3,8,1,2,2,4,1,5,5,2,1,4,3,7,13,4,1,4,1,3,1,1,1,5,5,10,1,6,1,5,2,1,5,2,4,1,4,5,7,3,18,2,9,11,32,4,3,3,2,4,7,11,16,9,11,8,13,38,
        32,8,4,2,1,1,2,1,2,4,4,1,1,1,4,1,21,3,11,1,16,1,1,6,1,3,2,4,9,8,57,7,44,1,3,3,13,3,10,1,1,7,5,2,7,21,47,63,3,15,4,7,1,16,1,1,2,8,2,3,42,15,4,
        1,29,7,22,10,3,78,16,12,20,18,4,67,11,5,1,3,15,6,21,31,32,27,18,13,71,35,5,142,4,10,1,2,50,19,33,16,35,37,16,19,27,7,1,133,19,1,4,8,7,20,1,4,
        4,1,10,3,1,6,1,2,51,5,40,15,24,43,22928,11,1,13,154,70,3,1,1,7,4,10,1,2,1,1,2,1,2,1,2,2,1,1,2,1,1,1,1,1,2,1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,
        3,2,1,1,1,1,2,1,1,
    };
    static ImWchar base_ranges[] = // not zero-terminated
    {
        0x0020, 0x00FF, // Basic Latin + Latin Supplement
        0x3000, 0x30FF, // CJK Symbols and Punctuations, Hiragana, Katakana
        0x31F0, 0x31FF, // Katakana Phonetic Extensions
        0xFF00, 0xFFEF, // Half-width characters
        0xFFFD, 0xFFFD  // Invalid
    };
    static ImWchar full_ranges[IM_ARRAYSIZE(base_ranges) + IM_ARRAYSIZE(accumulative_offsets_from_0x4E00)*2 + 1] = { 0 };
    if (!full_ranges[0])
    {
        memcpy(full_ranges, base_ranges, sizeof(base_ranges));
        UnpackAccumulativeOffsetsIntoRanges(0x4E00, accumulative_offsets_from_0x4E00, IM_ARRAYSIZE(accumulative_offsets_from_0x4E00), full_ranges + IM_ARRAYSIZE(base_ranges));
    }
    return &full_ranges[0];
}

const ImWchar*  ImFontAtlas::GetGlyphRangesCyrillic()
{
    static const ImWchar ranges[] =
    {
        0x0020, 0x00FF, // Basic Latin + Latin Supplement
        0x0400, 0x052F, // Cyrillic + Cyrillic Supplement
        0x2DE0, 0x2DFF, // Cyrillic Extended-A
        0xA640, 0xA69F, // Cyrillic Extended-B
        0,
    };
    return &ranges[0];
}

const ImWchar*  ImFontAtlas::GetGlyphRangesThai()
{
    static const ImWchar ranges[] =
    {
        0x0020, 0x00FF, // Basic Latin
        0x2010, 0x205E, // Punctuations
        0x0E00, 0x0E7F, // Thai
        0,
    };
    return &ranges[0];
}

const ImWchar*  ImFontAtlas::GetGlyphRangesVietnamese()
{
    static const ImWchar ranges[] =
    {
        0x0020, 0x00FF, // Basic Latin
        0x0102, 0x0103,
        0x0110, 0x0111,
        0x0128, 0x0129,
        0x0168, 0x0169,
        0x01A0, 0x01A1,
        0x01AF, 0x01B0,
        0x1EA0, 0x1EF9,
        0,
    };
    return &ranges[0];
}
