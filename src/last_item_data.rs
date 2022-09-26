use crate::rect::ImRect;
use crate::type_defs::{ImGuiID, ImGuiItemFlags, ImGuiItemStatusFlags};

// Status storage for the last submitted item
#[derive(Default,Debug,Clone)]
pub struct ImGuiLastItemData
{
pub ID: ImGuiID,
pub InFlags: ImGuiItemFlags,            // See ImGuiItemFlags_
pub StatusFlags: ImGuiItemStatusFlags,        // See ImGuiItemStatusFlags_
pub Rect: ImRect,               // Full rectangle
pub NavRect: ImRect,            // Navigation scoring rectangle (not displayed)
pub DisplayRect: ImRect,        // Display rectangle (only if ImGuiItemStatusFlags_HasDisplayRect is set)

    // ImGuiLastItemData()     { memset(this, 0, sizeof(*this)); }
}
