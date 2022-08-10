use crate::Context;
use crate::defines::DimgFontConfig;
use crate::draw::list::DrawList;
use font_atlas::FontAtlas;
use font_glyph::FontGlyph;
use crate::globals::GImGui;
use crate::types::{DimgWchar, Id32};
use crate::vectors::vector_2d::Vector2D;

pub mod font_atlas;
mod font_glyph;
mod config;

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
    // Members: Hot ~20/24 bytes (for calc_text_size)
    pub index_advance_x: Vec<f32>, // ImVector<float>             index_advance_x;      // 12-16 // out //            // Sparse. glyphs->advance_x in a directly indexable way (cache-friendly for calc_text_size functions which only this this info, and are often bottleneck in large UI).
    pub fallback_advance_x: f32,  // 4     // out // = fallback_glyph->advance_x
    pub font_size: f32,          // 4     // in  //            // height of characters/line, set during loading (don't change after loading)

    // Members: Hot ~28/40 bytes (for calc_text_size + render loop)
    pub index_lookup: Vec<DimgWchar>, //ImVector<ImWchar>           index_lookup;        // 12-16 // out //            // Sparse. index glyphs by Unicode code-point.
    pub glyphs: Vec<FontGlyph>, // ImVector<ImFontGlyph>       glyphs;             // 12-16 // out //            // All glyphs.
    pub fallback_glyph: FontGlyph, // const ImFontGlyph*          fallback_glyph;      // 4-8   // out // = find_glyph(FontFallbackChar)

    // Members: Cold ~32/40 bytes
    pub container_atlas: Option<FontAtlas>, // ImFontAtlas*                container_atlas;     // 4-8   // out //            // What we has been loaded into
    // const ImFontConfig*         config_data;         // 4-8   // in  //            // Pointer within container_atlas->config_data
    pub config_data: Option<FontConfig>,
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
    //      void  add_text(const char* text, const char* text_end = None);     // Add string (each character of the UTF-8 string are added)
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
pub struct FontConfig
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
    pub glyph_ranges: Vec<DimgWchar>, // const ImWchar*  glyph_ranges;            // None     // Pointer to a user-provided list of Unicode range (2 value per range, values are inclusive, zero-terminated list). THE ARRAY DATA NEEDS TO PERSIST AS LONG AS THE FONT IS ALIVE.
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

// Important: this alone doesn't alter current ImDrawList state. This is called by PushFont/PopFont only.
// void ImGui::sec_current_font(ImFont* font)
pub fn set_current_font(g: &mut Context, font: &mut Font)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(font && font.IsLoaded());    // font Atlas not created. Did you call io.fonts->GetTexDataAsRGBA32 / GetTexDataAsAlpha8 ?
    // IM_ASSERT(font.Scale > 0.0);
    g.font = font;
    g.FontBaseSize = ImMax(1.0, g.io.FontGlobalScale * g.font.font_size * g.font.Scale);
    g.font_size = g.current_window ? g.current_window.CalcFontSize() : 0.0;

    ImFontAtlas* atlas = g.font.container_atlas;
    g.draw_list_shared_data.TexUvWhitePixel = atlas.TexUvWhitePixel;
    g.draw_list_shared_data.tex_uv_lines = atlas.tex_uv_lines;
    g.draw_list_shared_data.font = g.font;
    g.draw_list_shared_data.font_size = g.font_size;
}

// void ImGui::PushFont(ImFont* font)
pub fn push_font(g: &mut Context, font: &mut Font)
{
    // ImGuiContext& g = *GImGui;
    if (!font)
        font = get_default_font();
    sec_current_font(font);
    g.font_stack.push_back(font);
    g.current_window.draw_list.push_texture_id(font.container_atlas.TexID);
}

// void  ImGui::PopFont()
pub fn pop_font(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    g.current_window.draw_list.pop_texture_id();
    g.font_stack.pop_back();
    sec_current_font(g.font_stack.empty() ? get_default_font() : g.font_stack.back());
}

// ImFont* GetFont()
pub fn get_font(g: &mut Context) -> &mut Font
{
    return g.Font;
}

// float GetFontSize()
pub fn get_font_size(g: &mut Context) -> f32
{
    return g.font_size;
}

// Vector2D GetFontTexUvWhitePixel()
pub fn get_font_tex_uv_white_pixel(g: &mut Context) -> Vector2D
{
    return g.DrawListSharedData.TexUvWhitePixel;
}

// void SetWindowFontScale(float scale)
pub fn set_window_font_scale(g: &mut Context, scale: f32)
{
    // IM_ASSERT(scale > 0.0);
    // ImGuiContext& g = *GImGui;
    Window* window = GetCurrentWindow();
    window.font_window_scale = scale;
    g.font_size = g.draw_list_shared_data.font_size = window.CalcFontSize();
}


ImFont::ImFont()
{
    FontSize = 0.0;
    FallbackAdvanceX = 0.0;
    FallbackChar = (ImWchar)-1;
    EllipsisChar = (ImWchar)-1;
    DotChar = (ImWchar)-1;
    FallbackGlyph = None;
    ContainerAtlas = None;
    ConfigData = None;
    ConfigDataCount = 0;
    DirtyLookupTables = false;
    Scale = 1.0;
    Ascent = Descent = 0.0;
    MetricsTotalSurface = 0;
    memset(Used4kPagesMap, 0, sizeof(Used4kPagesMap));
}

ImFont::~ImFont()
{
    ClearOutputData();
}

void    ImFont::ClearOutputData()
{
    FontSize = 0.0;
    FallbackAdvanceX = 0.0;
    Glyphs.clear();
    IndexAdvanceX.clear();
    IndexLookup.clear();
    FallbackGlyph = None;
    ContainerAtlas = None;
    DirtyLookupTables = true;
    Ascent = Descent = 0.0;
    MetricsTotalSurface = 0;
}

static ImWchar FindFirstExistingGlyph(ImFont* font, const ImWchar* candidate_chars, int candidate_chars_count)
{
    for (int n = 0; n < candidate_chars_count; n += 1)
        if (font.FindGlyphNoFallback(candidate_chars[n]) != None)
            return candidate_chars[n];
    return (ImWchar)-1;
}

