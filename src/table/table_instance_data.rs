// Per-instance data that needs preserving across frames (seemingly most others do not need to be preserved aside from debug needs, does that needs they could be moved to ImGuiTableTempData ?)

use libc::c_float;

#[derive(Default,Debug,Clone)]
pub struct ImGuiTableInstanceData
{
pub LastOuterHeight:  c_float,            // Outer height from last frame // FIXME: multi-instance issue (#3955)
pub LastFirstRowHeight:  c_float,         // Height of first row from last frame // FIXME: possible multi-instance issue?

    
}

impl ImGuiTableInstanceData {
    pub fn new() -> Self {
        Self {
            LastOuterHeight: 0.0,
            LastFirstRowHeight: 0.0,
        }
    }
}
