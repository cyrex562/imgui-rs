// flags for ImFontAtlas build
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum FontAtlasFlags {
    None = 0,
    NoPowerOfTwoHeight,
    // Don't round the height to next power of two
    NoMouseCursors,
    // Don't build software mouse cursors into the atlas (save a little texture memory)
    NoBakedLines, // Don't build thick line textures into the atlas (save a little texture memory, allow support for point/nearest filtering). The AntiAliasedLinesUseTex features uses them, otherwise they will be rendered using polygons (more expensive for CPU/GPU).
}