void ImFont::BuildLookupTable()
{
    int max_codepoint = 0;
    for (int i = 0; i != Glyphs.size; i += 1)
        max_codepoint = ImMax(max_codepoint, Glyphs[i].Codepoint);

    // build lookup table
    // IM_ASSERT(Glyphs.size < 0xFFFF); // -1 is reserved
    IndexAdvanceX.clear();
    IndexLookup.clear();
    DirtyLookupTables = false;
    memset(Used4kPagesMap, 0, sizeof(Used4kPagesMap));
    GrowIndex(max_codepoint + 1);
    for (int i = 0; i < Glyphs.size; i += 1)
    {
        int codepoint = Glyphs[i].Codepoint;
        IndexAdvanceX[codepoint] = Glyphs[i].AdvanceX;
        IndexLookup[codepoint] = (ImWchar)i;

        // Mark 4K page as used
        let page_n = codepoint / 4096;
        Used4kPagesMap[page_n >> 3] |= 1 << (page_n & 7);
    }

    // Create a glyph to handle TAB
    // FIXME: Needs proper TAB handling but it needs to be contextualized (or we could arbitrary say that each string starts at "column 0" ?)
    if (FindGlyph((ImWchar)' '))
    {
        if (Glyphs.back().Codepoint != '\t')   // So we can call this function multiple times (FIXME: Flaky)
            Glyphs.resize(Glyphs.size + 1);
        ImFontGlyph& tab_glyph = Glyphs.back();
        tab_glyph = *FindGlyph((ImWchar)' ');
        tab_glyph.Codepoint = '\t';
        tab_glyph.AdvanceX *= IM_TABSIZE;
        IndexAdvanceX[tab_glyph.Codepoint] = tab_glyph.AdvanceX;
        IndexLookup[tab_glyph.Codepoint] = (ImWchar)(Glyphs.size - 1);
    }

    // Mark special glyphs as not visible (note that add_glyph already mark as non-visible glyphs with zero-size polygons)
    SetGlyphVisible((ImWchar)' ', false);
    SetGlyphVisible((ImWchar)'\t', false);

    // Ellipsis character is required for rendering elided text. We prefer using U+2026 (horizontal ellipsis).
    // However some old fonts may contain ellipsis at U+0085. Here we auto-detect most suitable ellipsis character.
    // FIXME: Note that 0x2026 is rarely included in our font ranges. Because of this we are more likely to use three individual dots.
    const ImWchar ellipsis_chars[] = { (ImWchar)0x2026, (ImWchar)0x0085 };
    const ImWchar dots_chars[] = { (ImWchar)'.', (ImWchar)0xFF0E };
    if (EllipsisChar == (ImWchar)-1)
        EllipsisChar = FindFirstExistingGlyph(this, ellipsis_chars, IM_ARRAYSIZE(ellipsis_chars));
    if (DotChar == (ImWchar)-1)
        DotChar = FindFirstExistingGlyph(this, dots_chars, IM_ARRAYSIZE(dots_chars));

    // Setup fallback character
    const ImWchar fallback_chars[] = { (ImWchar)IM_UNICODE_CODEPOINT_INVALID, (ImWchar)'?', (ImWchar)' ' };
    FallbackGlyph = FindGlyphNoFallback(FallbackChar);
    if (FallbackGlyph == None)
    {
        FallbackChar = FindFirstExistingGlyph(this, fallback_chars, IM_ARRAYSIZE(fallback_chars));
        FallbackGlyph = FindGlyphNoFallback(FallbackChar);
        if (FallbackGlyph == None)
        {
            FallbackGlyph = &Glyphs.back();
            FallbackChar = (ImWchar)FallbackGlyph.Codepoint;
        }
    }

    FallbackAdvanceX = FallbackGlyph.AdvanceX;
    for (int i = 0; i < max_codepoint + 1; i += 1)
        if (IndexAdvanceX[i] < 0.0)
            IndexAdvanceX[i] = FallbackAdvanceX;
}

// API is designed this way to avoid exposing the 4K page size
// e.g. use with is_glyph_range_unused(0, 255)
bool ImFont::IsGlyphRangeUnused(unsigned int c_begin, unsigned int c_last)
{
    unsigned int page_begin = (c_begin / 4096);
    unsigned int page_last = (c_last / 4096);
    for (unsigned int page_n = page_begin; page_n <= page_last; page_n += 1)
        if ((page_n >> 3) < sizeof(Used4kPagesMap))
            if (Used4kPagesMap[page_n >> 3] & (1 << (page_n & 7)))
                return false;
    return true;
}

void ImFont::SetGlyphVisible(ImWchar c, bool visible)
{
    if (ImFontGlyph* glyph = (ImFontGlyph*)(void*)FindGlyph((ImWchar)c))
        glyph.Visible = visible ? 1 : 0;
}

void ImFont::GrowIndex(int new_size)
{
    // IM_ASSERT(IndexAdvanceX.size == IndexLookup.size);
    if (new_size <= IndexLookup.size)
        return;
    IndexAdvanceX.resize(new_size, -1.0);
    IndexLookup.resize(new_size, (ImWchar)-1);
}

// x0/y0/x1/y1 are offset from the character upper-left layout position, in pixels. Therefore x0/y0 are often fairly close to zero.
// Not to be mistaken with texture coordinates, which are held by u0/v0/u1/v1 in normalized format (0.0..1.0 on each texture axis).
// 'cfg' is not necessarily == 'this->config_data' because multiple source fonts+configs can be used to build one target font.
void ImFont::AddGlyph(const ImFontConfig* cfg, ImWchar codepoint, float x0, float y0, float x1, float y1, float u0, float v0, float u1, float v1, float advance_x)
{
    if (cfg != None)
    {
        // Clamp & recenter if needed
        let advance_x_original = advance_x;
        advance_x = ImClamp(advance_x, cfg.GlyphMinAdvanceX, cfg.GlyphMaxAdvanceX);
        if (advance_x != advance_x_original)
        {
            let char_off_x =  cfg.PixelSnapH ? f32::floor((advance_x - advance_x_original) * 0.5) : (advance_x - advance_x_original) * 0.5;
            x0 += char_off_x;
            x1 += char_off_x;
        }

        // Snap to pixel
        if (cfg.PixelSnapH)
            advance_x = IM_ROUND(advance_x);

        // Bake spacing
        advance_x += cfg.GlyphExtraSpacing.x;
    }

    Glyphs.resize(Glyphs.size + 1);
    ImFontGlyph& glyph = Glyphs.back();
    glyph.Codepoint = (unsigned int)codepoint;
    glyph.Visible = (x0 != x1) && (y0 != y1);
    glyph.Colored = false;
    glyph.X0 = x0;
    glyph.Y0 = y0;
    glyph.X1 = x1;
    glyph.Y1 = y1;
    glyph.U0 = u0;
    glyph.V0 = v0;
    glyph.U1 = u1;
    glyph.V1 = v1;
    glyph.AdvanceX = advance_x;

    // Compute rough surface usage metrics (+1 to account for average padding, +0.99 to round)
    // We use (u1-u0)*tex_width instead of x1-x0 to account for oversampling.
    let pad =  ContainerAtlas.TexGlyphPadding + 0.99;
    DirtyLookupTables = true;
    MetricsTotalSurface += ((glyph.U1 - glyph.U0) * ContainerAtlas.TexWidth + pad) * ((glyph.V1 - glyph.V0) * ContainerAtlas.TexHeight + pad);
}

void ImFont::AddRemapChar(ImWchar dst, ImWchar src, bool overwrite_dst)
{
    // IM_ASSERT(IndexLookup.size > 0);    // Currently this can only be called AFTER the font has been built, aka after calling ImFontAtlas::GetTexDataAs*() function.
    unsigned int index_size = (unsigned int)IndexLookup.size;

    if (dst < index_size && IndexLookup.data[dst] == (ImWchar)-1 && !overwrite_dst) // 'dst' already exists
        return;
    if (src >= index_size && dst >= index_size) // both 'dst' and 'src' don't exist -> no-op
        return;

    GrowIndex(dst + 1);
    IndexLookup[dst] = (src < index_size) ? IndexLookup.data[src] : (ImWchar)-1;
    IndexAdvanceX[dst] = (src < index_size) ? IndexAdvanceX.data[src] : 1.0;
}

const ImFontGlyph* ImFont::FindGlyph(ImWchar c) const
{
    if (c >= IndexLookup.size)
        return FallbackGlyph;
    const ImWchar i = IndexLookup.data[c];
    if (i == (ImWchar)-1)
        return FallbackGlyph;
    return &Glyphs.data[i];
}

const ImFontGlyph* ImFont::FindGlyphNoFallback(ImWchar c) const
{
    if (c >= IndexLookup.size)
        return None;
    const ImWchar i = IndexLookup.data[c];
    if (i == (ImWchar)-1)
        return None;
    return &Glyphs.data[i];
}

