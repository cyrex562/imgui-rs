// This structure is likely to evolve as we add support for incremental atlas updates 
// struct ImFontBuilderIO
#[derive(Default, Debug, Clone, Copy)]
pub struct ImFontBuilderIO {
    // bool    ( * FontBuilder_Build)( * mut ImFontAtlas atlas);
    pub FontBuilder_Build: fn(atlas: *mut ImFontAtlas) -> bool,
}
