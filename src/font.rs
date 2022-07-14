use crate::defines::DimgFontConfig;
use crate::draw_list::DrawList;
use crate::font_atlas::FontAtlas;
use crate::font_glyph::DimgFontGlyph;
use crate::types::{Id32, DimgWchar};
use crate::vectors::Vector2D;

// This structure is likely to evolve as we add support for incremental atlas updates
#[derive(Default,Debug,Clone)]
pub struct ImFontBuilderIO
{
    // bool    (*FontBuilder_Build)(ImFontAtlas* atlas);
    pub font_builder_build_fn: Option<fn(atlas: &mut FontAtlas)>,
}


// font runtime data and rendering
// ImFontAtlas automatically loads a default embedded font for you when you call GetTexDataAsAlpha8() or GetTexDataAsRGBA32().
#[derive(Debug,Clone,Default)]
pub struct Font
{
    // Members: Hot ~20/24 bytes (for CalcTextSize)
    pub index_advance_x: Vec<f32>, // ImVector<float>             index_advance_x;      // 12-16 // out //            // Sparse. glyphs->advance_x in a directly indexable way (cache-friendly for CalcTextSize functions which only this this info, and are often bottleneck in large UI).
    pub fallback_advance_x: f32,  // 4     // out // = fallback_glyph->advance_x
    pub font_size: f32,          // 4     // in  //            // height of characters/line, set during loading (don't change after loading)

    // Members: Hot ~28/40 bytes (for CalcTextSize + render loop)
    pub index_lookup: Vec<DimgWchar>, //ImVector<ImWchar>           index_lookup;        // 12-16 // out //            // Sparse. index glyphs by Unicode code-point.
    pub glyphs: Vec<DimgFontGlyph>, // ImVector<ImFontGlyph>       glyphs;             // 12-16 // out //            // All glyphs.
    pub fallback_glyph: DimgFontGlyph, // const ImFontGlyph*          fallback_glyph;      // 4-8   // out // = find_glyph(FontFallbackChar)

    // Members: Cold ~32/40 bytes
    pub container_atlas: Option<FontAtlas>, // ImFontAtlas*                container_atlas;     // 4-8   // out //            // What we has been loaded into
    // const ImFontConfig*         config_data;         // 4-8   // in  //            // Pointer within container_atlas->config_data
    pub config_data: Option<DimgFontConfig>,
// short                       config_data_count;    // 2     // in  // ~ 1        // Number of ImFontConfig involved in creating this font. Bigger than 1 when merging multiple font sources into one ImFont.
    pub config_data_count: isize,
    // ImWchar                     fallback_char;       // 2     // out // = FFFD/'?' // Character used if a glyph isn't found.
    pub fallback_char: DimgWchar,
    // ImWchar                     ellipsis_char;       // 2     // out // = '...'    // Character used for ellipsis rendering.
    pub ellipsis_char: DimgWchar,
    // ImWchar                     dot_char;            // 2     // out // = '.'      // Character used for ellipsis rendering (if a single '...' character isn't found)
    pub dot_char: DimgWchar,
    pub dirty_lookup_tables: bool,  // 1     // out //
    pub scale: f32,             // 4     // in  // = 1.f      // Base font scale, multiplied by the per-window font scale which you can adjust with SetWindowFontScale()
    // float                       ascent, descent;    // 4+4   // out //            // ascent: distance from top to bottom of e.g. 'A' [0..font_size]
    pub ascent: f32,
    pub descent: f32,
// int                         metrics_total_surface;// 4     // out //            // Total surface in pixels to get an idea of the font rasterization/texture cost (not exact, we approximate the cost of padding between glyphs)
    pub metrics_total_surface: i32,
    // ImU8                        used4k_pages_map[(IM_UNICODE_CODEPOINT_MAX+1)/4096/8]; // 2 bytes if ImWchar=ImWchar16, 34 bytes if ImWchar==ImWchar32. Store 1-bit for each block of 4K
    // codepoints that has one active glyph. This is mainly used to facilitate iterations across all used codepoints.
    pub used4k_pages_map: Vec<u8>,
    // Methods

}

