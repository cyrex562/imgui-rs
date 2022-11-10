#![allow(non_snake_case)]

use crate::drawing::draw_cmd::ImDrawCmd;
use crate::core::type_defs::DrawIndex;

// [Internal] For use by ImDrawListSplitter
#[derive(Default, Debug, Clone, Copy)]
pub struct ImDrawChannel {
    pub _CmdBuffer: Vec<ImDrawCmd>,
    pub _IdxBuffer: Vec<DrawIndex>,
}
