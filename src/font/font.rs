use crate::color::COLOR32_A_MASK;
use crate::draw::DrawList;
use crate::font::find_first_existing_glyph;
use crate::font::font_atlas::FontAtlas;
use crate::font::font_glyph::FontGlyph;
use crate::font::font_config::FontConfig;
use crate::string::char_is_blank_w;
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
    pub index_lookup: Vec<char>, //ImVector<ImWchar>           index_lookup;        // 12-16 // out //            // Sparse. index glyphs by Unicode code-point.
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

    pub fn find_glyph(&self, c: char) -> FontGlyph {
        if (c as usize) >= self.index_lookup.size {
            return self.fallback_glyph.clone();
        }
        let i = self.index_lookup[c];
        if i == '\0' {
            return self.fallback_glyph.clone();
        }
        return self.glyphs[i].clone();
    }
    //      const ImFontGlyph*find_glyph_no_fallback(ImWchar c) const;
    pub fn find_glyph_no_fallback(&self, c: char) -> Option<FontGlyph> {

            if c as usize >= self.index_lookup.len() {
                return None;
            }
    let i = self.index_lookup.data[c];
    if i == '\0' {
        return None;
    }
    return self.glyphs.data[i].clone();
    }

    pub fn get_char_advance(&self, c: char) -> f32 {
        if c < self.index_advance_x.len() as char {
            self.index_advance_x[c]
        }
        self.fallback_advance_x
    }

    pub fn is_loaded(&self) -> bool {
        self.container_atlas.is_some()
    }

    pub fn get_debug_name(&self) -> String {
        if self.config_data.is_some() {
            self.config_data.unwrap().name
        }
        "<unknown>".to_string()
    }

    //
    //     // 'max_width' stops rendering after a certain width (could be turned into a 2d size). FLT_MAX to disable.
    //     // 'wrap_width' enable automatic word-wrapping across multiple lines to fit into given width. 0.0 to disable.
    pub fn calc_text_size_a(&self, size: f32, max_width: f32, wrap_width: f32, text: &String) -> Vector2D {
        // if (!text_end)
        // text_end = text_begin + strlen(text_begin); // FIXME-OPT: Need to avoid this.

        let line_height = size;
        let scale = size / self.font_size;

        let mut text_size = Vector2D::new(0f32, 0f32);
        let line_width =  0.0;

        let word_wrap_enabled = (wrap_width > 0.0);
        // const char* word_wrap_eol = None;
        let mut word_wrap_eol_offset: usize = 0;

        // const char* s = text_begin;
        let s_offset: usize = 0;
        let text_end_offset: usize = text.len();
        while s_offset < text_end_offset
        {
            if word_wrap_enabled
            {
                // Calculate how far we can render. Requires two passes on the string data but keeps the code simple and not intrusive for what's essentially an uncommon feature.
                if !word_wrap_eol_offset
                {
                    word_wrap_eol_offset = self.calc_word_wrap_position_a(scale, &text[s_offset..],  wrap_width - line_width);
                    // Wrap_width is too small to fit anything. Force displaying 1 character to minimize the height discontinuity.
                    if word_wrap_eol_offset == s_offset {
                        // +1 may not be a character start point in UTF-8 but it's ok because we use s >= word_wrap_eol below
                        word_wrap_eol += 1;
                    }
                }

                if s >= word_wrap_eol
                {
                    if text_size.x < line_width {
                        text_size.x = line_width;
                    }
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

            let char_width = (c < index_advance_x.size ? index_advance_x.data[c] : fallback_advance_x) * scale;
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


    pub fn calc_word_wrap_position_a(&self, scale: f32, text: &str, mut wrap_width: f32) -> usize{
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

        let mut line_width =  0.0;
        let mut word_width =  0.0;
        let mut blank_width =  0.0;
        // We work with unscaled widths to avoid scaling every characters
        wrap_width /= scale;
        let mut word_end: usize = text.len() - 1;
        // const char* prev_word_end = None;
        let mut prev_word_end: usize = usize::MAX;
        let mut inside_word = true;
        let text_end: usize = text.len() - 1;

        let mut s_offset:usize = 0;
        while s_offset < text_end
        {
            // unsigned int c = (unsigned int)*s;
            let c = text[s_offset];

            // const char* next_s;
            let mut next_s_offset: usize = usize::MAX;
            if c < 0x80 {
                next_s_offset = s_offset + 1;
            }
            else {
                // next_s = s + text_char_from_utf8(&c, s, text_end);
            }
            if c == '\0' {
                break;
            }

            if c < 32
            {
                if c == '\n'
                {
                    blank_width = 0.0;
                    line_width = 0.0;
                    word_width = 0.0;
                    inside_word = true;
                    s_offset = next_s_offset;
                    continue;
                }
                if c == '\r'
                {
                    s_offset = next_s_offset;
                    continue;
                }
            }

            let char_width =if  c < self.index_advance_x.len() { self.index_advance_x.data[c] } else { self.fallback_advance_x };
            if char_is_blank_w(c)
            {
                if inside_word
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
                if inside_word
                {
                    word_end = next_s;
                }
                else
                {
                    prev_word_end = word_end;
                    line_width += word_width + blank_width;
                    word_width = 0.0;
                    blank_width = 0.0;
                }

                // Allow wrapping after punctuation.
                inside_word = (c != '.' && c != ',' && c != ';' && c != '!' && c != '?' && c != '\"');
            }

            // We ignore blank width at the end of the line (they can be skipped)
            if line_width + word_width > wrap_width
            {
                // Words that cannot possibly fit within an entire line will be cut anywhere.
                if word_width < wrap_width {
                    s_offset = if prev_word_end { prev_word_end } else { word_end };
                }
                break;
            }

            s_offset = next_s_offset;
        }

        return s_offset;
    }


    pub fn render_char(&self, draw_list: &DrawList, size: f32, pos: &Vector2D, col: u32, c: char) {
        todo!()
    }


    pub fn render_text(&self, draw_list: &mut DrawList, size: f32, pos: &Vector2D, col: u32, clip_rect: &Vector4D, text: &String, wrap_width: f32, cpu_fine_clip: bool) {
        // if (!text_end)
        // text_end = text_begin + strlen(text_begin); // ImGui:: functions generally already provides a valid text_end, so this is merely to handle direct calls.

        // Align to be pixel perfect
        let mut x = f32::floor(pos.x);
        let mut y = f32::floor(pos.y);
        if y > clip_rect.w {
            return;
        }

        let start_x = x;
        let scale = size / self.font_size;
        let line_height = self.font_size * scale;
        let word_wrap_enabled = (wrap_width > 0.0);
        // let word_wrap_eol: usize = None;
        let mut word_wrap_eol_offset = 0usize;

        // Fast-forward to first visible line
        // const char* s = text_begin;
        let mut s_offset: usize = 0;
        let text_begin_offset: usize = 0;
        let mut text_end_offset: usize = text.len() - 1;

        if y + line_height < clip_rect.y && !word_wrap_enabled {
            while y + line_height < clip_rect.y && s_offset < text_end_offset {
                let s_offset_opt = text[s_offset..].find('\n');
                s_offset = if s_offset_opt.is_some() {
                    s_offset + 1
                } else {
                    text_end_offset
                };
                y += line_height;
            }
        }

        // For large text, scan for the last visible line in order to avoid over-reserving in the call to PrimReserve()
        // Note that very large horizontal line will still be affected by the issue (e.g. a one megabyte string buffer without a newline will likely crash atm)
        if text_end_offset - s_offset > 10000 && !word_wrap_enabled {
            let mut s_end_offset = s_offset;
            // const char* s_end = s;
            let mut y_end = y;
            while y_end < clip_rect.w && s_end_offset < text_end_offset {
                let s_end_offset_opt = text[s_end_offset..].find('\n');
                s_end_offset = if s_end_offset_opt.is_some() {
                    s_end_offset + 1
                } else {
                    text_end_offset
                };
                // s_end = (const char*)memchr(s_end, '\n', text_end - s_end);
                // s_end = s_end ? s_end + 1 : text_end;
                y_end += line_height;
            }
            text_end_offset = s_end_offset;
        }
        if s_offset == text_end_offset {
            return;
        }

        // Reserve vertices for remaining worse case (over-reserving is useful and easily amortized)
        let vtx_count_max = (text_end_offset - s_offset) * 4;
        let idx_count_max = (text_end_offset - s_offset) * 6;
        let idx_expected_size = draw_list.idx_buffer.len() + idx_count_max;
        draw_list.prim_reserve(idx_count_max, vtx_count_max);

        // ImDrawVert* vtx_write = draw_list->vtx_write_ptr;
        // ImDrawIdx* idx_write = draw_list->idx_write_ptr;
        self.vtx_current_idx = draw_list.vtx_current_idx;

        let col_untinted = col | !COLOR32_A_MASK;

        while s_offset < text_end_offset {
            if word_wrap_enabled {
                // Calculate how far we can render. Requires two passes on the string data but keeps the code simple and not intrusive for what's essentially an uncommon feature.
                if !word_wrap_eol_offset {
                    word_wrap_eol = calc_word_wrap_position_a(scale, &text[s_offset..], &text[text_end_offset..], wrap_width - (x - start_x));
                    if word_wrap_eol_offset == s_offset { // Wrap_width is too small to fit anything. Force displaying 1 character to minimize the height discontinuity.
                        word_wrap_eol_offset += 1;
                    }   // +1 may not be a character start point in UTF-8 but it's ok because we use s >= word_wrap_eol below
                }

                if s_offset >= word_wrap_eol_offset {
                    x = start_x;
                    y += line_height;
                    word_wrap_eol_offset = 0;

                    // Wrapping skips upcoming blanks
                    while s_offset < text_end_offset {
                        let c: char = text[s_offset];
                        if c.is_empty() || c.is_whitespace() {
                            s_offset += 1;
                            if c == '\n' {
                                break;
                            } else {
                                break;
                            }
                        }
                        // if (char_is_blank_a(c)) { s += 1; } else if (c == '\n') { s += 1; break; } else { break; }
                    }
                    continue;
                }
            }

            // Decode and advance source
            // unsigned int c = (unsigned int)*s;
            let c: char = text[s_offset];
            // if c.as_() < 0x80
            // {
            //     s_offset += 1;
            // }
            // else
            // {
            //     s_offset += text_char_from_utf8(&c, s, text_end);
            //     if (c == 0) // Malformed UTF-8?
            //         break;
            // }
            //
            // if (c < 32)
            // {
            //     if (c == '\n')
            //     {
            //         x = start_x;
            //         y += line_height;
            //         if (y > clip_rect.w)
            //             break; // break out of main loop
            //         continue;
            //     }
            //     if (c == '\r')
            //         continue;
            // }

            // const ImFontGlyph* glyph = FindGlyph((ImWchar)c);
            let glyph_opt: Option<&mut FontGlyph> = find_glyph(c);
            if glyph_opt.is_none() {
                continue;
            }
            let glyph: &mut FontGlyph = glyph_opt.unwrap();

            let char_width = glyph.advance_x * scale;
            if glyph.visible {
                // We don't do a second finer clipping test on the Y axis as we've already skipped anything before clip_rect.y and exit once we pass clip_rect.w
                let mut x1 = x + glyph.x0 * scale;
                let mut x2 = x + glyph.x1 * scale;
                let mut y1 = y + glyph.y0 * scale;
                let mut y2 = y + glyph.y1 * scale;
                if x1 <= clip_rect.z && x2 >= clip_rect.x {
                    // Render a character
                    let mut u1 = glyph.u0;
                    let mut v1 = glyph.v0;
                    let mut u2 = glyph.u1;
                    let v2 = glyph.V1;

                    // CPU side clipping used to fit text in their frame when the frame is too small. Only does clipping for axis aligned quads.
                    if cpu_fine_clip {
                        if x1 < clip_rect.x {
                            u1 = u1 + (1.0 - (x2 - clip_rect.x) / (x2 - x1)) * (u2 - u1);
                            x1 = clip_rect.x;
                        }
                        if y1 < clip_rect.y {
                            v1 = v1 + (1.0 - (y2 - clip_rect.y) / (y2 - y1)) * (v2 - v1);
                            y1 = clip_rect.y;
                        }
                        if x2 > clip_rect.z {
                            u2 = u1 + ((clip_rect.z - x1) / (x2 - x1)) * (u2 - u1);
                            x2 = clip_rect.z;
                        }
                        if y2 > clip_rect.w {
                            v2 = v1 + ((clip_rect.w - y1) / (y2 - y1)) * (v2 - v1);
                            y2 = clip_rect.w;
                        }
                        if y1 >= y2 {
                            x += char_width;
                            continue;
                        }
                    }

                    // Support for untinted glyphs
                    // ImU32 glyph_col = glyph.Colored ? col_untinted : col;
                    let glyph_col: u32 = if glyph.colored {
                        col_untinted
                    } else {
                        col
                    };

                    // We are NOT calling prim_rect_uv() here because non-inlined causes too much overhead in a debug builds. Inlined here:
                    {
                        idx_write[0] = (self.vtx_current_idx);
                        idx_write[1] = (self.vtx_current_idx + 1);
                        idx_write[2] = (self.vtx_current_idx + 2);
                        idx_write[3] = (self.vtx_current_idx);
                        idx_write[4] = (self.vtx_current_idx + 2);
                        idx_write[5] = (self.vtx_current_idx + 3);
                        vtx_write[0].pos.x = x1;
                        vtx_write[0].pos.y = y1;
                        vtx_write[0].col = glyph_col;
                        vtx_write[0].uv.x = u1;
                        vtx_write[0].uv.y = v1;
                        vtx_write[1].pos.x = x2;
                        vtx_write[1].pos.y = y1;
                        vtx_write[1].col = glyph_col;
                        vtx_write[1].uv.x = u2;
                        vtx_write[1].uv.y = v1;
                        vtx_write[2].pos.x = x2;
                        vtx_write[2].pos.y = y2;
                        vtx_write[2].col = glyph_col;
                        vtx_write[2].uv.x = u2;
                        vtx_write[2].uv.y = v2;
                        vtx_write[3].pos.x = x1;
                        vtx_write[3].pos.y = y2;
                        vtx_write[3].col = glyph_col;
                        vtx_write[3].uv.x = u1;
                        vtx_write[3].uv.y = v2;
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
        draw_list.vtx_write_ptr = vtx_write;
        draw_list.idx_write_ptr = idx_write;
        draw_list.vtx_current_idx = self.vtx_current_idx;
    }
    //
    //     // [Internal] Don't use!
    //      void              build_lookup_table();
    pub fn build_lookup_table(&mut self) {
         let mut max_codepoint: u32 = 0;
    // for (int i = 0; i != Glyphs.size; i += 1)
    for glyph in self.glyphs.iter()
    {
        // max_codepoint = ImMax(max_codepoint, Glyphs[i].Codepoint);
        max_codepoint = u32::max(max_codepoint, glyph.codepoint.into());
    }

    // build lookup table
    // IM_ASSERT(Glyphs.size < 0xFFFF); // -1 is reserved
    self.index_advance_x.clear();
    self.index_lookup.clear();
    self.dirty_lookup_tables = false;
    // memset(Used4kPagesMap, 0, sizeof(Used4kPagesMap));
    self.used4k_pages_map.clear();
    self.grow_index((max_codepoint + 1) as usize);
    // for (int i = 0; i < Glyphs.size; i += 1)
    for glyph in self.glyphs.iter()
        {
        // int codepoint = Glyphs[i].Codepoint;
        let codepoint = glyph.codepoint;
            self.index_advance_x[codepoint] = glyph.advance_x;
        self.index_lookup[codepoint] = i.into();

        // Mark 4K page as used
        let page_n = codepoint / 4096;
        self.used4k_pages_map[page_n >> 3] |= 1 << (page_n & 7);
    }

    // Create a glyph to handle TAB
    // FIXME: Needs proper TAB handling but it needs to be contextualized (or we could arbitrary say that each string starts at "column 0" ?)
    if find_glyph(' ')
    {
        // So we can call this function multiple times (FIXME: Flaky)
        if self.glyphs.last().unwrap().codepoint != '\t' {
            self.glyphs.reserve(1);
        }
        let mut tab_glyph = self.glyphs.last_mut().unwrap();
        tab_glyph = find_glyph(' ');
        tab_glyph.codepoint = '\t';
        tab_glyph.advance_x *= TABSIZE;
        self.index_advance_x[tab_glyph.codepoint] = tab_glyph.advance_x;
        self.index_lookup[tab_glyph.codepoint] = (self.glyphs.size - 1);
    }

    // Mark special glyphs as not visible (note that add_glyph already mark as non-visible glyphs with zero-size polygons)
    set_glyph_visible(' ', false);
    set_glyph_visible('\t', false);

    // Ellipsis character is required for rendering elided text. We prefer using U+2026 (horizontal ellipsis).
    // However some old fonts may contain ellipsis at U+0085. Here we auto-detect most suitable ellipsis character.
    // FIXME: Note that 0x2026 is rarely included in our font ranges. Because of this we are more likely to use three individual dots.
    let ellipsis_chars: [char;2] =  ['\u{2026}', '\u{0085}' ];
    let dots_chars: [char;2] = [ '.', '\u{FF0E}' ];
    if self.ellipsis_char == '\0' {
    self.ellipsis_char = find_first_existing_glyph(self, &ellipsis_chars.into_vec()).unwrap();
    }
    if self.dot_char == '\0'

        {
            self.dot_char = find_first_existing_glyph(self, &dots_chars.into_vec()).unwrap();
        }

    // Setup fallback character
    let fallback_chars: [char;3] = [ UNICODE_CODEPOINT_INVALID, '?', ' ' ];
    self.fallback_glyph = find_glyph_no_fallback(fallback_chars.into_vec());
    if self.fallback_glyph == '\0'
    {
        self.fallback_char = find_first_existing_glyph(self, &fallback_chars.into_vec()).unwrap();
        self.fallback_glyph = find_glyph_no_fallback(self.fallback_char);
        if FallbackGlyph == '\0'
        {
            self.fallback_glyhph = self.glyphs.last().unwrap();
            self.fallback_char = self.fallback_glyph.codepoint;
        }
    }

    self.fallback_advance_x = self.fallback_glyph.advance_x;
    // for (int i = 0; i < max_codepoint + 1; i += 1)
    for x in self.index_advance_x.iter_mut()
        {
        // if (index_advance_x[i] < 0.0) {
        //     index_advance_x[i] = fallback_advance_x;
        // }
            if *x < 0.0 {
                *x = self.fallback_advance_x;
            }
    }


    }

    pub fn grow_index(&mut self, new_size: usize) {

        // IM_ASSERT(index_advance_x.size == IndexLookup.size);
        if new_size <= self.index_lookup.size {
            return;
        }
        self.fallback_advance_x.resize(new_size, -1.0);
        self.index_lookup.resize(new_size, '\0');
    }

    pub fn add_glyph(&mut self, src_cfg: &FontConfig, c: char, mut x0: f32, y0: f32, mut x1: f32, y1: f32, u0: f32, v0: f32, u1: f32, v1: f32, mut advance_x: f32) {

        // Clamp & recenter if needed
        let advance_x_original = advance_x;
        advance_x = f32::clamp(advance_x, cfg.glyph_min_advance_x, cfg.glyph_max_advance_x);
        if advance_x != advance_x_original {
            let char_off_x = if cfg.pixel_snap_h { f32::floor((advance_x - advance_x_original) * 0.5) } else { (advance_x - advance_x_original) * 0.5 };
            x0 += char_off_x;
            x1 += char_off_x;
        }

        // Snap to pixel
        if cfg.pixel_snap_h {
            advance_x = f32::round(advance_x);
        }

        // Bake spacing
        advance_x += cfg.glyph_extra_spacing.x;


        // self.glyphs.resize(Glyphs.size + 1);
        let glyph = self.glyphs.last_mut().unwrap();
        glyph.codepoint = c;
        glyph.visible = (x0 != x1) && (y0 != y1);
        glyph.colored = false;
        glyph.x0 = x0;
        glyph.y0 = y0;
        glyph.x1 = x1;
        glyph.y1 = y1;
        glyph.u0 = u0;
        glyph.v0 = v0;
        glyph.u1 = u1;
        glyph.v1 = v1;
        glyph.advance_x = advance_x;

        // Compute rough surface usage metrics (+1 to account for average padding, +0.99 to round)
        // We use (u1-u0)*tex_width instead of x1-x0 to account for oversampling.
        let pad = self.container_atlas.text_glyph_padding + 0.99;
        self.dirty_lookup_tables = true;
        self.metrics_total_surface += ((glyph.u1 - glyph.u0) * self.container_atlas.tex_width + pad) * ((glyph.v1 - glyph.v0) * self.container_atlas.tex_height + pad);
    }

    pub fn add_remap_char(&mut self, dst: char, src: char) {
        // IM_ASSERT(IndexLookup.size > 0);    // Currently this can only be called AFTER the font has been built, aka after calling ImFontAtlas::GetTexDataAs*() function.
        // unsigned int index_size = (unsigned int)IndexLookup.size;
        let mut index_size: usize = self.index_lookup.len();

        // 'dst' already exists
        if ((dst as usize) < index_size) && (self.index_lookup[dst as usize] == '\0') {
            return;
        }
        if (src as usize) >= index_size && (dst as usize) >= index_size { // both 'dst' and 'src' don't exist -> no-op
            return;
        }

        self.grow_index(dst + 1);
        self.index_lookup[dst] = if (src as usize) < index_size { self.index_lookup.data[src] } else { -1 };
        self.index_advance_x[dst] = if (src as usize) < index_size { self.index_advance_x.data[src] } else { 1.0 };
    }

    pub fn set_glyph_visible(&mut self, c: char, visible: bool) {

        // if (ImFontGlyph* glyph = (ImFontGlyph*)(void*)FindGlyph((ImWchar)c))
        let glyph = find_glyph(c);
        if glyph.is_some() {
            glyph.unwrap().visible = visible;
        }
        // glyph.Visible = visible ? 1 : 0;
    }

    pub fn glyph_range_unused(&mut self, c_begin: u32, c_lst: u32) -> bool {
        let page_begin = (c_begin / 4096);
        let page_last = (c_last / 4096);
        // for (unsigned int page_n = page_begin; page_n <= page_last; page_n += 1)
        for page_n in page_begin..page_last {
            if (page_n >> 3) < self.used4k_pages_map.len() as u32 {
                if self.used4k_pages_map[page_n >> 3] & (1 << (page_n & 7)) {
                    return false;
                }
            }
        }
        return true;
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



// void ImFont::BuildLookupTable()
//
// {
//
// }

// API is designed this way to avoid exposing the 4K page size
// e.g. use with is_glyph_range_unused(0, 255)
// bool ImFont::IsGlyphRangeUnused(unsigned int c_begin, unsigned int c_last)
// {
//
// }

// void ImFont::SetGlyphVisible(ImWchar c, bool visible)
// {
//
// }

// void ImFont::GrowIndex(int new_size)
// {
//
// }

// x0/y0/x1/y1 are offset from the character upper-left layout position, in pixels. Therefore x0/y0 are often fairly close to zero.
// Not to be mistaken with texture coordinates, which are held by u0/v0/u1/v1 in normalized format (0.0..1.0 on each texture axis).
// 'cfg' is not necessarily == 'this->config_data' because multiple source fonts+configs can be used to build one target font.
// void ImFont::AddGlyph(const ImFontConfig* cfg, ImWchar codepoint, float x0, float y0, float x1, float y1, float u0, float v0, float u1, float v1, float advance_x)
// {
//
// }

// void ImFont::AddRemapChar(ImWchar dst, ImWchar src, bool overwrite_dst)
// {
//     // IM_ASSERT(IndexLookup.size > 0);    // Currently this can only be called AFTER the font has been built, aka after calling ImFontAtlas::GetTexDataAs*() function.
//     unsigned int index_size = (unsigned int)IndexLookup.size;
//
//     if (dst < index_size && IndexLookup.data[dst] == (ImWchar)-1 && !overwrite_dst) // 'dst' already exists
//         return;
//     if (src >= index_size && dst >= index_size) // both 'dst' and 'src' don't exist -> no-op
//         return;
//
//     GrowIndex(dst + 1);
//     IndexLookup[dst] = (src < index_size) ? IndexLookup.data[src] : (ImWchar)-1;
//     index_advance_x[dst] = (src < index_size) ? index_advance_x.data[src] : 1.0;
// }

// const ImFontGlyph* ImFont::FindGlyph(ImWchar c) const
// {
//
// }

// const ImFontGlyph* ImFont::FindGlyphNoFallback(ImWchar c) const
// {
//
// }
//
// const char* ImFont::calc_word_wrap_position_a(float scale, const char* text, const char* text_end, float wrap_width) const
// {
//
// }

// Vector2D ImFont::calc_text_size_a(float size, float max_width, float wrap_width, const char* text_begin, const char* text_end, const char** remaining) const
// {
//
// }

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

}
