use crate::draw::DrawList;
use crate::font::font_atlas::FontAtlas;
use crate::font::font_glyph::FontGlyph;
use crate::font::font_config::FontConfig;
use crate::vectors::{Vector2D, Vector4D};

// font runtime data and rendering
// ImFontAtlas automatically loads a default embedded font for you when you call GetTexDataAsAlpha8() or GetTexDataAsRGBA32().
#[derive(Debug,Clone,Default)]
pub struct Font
{
    // Members: Hot ~20/24 bytes (for calc_text_size)
    pub index_advance_x: Vec<f32>, // ImVector<float>             index_advance_x;      // 12-16 // out //            // Sparse. glyphs->advance_x in a directly indexable way (cache-friendly for calc_text_size functions which only this this info, and are often bottleneck in large UI).
    pub fallback_advance_x: f32,  // 4     // out // = fallback_glyph->advance_x
    pub font_size: f32,          // 4     // in  //            // height of characters/line, set during loading (don't change after loading)

    // Members: Hot ~28/40 bytes (for calc_text_size + render loop)
    pub index_lookup: Vec<DimgWchar>, //ImVector<ImWchar>           index_lookup;        // 12-16 // out //            // Sparse. index glyphs by Unicode code-point.
    pub glyphs: Vec<FontGlyph>, // ImVector<ImFontGlyph>       glyphs;             // 12-16 // out //            // All glyphs.
    pub fallback_glyph: FontGlyph, // const ImFontGlyph*          fallback_glyph;      // 4-8   // out // = find_glyph(FontFallbackChar)

    // Members: Cold ~32/40 bytes
    pub container_atlas: FontAtlas, // ImFontAtlas*                container_atlas;     // 4-8   // out //            // What we has been loaded into
    // const ImFontConfig*         config_data;         // 4-8   // in  //            // Pointer within container_atlas->config_data
    pub config_data: FontConfig,
// short                       config_data_count;    // 2     // in  // ~ 1        // Number of ImFontConfig involved in creating this font. Bigger than 1 when merging multiple font sources into one ImFont.
    pub config_data_count: isize,
    // ImWchar                     fallback_char;       // 2     // out // = FFFD/'?' // Character used if a glyph isn't found.
    pub fallback_char: char,
    // ImWchar                     ellipsis_char;       // 2     // out // = '...'    // Character used for ellipsis rendering.
    pub ellipsis_char: char,
    // ImWchar                     dot_char;            // 2     // out // = '.'      // Character used for ellipsis rendering (if a single '...' character isn't found)
    pub dot_char: char,
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

    pub fn new() -> Self {
        Self {
            index_advance_x: vec![],
            font_size: 0.0,
            index_lookup: vec![],
            fallback_advance_x: 0.0,
            fallback_char: '\u{24E7}',
            ellipsis_char: '\u{2026}',
            dot_char: '\u{00b7}',
            dirty_lookup_tables: false,
            scale: 1.0,
            ascent: 0.0,
            descent: 0.0,
            metrics_total_surface: 0,
            fallback_glyph: FontGlyph::default(),
            container_atlas: Default::default(),
            config_data: Default::default(),
            glyphs: vec![],
            config_data_count: 0,
            used4k_pages_map: vec![]
        }
    }

    //  ImFont();
    //      ~ImFont();
    //      const ImFontGlyph*find_glyph(ImWchar c) const;
    pub fn find_glyph(&self, c: DimgWchar) -> FontGlyph {
        todo!()
    }
    //      const ImFontGlyph*find_glyph_no_fallback(ImWchar c) const;
    pub fn find_glyph_no_fallback(&self, c: DimgWchar) -> FontGlyph {
        todo!()
    }
    //     float                       get_char_advance(ImWchar c) const     { return (c < index_advance_x.size) ? index_advance_x[c] : fallback_advance_x; }
    pub fn get_char_advance(&self, c: DimgWchar) -> f32 {
        if c < self.index_advance_x.len() as DimgWchar {
            self.index_advance_x[c]
        }
        self.fallback_advance_x
    }
    //     bool                        is_loaded() const                    { return container_atlas != None; }
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
    //      Vector2D            calc_text_size_a(float size, float max_width, float wrap_width, const char* text_begin, const char* text_end = None, const char** remaining = None) const; // utf8
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
    pub fn add_glyph(&mut self, src_cfg: &FontConfig, c: DimgWchar, x0: f32, y0: f32, x1: f32, y1: f32, u0: f32, v0: f32, u1: f32, v1: f32, advance_x: f32) {
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
    pub fn glyph_range_unused(&mut self, c_begin: u32, c_lst: u32) -> bool {
        todo!()
    }
    
    pub fn clear_output_data(&mut self) {
        self.font_size = 0.0;
        self.fallback_advance_x = 0.0;
        self.glyphs.clear();
        self.index_advance_x.clear();
        self.index_lookup.clear();
        self.fallback_glyph = FontGlyph::default();
        self.container_atlas = FontAtlas::default();
        self.dirty_lookup_tables = true;
        self.ascent = 0.0;
        self.descent = 0.0;
        self.metrics_total_surface = 0;
    }
}
