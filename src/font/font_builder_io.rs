use crate::font::font_atlas::FontAtlas;

// This structure is likely to evolve as we add support for incremental atlas updates
#[derive(Default,Debug,Clone)]
pub struct FontBuilderIo
{
    // bool    (*FontBuilder_Build)(ImFontAtlas* atlas);
    pub font_builder_build_fn: Option<fn(atlas: &mut FontAtlas)>,
}