impl Font {
    //  ImFont();
    //      ~ImFont();
    //      const ImFontGlyph*find_glyph(ImWchar c) const;
    pub fn find_glyph(&self, c: DimgWchar) -> DimgFontGlyph {
        todo!()
    }
    //      const ImFontGlyph*find_glyph_no_fallback(ImWchar c) const;
    pub fn find_glyph_no_fallback(&self, c: DimgWchar) -> DimgFontGlyph {
        todo!()
    }
    //     float                       get_char_advance(ImWchar c) const     { return (c < index_advance_x.size) ? index_advance_x[c] : fallback_advance_x; }
    pub fn get_char_advance(&self, c: DimgWchar) -> f32 {
        if c < self.index_advance_x.len() as DimgWchar {
            self.index_advance_x[c]
        }
        self.fallback_advance_x
    }
    //     bool                        is_loaded() const                    { return container_atlas != NULL; }
    pub fn is_loaded(&self) -> bool {
        self.container_atlas.is_some()
    }
    //     const char*                 get_debug_name() const                { return config_data ? config_data->name : "<unknown>"; }
    pub fn get_debug_name(&self) -> String {
        if self.config_data.is_some() {
            self.config_data.unwrap().name
        }
        "<unknown>".to_string()
    }
    //
    //     // 'max_width' stops rendering after a certain width (could be turned into a 2d size). FLT_MAX to disable.
    //     // 'wrap_width' enable automatic word-wrapping across multiple lines to fit into given width. 0.0 to disable.
    //      Vector2D            calc_text_size_a(float size, float max_width, float wrap_width, const char* text_begin, const char* text_end = NULL, const char** remaining = NULL) const; // utf8
    pub fn calc_text_size_a(&self, size: f32, max_width: f32, wrap_width: f32, text: &str) -> Vector2D {
        todo!()
    }
    //      const char*       calc_word_wrap_position_a(float scale, const char* text, const char* text_end, float wrap_width) const;
    pub fn calc_word_wrap_position_a(&self, scale: f32, text: &String, wrap_width: f32) -> String{
        todo!()
    }
    //      void              render_char(ImDrawList* draw_list, float size, const Vector2D& pos, ImU32 col, ImWchar c) const;
    pub fn render_char(&self, draw_list: &DrawList, size: f32, pos: &Vector2D, col: u32, c: DimgWchar) {
        todo!()
    }
    //      void              render_text(ImDrawList* draw_list, float size, const Vector2D& pos, ImU32 col, const Vector4D& clip_rect, const char* text_begin, const char* text_end, float wrap_width = 0.0, bool cpu_fine_clip = false) const;
    pub fn render_text(&self, draw_list: &mut DrawList, size: f32, pos: &Vector2D, col: u32, clip_rect: &Vector4D, text: &String, wrap_width: f32, cpu_fine_clip: bool) {
        todo!()
    }
    //
    //     // [Internal] Don't use!
    //      void              build_lookup_table();
    pub fn build_lookup_table(&mut self) {
        todo!()
    }
    //      void              clear_output_data();
    pub fn clear_output_data(&mut self) {
        todo!()
    }
    //      void              grow_index(int new_size);
    pub fn grow_index(&mut self) {
        todo!()
    }
    //      void              add_glyph(const ImFontConfig* src_cfg, ImWchar c, float x0, float y0, float x1, float y1, float u0, float v0, float u1, float v1, float advance_x);
    pub fn add_glyph(&mut self, src_cfg: &DimgFontConfig, c: DimgWchar, x0: f32, y0: f32, x1: f32, y1: f32, u0: f32, v0: f32, u1: f32, v1: f32, advance_x: f32) {
        todo!()
    }
    //      void              add_remap_char(ImWchar dst, ImWchar src, bool overwrite_dst = true); // Makes 'dst' character/glyph points to 'src' character/glyph. Currently needs to be called AFTER fonts have been built.
    pub fn add_remap_char(&mut self, dst: DimgWchar, src: DimgWchar, overwrite_dst: bool){
        todo!()
    }
    //      void              set_glyph_visible(ImWchar c, bool visible);
    pub fn set_glyph_visible(&mut self, c: DimgWchar, visible: bool) {
        todo!()
    }
    //      bool              is_glyph_range_unused(unsigned int c_begin, unsigned int c_last);
    pub fn is_glyph_range_unused(&mut self, c_begin: u32, c_lst: u32) -> bool {
        todo!()
    }
}

// Helper to build glyph ranges from text/string data. Feed your application strings/characters to it then call build_ranges().
// This is essentially a tightly packed of vector of 64k booleans = 8KB storage.
#[derive(Clone,Debug,Default)]
pub struct ImFontGlyphRangesBuilder
{
    pub used_chars: Vec<u32>, //ImVector<ImU32> used_chars;            // Store 1-bit per Unicode code point (0=unused, 1=used)


}

