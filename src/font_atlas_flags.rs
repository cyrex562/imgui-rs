#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImFontAtlasFlags;       // -> enum ImFontAtlasFlags_     // Flags: for ImFontAtlas build
pub type ImFontAtlasFlags = c_int;

// Flags for ImFontAtlas build
// enum ImFontAtlasFlags_
// {
pub const ImFontAtlasFlags_None: ImFontAtlasFlags = 0;
pub const ImFontAtlasFlags_NoPowerOfTwoHeight: ImFontAtlasFlags = 1 << 0; // Don't round the height to next power of two
pub const ImFontAtlasFlags_NoMouseCursors: ImFontAtlasFlags = 1 << 1; // Don't build software mouse cursors into the atlas (save a little texture memory)
pub const ImFontAtlasFlags_NoBakedLines: ImFontAtlasFlags = 1 << 2; // Don't build thick line textures into the atlas (save a little texture memory, allow support for point/nearest filtering). The AntiAliasedLinesUseTex features uses them, otherwise they will be rendered using polygons (more expensive for CPU/GPU).
                                                                    // };
