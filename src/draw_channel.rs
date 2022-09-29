#![allow(non_snake_case)]

use crate::draw_cmd::ImDrawCmd;
use crate::type_defs::ImDrawIdx;

// [Internal] For use by ImDrawListSplitter
#[derive(Default, Debug, Clone)]
pub struct ImDrawChannel {
    pub _CmdBuffer: Vec<ImDrawCmd>,
    pub _IdxBuffer: Vec<ImDrawIdx>,
}