const char* ImFont::CalcWordWrapPositionA(float scale, const char* text, const char* text_end, float wrap_width) const
{
    // Simple word-wrapping for English, not full-featured. Please submit failing cases!
    // FIXME: Much possible improvements (don't cut things like "word !", "word!!!" but cut within "word,,,,", more sensible support for punctuations, support for Unicode punctuations, etc.)

    // For references, possible wrap point marked with ^
    //  "aaa bbb, ccc,ddd. eee   fff. ggg!"
    //      ^    ^    ^   ^   ^__    ^    ^

    // List of hardcoded separators: .,;!?'"

    // Skip extra blanks after a line returns (that includes not counting them in width computation)
    // e.g. "Hello    world" --> "Hello" "World"

    // Cut words that cannot possibly fit within one line.
    // e.g.: "The tropical fish" with ~5 characters worth of width --> "The tr" "opical" "fish"

    let line_width =  0.0;
    let word_width =  0.0;
    let blank_width =  0.0;
    wrap_width /= scale; // We work with unscaled widths to avoid scaling every characters

    const char* word_end = text;
    const char* prev_word_end = None;
    bool inside_word = true;

    const char* s = text;
    while (s < text_end)
    {
        unsigned int c = (unsigned int)*s;
        const char* next_s;
        if (c < 0x80)
            next_s = s + 1;
        else
            next_s = s + text_char_from_utf8(&c, s, text_end);
        if (c == 0)
            break;

        if (c < 32)
        {
            if (c == '\n')
            {
                line_width = word_width = blank_width = 0.0;
                inside_word = true;
                s = next_s;
                continue;
            }
            if (c == '\r')
            {
                s = next_s;
                continue;
            }
        }

        let char_width = (c < IndexAdvanceX.size ? IndexAdvanceX.data[c] : FallbackAdvanceX);
        if (char_is_blank_w(c))
        {
            if (inside_word)
            {
                line_width += blank_width;
                blank_width = 0.0;
                word_end = s;
            }
            blank_width += char_width;
            inside_word = false;
        }
        else
        {
            word_width += char_width;
            if (inside_word)
            {
                word_end = next_s;
            }
            else
            {
                prev_word_end = word_end;
                line_width += word_width + blank_width;
                word_width = blank_width = 0.0;
            }

            // Allow wrapping after punctuation.
            inside_word = (c != '.' && c != ',' && c != ';' && c != '!' && c != '?' && c != '\"');
        }

        // We ignore blank width at the end of the line (they can be skipped)
        if (line_width + word_width > wrap_width)
        {
            // Words that cannot possibly fit within an entire line will be cut anywhere.
            if (word_width < wrap_width)
                s = prev_word_end ? prev_word_end : word_end;
            break;
        }

        s = next_s;
    }

    return s;
}

Vector2D ImFont::calc_text_size_a(float size, float max_width, float wrap_width, const char* text_begin, const char* text_end, const char** remaining) const
{
    if (!text_end)
        text_end = text_begin + strlen(text_begin); // FIXME-OPT: Need to avoid this.

    let line_height = size;
    let scale = size / FontSize;

    Vector2D text_size = Vector2D::new(0, 0);
    let line_width =  0.0;

    let word_wrap_enabled = (wrap_width > 0.0);
    const char* word_wrap_eol = None;

    const char* s = text_begin;
    while (s < text_end)
    {
        if (word_wrap_enabled)
        {
            // Calculate how far we can render. Requires two passes on the string data but keeps the code simple and not intrusive for what's essentially an uncommon feature.
            if (!word_wrap_eol)
            {
                word_wrap_eol = CalcWordWrapPositionA(scale, s, text_end, wrap_width - line_width);
                if (word_wrap_eol == s) // Wrap_width is too small to fit anything. Force displaying 1 character to minimize the height discontinuity.
                    word_wrap_eol += 1;    // +1 may not be a character start point in UTF-8 but it's ok because we use s >= word_wrap_eol below
            }

            if (s >= word_wrap_eol)
            {
                if (text_size.x < line_width)
                    text_size.x = line_width;
                text_size.y += line_height;
                line_width = 0.0;
                word_wrap_eol = None;

                // Wrapping skips upcoming blanks
                while (s < text_end)
                {
                    const char c = *s;
                    if (char_is_blank_a(c)) { s += 1; } else if (c == '\n') { s += 1; break; } else { break; }
                }
                continue;
            }
        }

        // Decode and advance source
        const char* prev_s = s;
        unsigned int c = (unsigned int)*s;
        if (c < 0x80)
        {
            s += 1;
        }
        else
        {
            s += text_char_from_utf8(&c, s, text_end);
            if (c == 0) // Malformed UTF-8?
                break;
        }

        if (c < 32)
        {
            if (c == '\n')
            {
                text_size.x = ImMax(text_size.x, line_width);
                text_size.y += line_height;
                line_width = 0.0;
                continue;
            }
            if (c == '\r')
                continue;
        }

        let char_width = (c < IndexAdvanceX.size ? IndexAdvanceX.data[c] : FallbackAdvanceX) * scale;
        if (line_width + char_width >= max_width)
        {
            s = prev_s;
            break;
        }

        line_width += char_width;
    }

    if (text_size.x < line_width)
        text_size.x = line_width;

    if (line_width > 0 || text_size.y == 0.0)
        text_size.y += line_height;

    if (remaining)
        *remaining = s;

    return text_size;
}

// Note: as with every ImDrawList drawing function, this expects that the font atlas texture is bound.
void ImFont::RenderChar(ImDrawList* draw_list, float size, const Vector2D& pos, ImU32 col, ImWchar c) const
{
    const ImFontGlyph* glyph = FindGlyph(c);
    if (!glyph || !glyph.Visible)
        return;
    if (glyph.Colored)
        col |= ~COLOR32_A_MASK;
    let scale =  (size >= 0.0) ? (size / FontSize) : 1.0;
    let x =  f32::floor(pos.x);
    let y =  f32::floor(pos.y);
    draw_list.prim_reserve(6, 4);
    draw_list.prim_rect_uv(Vector2D::new(x + glyph.X0 * scale, y + glyph.Y0 * scale), Vector2D::new(x + glyph.X1 * scale, y + glyph.Y1 * scale), Vector2D::new(glyph.U0, glyph.V0), Vector2D::new(glyph.U1, glyph.V1), col);
}

