use std::collections::HashSet;
use crate::{Context, INVALID_ID};
use crate::condition::Condition;
use crate::config::ConfigFlags;
use crate::types::Direction;
use crate::imgui_h::Id32;
use crate::imgui_vec::Vector2D;
use crate::imgui_window::Window;
use crate::input::mouse::{is_mouse_pos_valid, is_mouse_released};
use crate::input::{MouseButton, NavLayer};
use crate::item::{is_any_item_hovered, is_item_hovered};
use crate::nav::{nav_move_request_try_wrapping, nav_restore_last_child_nav_window, NavMoveFlags};
use crate::orig_imgui_single_file::Id32;
use crate::rect::Rect;
use crate::types::Id32;
use crate::vectors::vector_2d::Vector2D;
use crate::viewport::get_main_viewport;
use crate::window::{HoveredFlags, Window, WindowFlags};
use crate::window::checks::{is_window_active_and_visible, is_window_hovered};
use crate::window::get::is_window_within_begin_stack_of;
use crate::window::lifecycle::{begin, end};
use crate::window::next_window::{NextWindowDataFlags, set_next_window_pos};

// Storage for current popup stack
#[derive(Debug, Clone)]
pub struct PopupData {
    // Id32             popup_id;        // Set on OpenPopup()
    pub popup_id: Id32,
    // Window*        window;         // Resolved on begin_popup() - may stay unresolved if user never calls OpenPopup()
    pub window_id: Id32,
    // Window*        source_window;   // Set on OpenPopup() copy of nav_window at the time of opening the popup
    pub source_window_id: Id32,
    // int                 parent_nav_layer; // Resolved on begin_popup(). Actually a ImGuiNavLayer type (declared down below), initialized to -1 which is not part of an enum, but serves well-enough as "not any of layers" value
    pub parent_nav_layer: NavLayer,
    // int                 open_frame_count; // Set on OpenPopup()
    pub open_frame_count: usize,
    // Id32             open_parent_id;   // Set on OpenPopup(), we need this to differentiate multiple menu sets from each others (e.g. inside menu bar vs loose menu items)
    pub open_parent_id: Id32,
    // Vector2D              open_popup_pos;   // Set on OpenPopup(), preferred popup position (typically == open_mouse_pos when using mouse)
    pub open_popup_pos: Vector2D,
    // Vector2D              open_mouse_pos;   // Set on OpenPopup(), copy of mouse position at the time of opening popup
    pub open_mouse_pos: Vector2D,
}

