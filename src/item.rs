use std::collections::HashSet;
use crate::color::make_color_32;

use crate::condition::Condition;
use crate::content::get_content_region_max_abs;
use crate::context::Context;
use crate::draw::list::foreground_draw_list;
use crate::globals::GImGui;
use crate::id::{clear_active_id, keep_alive_id, set_hovered_id};
use crate::input::mouse::{is_mouse_clicked, is_mouse_hovering_rect};
use crate::input::MouseButton;
use crate::INVALID_ID;
use crate::layout::{LayoutType, same_line};
use crate::nav::nav_process_item;
use crate::window::{HoveredFlags, WindowFlags};
use crate::rect::Rect;
use crate::types::Id32;
use crate::utils::set_hash_set;
use crate::vectors::vector_2d::Vector2D;
use crate::window::checks::{is_clipped_ex, is_window_content_hoverable};

impl NextItemData {
    // ImGuiNextItemData()         { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     inline void ClearFlags()    { flags = NextItemDataFlags::None; } // Also cleared manually by ItemAdd()!
    pub fn clear_flags(&mut self) {
        self.flags = NextItemDataFlags::None
    }
}

#[derive(Debug, Clone, Default)]
pub struct NextItemData {
    // NextItemDataFlags      flags;
    pub flags: NextItemDataFlags,
    // float                       width;          // Set by SetNextItemWidth()
    pub width: f32,
    // Id32                     focus_scope_id;   // Set by SetNextItemMultiSelectData() (!= 0 signify value has been set, so it's an alternate version of HasSelectionData, we don't use flags for this because they are cleared too early. This is mostly used for debugging)
    pub focus_scope_id: Id32,
    // ImGuiCond                   open_cond;
    pub open_cond: Condition,
    // bool                        open_val;        // Set by SetNextItemOpen()
    pub open_val: bool,
}

/// Status storage for the last submitted item
#[derive(Debug, Clone, Default)]
pub struct LastItemData {
    // Id32                 id;
    pub id: Id32,
    // ImGuiItemFlags          in_flags;            // See ImGuiItemFlags_
    pub in_flags: HashSet<ItemFlags>,
    // ImGuiItemStatusFlags    status_flags;        // See ItemStatusFlags::
    pub status_flags: HashSet<ItemStatusFlags>,
    // ImRect                  rect;               // Full rectangle
    pub rect: Rect,
    // ImRect                  nav_rect;            // Navigation scoring rectangle (not displayed)
    pub nav_rect: Rect,
    // ImRect                  display_rect;        // Display rectangle (only if ItemStatusFlags::HasDisplayRect is set)
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
    HoveredRect,
    // Mouse position is within item rectangle (does NOT mean that the window is in correct z-order and can be hovered!, this is only one part of the most-common IsItemHovered test)
    HasDisplayRect,
    // g.last_item_data.display_rect is valid
    Edited,
    // value exposed by item was edited in the current frame (should match the bool return value of most widgets)
    ToggledSelection,
    // Set when selectable(), TreeNode() reports toggling a selection. We can't report "Selected", only state changes, in order to easily handle clipping with less issues.
    ToggledOpen,
    // Set when TreeNode() reports toggling their open state.
    HasDeactivated,
    // Set if the widget/group is able to provide data for the Deactivated flag.
    Deactivated,
    // Only valid if HasDeactivated is set.
    HoveredWindow,
    // Override the hovered_window test to allow cross-window hover testing.
    FocusedByTabbing,
    // Set when the Focusable item just got focused by Tabbing (FIXME: to be removed soon)
// #ifdef IMGUI_ENABLE_TEST_ENGINE
    // [imgui_tests only]
    Openable,
    // Item is an openable (e.g. TreeNode)
    Opened,
    //
    Checkable,
    // Item is a checkable (e.g. CheckBox, menu_item)
    Checked,   //
// #endif
}

pub enum NextItemDataFlags {
    None = 0,
    HasWidth,
    HasOpen,
}