// Note: as with every ImDrawList drawing function, this expects that the font atlas texture is bound.
void ImFont::render_text(ImDrawList* draw_list, float size, const Vector2D& pos, ImU32 col, const Vector4D& clip_rect, const char* text_begin, const char* text_end, float wrap_width, bool cpu_fine_clip) const
{
    if (!text_end)
        text_end = text_begin + strlen(text_begin); // ImGui:: functions generally already provides a valid text_end, so this is merely to handle direct calls.

    // Align to be pixel perfect
    let x =  f32::floor(pos.x);
    let y =  f32::floor(pos.y);
    if (y > clip_rect.w)
        return;

    let start_x = x;
    let scale = size / FontSize;
    let line_height = FontSize * scale;
    let word_wrap_enabled = (wrap_width > 0.0);
    const char* word_wrap_eol = None;

    // Fast-forward to first visible line
    const char* s = text_begin;
    if (y + line_height < clip_rect.y && !word_wrap_enabled)
        while (y + line_height < clip_rect.y && s < text_end)
        {
            s = (const char*)memchr(s, '\n', text_end - s);
            s = s ? s + 1 : text_end;
            y += line_height;
        }

    // For large text, scan for the last visible line in order to avoid over-reserving in the call to PrimReserve()
    // Note that very large horizontal line will still be affected by the issue (e.g. a one megabyte string buffer without a newline will likely crash atm)
    if (text_end - s > 10000 && !word_wrap_enabled)
    {
        const char* s_end = s;
        let y_end =  y;
        while (y_end < clip_rect.w && s_end < text_end)
        {
            s_end = (const char*)memchr(s_end, '\n', text_end - s_end);
            s_end = s_end ? s_end + 1 : text_end;
            y_end += line_height;
        }
        text_end = s_end;
    }
    if (s == text_end)
        return;

    // Reserve vertices for remaining worse case (over-reserving is useful and easily amortized)
    let vtx_count_max = (text_end - s) * 4;
    let idx_count_max = (text_end - s) * 6;
    let idx_expected_size = draw_list.idx_buffer.size + idx_count_max;
    draw_list.prim_reserve(idx_count_max, vtx_count_max);

    ImDrawVert* vtx_write = draw_list->vtx_write_ptr;
    ImDrawIdx* idx_write = draw_list->idx_write_ptr;
    unsigned int self.vtx_current_idx = draw_list->vtx_current_idx;

    const ImU32 col_untinted = col | ~COLOR32_A_MASK;

    while (s < text_end)
    {
        if (word_wrap_enabled)
        {
            // Calculate how far we can render. Requires two passes on the string data but keeps the code simple and not intrusive for what's essentially an uncommon feature.
            if (!word_wrap_eol)
            {
                word_wrap_eol = CalcWordWrapPositionA(scale, s, text_end, wrap_width - (x - start_x));
                if (word_wrap_eol == s) // Wrap_width is too small to fit anything. Force displaying 1 character to minimize the height discontinuity.
                    word_wrap_eol += 1;    // +1 may not be a character start point in UTF-8 but it's ok because we use s >= word_wrap_eol below
            }

            if (s >= word_wrap_eol)
            {
                x = start_x;
                y += line_height;
                word_wrap_eol = None;

                // Wrapping skips upcoming blanks
                while (s < text_end)
                {
                    const char c = *s;
                    if (char_is_blank_a(c)) { s += 1; } else if (c == '\n') { s += 1; break; } else { break; }
                }
                continue;
            }
        }

        // Decode and advance source
        unsigned int c = (unsigned int)*s;
        if (c < 0x80)
        {
            s += 1;
        }
        else
        {
            s += text_char_from_utf8(&c, s, text_end);
            if (c == 0) // Malformed UTF-8?
                break;
        }

        if (c < 32)
        {
            if (c == '\n')
            {
                x = start_x;
                y += line_height;
                if (y > clip_rect.w)
                    break; // break out of main loop
                continue;
            }
            if (c == '\r')
                continue;
        }

        const ImFontGlyph* glyph = FindGlyph((ImWchar)c);
        if (glyph == None)
            continue;

        let char_width =  glyph.AdvanceX * scale;
        if (glyph.Visible)
        {
            // We don't do a second finer clipping test on the Y axis as we've already skipped anything before clip_rect.y and exit once we pass clip_rect.w
            let x1 =  x + glyph.X0 * scale;
            let x2 =  x + glyph.X1 * scale;
            let y1 =  y + glyph.Y0 * scale;
            let y2 =  y + glyph.Y1 * scale;
            if (x1 <= clip_rect.z && x2 >= clip_rect.x)
            {
                // Render a character
                let u1 =  glyph.U0;
                let v1 =  glyph.V0;
                let u2 =  glyph.U1;
                let v2 =  glyph.V1;

                // CPU side clipping used to fit text in their frame when the frame is too small. Only does clipping for axis aligned quads.
                if (cpu_fine_clip)
                {
                    if (x1 < clip_rect.x)
                    {
                        u1 = u1 + (1.0 - (x2 - clip_rect.x) / (x2 - x1)) * (u2 - u1);
                        x1 = clip_rect.x;
                    }
                    if (y1 < clip_rect.y)
                    {
                        v1 = v1 + (1.0 - (y2 - clip_rect.y) / (y2 - y1)) * (v2 - v1);
                        y1 = clip_rect.y;
                    }
                    if (x2 > clip_rect.z)
                    {
                        u2 = u1 + ((clip_rect.z - x1) / (x2 - x1)) * (u2 - u1);
                        x2 = clip_rect.z;
                    }
                    if (y2 > clip_rect.w)
                    {
                        v2 = v1 + ((clip_rect.w - y1) / (y2 - y1)) * (v2 - v1);
                        y2 = clip_rect.w;
                    }
                    if (y1 >= y2)
                    {
                        x += char_width;
                        continue;
                    }
                }

                // Support for untinted glyphs
                ImU32 glyph_col = glyph.Colored ? col_untinted : col;

                // We are NOT calling prim_rect_uv() here because non-inlined causes too much overhead in a debug builds. Inlined here:
                {
                    idx_write[0] = (self.vtx_current_idx); idx_write[1] = (self.vtx_current_idx+1); idx_write[2] = (self.vtx_current_idx+2);
                    idx_write[3] = (self.vtx_current_idx); idx_write[4] = (self.vtx_current_idx+2); idx_write[5] = (self.vtx_current_idx+3);
                    vtx_write[0].pos.x = x1; vtx_write[0].pos.y = y1; vtx_write[0].col = glyph_col; vtx_write[0].uv.x = u1; vtx_write[0].uv.y = v1;
                    vtx_write[1].pos.x = x2; vtx_write[1].pos.y = y1; vtx_write[1].col = glyph_col; vtx_write[1].uv.x = u2; vtx_write[1].uv.y = v1;
                    vtx_write[2].pos.x = x2; vtx_write[2].pos.y = y2; vtx_write[2].col = glyph_col; vtx_write[2].uv.x = u2; vtx_write[2].uv.y = v2;
                    vtx_write[3].pos.x = x1; vtx_write[3].pos.y = y2; vtx_write[3].col = glyph_col; vtx_write[3].uv.x = u1; vtx_write[3].uv.y = v2;
                    vtx_write += 4;
                    self.vtx_current_idx += 4;
                    idx_write += 6;
                }
            }
        }
        x += char_width;
    }

    // Give back unused vertices (clipped ones, blanks) ~ this is essentially a PrimUnreserve() action.
    draw_list.vtx_buffer.size = (vtx_write - draw_list.vtx_buffer.data); // Same as calling shrink()
    draw_list.idx_buffer.size = (idx_write - draw_list.idx_buffer.data);
    draw_list.cmd_buffer[draw_list.cmd_buffer.size - 1].elem_count -= (idx_expected_size - draw_list.idx_buffer.size);
    draw_list->vtx_write_ptr = vtx_write;
    draw_list->idx_write_ptr = idx_write;
    draw_list->vtx_current_idx = self.vtx_current_idx;
}


