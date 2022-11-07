use crate::item_flags::ImGuiItemFlags;
use crate::item_status_flags::ImGuiItemStatusFlags;
use crate::rect::ImRect;
use crate::type_defs::ImguiHandle;

// Status storage for the last submitted item
#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiLastItemData {
    pub id: ImguiHandle,
    pub in_flags: ImGuiItemFlags,           // See ImGuiItemFlags_
    pub status_flags: ImGuiItemStatusFlags, // See ImGuiItemStatusFlags_
    pub rect: ImRect,                      // Full rectangle
    pub NavRect: ImRect,                   // Navigation scoring rectangle (not displayed)
    pub DisplayRect: ImRect, // Display rectangle (only if ImGuiItemStatusFlags_HasDisplayRect is set)

                             // ImGuiLastItemData()     { memset(this, 0, sizeof(*this)); }
}
