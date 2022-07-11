use std::collections::HashSet;

use crate::condition::Cond;
use crate::context::Context;
use crate::window::{HoveredFlags, is_window_content_hoverable};
use crate::rect::Rect;
use crate::types::Id32;
use crate::utils::set_hash_set;
use crate::window::ItemFlags;

impl NextItemData {
    // ImGuiNextItemData()         { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     inline void ClearFlags()    { flags = ImGuiNextItemDataFlags_None; } // Also cleared manually by ItemAdd()!
    pub fn ClearFlags(&mut self) {
        self.flags = NextItemDataFlags::None
    }
}

#[derive(Debug, Clone, Default)]
pub struct NextItemData {
    // ImGuiNextItemDataFlags      flags;
    pub flags: NextItemDataFlags,
    // float                       width;          // Set by SetNextItemWidth()
    pub width: f32,
    // ImGuiID                     focus_scope_id;   // Set by SetNextItemMultiSelectData() (!= 0 signify value has been set, so it's an alternate version of HasSelectionData, we don't use flags for this because they are cleared too early. This is mostly used for debugging)
    pub focus_scope_id: Id32,
    // ImGuiCond                   open_cond;
    pub open_cond: Cond,
    // bool                        open_val;        // Set by SetNextItemOpen()
    pub open_val: bool,
}

/// Status storage for the last submitted item
#[derive(Debug, Clone, Default)]
pub struct LastItemData {
    // ImGuiID                 id;
    pub id: Id32,
    // ImGuiItemFlags          in_flags;            // See ImGuiItemFlags_
    pub in_flags: HashSet<ItemFlags>,
    // ImGuiItemStatusFlags    status_flags;        // See ImGuiItemStatusFlags_
    pub status_flags: HashSet<ItemStatusFlags>,
    // ImRect                  rect;               // Full rectangle
    pub rect: Rect,
    // ImRect                  nav_rect;            // Navigation scoring rectangle (not displayed)
    pub nav_rect: Rect,
    // ImRect                  display_rect;        // Display rectangle (only if ImGuiItemStatusFlags_HasDisplayRect is set)
    pub display_rect: Rect,
    // ImGuiLastItemData()     { memset(this, 0, sizeof(*this)); }
}

impl LastItemData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

pub enum ItemStatusFlags {
    None = 0,
    HoveredRect = 1 << 0,
    // Mouse position is within item rectangle (does NOT mean that the window is in correct z-order and can be hovered!, this is only one part of the most-common IsItemHovered test)
    HasDisplayRect = 1 << 1,
    // g.last_item_data.display_rect is valid
    Edited = 1 << 2,
    // Value exposed by item was edited in the current frame (should match the bool return value of most widgets)
    ToggledSelection = 1 << 3,
    // Set when Selectable(), TreeNode() reports toggling a selection. We can't report "Selected", only state changes, in order to easily handle clipping with less issues.
    ToggledOpen = 1 << 4,
    // Set when TreeNode() reports toggling their open state.
    HasDeactivated = 1 << 5,
    // Set if the widget/group is able to provide data for the Deactivated flag.
    Deactivated = 1 << 6,
    // Only valid if HasDeactivated is set.
    HoveredWindow = 1 << 7,
    // Override the hovered_window test to allow cross-window hover testing.
    FocusedByTabbing = 1 << 8,
    // Set when the Focusable item just got focused by Tabbing (FIXME: to be removed soon)
// #ifdef IMGUI_ENABLE_TEST_ENGINE
    // [imgui_tests only]
    Openable = 1 << 20,
    // Item is an openable (e.g. TreeNode)
    Opened = 1 << 21,
    //
    Checkable = 1 << 22,
    // Item is a checkable (e.g. CheckBox, MenuItem)
    Checked = 1 << 23,   //
// #endif
}

pub enum NextItemDataFlags {
    None = 0,
    HasWidth = 1 << 0,
    HasOpen = 1 << 1,
}


/// Internal facing ItemHoverable() used when submitting widgets. Differs slightly from IsItemHovered().
// bool ImGui::ItemHoverable(const ImRect& bb, ImGuiID id)
pub fn item_hoverable(g: &mut Context, bb: &Rect, id: Id32) -> Result<bool, &'static str> {
    // ImGuiContext& g = *GImGui;
    if g.hovered_id != 0 && g.hovered_id != id && !g.hovered_id_allow_overlap {
        return Ok(false);
    }

    let window = g.get_current_window()?;
    if g.hovered_window_id != g.current_window_id {
        return Ok(false);
    }
    if g.active_id != 0 && g.active_id != id && !g.active_id_allow_overlap {
        return Ok(false);
    }
    if !IsMouseHoveringRect(&bb.min, &bb.max) {
        return Ok(false);
    }
    if !is_window_content_hoverable(g, window, HoveredFlags::None) {
        g.hovered_id_disabled = true;
        return Ok(false);
    }

    // We exceptionally allow this function to be called with id==0 to allow using it for easy high-level
    // hover test in widgets code. We could also decide to split this function is two.
    if id != 0 {
        SetHoveredID(id);
    }

    // When disabled we'll return false but still set hovered_id
    ImGuiItemFlags
    item_flags = (g.LastItemData.ID == id?
    g.LastItemData.InFlags: g.CurrentItemFlags);
    if item_flags & ImGuiItemFlags_Disabled {
        // Release active id if turning disabled
        if g.ActiveId == id {
            ClearActiveID();
        }
        g.HoveredIdDisabled = true;
        return false;
    }

    if id != 0 {
        // [DEBUG] Item Picker tool!
        // We perform the check here because SetHoveredID() is not frequently called (1~ time a frame), making
        // the cost of this tool near-zero. We can get slightly better call-stack and support picking non-hovered
        // items if we perform the test in ItemAdd(), but that would incur a small runtime cost.
        // #define IMGUI_DEBUG_TOOL_ITEM_PICKER_EX in imconfig.h if you want this check to also be performed in ItemAdd().
        if (g.DebugItemPickerActive && g.HoveredIdPreviousFrame == id) {
            GetForegroundDrawList().AddRect(bb.min, bb.max, make_color_32(255, 255, 0, 255));
        }
        if (g.DebugItemPickerBreakId == id) {
            IM_DEBUG_BREAK();
        }
    }

    if (g.NavDisableMouseHover) {
        return false;
    }

    return true;
}

/// This is also inlined in ItemAdd()
/// Note: if ImGuiItemStatusFlags_HasDisplayRect is set, user needs to set window->dc.LastItemDisplayRect!
/// void ImGui::SetLastItemData(ImGuiID item_id, ImGuiItemFlags in_flags, ImGuiItemStatusFlags item_flags, const ImRect& item_rect)
pub fn set_last_item_data(g: &mut Context, item_id: Id32, in_flags: &HashSet<ItemFlags>, item_flags: &HashSet<ItemStatusFlags>, item_rect: &Rect)
{
    // ImGuiContext& g = *GImGui;
    g.last_item_data.id = item_id;
    // g.last_item_data.in_flags = in_flags.clone();
    set_hash_set(&mut g.last_item_data.in_flags, in_flags);
    // g.last_item_data.status_flags = item_flags;
    set_hash_set(&mut g.last_item_data.status_flags, item_flags);
    g.last_item_data.rect.clone_from(item_rect);
}

