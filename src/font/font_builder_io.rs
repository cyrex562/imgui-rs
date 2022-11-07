use crate::font_atlas::ImFontAtlas;

// This structure is likely to evolve as we add support for incremental atlas updates
#[derive(Default, Debug, Copy, Clone)]
pub struct ImFontBuilderIO {
    // bool    (*FontBuilder_Build)(*mut ImFontAtlas atlas);
    pub FontBuilder_Build: fn(atlas: *mut ImFontAtlas) -> bool,
}