impl ImFontGlyphRangesBuilder {
    // ImFontGlyphRangesBuilder()              { clear(); }
    //     inline void     clear()                 { int size_in_bytes = (IM_UNICODE_CODEPOINT_MAX + 1) / 8; used_chars.resize(size_in_bytes / sizeof); memset(used_chars.data, 0, (size_t)size_in_bytes); }
    pub fn clear(&mut self) {
        self.used_chars.clear()
    }
    //     inline bool     get_bit(size_t n) const  { int off = (n >> 5); ImU32 mask = 1u << (n & 31); return (used_chars[off] & mask) != 0; }  // Get bit n in the array
    pub fn get_bit(&mut self, n: usize) -> bool {
        let off = n >> 5;
        let mask: u32 = 1 << (n * 31);
        self.used_chars[off] & mask != 0
    }
    //     inline void     set_bit(size_t n)        { int off = (n >> 5); ImU32 mask = 1u << (n & 31); used_chars[off] |= mask; }               // Set bit n in the array
    pub fn set_bit(&mut self, n: usize) {
        let off = n >> 5;
        let mask: u32 = 1 << (n & 31);
        self.used_chars[off] |= mask;
    }
    //     inline void     add_char(ImWchar c)      { set_bit(c); }                      // Add character
    pub fn add_char(&mut self, c: u8) {
        self.set_bit(c as usize)
    }
    //      void  add_text(const char* text, const char* text_end = NULL);     // Add string (each character of the UTF-8 string are added)
    pub fn add_text(&mut self, text: &String) {
        todo!()
    }
    //      void  add_ranges(const ImWchar* ranges);                           // Add ranges, e.g. builder.add_ranges(ImFontAtlas::get_glyph_ranges_default()) to force add all of ASCII/Latin+Ext
    pub fn add_ranges(&mut self, ranges: &[DimgWchar]) {
        todo!()
    }
    //      void  build_ranges(ImVector<ImWchar>* out_ranges);                 // Output new ranges
    pub fn build_ranges(&mut self, out_ranges: &mut Vec<DimgWchar>) {
        todo!()
    }
}

#[derive(Clone,Debug,Default)]
pub struct DimgFontConfig
{
    pub id: Id32,
    pub font_data: Vec<u8>, // void*           font_data;               //          // TTF/OTF data
    pub font_data_size: i32,         //          // TTF/OTF data size
    pub font_data_owned_by_atlas: bool,   // true     // TTF/OTF data ownership taken by the container ImFontAtlas (will delete memory itself).
    pub font_no: i32,               // 0        // index of font within TTF/OTF file
    pub size_pixels: f32,            //          // size in pixels for rasterizer (more or less maps to the resulting font height).
    pub oversample_h: i32,          // 3        // Rasterize at higher quality for sub-pixel positioning. Note the difference between 2 and 3 is minimal so you can reduce this to 2 to save memory. Read https://github.com/nothings/stb/blob/master/tests/oversample/README.md for details.
    pub oversample_v: i32,          // 1        // Rasterize at higher quality for sub-pixel positioning. This is not really useful as we don't use sub-pixel positions on the Y axis.
    pub pixel_snap_h: bool,             // false    // Align every glyph to pixel boundary. Useful e.g. if you are merging a non-pixel aligned font with the default font. If enabled, you can set oversample_h/V to 1.
    pub glyph_extra_spacing: Vector2D,      // 0, 0     // Extra spacing (in pixels) between glyphs. Only x axis is supported for now.
    pub glyph_offset: Vector2D,            // 0, 0     // Offset all glyphs from this font input.
    pub glyph_ranges: Vec<DimgWchar>, // const ImWchar*  glyph_ranges;            // NULL     // Pointer to a user-provided list of Unicode range (2 value per range, values are inclusive, zero-terminated list). THE ARRAY DATA NEEDS TO PERSIST AS LONG AS THE FONT IS ALIVE.
    pub glyph_min_advance_x: f32,      // 0        // Minimum advance_x for glyphs, set min to align font icons, set both min/max to enforce mono-space font
    pub glyph_max_advance_x: f32,      // FLT_MAX  // Maximum advance_x for glyphs
    pub merge_mode: bool,              // false    // merge into previous ImFont, so you can combine multiple inputs font into one ImFont (e.g. ASCII font + icons + Japanese glyphs). You may want to use glyph_offset.y when merge font of different heights.
    pub font_builder_flags: u32,     // 0        // Settings for custom font builder. THIS IS BUILDER IMPLEMENTATION DEPENDENT. Leave as zero if unsure.
    pub rasterizer_multiply: f32,    // 1.0     // Brighten (>1.0) or darken (<1.0) font output. Brightening small fonts may be a good workaround to make them more readable.
    // ImWchar         ellipsis_char;           // -1       // Explicitly specify unicode codepoint of ellipsis character. When fonts are being merged first specified ellipsis will be used.
    pub ellipsis_char: DimgWchar,
    // [Internal]
    // char            name[40];               // name (strictly to ease debugging)
    pub name: String,
    // ImFont*         dst_font;
    pub dst_font: Id32,
    //  ImFontConfig();
}
