use std::collections::HashSet;

use crate::condition::Condition;
use crate::context::Context;
use crate::globals::GImGui;
use crate::input::MouseButton;
use crate::INVALID_ID;
use crate::window::HoveredFlags;
use crate::rect::Rect;
use crate::types::Id32;
use crate::utils::set_hash_set;
use crate::vectors::two_d::Vector2D;
use crate::window::checks::is_window_content_hoverable;

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
    pub open_cond: Condition,
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
    // value exposed by item was edited in the current frame (should match the bool return value of most widgets)
    ToggledSelection = 1 << 3,
    // Set when Selectable(), TreeNode() reports toggling a selection. We can't report "Selected", only state changes, in order to easily handle clipping with less issues.
    ToggledOpen = 1 << 4,
    // Set when TreeNode() reports toggling their open state.
    HasDeactivated = 1 << 5,
    // Set if the widget/group is able to provide data for the Deactivated flag.
    Deactivated = 1 << 6,
    // Only valid if HasDeactivated is set.
    hovered_window = 1 << 7,
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
    item_flags = (g.LastItemData.id == id?
    g.LastItemData.InFlags: g.current_item_flags);
    if item_flags & ItemFlags::Disabled {
        // Release active id if turning disabled
        if g.active_id == id {
            clear_active_id();
        }
        g.hovered_id_disabled = true;
        return false;
    }

    if id != 0 {
        // [DEBUG] Item Picker tool!
        // We perform the check here because SetHoveredID() is not frequently called (1~ time a frame), making
        // the cost of this tool near-zero. We can get slightly better call-stack and support picking non-hovered
        // items if we perform the test in ItemAdd(), but that would incur a small runtime cost.
        // #define IMGUI_DEBUG_TOOL_ITEM_PICKER_EX in imconfig.h if you want this check to also be performed in ItemAdd().
        if (g.DebugItemPickerActive && g.hovered_id_previous_frame == id) {
            get_foreground_draw_list().AddRect(bb.min, bb.max, make_color_32(255, 255, 0, 255));
        }
        if (g.DebugItemPickerBreakId == id) {
            IM_DEBUG_BREAK();
        }
    }

    if (g.nav_disable_mouse_hover) {
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

// bool ImGui::IsItemActive()
pub fn is_item_active(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    if g.active_id {
        return g.active_id == g.last_item_data.id;
    }
    return false;
}

// bool ImGui::IsItemActivated()
pub fn is_item_activated(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    if g.active_id != INVALID_ID {
        if g.active_id == g.last_item_data.id && g.active_id_previous_frame != g.last_item_data.id {
            return true;
        }
    }
    return false;
}

// bool ImGui::is_item_deactivated()
pub fn is_item_deactivated(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    if g.last_item_data.status_flags.contains(&ImGuiItemStatusFlags_HasDeactivated) {
        return g.last_item_data.status_flags.contains(ImGuiItemStatusFlags_Deactivated);
    }
    return g.active_id_previous_frame == g.last_item_data.id && g.active_id_previous_frame != 0 && g.active_id != g.last_item_data.id;
}

// bool ImGui::IsItemDeactivatedAfterEdit()
pub fn is_item_deactivated_after_edit(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    return is_item_deactivated(g) && (g.active_id_previous_frame_has_been_edited_before || (g.active_id == INVALID_ID && g.active_id_has_been_edited_before));
}

// == GetItemID() == GetFocusID()
// bool ImGui::is_item_focused()
pub fn is_item_focused(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    if g.nav_id != g.last_item_data.id || g.nav_id == 0 {
        return false;
    }

    // Special handling for the dummy item after Begin() which represent the title bar or tab.
    // When the window is collapsed (skip_items==true) that last item will never be overwritten so we need to detect the case.
    // ImGuiWindow* window = g.current_window;
    let window = g.get_current_window().unwrap();
    if g.last_item_data.id == window.id && window.write_accessed {
        return false;
    }

    return true;
}

// Important: this can be useful but it is NOT equivalent to the behavior of e.g.Button()!
// Most widgets have specific reactions based on mouse-up/down state, mouse position etc.
// bool ImGui::IsItemClicked(ImGuiMouseButton mouse_button)
pub fn is_item_clicked(g: &mut Context, mouse_button: &MouseButton) -> bool
{
    return is_mouse_clicked(mouse_button) && is_item_hovered(g, &HashSet::from([HoveredFlags::None]));
}

// bool ImGui::IsItemToggledOpen()
pub fn is_item_toggled_open(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    g.last_item_data.status_flags.contains(&ImGuiItemStatusFlags_ToggledOpen)
}

// bool ImGui::IsItemToggledSelection()
pub fn is_item_toggled_selection(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    let x = g.last_item_data.status_flags.contains(&ItemStatusFlags::ToggledSelection);
    x
}

// bool ImGui::IsAnyItemHovered()
pub fn is_any_item_hovered(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    // return g.hovered_id != 0 || g.hovered_id_previous_frame != 0;
    g.hovered_id != INVALID_ID || g.hovered_id_previous_frame != INVALID_ID
}

// bool ImGui::IsAnyItemActive()
pub fn is_any_item_active(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    // return g.active_id != 0;
    g.active_id != INVALID_ID
}

// bool ImGui::IsAnyItemFocused()
pub fn is_any_item_focused(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    // return g.nav_id != 0 && !g.nav_disable_highlight;
    g.nav_id != INVALID_ID && g.nav_disable_highlight == false
}

// bool ImGui::IsItemVisible()
pub fn is_item_visible(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    // return g.current_window->ClipRect.Overlaps(g.last_item_data.Rect);
    let curr_win = g.get_current_window().unwrap();
    curr_win.clip_rect.overlaps_rect(&g.last_item_data.rect)
}

// bool ImGui::IsItemEdited()
pub fn is_item_edited(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    // return (g.last_item_data.status_flags & ImGuiItemStatusFlags_Edited) != 0;
    g.last_item_data.status_flags.contains(&ItemStatusFlags::Edited)
}

// Allow last item to be overlapped by a subsequent item. Both may be activated during the same frame before the later one takes priority.
// FIXME: Although this is exposed, its interaction and ideal idiom with using ImGuiButtonFlags_AllowItemOverlap flag are extremely confusing, need rework.
// void ImGui::SetItemAllowOverlap()
pub fn sest_item_allow_overlap(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // ImGuiID id = g.last_item_data.id;
    let id = g.last_item_data.id;
    if (g.hovered_id == id) {
        g.hovered_id_allow_overlap = true;
    }
    if (g.active_id == id) {
        g.ActiveIdAllowOverlap = true;
    }
}

// void ImGui::SetItemUsingMouseWheel()
pub fn set_item_using_mouse_wheel(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // ImGuiID id = g.last_item_data.id;
    let id = g.last_item_data.id;
    if g.hovered_id == id {
        g.hovered_id_using_mouse_wheel = true;
    }
    if g.active_id == id {
        g.active_id_using_mouse_wheel = true;
    }
}

// Vector2D ImGui::GetItemRectMin()
pub fn get_item_rect_min(g: &mut Context) -> Vector2D
{
    // ImGuiContext& g = *GImGui;
    // return g.last_item_data.Rect.Min;
    g.last_item_data.rect.min.clone()
}

// Vector2D ImGui::GetItemRectMax()
pub fn get_item_rect_max(g: &mut Context) -> Vector2D
{
    // ImGuiContext& g = *GImGui;
    // return g.last_item_data.Rect.Max;
    g.last_item_data.rect.max.clone()
}

// Vector2D ImGui::GetItemRectSize()
pub fn get_item_rect_size(g: &mut Context) -> Vector2D
{
    // ImGuiContext& g = *GImGui;
    // return g.last_item_data.Rect.GetSize();
    g.last_item_data.rect.get_size()
}


// This is roughly matching the behavior of internal-facing ItemHoverable()
// - we allow hovering to be true when active_id==window->MoveID, so that clicking on non-interactive items such as a Text() item still returns true with IsItemHovered()
// - this should work even for non-interactive items that have no id, so we cannot use LastItemId
// bool ImGui::IsItemHovered(ImGuiHoveredFlags flags)
pub fn is_item_hovered(g: &mut Context, flags: &HashSet<HoveredFlags>) -> bool
{
    // ImGuiContext& g = *GImGui;
    let window = g.get_window(g.current_window_id).unwrap();
    if g.nav_disable_mouse_hover && !g.nav_disable_highlight && !(flags.contains(&HoveredFlags::NoNavOverride))
    {
        if (g.last_item_data.in_flags.contains(&ItemFlags::Disabled)) && !(flags.contains(&HoveredFlags::AllowWhenDisabled) ){
            return false;
        }
        if !is_item_focused() {
            return false;
        }
    }
    else
    {
        // Test for bounding box overlap, as updated as ItemAdd()
        let status_flags = &g.last_item_data.status_flags;
        if !(status_flags.contains(&ItemStatusFlags::HoveredRect)) {
            return false;
        }
        // IM_ASSERT((flags & (ImGuiHoveredFlags_AnyWindow | ImGuiHoveredFlags_RootWindow | ImGuiHoveredFlags_ChildWindows | ImGuiHoveredFlags_NoPopupHierarchy | ImGuiHoveredFlags_DockHierarchy)) == 0);   // flags not supported by this function

        // Test if we are hovering the right window (our window could be behind another window)
        // [2021/03/02] Reworked / reverted the revert, finally. Note we want e.g. BeginGroup/ItemAdd/EndGroup to work as well. (#3851)
        // [2017/10/16] Reverted commit 344d48be3 and testing RootWindow instead. I believe it is correct to NOT test for RootWindow but this leaves us unable
        // to use IsItemHovered() after EndChild() itself. Until a solution is found I believe reverting to the test from 2017/09/27 is safe since this was
        // the test that has been running for a long while.
        if (g.hovered_window_id != window.id && (!status_flags.contains(&ItemStatusFlags::HoveredWindow))) && (!flags.contains(&HoveredFlags::AllowWhenOverlapped)) {
            return false;
        }

        // Test if another item is active (e.g. being dragged)
        if (!flags.contains(&HoveredFlags::AllowWhenBlockedByActiveItem)) && (g.active_id != 0 && g.active_id != g.last_item_data.id && !g.active_id_allow_overlap) && (g.active_id != window.move_id && g.active_id != window.TabId) {
            return false;
        }

        // Test if interactions on this window are blocked by an active popup or modal.
        // The ImGuiHoveredFlags_AllowWhenBlockedByPopup flag will be tested here.
        if !is_window_content_hoverable(g, window, flags) {
            return false;
        }

        // Test if the item is disabled
        if (g.last_item_data.in_flags.contains(&ItemFlags::Disabled)) && !(flags.contains(&HoveredFlags::AllowWhenDisabled)) {
            return false;
        }

        // Special handling for calling after Begin() which represent the title bar or tab.
        // When the window is skipped/collapsed (skip_items==true) that last item (always ->move_id submitted by Begin)
        // will never be overwritten so we need to detect the case.
        if g.last_item_data.id == window.move_id && window.write_accessed {
            return false;
        }
    }

    return true;
}


// Transient per-window flags, reset at the beginning of the frame. For child window, inherited from parent on first Begin().
// This is going to be exposed in imgui.h when stabilized enough.
pub enum ItemFlags {
    None = 0,
    NoTabStop = 1 << 0,
    // false     // Disable keyboard tabbing (FIXME: should merge with _NoNav)
    ButtonRepeat = 1 << 1,
    // false     // Button() will return true multiple times based on io.key_repeat_delay and io.key_repeat_rate settings.
    Disabled = 1 << 2,
    // false     // Disable interactions but doesn't affect visuals. See BeginDisabled()/EndDisabled(). See github.com/ocornut/imgui/issues/211
    NoNav = 1 << 3,
    // false     // Disable keyboard/gamepad directional navigation (FIXME: should merge with _NoTabStop)
    NoNavDefaultFocus = 1 << 4,
    // false     // Disable item being a candidate for default focus (e.g. used by title bar items)
    SelectableDontClosePopup = 1 << 5,
    // false     // Disable MenuItem/Selectable() automatically closing their popup window
    MixedValue = 1 << 6,
    // false     // [BETA] Represent a mixed/indeterminate value, generally multi-selection where values differ. Currently only supported by Checkbox() (later should support all sorts of widgets)
    ReadOnly = 1 << 7,
    // false     // [ALPHA] Allow hovering interactions but underlying value is not changed.
    Inputable = 1 << 8,   // false     // [WIP] Auto-activate input mode when tab focused. Currently only used and supported by a few items before it becomes a generic feature.
}

// void ImGui::PushItemFlag(ImGuiItemFlags option, bool enabled)
pub fn push_item_flag(g: &mut Context, option: &ItemFlags, enabled: bool)
{
    ImGuiContext& g = *GImGui;
    ImGuiItemFlags item_flags = g.current_item_flags;
    IM_ASSERT(item_flags == g.item_flags_stack.back());
    if (enabled)
        item_flags |= option;
    else
        item_flags &= ~option;
    g.current_item_flags = item_flags;
    g.item_flags_stack.push_back(item_flags);
}

// void ImGui::PopItemFlag()
pub fn pop_item_flag(g: &mut Context)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(g.item_flags_stack.size > 1); // Too many calls to PopItemFlag() - we always leave a 0 at the bottom of the stack.
    g.item_flags_stack.pop_back();
    g.current_item_flags = g.item_flags_stack.back();
}

// void ActivateItem(ImGuiID id)
pub fn activate_item(g: &mut Context, id: Id32)
{
    ImGuiContext& g = *GImGui;
    g.NavNextActivateId = id;
    g.NavNextActivateFlags = ImGuiActivateFlags_None;
}
