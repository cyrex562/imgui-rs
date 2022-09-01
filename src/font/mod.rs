use font::Font;
use crate::{Context, INVALID_ID};
use crate::draw::list::DrawList;
use font_atlas::FontAtlas;
use font_glyph::FontGlyph;
use crate::globals::GImGui;
use crate::types::Id32;
use crate::vectors::vector_2d::Vector2D;

pub mod font_atlas;
pub mod font_glyph;
pub mod font_config;
pub mod font_builder_io;
pub mod font;
pub mod font_glyph_ranges_builder;
pub mod font_freetype;
pub mod ops;
pub mod embedded_font;
pub mod font_atlas_custom_rect;
pub mod font_atlas_default_tex_data;
mod font_atlas_flags;
mod base85;


// static ImWchar FindFirstExistingGlyph(ImFont* font, const ImWchar* candidate_chars, int candidate_chars_count)
pub fn find_first_existing_glyph(font: &mut Font, candidate_chars: &Vec<char>) -> Option<char>
{
    for c in candidate_chars {
        if font.find_glyph_no_fallback(c).is_some() {
            return Some(c)
        }
    }

    return None

}