/// Internal facing ItemHoverable() used when submitting widgets. Differs slightly from IsItemHovered().
// bool ImGui::ItemHoverable(const ImRect& bb, Id32 id)
pub fn item_hoverable(g: &mut Context, bb: &Rect, id: Id32) -> bool {
    // ImGuiContext& g = *GImGui;
    if g.hovered_id != 0 && g.hovered_id != id && !g.hovered_id_allow_overlap {
        return false;
    }

    let window = g.current_window_mut()?;
    if g.hovered_window_id != g.current_window_id {
        return false;
    }
    if g.active_id != 0 && g.active_id != id && !g.active_id_allow_overlap {
        return false;
    }
    if !is_mouse_hovering_rect(g, &bb.min, &bb.max, false) {
        return false;
    }
    let flags = HashSet::from([HoveredFlags::None]);
    if !is_window_content_hoverable(g, window, &flags) {
        g.hovered_id_disabled = true;
        return false;
    }

    // We exceptionally allow this function to be called with id==0 to allow using it for easy high-level
    // hover test in widgets code. We could also decide to split this function is two.
    if id != 0 {
        set_hovered_id(g, id);
    }

    // When disabled we'll return false but still set hovered_id
    let item_flags = if g.last_item_data.id == id { g.last_item_data.in_flags.clone() } else { g.current_item_flags.clone() };
    if item_flags.contains(&ItemFlags::Disabled) {
        // Release active id if turning disabled
        if g.active_id == id {
            clear_active_id(g);
        }
        g.hovered_id_disabled = true;
        return false;
    }

    if id != 0 {
        // [DEBUG] Item Picker tool!
        // We perform the check here because set_hovered_id() is not frequently called (1~ time a frame), making
        // the cost of this tool near-zero. We can get slightly better call-stack and support picking non-hovered
        // items if we perform the test in ItemAdd(), but that would incur a small runtime cost.
        // #define IMGUI_DEBUG_TOOL_ITEM_PICKER_EX in imconfig.h if you want this check to also be performed in ItemAdd().
        if g.debug_item_picker_active && g.hovered_id_previous_frame == id {
            foreground_draw_list(g, None).add_rect(&bb.min, &bb.max, make_color_32(255, 255, 0, 255), 0.0, None, 0.0);
        }
        if g.debug_item_picker_break_id == id {
            // IM_DEBUG_BREAK();
        }
    }

    if g.nav_disable_mouse_hover {
        return false;
    }

    return true;
}

/// This is also inlined in ItemAdd()
/// Note: if ItemStatusFlags::HasDisplayRect is set, user needs to set window->dc.LastItemdisplay_rect!
/// void ImGui::SetLastItemData(Id32 item_id, ImGuiItemFlags in_flags, ImGuiItemStatusFlags item_flags, const ImRect& item_rect)
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
    if g.last_item_data.status_flags.contains(&ItemStatusFlags::HasDeactivated) {
        return g.last_item_data.status_flags.contains(&ItemStatusFlags::Deactivated);
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
    // Window* window = g.current_window;
    let window = g.current_window_mut().unwrap();
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
    return is_mouse_clicked(g, mouse_button, false) && is_item_hovered(g, &HashSet::from([HoveredFlags::None]));
}

// bool ImGui::IsItemToggledOpen()
pub fn is_item_toggled_open(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    g.last_item_data.status_flags.contains(&ItemStatusFlags::ToggledOpen)
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
    // return g.current_window->clip_rect.Overlaps(g.last_item_data.rect);
    let curr_win = g.current_window_mut().unwrap();
    curr_win.clip_rect.overlaps_rect(&g.last_item_data.rect)
}

// bool ImGui::IsItemEdited()
pub fn is_item_edited(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    // return (g.last_item_data.status_flags & ItemStatusFlags::Edited) != 0;
    g.last_item_data.status_flags.contains(&ItemStatusFlags::Edited)
}