impl PopupData {
    // ImGuiPopupData()    { memset(this, 0, sizeof(*this)); parent_nav_layer = open_frame_count = -1; }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for PopupData {
    fn default() -> Self {
        Self {
            popup_id: INVALID_ID,
            window_id: INVALID_ID,
            source_window_id: INVALID_ID,
            parent_nav_layer: NavLayer::None,
            open_frame_count: 0,
            open_parent_id: INVALID_ID,
            open_popup_pos: Vector2D::default(),
            open_mouse_pos: Vector2D::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum PopupPositionPolicy {
    Default,
    ComboBox,
    Tooltip,
}

// pub const AnyPopup: i32                = DimgPopupFlags::AnyPopupId | DimgPopupFlags::AnyPopupLevel;
pub const POPUP_FLAGS_ANY_POPUP: HashSet<PopupFlags> = HashSet::from([
    PopupFlags::AnyPopupId, PopupFlags::AnyPopupLevel
]);

// flags for OpenPopup*(), begin_popupContext*(), IsPopupOpen() functions.
// - To be backward compatible with older API which took an 'int mouse_button = 1' argument, we need to treat
//   small flags values as a mouse button index, so we encode the mouse button in the first few bits of the flags.
//   It is therefore guaranteed to be legal to pass a mouse button index in ImGuiPopupFlags.
// - For the same reason, we exceptionally default the ImGuiPopupFlags argument of begin_popupContextXXX functions to 1 instead of 0.
//   IMPORTANT: because the default parameter is 1 (==PopupFlags::MouseButtonRight), if you rely on the default parameter
//   and want to another another flag, you need to pass in the PopupFlags::MouseButtonRight flag.
// - Multiple buttons currently cannot be combined/or-ed in those functions (we could allow it later).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PopupFlags {
    None,
    // PopupFlags::MouseButtonLeft         = 0,        // For begin_popupContext*(): open on Left Mouse release. Guaranteed to always be == 0 (same as MouseButton::Left)
    MouseButtonRight,
    // For begin_popupContext*(): open on Right Mouse release. Guaranteed to always be == 1 (same as MouseButton::Right)
    MouseButtonMiddle,
    // For begin_popupContext*(): open on Middle Mouse release. Guaranteed to always be == 2 (same as MouseButton::Middle)
    MouseButtonLeft,
    // MouseButtonMask_        = 0x1F,
    // PopupFlags::MouseButtonDefault_     = 1,
    NoOpenOverExistingPopup,
    // For OpenPopup*(), begin_popupContext*(): don't open if there's already a popup at the same level of the popup stack
    NoOpenOverItems,
    // For begin_popupContextWindow(): don't return true when hovering items, only when hovering empty space
    AnyPopupId,
    // For IsPopupOpen(): ignore the Id32 parameter and test for any popup.
    AnyPopupLevel,   // For IsPopupOpen(): search/test at any level of the popup stack (default test in the current level)
}

// Supported flags: PopupFlags::AnyPopupId, PopupFlags::AnyPopupLevel
// bool IsPopupOpen(Id32 id, ImGuiPopupFlags popup_flags)
pub fn is_popup_open(g: &mut Context, id: Id32, popup_flags: &HashSet<PopupFlags>) -> bool {
    // ImGuiContext& g = *GImGui;
    if popup_flags.unwrap().contains(&PopupFlags::AnyPopupId) {
        // Return true if any popup is open at the current begin_popup() level of the popup stack
        // This may be used to e.g. test for another popups already opened to handle popups priorities at the same level.
        // IM_ASSERT(id == 0);
        if popup_flags.unwrap.contains(&PopupFlags::AnyPopupLevel) {
            return g.open_popup_stack.size > 0;
        } else {
            return g.open_popup_stack.size > g.begin_popup_stack.size;
        }
    } else {
        if popup_flags.unwrap().contains(&PopupFlags::AnyPopupLevel) {
            // Return true if the popup is open anywhere in the popup stack
            // for (int n = 0; n < g.open_popup_stack.size; n += 1)
            for n in 0..g.open_popup_stack.len() {
                if g.open_popup_stack[n].popup_id == id {
                    return true;
                }
            }
            return false;
        } else {
            // Return true if the popup is open at the current begin_popup() level of the popup stack (this is the most-common query)
            return g.open_popup_stack.size > g.begin_popup_stack.size && g.open_popup_stack[g.begin_popup_stack.size].popup_id == id;
        }
    }
}

// bool IsPopupOpen(const char* str_id, ImGuiPopupFlags popup_flags)
pub fn is_popup_open_2(g: &mut Context, str_id: &str, popup_flags: &HashSet<PopupFlags>) -> bool {
    // ImGuiContext& g = *GImGui;
    let id = if popup_flags.contains(&PopupFlags::AnyPopupId) { 0 } else {
        g.current_window.get_id(str_id)
    };
    if popup_flags.contains(&PopupFlags::AnyPopupLevel) && id != 0 {}
    // IM_ASSERT(0 && "Cannot use IsPopupOpen() with a string id and PopupFlags::AnyPopupLevel."); // But non-string version is legal and used internally
    return is_popup_open(g, id, popup_flags);
}

// Window* get_top_most_popup_modal()
pub fn get_top_most_popup_modal(g: &mut Context) -> Option<&mut Window> {
    // ImGuiContext& g = *GImGui;
    // for (int n = g.open_popup_stack.size - 1; n >= 0; n -= 1 )
    for n in g.open_popup_stack.len() - 1..0 {
        if let popup = g.window_mut(g.open_popup_stack[n].window_id) {
            if popup.flags.contains(&WindowFlags::Modal) {
                return Some(popup);
            }
        }
    }
    return None;
}

// Window* GetTopMostAndVisiblePopupModal()
pub fn get_top_most_and_visible_popup_modal(g: &mut Context) -> Option<&mut Window> {
    // ImGuiContext& g = *GImGui;
    //for (int n = g.open_popup_stack.size - 1; n >= 0; n -= 1 )
    for n in g.open_popup_stack.len() - 1..0 {
        if let pop = g.window_mut(g.open_popup_stack[n].window_id) {
            // if (Window * popup = g.open_popup_stack.data[n].Window) {
            if popup.flags.contains(&WindowFlags::Modal) && is_window_active_and_visible(popup) {
                return popup;
            }
        }
    }
    return None;
}

// void OpenPopup(const char* str_id, ImGuiPopupFlags popup_flags)
pub fn open_popup(g: &mut Context, str_id: &str, popup_flags: &HashSet<PopupFlags>) {
    // ImGuiContext& g = *GImGui;
    let id = g.current_window_id;
    // IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopup(\"%s\" -> 0x%08X\n", str_id, id);
    open_popup_ex(g, id, popup_flags);
}

// OpenPopup(Id32 id, ImGuiPopupFlags popup_flags)
pub fn open_popup2(g: &mut Context, id: Id32, popup_flags: &HashSet<PopupFlags>) {
    open_popup_ex(g, id, popup_flags);
}

// Mark popup as open (toggle toward open state).
// Popups are closed when user click outside, or activate a pressable item, or CloseCurrentPopup() is called within a begin_popup()/EndPopup() block.
// Popup identifiers are relative to the current id-stack (so OpenPopup and begin_popup needs to be at the same level).
// One open popup per level of the popup hierarchy (NB: when assigning we reset the window member of ImGuiPopupRef to None)
// void OpenPopupEx(Id32 id, ImGuiPopupFlags popup_flags)
pub fn open_popup_ex(g: &mut Context, id: Id32, popup_flags: &HashSet<PopupFlags>) {
    // ImGuiContext& g = *GImGui;
    // Window* parent_window = g.current_window;
    let parent_window = g.current_window_mut();
    let current_stack_size = g.begin_popup_stack.size;

    if popup_flags.contains(&PopupFlags::NoOpenOverExistingPopup) {
        if is_popup_open(g, 0, &HashSet::from([PopupFlags::AnyPopupId])) {
            return;
        }
    }

    let mut popup_ref: PopupData = PopupData::default(); // Tagged as new ref as window will be set back to None if we write this into open_popup_stack.
    popup_ref.popup_id = id;
    popup_ref.window_id = INVALID_ID;
    popup_ref.source_window_id = g.nav_window_id;
    popup_ref.open_frame_count = g.frame_count;
    popup_ref.open_parent_id = parent_window.id_stack.back();
    popup_ref.open_popup_pos = nav_calc_preferred_ref_pos();
    popup_ref.open_mouse_pos = if is_mouse_pos_valid(g, &g.io.mouse_pos) { g.io.mouse_pos } else { popup_ref.open_popup_pos };

    // IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopupEx(0x%08X)\n", id);
    if g.open_popup_stack.size < current_stack_size + 1 {
        g.open_popup_stack.push_back(popup_ref);
    } else {
        // Gently handle the user mistakenly calling OpenPopup() every frame. It is a programming mistake! However, if we were to run the regular code path, the ui
        // would become completely unusable because the popup will always be in hidden-while-calculating-size state _while_ claiming focus. Which would be a very confusing
        // situation for the programmer. Instead, we silently allow the popup to proceed, it will keep reappearing and the programming error will be more obvious to understand.
        if g.open_popup_stack[current_stack_size].popup_id == id && g.open_popup_stack[current_stack_size].OpenFrameCount == g.frame_count - 1 {
            g.open_popup_stack[current_stack_size].OpenFrameCount = popup_ref.OpenFrameCount;
        } else {
            // Close child popups if any, then flag popup for open/reopen
            close_popup_to_level(g, current_stack_size, false);
            g.open_popup_stack.push_back(popup_ref);
        }

        // When reopening a popup we first refocus its parent, otherwise if its parent is itself a popup it would get closed by close_popups_over_window().
        // This is equivalent to what close_popup_to_level() does.
        //if (g.open_popup_stack[current_stack_size].popup_id == id)
        //    focus_window(parent_window);
    }
}

// When popups are stacked, clicking on a lower level popups puts focus back to it and close popups above it.
// This function closes any popups that are over 'ref_window'.
// void close_popups_over_window(Window* ref_window, bool restore_focus_to_window_under_popup)
pub fn close_popups_over_window(g: &mut Context, ref_window: &mut Window, restore_focus_to_window_under_popup: bool) {
    // ImGuiContext& g = *GImGui;
    if g.open_popup_stack.len() == 0 {
        return;
    }

    // Don't close our own child popup windows.
    let mut popup_count_to_keep = 0;
    if ref_window {
        // Find the highest popup which is a descendant of the reference window (generally reference window = nav_window)
        // for (; popup_count_to_keep < g.open_popup_stack.size; popup_count_to_keep += 1)
        for popup_count_to_keep in 0..g.open_popup_stack.len() {
            let mut popup = &mut g.open_popup_stack[popup_count_to_keep];
            if !popup.window_id != INVALID_ID {
                continue;
            }
            // IM_ASSERT((popup.Window.flags & WindowFlags::Popup) != 0);
            if g.window_mut(popup.window_id).flags.contains(&WindowFlags::ChildWindow) {
                continue;
            }

            // Trim the stack unless the popup is a direct parent of the reference window (the reference window is often the nav_window)
            // - With this stack of window, clicking/focusing Popup1 will close Popup2 and Popup3:
            //     window -> Popup1 -> Popup2 -> Popup3
            // - Each popups may contain child windows, which is why we compare ->root_window_dock_tree!
            //     window -> Popup1 -> Popup1_Child -> Popup2 -> Popup2_Child
            let mut ref_window_is_descendent_of_popup = false;
            // for (int n = popup_count_to_keep; n < g.open_popup_stack.size; n += 1)
            for n in popup_count_to_keep..g.open_popup_stack.len() {
                let mut popup_window = g.window_mut(g.open_popup_stack[n].window_id);
                if popup_window.id != INVALID_ID {
                    //if (popup_window->root_window_dock_tree == ref_window->root_window_dock_tree) // FIXME-MERGE
                    if is_window_within_begin_stack_of(g, ref_window, popup_window) {
                        ref_window_is_descendent_of_popup = true;
                        break;
                    }
                }
            }
            if !ref_window_is_descendent_of_popup {
                break;
            }
        }
    }
    if popup_count_to_keep < g.open_popup_stack.size // This test is not required but it allows to set a convenient breakpoint on the statement below
    {
        // IMGUI_DEBUG_LOG_POPUP("[popup] close_popups_over_window(\"%s\")\n", ref_window ? ref_window.name : "<None>");
        close_popup_to_level(g, popup_count_to_keep, restore_focus_to_window_under_popup);
    }
}

// void ClosePopupsExceptModals()
pub fn close_popups_except_modals(g: &mut Context) {
    // ImGuiContext& g = *GImGui;

    let mut popup_count_to_keep = 0;
    // for (popup_count_to_keep = g.open_popup_stack.size; popup_count_to_keep > 0; popup_count_to_keep -= 1 )
    for popup_count_to_keep in g.open_popup_stack.len()..0 {
        // Window* window = g.open_popup_stack[popup_count_to_keep - 1].Window;
        let window = g.window_mut(g.open_popup_stack[popup_count_to_keep - 1].window_id);
        if window.id != INVALID_ID || window.flags.contains(&WindowFlags::Modal) {
            break;
        }
    }
    if popup_count_to_keep < g.open_popup_stack.size { // This test is not required but it allows to set a convenient breakpoint on the statement below
        close_popup_to_level(g, popup_count_to_keep, true);
    }
}

// void close_popup_to_level(int remaining, bool restore_focus_to_window_under_popup)
pub fn close_popup_to_level(g: &mut Context, remaining: usize, restore_focus_to_window_under_popup: bool) {
    // ImGuiContext& g = *GImGui;
    // IMGUI_DEBUG_LOG_POPUP("[popup] close_popup_to_level(%d), restore_focus_to_window_under_popup=%d\n", remaining, restore_focus_to_window_under_popup);
    // IM_ASSERT(remaining >= 0 && remaining < g.open_popup_stack.size);

    // Trim open popup stack
    // Window* focus_window = g.open_popup_stack[remaining].SourceWindow;
    let mut focus_window = g.window_mut(g.open_popup_stack[remaining].source_window_id);
    // Window* popup_window = g.open_popup_stack[remaining].Window;
    let popup_window = g.window_mut(g.open_popup_stack[remaining].window_id);
    g.open_popup_stack.resize(remaining, PopupData::default());

    if restore_focus_to_window_under_popup {
        if focus_window.id != INVALID_ID && !focus_window.was_active && popup_window.id != INVALID_ID {
            // Fallback
            focus_topmost_window_under_one(popup_window, None);
        } else {
            if g.nav_layer == NavLayer::Main && focus_window.id != INVALID_ID {
                focus_window = nav_restore_last_child_nav_window(g, focus_window);
            }
            focus_window(focus_window);
        }
    }
}

// Close the popup we have begin-ed into.
// void CloseCurrentPopup()
pub fn close_current_popup(g: &mut Context) {
    // ImGuiContext& g = *GImGui;
    // int popup_idx = g.begin_popup_stack.size - 1;
    let mut popup_idx: usize = g.begin_popup_stack.len() - 1;
    if popup_idx < 0 || popup_idx >= g.open_popup_stack.size || g.begin_popup_stack[popup_idx].popup_id != g.open_popup_stack[popup_idx].popup_id {
        return;
    }

    // Closing a menu closes its top-most parent popup (unless a modal)
    while popup_idx > 0 {
        // Window* popup_window = g.open_popup_stack[popup_idx].Window;
        let popup_window = g.window_mut(g.open_popup_stack[popup_idx].window_id);
        // Window* parent_popup_window = g.open_popup_stack[popup_idx - 1].Window;
        let parent_popup_window = g.window_mut(g.open_popup_stack[popup_idx - 1].window_id);
        let mut close_parent = false;
        if popup_window.id != INVALID_ID && (popup_window.flags.contains(&WindowFlags::ChildMenu)) {
            if parent_popup_window.id != INVALID_ID && !(parent_popup_window.flags.contains(&WindowFlags::MenuBar)) {
                close_parent = true;
            }
        }
        if !close_parent {
            break;
        }
        popup_idx -= 1;
    }
    // IMGUI_DEBUG_LOG_POPUP("[popup] CloseCurrentPopup %d -> %d\n", g.begin_popup_stack.size - 1, popup_idx);
    close_popup_to_level(g, popup_idx, true);

    // A common pattern is to close a popup when selecting a menu item/selectable that will open another window.
    // To improve this usage pattern, we avoid nav highlight for a single frame in the parent window.
    // Similarly, we could avoid mouse hover highlight in this window but it is less visually problematic.
    // if (Window* window = g.nav_window)
    let window = g.nav_window_mut();
    if window.id != INVALID_ID {
        window.dc.nav_hide_highlight_one_frame = true;
    }
}

// Attention! begin_popup() adds default flags which begin_popupEx()!
// bool begin_popupEx(Id32 id, WindowFlags flags)
pub fn begin_popup_ex(g: &mut Context, id: Id32, flags: &mut HashSet<WindowFlags>) -> bool {
    // ImGuiContext& g = *GImGui;
    if !is_popup_open(g, id, &HashSet::new()) {
        g.next_window_data.clear_flags(); // We behave like Begin() and need to consume those values
        return false;
    }

    // char name[20];
    let mut name = String::new();
    if flags.contains(&WindowFlags::ChildMenu) {
        // ImFormatString(name, IM_ARRAYSIZE(name), "##Menu_%02d", g.BeginMenuCount);
        name = format!("##Menu_{:02}", g.begin_menu_count);
    }// Recycle windows based on depth
    else {
        // ImFormatString(name, IM_ARRAYSIZE(name), "##Popup_%08x", id); // Not recycling, so we can close/open during the same frame
        name = format!("##Popup_{:08x}", id);
    }

    // flags |= WindowFlags::Popup | WindowFlags::NoDocking;
    flags.insert(WindowFlags::Popup);
    flags.insert(WindowFlags::NoDocking);
    let is_open = begin(g, name.as_str(), None, Some(flags));
    if !is_open { // NB: Begin can return false when the popup is completely clipped (e.g. zero size display)
        end_popup(g);
    }

    return is_open;
}

// bool begin_popup(const char* str_id, WindowFlags flags)
pub fn begin_popup(g: &mut Context, str_id: &str, flags: &mut HashSet<WindowFlags>) -> bool {
    // ImGuiContext& g = *GImGui;
    if g.open_popup_stack.size <= g.begin_popup_stack.size // Early out for performance
    {
        g.next_window_data.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }
    // flags |= WindowFlags::AlwaysAutoResize | WindowFlags::NoTitleBar | WindowFlags::NoSavedSettings;
    flags.insert(WindowFlags::AlwaysAutoResize);
    flags.insert(WindowFlags::NoTitleBar);
    flags.insert(WindowFlags::NoSavedSettings);

    // Id32 id = g.current_window.get_id(str_id);
    let id = g.current_window_id;
    return begin_popup_ex(g, id, flags);
}

// If 'p_open' is specified for a modal popup window, the popup will have a regular close button which will close the popup.
// Note that popup visibility status is owned by Dear ImGui (and manipulated with e.g. OpenPopup) so the actual value of *p_open is meaningless here.
// bool begin_popupModal(const char* name, bool* p_open, WindowFlags flags)
pub fn begin_popup_modal(g: &mut Context, name: &str, p_open: &mut bool, flags: &mut HashSet<WindowFlags>) -> bool {
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();
    // const Id32 id = window.get_id(name);
    let id = window.id;
    if !is_popup_open(g, id, &HashSet::new()) {
        g.next_window_data.clear_flags(); // We behave like Begin() and need to consume those values
        return false;
    }

    // Center modal windows by default for increased visibility
    // (this won't really last as settings will kick in, and is mostly for backward compatibility. user may do the same themselves)
    // FIXME: Should test for (pos_cond & window->set_window_pos_allow_flags) with the upcoming window.
    if g.next_window_data.flags.contains(&NextWindowDataFlags::HasPos) == false {
        let viewport = if window.was_active { g.viewport_mut(window.viewport_id) } else { get_main_viewport(g) }; // FIXME-VIEWPORT: What may be our reference viewport?
        set_next_window_pos(g, viewport.get_center(), Condition::FirstUseEver, Some(Vector2D::new(0.5, 0.5)));
    }

    // flags |= WindowFlags::Popup | WindowFlags::Modal | WindowFlags::NoCollapse | WindowFlags::NoDocking;
    flags.insert(WindowFlags::Popup);
    flags.insert(WindowFlags::Modal);
    flags.insert(WindowFlags::NoCollapse);
    flags.insert(WindowFlags::NoDocking);

    let is_open = begin(g, name, Some(p_open), Some(flags));
    if !is_open || (!*p_open) { // NB: is_open can be 'false' when the popup is completely clipped (e.g. zero size display) {
        end_popup(g);
        if is_open {
            close_popup_to_level(g, g.begin_popup_stack.size, true);
        }
        return false;
    }
    return is_open;
}

// void EndPopup()
pub fn end_popup(g: &mut Context) {
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();
    // IM_ASSERT(window.flags & WindowFlags::Popup);  // Mismatched begin_popup()/EndPopup() calls
    // IM_ASSERT(g.begin_popup_stack.size > 0);

    // Make all menus and popups wrap around for now, may need to expose that policy (e.g. focus scope could include wrap/loop policy flags used by new move requests)
    if g.nav_window_id == window.id {
        nav_move_request_try_wrapping(g, window, &HashSet::from([NavMoveFlags::LoopY]));
    }

    // Child-popups don't need to be laid out
    // IM_ASSERT(g.within_end_child == false);
    if window.flags.contains(&WindowFlags::ChildWindow) {
        g.within_end_child = true;
    }
    end(g);
    g.within_end_child = false;
}

// Helper to open a popup if mouse button is released over the item
// - This is essentially the same as begin_popupContextItem() but without the trailing begin_popup()
// void OpenPopupOnItemClick(const char* str_id, ImGuiPopupFlags popup_flags)
pub fn open_popup_on_item_click(g: &mut Context, str_id: &str, popup_flags: &HashSet<PopupFlags>) {
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();
    // let mouse_button = popup_flags.contains(&PopupFlags::MouseButtonMask_);
    let mut mouse_button = MouseButton::None;
    if popup_flags.contains(&PopupFlags::MouseButtonMiddle) {
        mouse_button = MouseButton::Middle;
    } else if popup_flags.contains(&PopupFlags::MouseButtonRight) {
        mouse_button = MouseButton::Right
    } else if popup_flags.contains(&PopupFlags::MouseButtonLeft) {
        mouse_button = MouseButton::Left
    }
    if is_mouse_released(g, mouse_button) && is_item_hovered(g, &HashSet::from([HoveredFlags::AllowWhenBlockedByPopup])) {
        let id = if str_id { window.get_id(g, str_id) } else { g.last_item_data.id };    // If user hasn't passed an id, we can use the LastItemID. Using LastItemID as a Popup id won't conflict!
        // IM_ASSERT(id != 0);                                             // You cannot pass a None str_id if the last item has no identifier (e.g. a Text() item)
        open_popup_ex(g, id, popup_flags);
    }
}

// This is a helper to handle the simplest case of associating one named popup to one given widget.
// - To create a popup associated to the last item, you generally want to pass a None value to str_id.
// - To create a popup with a specific identifier, pass it in str_id.
//    - This is useful when using using begin_popupContextItem() on an item which doesn't have an identifier, e.g. a Text() call.
//    - This is useful when multiple code locations may want to manipulate/open the same popup, given an explicit id.
// - You may want to handle the whole on user side if you have specific needs (e.g. tweaking IsItemHovered() parameters).
//   This is essentially the same as:
//       id = str_id ? GetID(str_id) : GetItemID();
//       OpenPopupOnItemClick(str_id, PopupFlags::MouseButtonRight);
//       return begin_popup(id);
//   Which is essentially the same as:
//       id = str_id ? GetID(str_id) : GetItemID();
//       if (IsItemHovered() && IsMouseReleased(MouseButton::Right))
//           OpenPopup(id);
//       return begin_popup(id);
//   The main difference being that this is tweaked to avoid computing the id twice.
// bool begin_popupContextItem(const char* str_id, ImGuiPopupFlags popup_flags)
pub fn begin_popup_context_item(g: &mut Context, str_id: &str, popup_flags: &HashSet<PopupFlags>) -> bool {
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();
    if window.skip_items {
        return false;
    }
    let id = if str_id { window.get_id(g, str_id) } else { g.last_item_data.id };    // If user hasn't passed an id, we can use the LastItemID. Using LastItemID as a Popup id won't conflict!
    // IM_ASSERT(id != 0);                                             // You cannot pass a None str_id if the last item has no identifier (e.g. a Text() item)
    // int mouse_button = (popup_flags & PopupFlags::MouseButtonMask_);
    let mut mouse_button = MouseButton::None;
    if popup_flags.contains(&PopupFlags::MouseButtonMiddle) {
        mouse_button = MouseButton::Middle;
    } else if popup_flags.contains(&PopupFlags::MouseButtonRight) {
        mouse_button = MouseButton::Right
    } else if popup_flags.contains(&PopupFlags::MouseButtonLeft) {
        mouse_button = MouseButton::Left
    }
    if is_mouse_released(g, mouse_button) && is_item_hovered(g, &HashSet::from([HoveredFlags::AllowWhenBlockedByPopup])) {
        open_popup_ex(g, id, popup_flags);
    }
    return begin_popup_ex(g, id, WindowFlags::AlwaysAutoResize | WindowFlags::NoTitleBar | WindowFlags::NoSavedSettings);
}

// bool begin_popupContextWindow(const char* str_id, ImGuiPopupFlags popup_flags)
pub fn begin_popup_context_window(g: &mut Context, str_id: &mut String, popup_flags: &HashSet<PopupFlags>) -> bool {
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();
    if !str_id {
        *str_id = String::from("window_context");
    }
    let id = window.get_id(g, str_id);
    let mut mouse_button = MouseButton::None;
    if popup_flags.contains(&PopupFlags::MouseButtonMiddle) {
        mouse_button = MouseButton::Middle;
    } else if popup_flags.contains(&PopupFlags::MouseButtonRight) {
        mouse_button = MouseButton::Right
    } else if popup_flags.contains(&PopupFlags::MouseButtonLeft) {
        mouse_button = MouseButton::Left
    }
    if is_mouse_released(g, mouse_button) && is_window_hovered(g, &HashSet::from([HoveredFlags::AllowWhenBlockedByPopup])) {
        if !popup_flags.contains(&PopupFlags::NoOpenOverItems) || !is_any_item_hovered(g) {
            open_popup_ex(g, id, popup_flags);
        }
    }
    return begin_popup_ex(g, id, &mut HashSet::from([WindowFlags::AlwaysAutoResize, WindowFlags::NoTitleBar, WindowFlags::NoSavedSettings]));
}

// bool begin_popupContextVoid(const char* str_id, ImGuiPopupFlags popup_flags)
pub fn begin_popup_context_void(g: &mut Context, str_id: &mut String, popup_flags: &HashSet<PopupFlags>) -> bool {
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();
    if !str_id {
        *str_id = String::from("void_context");
    }
    let id = window.get_id(g, str_id);
    let mut mouse_button = MouseButton::None;
    if popup_flags.contains(&PopupFlags::MouseButtonMiddle) {
        mouse_button = MouseButton::Middle;
    } else if popup_flags.contains(&PopupFlags::MouseButtonRight) {
        mouse_button = MouseButton::Right
    } else if popup_flags.contains(&PopupFlags::MouseButtonLeft) {
        mouse_button = MouseButton::Left
    }
    if is_mouse_released(g, mouse_button) && !is_window_hovered(g, &HashSet::from([HoveredFlags::AnyWindow])) {
        if get_top_most_popup_modal(g) == None {
            open_popup_ex(g, id, popup_flags);
        }
    }
    return begin_popup_ex(g, id, &mut HashSet::from([WindowFlags::AlwaysAutoResize, WindowFlags::NoTitleBar, WindowFlags::NoSavedSettings]));
}

pub const DIR_PREFERRED_ORDER: [Direction; 4] = [
    Direction::Down, Direction::Right, Direction::Left, Direction::Up
];

// r_avoid = the rectangle to avoid (e.g. for tooltip it is a rectangle around the mouse cursor which we want to avoid. for popups it's a small point around the cursor.)
// r_outer = the visible area rectangle, minus safe area padding. If our popup size won't fit because of safe area padding we ignore it.
// (r_outer is usually equivalent to the viewport rectangle minus padding, but when multi-viewports are enabled and monitor
//  information are available, it may represent the entire platform monitor from the frame of reference of the current viewport.
//  this allows us to have tooltips/popups displayed out of the parent viewport.)
// Vector2D FindBestWindowPosForPopupEx(const Vector2D& ref_pos, const Vector2D& size, ImGuiDir* last_dir, const Rect& r_outer, const Rect& r_avoid, ImGuiPopupPositionPolicy policy)
pub fn find_best_window_pos_for_popup_ex(g: &mut Context, ref_pos: &Vector2D, size: &Vector2D, last_dir: &Direction, r_outer: &Rect, r_avoid: &Rect, policy: PopupPositionPolicy) -> Vector2D {
    let base_pos_clamped = Vector2D::clamp(ref_pos, &r_outer.min, &(&r_outer.max - size));
    //GetForegroundDrawList()->add_rect(r_avoid.min, r_avoid.max, IM_COL32(255,0,0,255));
    //GetForegroundDrawList()->add_rect(r_outer.min, r_outer.max, IM_COL32(0,255,0,255));

    // Combo Box policy (we want a connecting edge)
    if policy == PopupPositionPolicy::ComboBox {
        // const ImGuiDir dir_prefered_order[Direction::COUNT] = { Direction::Down, Direction::Right, Direction::Left, Direction::Up };
        // for (int n  (*last_dir != Direction::None) ? -1 : 0; n < Direction::COUNT; n += 1)
        for dir in DIR_PREFERRED_ORDER {
            // const ImGuiDir dir = if (n == -1) { *last_dir }else{ dir_prefered_order[n]};
            // if (n != -1 && dir == *last_dir) // Already tried this direction?
            //     continue;
            // Vector2D pos;
            let mut pos = Vector2D::default();
            if dir == Direction::Down { pos = Vector2D::new(r_avoid.min.x, r_avoid.max.y); }     // Below, Toward Right (default)
            else if dir == Direction::Right { pos = Vector2D::new(r_avoid.min.x, r_avoid.min.y - size.y); } // Above, Toward Right
            else if dir == Direction::Left { pos = Vector2D::new(r_avoid.max.x - size.x, r_avoid.max.y); } // Below, Toward Left
            else if dir == Direction::Up { pos = Vector2D::new(r_avoid.max.x - size.x, r_avoid.min.y - size.y); } // Above, Toward Left
            if !r_outer.contains(Rect::new(&pos, (&pos + &size))) {
                continue;
            }
            // *last_dir = dir;
            return pos;
        }
    }

    // Tooltip and Default popup policy
    // (Always first try the direction we used on the last frame, if any)
    if policy == PopupPositionPolicy::Tooltip || policy == PopupPositionPolicy::Default {
        // const ImGuiDir dir_prefered_order[Direction::COUNT] = { Direction::Right, Direction::Down, Direction::Up, Direction::Left };
        let dir_preferred_order: [Direction; 4] = [Direction::Right, Direction::Down, Direction::Up, Direction::Left];
        // for (int n = (*last_dir != Direction::None) ? -1 : 0; n < Direction::COUNT; n += 1)
        for dir in dir_preferred_order {
            // const ImGuiDir dir = if (n == -1) { *last_dir }else{ dir_prefered_order[n]};
            // if (n != -1 && dir == *last_dir) // Already tried this direction?
            //     continue;

            let avail_w = (if dir == Direction::Left { r_avoid.min.x } else { r_outer.max.x }) - (if dir == Direction::Right { r_avoid.max.x } else { r_outer.min.x });
            let avail_h = (if dir == Direction::Up { r_avoid.min.y } else { r_outer.max.y }) - (if dir == Direction::Down { r_avoid.max.y } else { r_outer.min.y });

            // If there not enough room on one axis, there's no point in positioning on a side on this axis (e.g. when not enough width, use a top/bottom position to maximize available width)
            if avail_w < size.x && (dir == Direction::Left || dir == Direction::Right) {
                continue;
            }
            if avail_h < size.y && (dir == Direction::Up || dir == Direction::Down) {
                continue;
            }

            let mut pos = Vector2D::default();
            pos.x = if dir == Direction::Left { r_avoid.min.x - size.x } else {
                if (dir == Direction::Right) {
                    r_avoid.max.x
                } else { base_pos_clamped.x }
            };
            pos.y = if (dir == Direction::Up) { r_avoid.min.y - size.y } else {
                if (dir == Direction::Down) {
                    r_avoid.max.y
                } else { base_pos_clamped.y }
            };

            // Clamp top-left corner of popup
            pos.x = ImMax(pos.x, r_outer.min.x);
            pos.y = ImMax(pos.y, r_outer.min.y);

            // *last_dir = dir;
            return pos;
        }
    }

    // Fallback when not enough room:
    // *last_dir = Direction::None;

    // For tooltip we prefer avoiding the cursor at all cost even if it means that part of the tooltip won't be visible.
    if policy == PopupPositionPolicy::Tooltip {
        return ref_pos + Vector2D::new(2f32, 2f32);
    }

    // Otherwise try to keep within display
    // Vector2D pos = ref_pos;
    let mut pos = ref_pos.clone();
    pos.x = f32::max(f32::min(pos.x + size.x, r_outer.max.x) - size.x, r_outer.min.x);
    pos.y = f32::max(f32::min(pos.y + size.y, r_outer.max.y) - size.y, r_outer.min.y);
    return pos;
}

// Note that this is used for popups, which can overlap the non work-area of individual viewports.
// Rect GetPopupAllowedExtentRect(Window* window)
pub fn get_popup_allowed_extent_rect(g: &mut Context, window: &mut Window) -> Rect {
    // ImGuiContext& g = *GImGui;
    // Rect r_screen;
    let mut r_screen = Rect::default();
    if window.viewport_allow_platform_monitor_extend >= 0 {
        // Extent with be in the frame of reference of the given viewport (so min is likely to be negative here)
        let monitor = g.platform_io.monitors[window.viewport_allow_platform_monitor_extend];
        r_screen.min = monitor.work_pos;
        r_screen.max = monitor.work_pos + monitor.work_size;
    } else {
        // Use the full viewport area (not work area) for popups
        // r_screen = window.viewport.get_main_rect();
        r_screen = g.viewport_mut(window.viewport_id).unwrap().get_main_rect();
    }
    let padding = g.style.display_safe_area_padding;
    r_screen.expand_vector(&Vector2D::new(if r_screen.width() > padding.x * 2 { -padding.x } else { 0.0 }, if r_screen.height() > padding.y * 2 { -padding.y } else { 0.0 }));
    return r_screen;
}

// Vector2D FindBestWindowPosForPopup(Window* window)
pub fn find_best_window_pos_for_popup(g: &mut Context, window: &mut Window) -> Vector2D {
    // ImGuiContext& g = *GImGui;

    let r_outer: &mut Rect = get_popup_allowed_extend_rect(window);
    if window.flags.contains(&WindowFlags::ChildMenu) {
        // Child menus typically request _any_ position within the parent menu item, and then we move the new menu outside the parent bounds.
        // This is how we end up with child menus appearing (most-commonly) on the right of the parent menu.
        // Window* parent_window = window.parent_window;
        let parent_window = g.window_mut(window.parent_window_id);
        let horizontal_overlap = g.style.item_inner_spacing.x; // We want some overlap to convey the relative depth of each menu (currently the amount of overlap is hard-coded to style.item_spacing.x).
        let mut r_avoid: Rect = Rect::default();
        if parent_window.dc.menu_bar_appending {
            r_avoid = Rect(f32::MIN, parent_window.clip_rect.min.y, f32::MIN, parent_window.clip_rect.max.y);
        } // Avoid parent menu-bar. If we wanted multi-line menu-bar, we may instead want to have the calling window setup e.g. a next_window_data.PosConstraintAvoidRect field else r_avoid = Rect(parent_window.pos.x + horizontal_overlap, -f32::MAX, parent_window.pos.x + parent_window.size.x - horizontal_overlap - parent_window.scrollbar_sizes.x, f32::MAX);
        return find_best_window_pos_for_popup_ex(g, &window.pos, &window.size, &window.auto_pos_last_direction, r_outer, &r_avoid, PopupPositionPolicy::Default);
    }
    if window.flags.contains(&WindowFlags::Popup) {
        return find_best_window_pos_for_popup_ex(g, &window.pos, &window.size, &window.auto_pos_last_direction, r_outer, Rect(window.pos, window.pos), PopupPositionPolicy::Default); // Ideally we'd disable r_avoid here
    }
    if window.flags.contains(&WindowFlags::Tooltip) {
        // Position tooltip (always follows mouse)
        let sc = g.style.mouse_cursor_scale;
        let ref_pos = nav_calc_preferred_ref_pos();
        let mut r_avoid: Rect = Rect::default();
        if !g.nav_disable_highlight && g.nav_disable_mouse_hover && !g.io.config_flags.contains(&ConfigFlags::NavEnableSetMousePos) {
            r_avoid = Rect::new4(ref_pos.x - 16, ref_pos.y - 8, ref_pos.x + 16, ref_pos.y + 8);
        } else {
            r_avoid = Rect::new4(ref_pos.x - 16, ref_pos.y - 8, ref_pos.x + 24 * sc, ref_pos.y + 24 * sc); // FIXME: Hard-coded based on mouse cursor shape expectation. Exact dimension not very important.
            return find_best_window_pos_for_popup_ex(g, ref_pos, &window.size, &window.auto_pos_last_direction, &r_outer, &r_avoid, PopupPositionPolicy::Tooltip);
        }
    }
    // IM_ASSERT(0);
    return window.pos;
}
