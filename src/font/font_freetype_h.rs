// dear imgui: FreeType font builder (used as a replacement for the stb_truetype builder)
// (headers)

#pragma once

#include "dock_style_color"      //

// Forward declarations
struct ImFontAtlas;
struct ImFontBuilderIO;

// Hinting greatly impacts visuals (and glyph sizes).
// - By default, hinting is enabled and the font's native hinter is preferred over the auto-hinter.
// - When disabled, FreeType generates blurrier glyphs, more or less matches the stb_truetype.h
// - The Default hinting mode usually looks good, but may distort glyphs in an unusual way.
// - The Light hinting mode generates fuzzier glyphs but better matches Microsoft's rasterizer.
// You can set those flags globaly in ImFontAtlas::font_builder_flags
// You can set those flags on a per font basis in ImFontConfig::font_builder_flags
enum ImGuiFreeTypeBuilderFlags
{
    FreeTypeBuilderFlags::NoHinting    ,   // Disable hinting. This generally generates 'blurrier' bitmap glyphs when the glyph are rendered in any of the anti-aliased modes.
    FreeTypeBuilderFlags::NoAutoHint   ,   // Disable auto-hinter.
    FreeTypeBuilderFlags::ForceAutoHint,   // Indicates that the auto-hinter is preferred over the font's native hinter.
    FreeTypeBuilderFlags::LightHinting ,   // A lighter hinting algorithm for gray-level modes. Many generated glyphs are fuzzier but better resemble their original shape. This is achieved by snapping glyphs to the pixel grid only vertically (Y-axis), as is done by Microsoft's ClearType and Adobe's proprietary font renderer. This preserves inter-glyph spacing in horizontal text.
    FreeTypeBuilderFlags::MonoHinting  ,   // Strong hinting algorithm that should only be used for monochrome output.
    FreeTypeBuilderFlags::Bold         ,   // Styling: Should we artificially embolden the font?
    FreeTypeBuilderFlags::Oblique      ,   // Styling: Should we slant the font, emulating italic style?
    FreeTypeBuilderFlags::Monochrome   ,   // Disable anti-aliasing. Combine this with MonoHinting for best results!
    FreeTypeBuilderFlags::LoadColor    ,   // Enable FreeType color-layered glyphs
    FreeTypeBuilderFlags::Bitmap        = 1 << 9    // Enable FreeType bitmap glyphs
};

namespace ImGuiFreeType
{
    // This is automatically assigned when using '#define IMGUI_ENABLE_FREETYPE'.
    // If you need to dynamically select between multiple builders:
    // - you can manually assign this builder with 'atlas->font_builder_io = ImGuiFreeType::get_builder_for_free_type()'
    // - prefer deep-copying this into your own ImFontBuilderIO instance if you use hot-reloading that messes up static data.
     const ImFontBuilderIO*    get_builder_for_free_type();

    // Override allocators. By default ImGuiFreeType will use IM_ALLOC()/IM_FREE()
    // However, as FreeType does lots of allocations we provide a way for the user to redirect it to a separate memory heap if desired.
     void                      SetAllocatorFunctions(void* (*alloc_func)(size_t sz, void* user_data), void (*free_func)(void* ptr, void* user_data), void* user_data = None);

    // Obsolete names (will be removed soon)
    // Prefer using '#define IMGUI_ENABLE_FREETYPE'
#ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    static inline bool BuildFontAtlas(ImFontAtlas* atlas, unsigned int flags = 0) { atlas.FontBuilderIO = get_builder_for_free_type(); atlas.font_builder_flags = flags; return atlas.Build(); }

}