//-----------------------------------------------------------------------------
// [SECTION] Default font data (ProggyClean.ttf)
//-----------------------------------------------------------------------------
// ProggyClean.ttf
// Copyright (c) 2004, 2005 Tristan Grimmer
// MIT license (see License.txt in http://www.upperbounds.net/download/ProggyClean.ttf.zip)
// Download and more information at http://upperbounds.net
//-----------------------------------------------------------------------------
// File: 'ProggyClean.ttf' (41208 bytes)
// Exported using misc/fonts/binary_to_compressed_c.cpp (with compression + base85 string encoding).
// The purpose of encoding as base85 instead of "0x00,0x01,..." style is only save on _source code_ size.
//-----------------------------------------------------------------------------
static const char proggy_clean_ttf_compressed_data_base85[11980 + 1] =
    "7])#######hV0qs'/###[),##/l:$#Q6>##5[n42>c-TH`->>#/e>11NNV=Bv(*:.F?uu#(gRU.o0XGH`$vhLG1hxt9?W`#,5LsCp#-i>.r$<$6pD>Lb';9Crc6tgXmKVeU2cD4Eo3R/"
    "2*>]b(MC;$jPfY.;h^`IWM9<Lh2TlS+f-s$o6Q<BWH`YiU.xfLq$N;$0iR/GX:U(jcW2p/W*q?-qmnUCI;jHSAiFWM.R*kU@C=GH?a9wp8f$e.-4^Qg1)Q-GL(lf(r/7GrRgwV%MS=C#"
    "`8ND>Qo#t'x#(v#Y9w0#1D$CIf;W'#pWUPXOuxXuU(H9M(1<q-UE31#^-V'8IRUo7Qf./L>=Ke$$'5F%)]0^#0X@U.a<r:QLtFsLcL6##lOj)#.Y5<-R&KgLwqJfLgN&;Q?gI^#DY2uL"
    "i@^rMl9t=cWq6##weg>$FBjVQTSDgEKnIS7EM9>ZY9w0#L;>>#Mx&4Mvt//L[MkA#W@lK.N'[0#7RL_&#w+F%HtG9M#XL`N&.,GM4Pg;-<nLENhvx>-VsM.M0rJfLH2eTM`*oJMHRC`N"
    "kfimM2J,W-jXS:)r0wK#@Fge$U>`w'N7G#$#fB#$E^$#:9:hk+eOe--6x)F7*E%?76%^GMHePW-Z5l'&GiF#$956:rS?dA#fiK:)Yr+`&#0j@'DbG&#^$PG.Ll+DNa<XCMKEV*N)LN/N"
    "*b=%Q6pia-Xg8I$<MR&,VdJe$<(7G;Ckl'&hF;;$<_=x(b.RS%%)###MPBuuE1V:v&cX&#2m#(&cV]`k9OhLMbn%s$G2,B$BfD3X*sp5#l,$R#]x_X1xKX%b5U*[r5iMfUo9U`N99hG)"
    "tm+/Us9pG)XPu`<0s-)WTt(gCRxIg(%6sfh=ktMKn3j)<6<b5Sk_/0(^]AaN#(p/L>&VZ>1i%h1S9u5o@YaaW$e+b<TWFn/Z:Oh(Cx2$lNEoN^e)#CFY@@I;BOQ*sRwZtZxRcU7uW6CX"
    "ow0i(?$Q[cjOd[P4d)]>ROPOpxTO7Stwi1::iB1q)C_=dV26J;2,]7op$]uQr@_V7$q^%lQwtuHY]=DX,n3L#0PHDO4f9>dC@O>HBuKPpP*E,N+b3L#lpR/MrTEH.IAQk.a>D[.e;mc."
    "x]Ip.PH^'/aqUO/$1WxLoW0[iLA<QT;5HKD+@qQ'NQ(3_PLhE48R.qAPSwQ0/WK?Z,[x?-J;jQTWA0X@KJ(_Y8N-:/M74:/-ZpKrUss?d#dZq]DAbkU*JqkL+nwX@@47`5>w=4h(9.`G"
    "CRUxHPeR`5Mjol(dUWxZa(>STrPkrJiWx`5U7F#.g*jrohGg`cg:lSTvEY/EV_7H4Q9[Z%cnv;JQYZ5q.l7Zeas:HOIZOB?G<Nald$qs]@]L<J7bR*>gv:[7MI2k).'2($5FNP&EQ(,)"
    "U]W]+fh18.vsai00);D3@4ku5P?DP8aJt+;qUM]=+b'8@;mViBKx0DE[-auGl8:PJ&Dj+M6OC]O^((##]`0i)drT;-7X`=-H3[igUnPG-NZlo.#k@h#=Ork$m>a>$-?Tm$UV(?#P6YY#"
    "'/###xe7q.73rI3*pP/$1>s9)W,JrM7SN]'/4C#v$U`0#V.[0>xQsH$fEmPMgY2u7Kh(G%siIfLSoS+MK2eTM$=5,M8p`A.;_R%#u[K#$x4AG8.kK/HSB==-'Ie/QTtG?-.*^N-4B/ZM"
    "_3YlQC7(p7q)&](`6_c)$/*JL(L-^(]$wIM`dPtOdGA,U3:w2M-0<q-]L_?^)1vw'.,MRsqVr.L;aN&#/EgJ)PBc[-f>+WomX2u7lqM2iEumMTcsF?-aT=Z-97UEnXglEn1K-bnEO`gu"
    "Ft(c%=;Am_Qs@jLooI&NX;]0#j4#F14;gl8-GQpgwhrq8'=l_f-b49'UOqkLu7-##oDY2L(te+Mch&gLYtJ,MEtJfLh'x'M=$CS-ZZ%P]8bZ>#S?YY#%Q&q'3^Fw&?D)UDNrocM3A76/"
    "/oL?#h7gl85[qW/NDOk%16ij;+:1a'iNIdb-ou8.P*w,v5#EI$TWS>Pot-R*H'-SEpA:g)f+O$%%`kA#G=8RMmG1&O`>to8bC]T&$,n.LoO>29sp3dt-52U%VM#q7'DHpg+#Z9%H[K<L"
    "%a2E-grWVM3@2=-k22tL]4$##6We'8UJCKE[d_=%wI;'6X-GsLX4j^SgJ$##R*w,vP3wK#iiW&#*h^D&R?jp7+/u&#(AP##XU8c$fSYW-J95_-Dp[g9wcO&#M-h1OcJlc-*vpw0xUX&#"
    "OQFKNX@QI'IoPp7nb,QU//MQ&ZDkKP)x<WSVL(68uVl&#c'[0#(s1X&xm$Y%B7*K:eDA323j998GXbA#pwMs-jgD$9QISB-A_(aN4xoFM^@C58D0+Q+q3n0#3U1InDjF682-SjMXJK)("
    "h$hxua_K]ul92%'BOU&#BRRh-slg8KDlr:%L71Ka:.A;%YULjDPmL<LYs8i#XwJOYaKPKc1h:'9Ke,g)b),78=I39B;xiY$bgGw-&.Zi9InXDuYa%G*f2Bq7mn9^#p1vv%#(Wi-;/Z5h"
    "o;#2:;%d&#x9v68C5g?ntX0X)pT`;%pB3q7mgGN)3%(P8nTd5L7GeA-GL@+%J3u2:(Yf>et`e;)f#Km8&+DC$I46>#Kr]]u-[=99tts1.qb#q72g1WJO81q+eN'03'eM>&1XxY-caEnO"
    "j%2n8)),?ILR5^.Ibn<-x-Mq7[a82Lq:F&#ce+S9wsCK*x`569E8ew'He]h:sI[2LM$[guka3ZRd6:t%IG:;$%YiJ:Nq=?eAw;/:nnDq0(CYcMpG)qLN4$##&J<j$UpK<Q4a1]MupW^-"
    "sj_$%[HK%'F####QRZJ::Y3EGl4'@%FkiAOg#p[##O`gukTfBHagL<LHw%q&OV0##F=6/:chIm0@eCP8X]:kFI%hl8hgO@RcBhS-@Qb$%+m=hPDLg*%K8ln(wcf3/'DW-$.lR?n[nCH-"
    "eXOONTJlh:.RYF%3'p6sq:UIMA945&^HFS87@$EP2iG<-lCO$%c`uKGD3rC$x0BL8aFn--`ke%#HMP'vh1/R&O_J9'um,.<tx[@%wsJk&bUT2`0uMv7gg#qp/ij.L56'hl;.s5CUrxjO"
    "M7-##.l+Au'A&O:-T72L]P`&=;ctp'XScX*rU.>-XTt,%OVU4)S1+R-#dg0/Nn?Ku1^0f$B*P:Rowwm-`0PKjYDDM'3]d39VZHEl4,.j']Pk-M.h^&:0FACm$maq-&sgw0t7/6(^xtk%"
    "LuH88Fj-ekm>GA#_>568x6(OFRl-IZp`&b,_P'$M<Jnq79VsJW/mWS*PUiq76;]/NM_>hLbxfc$mj`,O;&%W2m`Zh:/)Uetw:aJ%]K9h:TcF]u_-Sj9,VK3M.*'&0D[Ca]J9gp8,kAW]"
    "%(?A%R$f<->Zts'^kn=-^@c4%-pY6qI%J%1IGxfLU9CP8cbPlXv);C=b),<2mOvP8up,UVf3839acAWAW-W?#ao/^#%KYo8fRULNd2.>%m]UK:n%r$'sw]J;5pAoO_#2mO3n,'=H5(et"
    "Hg*`+RLgv>=4U8guD$I%D:W>-r5V*%j*W:Kvej.Lp$<M-SGZ':+Q_k+uvOSLiEo(<aD/K<CCc`'Lx>'?;++O'>()jLR-^u68PHm8ZFWe+ej8h:9r6L*0//c&iH&R8pRbA#Kjm%upV1g:"
    "a_#Ur7FuA#(tRh#.Y5K+@?3<-8m0$PEn;J:rh6?I6uG<-`wMU'ircp0LaE_OtlMb&1#6T.#FDKu#1Lw%u%+GM+x'e?YLfjM[VO0MbuFp7;>Q&#WIo)0@F%q7c#4XAXN-U&VB<HFF*qL("
    "$/V,;(kXZejWO`<[5?\?ewY(*9=%wDc;,u<'9t3W-(H1th3+G]ucQ]kLs7df($/*JL]@*t7Bu_G3_7mp7<iaQjO@.kLg;x3B0lqp7Hf,^Ze7-##@/c58Mo(3;knp0%)A7?-W+eI'o8)b<"
    "nKnw'Ho8C=Y>pqB>0ie&jhZ[?iLR@@_AvA-iQC(=ksRZRVp7`.=+NpBC%rh&3]R:8XDmE5^V8O(x<<aG/1N$#FX$0V5Y6x'aErI3I$7x%E`v<-BY,)%-?Psf*l?%C3.mM(=/M0:JxG'?"
    "7WhH%o'a<-80g0NBxoO(GH<dM]n.+%q@jH?f.UsJ2Ggs&4<-e47&Kl+f//9@`b+?.TeN_&B8Ss?v;^Trk;f#YvJkl&w$]>-+k?'(<S:68tq*WoDfZu';mM?8X[ma8W%*`-=;D.(nc7/;"
    ")g:T1=^J$&BRV(-lTmNB6xqB[@0*o.erM*<SWF]u2=st-*(6v>^](H.aREZSi,#1:[IXaZFOm<-ui#qUq2$##Ri;u75OK#(RtaW-K-F`S+cF]uN`-KMQ%rP/Xri.LRcB##=YL3BgM/3M"
    "D?@f&1'BW-)Ju<L25gl8uhVm1hL$##*8###'A3/LkKW+(^rWX?5W_8g)a(m&K8P>#bmmWCMkk&#TR`C,5d>g)F;t,4:@_l8G/5h4vUd%&%950:VXD'QdWoY-F$BtUwmfe$YqL'8(PWX("
    "P?^@Po3$##`MSs?DWBZ/S>+4%>fX,VWv/w'KD`LP5IbH;rTV>n3cEK8U#bX]l-/V+^lj3;vlMb&[5YQ8#pekX9JP3XUC72L,,?+Ni&co7ApnO*5NK,((W-i:$,kp'UDAO(G0Sq7MVjJs"
    "bIu)'Z,*[>br5fX^:FPAWr-m2KgL<LUN098kTF&#lvo58=/vjDo;.;)Ka*hLR#/k=rKbxuV`>Q_nN6'8uTG&#1T5g)uLv:873UpTLgH+#FgpH'_o1780Ph8KmxQJ8#H72L4@768@Tm&Q"
    "h4CB/5OvmA&,Q&QbUoi$a_%3M01H)4x7I^&KQVgtFnV+;[Pc>[m4k//,]1?#`VY[Jr*3&&slRfLiVZJ:]?=K3Sw=[$=uRB?3xk48@aeg<Z'<$#4H)6,>e0jT6'N#(q%.O=?2S]u*(m<-"
    "V8J'(1)G][68hW$5'q[GC&5j`TE?m'esFGNRM)j,ffZ?-qx8;->g4t*:CIP/[Qap7/9'#(1sao7w-.qNUdkJ)tCF&#B^;xGvn2r9FEPFFFcL@.iFNkTve$m%#QvQS8U@)2Z+3K:AKM5i"
    "sZ88+dKQ)W6>J%CL<KE>`.d*(B`-n8D9oK<Up]c$x$(,)M8Zt7/[rdkqTgl-0cuGMv'?>-XV1q['-5k'cAZ69e;D_?$ZPP&s^+7])$*$#@QYi9,5P&#9r+$%CE=68>K8r0=dSC%%(@p7"
    ".m7jilQ02'0-VWAg<a/''3u.=4L$Y)6k/K:_[3=&jvL<L0C/2'v:^;-DIBW,B4E68:kZ;%?8(Q8BH=kO65BW?xSG&#@uU,DS*,?.+(o(#1vCS8#CHF>TlGW'b)Tq7VT9q^*^$$.:&N@@"
    "$&)WHtPm*5_rO0&e%K&#-30j(E4#'Zb.o/(Tpm$>K'f@[PvFl,hfINTNU6u'0pao7%XUp9]5.>%h`8_=VYbxuel.NTSsJfLacFu3B'lQSu/m6-Oqem8T+oE--$0a/k]uj9EwsG>%veR*"
    "hv^BFpQj:K'#SJ,sB-'#](j.Lg92rTw-*n%@/;39rrJF,l#qV%OrtBeC6/,;qB3ebNW[?,Hqj2L.1NP&GjUR=1D8QaS3Up&@*9wP?+lo7b?@%'k4`p0Z$22%K3+iCZj?XJN4Nm&+YF]u"
    "@-W$U%VEQ/,,>>#)D<h#`)h0:<Q6909ua+&VU%n2:cG3FJ-%@Bj-DgLr`Hw&HAKjKjseK</xKT*)B,N9X3]krc12t'pgTV(Lv-tL[xg_%=M_q7a^x?7Ubd>#%8cY#YZ?=,`Wdxu/ae&#"
    "w6)R89tI#6@s'(6Bf7a&?S=^ZI_kS&ai`&=tE72L_D,;^R)7[$s<Eh#c&)q.MXI%#v9ROa5FZO%sF7q7Nwb&#ptUJ:aqJe$Sl68%.D###EC><?-aF&#RNQv>o8lKN%5/$(vdfq7+ebA#"
    "u1p]ovUKW&Y%q]'>$1@-[xfn$7ZTp7mM,G,Ko7a&Gu%G[RMxJs[0MM%wci.LFDK)(<c`Q8N)jEIF*+?P2a8g%)$q]o2aH8C&<SibC/q,(e:v;-b#6[$NtDZ84Je2KNvB#$P5?tQ3nt(0"
    "d=j.LQf./Ll33+(;q3L-w=8dX$#WF&uIJ@-bfI>%:_i2B5CsR8&9Z&#=mPEnm0f`<&c)QL5uJ#%u%lJj+D-r;BoF&#4DoS97h5g)E#o:&S4weDF,9^Hoe`h*L+_a*NrLW-1pG_&2UdB8"
    "6e%B/:=>)N4xeW.*wft-;$'58-ESqr<b?UI(_%@[P46>#U`'6AQ]m&6/`Z>#S?YY#Vc;r7U2&326d=w&H####?TZ`*4?&.MK?LP8Vxg>$[QXc%QJv92.(Db*B)gb*BM9dM*hJMAo*c&#"
    "b0v=Pjer]$gG&JXDf->'StvU7505l9$AFvgYRI^&<^b68?j#q9QX4SM'RO#&sL1IM.rJfLUAj221]d##DW=m83u5;'bYx,*Sl0hL(W;;$doB&O/TQ:(Z^xBdLjL<Lni;''x.`$#8+1GD"
    ":k$YUWsbn8ogh6rxZ2Z9]%nd+>V#*8U_72Lh+2Q8Cj0i:6hp&$C/:p(HK>T8Y[gHQ4`4)'$Ab(Nof%V'8hL&#<NEdtg(n'=S1A(Q1/I&4([%dM`,Iu'1:_hL>SfD07&6D<fp8dHM7/g+"
    "tlPN9J*rKaPct&?'uBCem^jn%9_K)<,C5K3s=5g&GmJb*[SYq7K;TRLGCsM-$$;S%:Y@r7AK0pprpL<Lrh,q7e/%KWK:50I^+m'vi`3?%Zp+<-d+$L-Sv:@.o19n$s0&39;kn;S%BSq*"
    "$3WoJSCLweV[aZ'MQIjO<7;x-x;&+dMLvu#^UsGEC9WEc[x(wI7#2.(F0jV*eZf<-Qv3J-c+J5AlrB#$p(H68LvEA'q3n0#m,[`*8Ft)FcYgEud]CWfm68,(aLA$@EFTgLXoBq/UPlp7"
    ":d[/;r_ix=:TF`S5H-b<LI&HY(K=h#)]Lk$K14lVfm:x$H<3^Ql<M`$OhapBnkup'D#L$Pb_`N*g]2e;x/Dtg,bsj&K#2[-:iYr'_wgH)NUIR8a1n#S?Yej'h8^58UbZd+^FKD*T@;6A"
    "7aQC[K8d-(v6GI$x:T<&'Gp5Uf>@M.*J:;$-rv29'M]8qMv-tLp,'886iaC=Hb*YJoKJ,(j%K=H`K.v9HggqBIiZu'QvBT.#=)0ukruV&.)3=(^1`o*Pj4<-<aN((^7('#Z0wK#5GX@7"
    "u][`*S^43933A4rl][`*O4CgLEl]v$1Q3AeF37dbXk,.)vj#x'd`;qgbQR%FW,2(?LO=s%Sc68%NP'##Aotl8x=BE#j1UD([3$M(]UI2LX3RpKN@;/#f'f/&_mt&F)XdF<9t4)Qa.*kT"
    "LwQ'(TTB9.xH'>#MJ+gLq9-##@HuZPN0]u:h7.T..G:;$/Usj(T7`Q8tT72LnYl<-qx8;-HV7Q-&Xdx%1a,hC=0u+HlsV>nuIQL-5<N?)NBS)QN*_I,?&)2'IM%L3I)x((e/dl2&8'<M"
    ":^#M*Q+[T.Xri.LYS3v%fF`68h;b-x[/En'CR.q7E)p'/kle2HM,u;^%OKC-N+Ll%F9CF<Nf'^#t2L,;27W:0O@6##U6W7:$rJfLWHj$#)woqBefIZ.PK<b*t7ed;p*_m;4ExK#h@&]>"
    "_>@kXQtMacfD.m-VAb8;IReM3$wf0''hra*so568'Ip&vRs849'MRYSp%:t:h5qSgwpEr$B>Q,;s(C#$)`svQuF$##-D,##,g68@2[T;.XSdN9Qe)rpt._K-#5wF)sP'##p#C0c%-Gb%"
    "hd+<-j'Ai*x&&HMkT]C'OSl##5RG[JXaHN;d'uA#x._U;.`PU@(Z3dt4r152@:v,'R.Sj'w#0<-;kPI)FfJ&#AYJ&#//)>-k=m=*XnK$>=)72L]0I%>.G690a:$##<,);?;72#?x9+d;"
    "^V'9;jY@;)br#q^YQpx:x#Te$Z^'=-=bGhLf:D6&bNwZ9-ZD#n^9HhLMr5G;']d&6'wYmTFmL<LD)F^%[tC'8;+9E#C$g%#5Y>q9wI>P(9mI[>kC-ekLC/R&CH+s'B;K-M6$EB%is00:"
    "+A4[7xks.LrNk0&E)wILYF@2L'0Nb$+pv<(2.768/FrY&h$^3i&@+G%JT'<-,v`3;_)I9M^AE]CN?Cl2AZg+%4iTpT3<n-&%H%b<FDj2M<hH=&Eh<2Len$b*aTX=-8QxN)k11IM1c^j%"
    "9s<L<NFSo)B?+<-(GxsF,^-Eh@$4dXhN$+#rxK8'je'D7k`e;)2pYwPA'_p9&@^18ml1^[@g4t*[JOa*[=Qp7(qJ_oOL^('7fB&Hq-:sf,sNj8xq^>$U4O]GKx'm9)b@p7YsvK3w^YR-"
    "CdQ*:Ir<($u&)#(&?L9Rg3H)4fiEp^iI9O8KnTj,]H?D*r7'M;PwZ9K0E^k&-cpI;.p/6_vwoFMV<->#%Xi.LxVnrU(4&8/P+:hLSKj$#U%]49t'I:rgMi'FL@a:0Y-uA[39',(vbma*"
    "hU%<-SRF`Tt:542R_VV$p@[p8DV[A,?1839FWdF<TddF<9Ah-6&9tWoDlh]&1SpGMq>Ti1O*H&#(AL8[_P%.M>v^-))qOT*F5Cq0`Ye%+$B6i:7@0IX<N+T+0MlMBPQ*Vj>SsD<U4JHY"
    "8kD2)2fU/M#$e.)T4,_=8hLim[&);?UkK'-x?'(:siIfL<$pFM`i<?%W(mGDHM%>iWP,##P`%/L<eXi:@Z9C.7o=@(pXdAO/NLQ8lPl+HPOQa8wD8=^GlPa8TKI1CjhsCTSLJM'/Wl>-"
    "S(qw%sf/@%#B6;/U7K]uZbi^Oc^2n<bhPmUkMw>%t<)'mEVE''n`WnJra$^TKvX5B>;_aSEK',(hwa0:i4G?.Bci.(x[?b*($,=-n<.Q%`(x=?+@Am*Js0&=3bh8K]mL<LoNs'6,'85`"
    "0?t/'_U59@]ddF<#LdF<eWdF<OuN/45rY<-L@&#+fm>69=Lb,OcZV/);TTm8VI;?%OtJ<(b4mq7M6:u?KRdF<gR@2L=FNU-<b[(9c/ML3m;Z[$oF3g)GAWqpARc=<ROu7cL5l;-[A]%/"
    "+fsd;l#SafT/f*W]0=O'$(Tb<[)*@e775R-:Yob%g*>l*:xP?Yb.5)%w_I?7uk5JC+FS(m#i'k.'a0i)9<7b'fs'59hq$*5Uhv##pi^8+hIEBF`nvo`;'l0.^S1<-wUK2/Coh58KKhLj"
    "M=SO*rfO`+qC`W-On.=AJ56>>i2@2LH6A:&5q`?9I3@@'04&p2/LVa*T-4<-i3;M9UvZd+N7>b*eIwg:CC)c<>nO&#<IGe;__.thjZl<%w(Wk2xmp4Q@I#I9,DF]u7-P=.-_:YJ]aS@V"
    "?6*C()dOp7:WL,b&3Rg/.cmM9&r^>$(>.Z-I&J(Q0Hd5Q%7Co-b`-c<N(6r@ip+AurK<m86QIth*#v;-OBqi+L7wDE-Ir8K['m+DDSLwK&/.?-V%U_%3:qKNu$_b*B-kp7NaD'QdWQPK"
    "Yq[@>P)hI;*_F]u`Rb[.j8_Q/<&>uu+VsH$sM9TA%?)(vmJ80),P7E>)tjD%2L=-t#fK[%`v=Q8<FfNkgg^oIbah*#8/Qt$F&:K*-(N/'+1vMB,u()-a.VUU*#[e%gAAO(S>WlA2);Sa"
    ">gXm8YB`1d@K#n]76-a$U,mF<fX]idqd)<3,]J7JmW4`6]uks=4-72L(jEk+:bJ0M^q-8Dm_Z?0olP1C9Sa&H[d&c$ooQUj]Exd*3ZM@-WGW2%s',B-_M%>%Ul:#/'xoFM9QX-$.QN'>"
    "[%$Z$uF6pA6Ki2O5:8w*vP1<-1`[G,)-m#>0`P&#eb#.3i)rtB61(o'$?X3B</R90;eZ]%Ncq;-Tl]#F>2Qft^ae_5tKL9MUe9b*sLEQ95C&`=G?@Mj=wh*'3E>=-<)Gt*Iw)'QG:`@I"
    "wOf7&]1i'S01B+Ev/Nac#9S;=;YQpg_6U`*kVY39xK,[/6Aj7:'1Bm-_1EYfa1+o&o4hp7KN_Q(OlIo@S%;jVdn0'1<Vc52=u`3^o-n1'g4v58Hj&6_t7$##?M)c<$bgQ_'SY((-xkA#"
    "Y(,p'H9rIVY-b,'%bCPF7.J<Up^,(dU1VY*5#WkTU>h19w,WQhLI)3S#f$2(eb,jr*b;3Vw]*7NH%$c4Vs,eD9>XW8?N]o+(*pgC%/72LV-u<Hp,3@e^9UB1J+ak9-TN/mhKPg+AJYd$"
    "MlvAF_jCK*.O-^(63adMT->W%iewS8W6m2rtCpo'RS1R84=@paTKt)>=%&1[)*vp'u+x,VrwN;&]kuO9JDbg=pO$J*.jVe;u'm0dr9l,<*wMK*Oe=g8lV_KEBFkO'oU]^=[-792#ok,)"
    "i]lR8qQ2oA8wcRCZ^7w/Njh;?.stX?Q1>S1q4Bn$)K1<-rGdO'$Wr.Lc.CG)$/*JL4tNR/,SVO3,aUw'DJN:)Ss;wGn9A32ijw%FL+Z0Fn.U9;reSq)bmI32U==5ALuG&#Vf1398/pVo"
    "1*c-(aY168o<`JsSbk-,1N;$>0:OUas(3:8Z972LSfF8eb=c-;>SPw7.6hn3m`9^Xkn(r.qS[0;T%&Qc=+STRxX'q1BNk3&*eu2;&8q$&x>Q#Q7^Tf+6<(d%ZVmj2bDi%.3L2n+4W'$P"
    "iDDG)g,r%+?,$@?uou5tSe2aN_AQU*<h`e-GI7)?OK2A.d7_c)?wQ5AS@DL3r#7fSkgl6-++D:'A,uq7SvlB$pcpH'q3n0#_%dY#xCpr-l<F0NR@-##FEV6NTF6##$l84N1w?AO>'IAO"
    "URQ##V^Fv-XFbGM7Fl(N<3DhLGF%q.1rC$#:T__&Pi68%0xi_&[qFJ(77j_&JWoF.V735&T,[R*:xFR*K5>>#`bW-?4Ne_&6Ne_&6Ne_&n`kr-#GJcM6X;uM6X;uM(.a..^2TkL%oR(#"
    ";u.T%fAr%4tJ8&><1=GHZ_+m9/#H1F^R#SC#*N=BA9(D?v[UiFY>>^8p,KKF.W]L29uLkLlu/+4T<XoIB&hx=T1PcDaB&;HH+-AFr?(m9HZV)FKS8JCw;SD=6[^/DZUL`EUDf]GGlG&>"
    "w$)F./^n3+rlo+DB;5sIYGNk+i1t-69Jg--0pao7Sm#K)pdHW&;LuDNH@H>#/x-TI(;P>#,Gc>#0Su>#4`1?#8lC?#<xU?#@.i?#D:%@#HF7@#LRI@#P_[@#Tkn@#Xw*A#]-=A#a9OA#"
    "d<F&#*;G##.GY##2Sl##6`($#:l:$#>xL$#B.`$#F:r$#JF.%#NR@%#R_R%#Vke%#Zww%#_-4&#3^Rh%Sflr-k'MS.o?.5/sWel/wpEM0%3'/1)K^f1-d>G21&v(35>V`39V7A4=onx4"
    "A1OY5EI0;6Ibgr6M$HS7Q<)58C5w,;WoA*#[%T*#`1g*#d=#+#hI5+#lUG+#pbY+#tnl+#x$),#&1;,#*=M,#.I`,#2Ur,#6b.-#;w[H#iQtA#m^0B#qjBB#uvTB##-hB#'9$C#+E6C#"
    "/QHC#3^ZC#7jmC#;v)D#?,<D#C8ND#GDaD#KPsD#O]/E#g1A5#KA*1#gC17#MGd;#8(02#L-d3#rWM4#Hga1#,<w0#T.j<#O#'2#CYN1#qa^:#_4m3#o@/=#eG8=#t8J5#`+78#4uI-#"
    "m3B2#SB[8#Q0@8#i[*9#iOn8#1Nm;#^sN9#qh<9#:=x-#P;K2#$%X9#bC+.#Rg;<#mN=.#MTF.#RZO.#2?)4#Y#(/#[)1/#b;L/#dAU/#0Sv;#lY$0#n`-0#sf60#(F24#wrH0#%/e0#"
    "TmD<#%JSMFove:CTBEXI:<eh2g)B,3h2^G3i;#d3jD>)4kMYD4lVu`4m`:&5niUA5@(A5BA1]PBB:xlBCC=2CDLXMCEUtiCf&0g2'tN?PGT4CPGT4CPGT4CPGT4CPGT4CPGT4CPGT4CP"
    "GT4CPGT4CPGT4CPGT4CPGT4CPGT4CP-qekC`.9kEg^+F$kwViFJTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5o,^<-28ZI'O?;xp"
    "O?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xp;7q-#lLYI:xvD=#";

static const char* GetDefaultCompressedFontDataTTFBase85()
{
    return proggy_clean_ttf_compressed_data_base85;
}