// Allow last item to be overlapped by a subsequent item. Both may be activated during the same frame before the later one takes priority.
// FIXME: Although this is exposed, its interaction and ideal idiom with using ImGuiButtonFlags_AllowItemOverlap flag are extremely confusing, need rework.
// void ImGui::SetItemAllowOverlap()
pub fn sest_item_allow_overlap(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // Id32 id = g.last_item_data.id;
    let id = g.last_item_data.id;
    if (g.hovered_id == id) {
        g.hovered_id_allow_overlap = true;
    }
    if (g.active_id == id) {
        g.active_id_allow_overlap = true;
    }
}

// void ImGui::SetItemUsingMouseWheel()
pub fn set_item_using_mouse_wheel(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // Id32 id = g.last_item_data.id;
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
    // return g.last_item_data.rect.Min;
    g.last_item_data.rect.min.clone()
}

// Vector2D ImGui::GetItemRectMax()
pub fn get_item_rect_max(g: &mut Context) -> Vector2D
{
    // ImGuiContext& g = *GImGui;
    // return g.last_item_data.rect.max;
    g.last_item_data.rect.max.clone()
}

// Vector2D ImGui::GetItemRectSize()
pub fn get_item_rect_size(g: &mut Context) -> Vector2D
{
    // ImGuiContext& g = *GImGui;
    // return g.last_item_data.rect.GetSize();
    g.last_item_data.rect.size()
}


