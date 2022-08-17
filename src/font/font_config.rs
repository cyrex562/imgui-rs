
// ImFontConfig::ImFontConfig()
// {
//     memset(this, 0, sizeof(*this));
//     font_data_owned_by_atlas = true;
//     OversampleH = 3; // FIXME: 2 may be a better default?
//     OversampleV = 1;
//     glyph_max_advance_x = f32::MAX;
//     RasterizerMultiply = 1.0;
//     EllipsisChar = (ImWchar)-1;
// }

use crate::popup::PopupPositionPolicy::Default;
use crate::types::Id32;
use crate::vectors::Vector2D;

#[derive(Clone,Debug,Default)]
pub struct FontConfig
{
    pub id: Id32,
    pub font_data: Vec<u8>, // void*           font_data;               //          // TTF/OTF data
    pub font_data_size: i32,         //          // TTF/OTF data size
    pub font_data_owned_by_atlas: bool,   // true     // TTF/OTF data ownership taken by the container ImFontAtlas (will delete memory itself).
    pub font_no: isize,               // 0        // index of font within TTF/OTF file
    pub size_pixels: f32,            //          // size in pixels for rasterizer (more or less maps to the resulting font height).
    pub oversample_h: i32,          // 3        // Rasterize at higher quality for sub-pixel positioning. Note the difference between 2 and 3 is minimal so you can reduce this to 2 to save memory. Read https://github.com/nothings/stb/blob/master/tests/oversample/README.md for details.
    pub oversample_v: i32,          // 1        // Rasterize at higher quality for sub-pixel positioning. This is not really useful as we don't use sub-pixel positions on the Y axis.
    pub pixel_snap_h: bool,             // false    // Align every glyph to pixel boundary. Useful e.g. if you are merging a non-pixel aligned font with the default font. If enabled, you can set oversample_h/V to 1.
    pub glyph_extra_spacing: Vector2D,      // 0, 0     // Extra spacing (in pixels) between glyphs. Only x axis is supported for now.
    pub glyph_offset: Vector2D,            // 0, 0     // Offset all glyphs from this font input.
    pub glyph_ranges: Vec<char>, // const ImWchar*  glyph_ranges;            // None     // Pointer to a user-provided list of Unicode range (2 value per range, values are inclusive, zero-terminated list). THE ARRAY DATA NEEDS TO PERSIST AS LONG AS THE FONT IS ALIVE.
    pub glyph_min_advance_x: f32,      // 0        // Minimum advance_x for glyphs, set min to align font icons, set both min/max to enforce mono-space font
    pub glyph_max_advance_x: f32,      // FLT_MAX  // Maximum advance_x for glyphs
    pub merge_mode: bool,              // false    // merge into previous ImFont, so you can combine multiple inputs font into one ImFont (e.g. ASCII font + icons + Japanese glyphs). You may want to use glyph_offset.y when merge font of different heights.
    pub font_builder_flags: u32,     // 0        // Settings for custom font builder. THIS IS BUILDER IMPLEMENTATION DEPENDENT. Leave as zero if unsure.
    pub rasterizer_multiply: f32,    // 1.0     // Brighten (>1.0) or darken (<1.0) font output. Brightening small fonts may be a good workaround to make them more readable.
    // ImWchar         ellipsis_char;           // -1       // Explicitly specify unicode codepoint of ellipsis character. When fonts are being merged first specified ellipsis will be used.
    pub ellipsis_char: char,
    // [Internal]
    // char            name[40];               // name (strictly to ease debugging)
    pub name: String,
    // ImFont*         dst_font;
    pub dst_font: Id32,
    //  ImFontConfig();
}

impl FontConfig {
    pub fn new() -> Self {
        Self {
            font_data_owned_by_atlas: true,
            oversample_h: 3,
            oversample_v: 1,
            glyph_max_advance_x: f32::MAX,
            rasterizer_multiply: 1.0,
            ellipsis_char: '\u{2026}',
            ..Default::default()
        }
    }
}