// This is roughly matching the behavior of internal-facing ItemHoverable()
// - we allow hovering to be true when active_id==window->MoveID, so that clicking on non-interactive items such as a Text() item still returns true with IsItemHovered()
// - this should work even for non-interactive items that have no id, so we cannot use LastItemId
// bool ImGui::IsItemHovered(ImGuiHoveredFlags flags)
pub fn is_item_hovered(g: &mut Context, flags: &HashSet<HoveredFlags>) -> bool
{
    // ImGuiContext& g = *GImGui;
    let window = g.window_mut(g.current_window_id).unwrap();
    if g.nav_disable_mouse_hover && !g.nav_disable_highlight && !(flags.contains(&HoveredFlags::NoNavOverride))
    {
        if (g.last_item_data.in_flags.contains(&ItemFlags::Disabled)) && !(flags.contains(&HoveredFlags::AllowWhenDisabled) ){
            return false;
        }
        if !is_item_focused(g) {
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
        if (!flags.contains(&HoveredFlags::AllowWhenBlockedByActiveItem)) && (g.active_id != 0 && g.active_id != g.last_item_data.id && !g.active_id_allow_overlap) && (g.active_id != window.move_id && g.active_id != window.tab_id) {
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
    NoTabStop,
    // false     // Disable keyboard tabbing (FIXME: should merge with _NoNav)
    ButtonRepeat,
    // false     // Button() will return true multiple times based on io.key_repeat_delay and io.key_repeat_rate settings.
    Disabled,
    // false     // Disable interactions but doesn't affect visuals. See BeginDisabled()/EndDisabled(). See github.com/ocornut/imgui/issues/211
    NoNav,
    // false     // Disable keyboard/gamepad directional navigation (FIXME: should merge with _NoTabStop)
    NoNavDefaultFocus,
    // false     // Disable item being a candidate for default focus (e.g. used by title bar items)
    selectableDontClosePopup,
    // false     // Disable menu_item/selectable() automatically closing their popup window
    MixedValue,
    // false     // [BETA] Represent a mixed/indeterminate value, generally multi-selection where values differ. Currently only supported by Checkbox() (later should support all sorts of widgets)
    ReadOnly,
    // false     // [ALPHA] Allow hovering interactions but underlying value is not changed.
    Inputable,   // false     // [WIP] Auto-activate input mode when tab focused. Currently only used and supported by a few items before it becomes a generic feature.
}

// void ImGui::push_item_flag(ImGuiItemFlags option, bool enabled)
pub fn push_item_flag(g: &mut Context, option: &ItemFlags, enabled: bool)
{
    // ImGuiContext& g = *GImGui;
    let mut item_flags = g.current_item_flags.clone();
    // IM_ASSERT(item_flags == g.item_flags_stack.back());
    if enabled {
        item_flags |= option;
    }
    else {
        item_flags &= !option;
    }
    g.current_item_flags = item_flags;
    g.item_flags_stack.push_back(&item_flags);
}

// void ImGui::PopItemFlag()
pub fn pop_item_flag(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.item_flags_stack.size > 1); // Too many calls to PopItemFlag() - we always leave a 0 at the bottom of the stack.
    g.item_flags_stack.pop_back();
    g.current_item_flags = g.item_flags_stack.back();
}

// void ActivateItem(Id32 id)
pub fn activate_item(g: &mut Context, id: Id32)
{
    // ImGuiContext& g = *GImGui;
    g.NavNextActivateId = id;
    g.NavNextActivateFlags = ImGuiActivateFlags_None;
}

// Advance cursor given item size for layout.
// Register minimum needed size so it can extend the bounding box used for auto-fit calculation.
// See comments in ItemAdd() about how/why the size provided to ItemSize() vs ItemAdd() may often different.
// void item_size(const Vector2D& size, float text_baseline_y)
pub fn item_size(g: &mut Context, size: &Vector2D, text_baseline_y: f32)
{
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();
    if window.skip_items {
        return;
    }

    // We increase the height in this function to accommodate for baseline offset.
    // In theory we should be offsetting the starting position (window->dc.cursor_pos), that will be the topic of a larger refactor,
    // but since ItemSize() is not yet an API that moves the cursor (to handle e.g. wrapping) enlarging the height has the same effect.
    let offset_to_match_baseline_y = if text_baseline_y >= 0.0 { f32::max(0.0, window.dc.curr_line_text_base_offset - text_baseline_y) }else{ 0.0};

    let line_y1 = if window.dc.Issame_line { window.dc.cursor_pos_prev_line.y }else{ window.dc.cursor_pos.y};
    let line_height = ImMax(window.dc.curr_line_size.y, /*ImMax(*/window.dc.cursor_pos.y - line_y1/*, 0.0)*/ + size.y + offset_to_match_baseline_y);

    // Always align ourselves on pixel boundaries
    //if (g.io.key_alt) window->draw_list->add_rect(window->dc.cursor_pos, window->dc.cursor_pos + Vector2D(size.x, line_height), IM_COL32(255,0,0,200)); // [DEBUG]
    window.dc.cursor_pos_prev_line.x = window.dc.cursor_pos.x + size.x;
    window.dc.cursor_pos_prev_line.y = line_y1;
    window.dc.cursor_pos.x = f32::floor(window.pos.x + window.dc.indent.x + window.dc.columns_offset.x);    // Next line
    window.dc.cursor_pos.y = f32::floor(line_y1 + line_height + g.style.item_spacing.y);                    // Next line
    window.dc.cursor_max_pos.x = ImMax(window.dc.cursor_max_pos.x, window.dc.cursor_pos_prev_line.x);
    window.dc.cursor_max_pos.y = ImMax(window.dc.cursor_max_pos.y, window.dc.cursor_pos.y - g.style.item_spacing.y);
    //if (g.io.key_alt) window->draw_list->add_circle(window->dc.CursorMaxPos, 3.0, IM_COL32(255,0,0,255), 4); // [DEBUG]

    window.dc.prev_line_size.y = line_height;
    window.dc.curr_line_size.y = 0.0;
    window.dc.PrevLineTextBaseOffset = ImMax(window.dc.curr_line_text_base_offset, text_baseline_y);
    window.dc.curr_line_text_base_offset = 0.0;
    window.dc.Issame_line = false;

    // Horizontal layout mode
    if window.dc.layout_type == LayoutType::Horizontal {
        same_line(g, 0.0,0.0);
    }
}

// Declare item bounding box for clipping and interaction.
// Note that the size can be different than the one provided to ItemSize(). Typically, widgets that spread over available surface
// declare their minimum size requirement to ItemSize() and provide a larger region to ItemAdd() which is used drawing/interaction.
// bool item_add(const Rect& bb, Id32 id, const Rect* nav_bb_arg, ImGuiItemFlags extra_flags)
pub fn item_add(g: &mut Context, bb: &mut Rect, id: Id32, nav_bb_arg: Option<&Rect>, extra_flags: Option<&HashSet<ItemFlags>>) -> bool
{
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();

    // Set item data
    // (display_rect is left untouched, made valid when ItemStatusFlags::HasDisplayRect is set)
    g.last_item_data.id = id;
    g.last_item_data.rect = bb.clone();
    g.last_item_data.nav_rect = if nav_bb_arg { *nav_bb_arg }else{ bb};
    g.last_item_data.in_flags = g.current_item_flags.clone() | extra_flags;
    g.last_item_data.status_flags = HashSet::new(); //HashSet<ItemStatusFlags> = HashSet::from([]);

    // Directional navigation processing
    if id != 0
    {
        keep_alive_id(g, id);

        // Runs prior to clipping early-out
        //  (a) So that nav_init_request can be honored, for newly opened windows to select a default widget
        //  (b) So that we can scroll up/down past clipped items. This adds a small O(N) cost to regular navigation requests
        //      unfortunately, but it is still limited to one window. It may not scale very well for windows with ten of
        //      thousands of item, but at least NavMoveRequest is only set on user interaction, aka maximum once a frame.
        //      We could early out with "if (is_clipped && !g.nav_init_request) return false;" but when we wouldn't be able
        //      to reach unclipped widgets. This would work if user had explicit scrolling control (e.g. mapped on a stick).
        // We intentionally don't check if g.nav_window != None because g.nav_any_request should only be set when it is non null.
        // If we crash on a None g.nav_window we need to fix the bug elsewhere.
        window.dc.nav_layers_active_mask_next |= (1 << window.dcnav_layer_current);
        if g.nav_id == id || g.nav_any_request {
            if g.nav_window.root_window_for_nav == window.root_window_for_nav {
                if window.id == g.nav_window_id || ((&window.flags | g.nav_window.flags) & WindowFlags::NavFlattened) {
                    nav_process_item(g);
                }
            }
        }

        // [DEBUG] People keep stumbling on this problem and using "" as identifier in the root of a window instead of "##something".
        // Empty identifier are valid and useful in a small amount of cases, but 99.9% of the time you want to use "##something".
        // READ THE FAQ: https://dearimgui.org/faq
        // IM_ASSERT(id != window.id && "Cannot have an empty id at the root of a window. If you need an empty label, use ## and read the FAQ about how the id Stack works!");

        // [DEBUG] Item Picker tool, when enabling the "extended" version we perform the check in ItemAdd()
// #ifdef IMGUI_DEBUG_TOOL_ITEM_PICKER_EX
        if id == g.debug_item_picker_break_id
        {
            // IM_DEBUG_BREAK();
            g.debug_item_picker_break_id = 0;
        }

    }
    // g.next_item_data.flags = NextItemDataFlags::None;
    &g.next_item_data.flags: HashSet<NextItemDataFlags> = HashSet::new();

// #ifdef IMGUI_ENABLE_TEST_ENGINE
    if id != 0 {
        // IMGUI_TEST_ENGINE_ITEM_ADD(nav_bb_arg? * nav_bb_arg: bb, id);
    }


    // Clipping test
    let is_clipped = is_clipped_ex(g, bb, id);
    if is_clipped {
        return false;
    }
    //if (g.io.key_alt) window->draw_list->add_rect(bb.min, bb.max, IM_COL32(255,255,0,120)); // [DEBUG]

    // We need to calculate this now to take account of the current clipping rectangle (as items like selectable may change them)
    if is_mouse_hovering_rect(g, &bb.min, &bb.max, false) {
        g.last_item_data.status_flags |= ItemStatusFlags::HoveredRect;
    }
    return true;
}

// Affect large frame+labels widgets only.
//void SetNextItemWidth(float item_width)
// pub fn SetNextItemWidth(item_width: f32)
pub fn set_next_item_width(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    g.next_item_data.flags |= NextItemDataFlags::HasWidth;
    g.next_item_data.width = item_width;
}

// FIXME: Remove the == 0.0 behavior?
// void PushItemWidth(float item_width)
pub fn push_item_width(g: &mut Context, item_width: f32)
{
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();
    window.dc.item_width_stack.push_back(window.dc.item_width); // Backup current width
    window.dc.item_width = if item_width == 0.0 { window.item_width_default }else{ item_width};
    g.next_item_data.flags &= !NextItemDataFlags::HasWidth;
}

// void PushMultiItemsWidths(int components, float w_full)
pub fn push_multi_items_widths(g: &mut Context, components: i32, w_full: f32)
{
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();
    // const ImGuiStyle& style = g.style;
    let style = &mut g.style;
    let w_item_one  = f32::max(1.0, f32::floor((w_full - (style.item_inner_spacing.x) * (components - 1)) / components));
    let w_item_last = f32::max(1.0, f32::floor(w_full - (w_item_one + style.item_inner_spacing.x) * (components - 1)));
    window.dc.item_width_stack.push_back(window.dc.item_width); // Backup current width
    window.dc.item_width_stack.push_back(w_item_last);
    // for (int i = 0; i < components - 2; i += 1)
    for i in 0 .. components - 2
    {
        window.dc.item_width_stack.push_back(w_item_one);
    }
    window.dc.item_width = if components == 1 { w_item_last }else{ w_item_one};
    g.next_item_data.flags &= !NextItemDataFlags::HasWidth;
}

// void PopItemWidth()
pub fn pop_item_width(g: &mut Context)
{
    // Window* window = GetCurrentWindow();
    let window = g.current_window_mut();
    window.dc.item_width = window.dc.item_width_stack.back();
    window.dc.item_width_stack.pop_back();
}

// Calculate default item width given value passed to PushItemWidth() or SetNextItemWidth().
// The SetNextItemWidth() data is generally cleared/consumed by ItemAdd() or next_item_data.ClearFlags()
// float CalcItemWidth()
pub fn calc_item_width(g: &mut Context) -> f32
{
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();
    let mut w = 0f32;
    if g.next_item_data.flags.contains(&NextItemDataFlags::HasWidth) {
        w = g.next_item_data.width;
    }
    else {
        w = window.dc.item_width;
    }
    if w < 0.0
    {
        let region_max_x =  get_content_region_max_abs(g).x;
        w = f32::max(1.0, region_max_x - window.dc.cursor_pos.x + w);
    }
    w = f32::floor(w);
    return w;
}

// [Internal] Calculate full item size given user provided 'size' parameter and default width/height. Default width is often == CalcItemWidth().
// Those two functions CalcItemWidth vs CalcItemSize are awkwardly named because they are not fully symmetrical.
// Note that only CalcItemWidth() is publicly exposed.
// The 4.0 here may be changed to match CalcItemWidth() and/or BeginChild() (right now we have a mismatch which is harmless but undesirable)
// Vector2D CalcItemSize(Vector2D size, float default_w, float default_h)
pub fn calc_item_size(g: &mut Context, mut size: Vector2D, default_w: f32, default_h: f32) -> Vector2D
{
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();

    // Vector2D region_max;
    let mut region_max: Vector2D = Vector2D::default();
    if size.x < 0.0 || size.y < 0.0 {
        region_max = get_content_region_max_abs(g);
    }

    if size.x == 0.0 {
        size.x = default_w;
    }
    else if size.x < 0.0 {
        size.x = f32::max(4.0, region_max.x - window.dc.cursor_pos.x + size.x);
    }

    if size.y == 0.0 {
        size.y = default_h;
    }
    else if size.y < 0.0 {
        size.y = f32::max(4.0, region_max.y - window.dc.cursor_pos.y + size.y);
    }

    return size;
}
