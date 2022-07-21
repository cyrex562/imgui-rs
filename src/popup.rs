use std::collections::HashSet;
use crate::Context;
use crate::direction::Direction;
use crate::imgui_h::ImGuiID;
use crate::imgui_vec::Vector2D;
use crate::imgui_window::ImGuiWindow;
use crate::input::NavLayer;
use crate::orig_imgui_single_file::ImGuiID;
use crate::rect::Rect;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;
use crate::window::{Window, WindowFlags};
use crate::window::checks::is_window_active_and_visible;
use crate::window::next_window::NextWindowDataFlags;

// Storage for current popup stack
#[derive(Debug,Default,Clone)]
pub struct PopupData
{
    // ImGuiID             popup_id;        // Set on OpenPopup()
    pub popup_id: Id32,
    // ImGuiWindow*        window;         // Resolved on BeginPopup() - may stay unresolved if user never calls OpenPopup()
    pub window_id: Id32,
    // ImGuiWindow*        source_window;   // Set on OpenPopup() copy of nav_window at the time of opening the popup
    pub source_window_id:Id32,
    // int                 parent_nav_layer; // Resolved on BeginPopup(). Actually a ImGuiNavLayer type (declared down below), initialized to -1 which is not part of an enum, but serves well-enough as "not any of layers" value
    pub parent_nav_layer: i32,
    // int                 open_frame_count; // Set on OpenPopup()
    pub open_frame_count: i32,
    // ImGuiID             open_parent_id;   // Set on OpenPopup(), we need this to differentiate multiple menu sets from each others (e.g. inside menu bar vs loose menu items)
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
            parent_nav_layer: -1,
            open_frame_count: -1,
            ..Default::default()
        }
    }
}

pub enum PopupPositionPolicy
{
    Default,
    ComboBox,
    Tooltip
}

// pub const AnyPopup: i32                = DimgPopupFlags::AnyPopupId | DimgPopupFlags::AnyPopupLevel;
pub const POPUP_FLAGS_ANY_POPUP: HashSet<PopupFlags> = HashSet::from([
    PopupFlags::AnyPopupId, PopupFlags::AnyPopupLevel
]);

// flags for OpenPopup*(), BeginPopupContext*(), IsPopupOpen() functions.
// - To be backward compatible with older API which took an 'int mouse_button = 1' argument, we need to treat
//   small flags values as a mouse button index, so we encode the mouse button in the first few bits of the flags.
//   It is therefore guaranteed to be legal to pass a mouse button index in ImGuiPopupFlags.
// - For the same reason, we exceptionally default the ImGuiPopupFlags argument of BeginPopupContextXXX functions to 1 instead of 0.
//   IMPORTANT: because the default parameter is 1 (==ImGuiPopupFlags_MouseButtonRight), if you rely on the default parameter
//   and want to another another flag, you need to pass in the ImGuiPopupFlags_MouseButtonRight flag.
// - Multiple buttons currently cannot be combined/or-ed in those functions (we could allow it later).
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum PopupFlags
{
    None                    = 0,
    // ImGuiPopupFlags_MouseButtonLeft         = 0,        // For BeginPopupContext*(): open on Left Mouse release. Guaranteed to always be == 0 (same as ImGuiMouseButton_Left)
    MouseButtonRight        = 1,        // For BeginPopupContext*(): open on Right Mouse release. Guaranteed to always be == 1 (same as ImGuiMouseButton_Right)
    MouseButtonMiddle       = 2,        // For BeginPopupContext*(): open on Middle Mouse release. Guaranteed to always be == 2 (same as ImGuiMouseButton_Middle)
    MouseButtonMask_        = 0x1F,
    // ImGuiPopupFlags_MouseButtonDefault_     = 1,
    NoOpenOverExistingPopup,   // For OpenPopup*(), BeginPopupContext*(): don't open if there's already a popup at the same level of the popup stack
    NoOpenOverItems        ,   // For BeginPopupContextWindow(): don't return true when hovering items, only when hovering empty space
    AnyPopupId             ,   // For IsPopupOpen(): ignore the ImGuiID parameter and test for any popup.
    AnyPopupLevel          ,   // For IsPopupOpen(): search/test at any level of the popup stack (default test in the current level)

}

// Supported flags: ImGuiPopupFlags_AnyPopupId, ImGuiPopupFlags_AnyPopupLevel
// bool IsPopupOpen(ImGuiID id, ImGuiPopupFlags popup_flags)
pub fn is_popup_open(g: &mut Context, id: Id32, popup_flags: &HashSet<PopupFlags>) -> bool
{
    // ImGuiContext& g = *GImGui;
    if (popup_flags & ImGuiPopupFlags_AnyPopupId)
    {
        // Return true if any popup is open at the current BeginPopup() level of the popup stack
        // This may be used to e.g. test for another popups already opened to handle popups priorities at the same level.
        // IM_ASSERT(id == 0);
        if (popup_flags & ImGuiPopupFlags_AnyPopupLevel)
            return g.open_popup_stack.size > 0;
        else
            return g.open_popup_stack.size > g.begin_popup_stack.size;
    }
    else
    {
        if (popup_flags & ImGuiPopupFlags_AnyPopupLevel)
        {
            // Return true if the popup is open anywhere in the popup stack
            for (int n = 0; n < g.open_popup_stack.size; n += 1)
                if (g.open_popup_stack[n].PopupId == id)
                    return true;
            return false;
        }
        else
        {
            // Return true if the popup is open at the current BeginPopup() level of the popup stack (this is the most-common query)
            return g.open_popup_stack.size > g.begin_popup_stack.size && g.open_popup_stack[g.begin_popup_stack.size].PopupId == id;
        }
    }
}

// bool IsPopupOpen(const char* str_id, ImGuiPopupFlags popup_flags)
pub fn is_popup_open_2(g: &mut Context, str_id: &str, popup_flags: &HashSet<PopupFlags>) -> bool
{
    // ImGuiContext& g = *GImGui;
    ImGuiID id = (popup_flags & ImGuiPopupFlags_AnyPopupId) ? 0 : g.current_window.get_id(str_id);
    if ((popup_flags & ImGuiPopupFlags_AnyPopupLevel) && id != 0)
        // IM_ASSERT(0 && "Cannot use IsPopupOpen() with a string id and ImGuiPopupFlags_AnyPopupLevel."); // But non-string version is legal and used internally
    return IsPopupOpen(id, popup_flags);
}

// ImGuiWindow* get_top_most_popup_modal()
pub fn get_top_most_popup_modal(g: &mut Context) -> &mut Window
{
    // ImGuiContext& g = *GImGui;
    for (int n = g.open_popup_stack.size - 1; n >= 0; n--)
        if (ImGuiWindow* popup = g.open_popup_stack.data[n].Window)
            if (popup.flags & WindowFlags::Modal)
                return popup;
    return NULL;
}

// ImGuiWindow* GetTopMostAndVisiblePopupModal()
pub fn get_top_most_and_visible_popup_modal(g: &mut Context) -> &mut Window
{
    // ImGuiContext& g = *GImGui;
    for (int n = g.open_popup_stack.size - 1; n >= 0; n--)
        if (ImGuiWindow* popup = g.open_popup_stack.data[n].Window)
            if ((popup.flags & WindowFlags::Modal) && is_window_active_and_visible(popup))
                return popup;
    return NULL;
}

// void OpenPopup(const char* str_id, ImGuiPopupFlags popup_flags)
pub fn open_popup(g: &mut Context, str_id: &str, popup_flags: &HashSet<PopupFlags>)
{
    // ImGuiContext& g = *GImGui;
    ImGuiID id = g.current_window.get_id(str_id);
    IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopup(\"%s\" -> 0x%08X\n", str_id, id);
    OpenPopupEx(id, popup_flags);
}

// OpenPopup(ImGuiID id, ImGuiPopupFlags popup_flags)
pub fn open_popup2(g: &mut Context, id: Id32, popup_flags: &HashSet<PopupFlags>)
{
   OpenPopupEx(id, popup_flags);
}

// Mark popup as open (toggle toward open state).
// Popups are closed when user click outside, or activate a pressable item, or CloseCurrentPopup() is called within a BeginPopup()/EndPopup() block.
// Popup identifiers are relative to the current id-stack (so OpenPopup and BeginPopup needs to be at the same level).
// One open popup per level of the popup hierarchy (NB: when assigning we reset the window member of ImGuiPopupRef to NULL)
// void OpenPopupEx(ImGuiID id, ImGuiPopupFlags popup_flags)
pub fn open_popup_ex(g: &mut Context, id: Id32, popup_flags: &HashSet<PopupFlags>)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* parent_window = g.current_window;
    const int current_stack_size = g.begin_popup_stack.size;

    if (popup_flags & ImGuiPopupFlags_NoOpenOverExistingPopup)
        if (IsPopupOpen(0u, ImGuiPopupFlags_AnyPopupId))
            return;

    ImGuiPopupData popup_ref; // Tagged as new ref as window will be set back to NULL if we write this into open_popup_stack.
    popup_ref.PopupId = id;
    popup_ref.Window = NULL;
    popup_ref.SourceWindow = g.nav_window;
    popup_ref.OpenFrameCount = g.frame_count;
    popup_ref.OpenParentId = parent_window.IDStack.back();
    popup_ref.OpenPopupPos = NavCalcPreferredRefPos();
    popup_ref.OpenMousePos = is_mouse_pos_valid(&g.io.mouse_pos) ? g.io.mouse_pos : popup_ref.OpenPopupPos;

    IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopupEx(0x%08X)\n", id);
    if (g.open_popup_stack.size < current_stack_size + 1)
    {
        g.open_popup_stack.push_back(popup_ref);
    }
    else
    {
        // Gently handle the user mistakenly calling OpenPopup() every frame. It is a programming mistake! However, if we were to run the regular code path, the ui
        // would become completely unusable because the popup will always be in hidden-while-calculating-size state _while_ claiming focus. Which would be a very confusing
        // situation for the programmer. Instead, we silently allow the popup to proceed, it will keep reappearing and the programming error will be more obvious to understand.
        if (g.open_popup_stack[current_stack_size].PopupId == id && g.open_popup_stack[current_stack_size].OpenFrameCount == g.frame_count - 1)
        {
            g.open_popup_stack[current_stack_size].OpenFrameCount = popup_ref.OpenFrameCount;
        }
        else
        {
            // Close child popups if any, then flag popup for open/reopen
            ClosePopupToLevel(current_stack_size, false);
            g.open_popup_stack.push_back(popup_ref);
        }

        // When reopening a popup we first refocus its parent, otherwise if its parent is itself a popup it would get closed by close_popups_over_window().
        // This is equivalent to what ClosePopupToLevel() does.
        //if (g.open_popup_stack[current_stack_size].popup_id == id)
        //    focus_window(parent_window);
    }
}

// When popups are stacked, clicking on a lower level popups puts focus back to it and close popups above it.
// This function closes any popups that are over 'ref_window'.
// void close_popups_over_window(ImGuiWindow* ref_window, bool restore_focus_to_window_under_popup)
pub fn close_popups_over_window(g: &mut Context, ref_window: &mut Window, restore_focus_to_window_under_popup: bool)
{
    // ImGuiContext& g = *GImGui;
    if (g.open_popup_stack.size == 0)
        return;

    // Don't close our own child popup windows.
    int popup_count_to_keep = 0;
    if (ref_window)
    {
        // Find the highest popup which is a descendant of the reference window (generally reference window = nav_window)
        for (; popup_count_to_keep < g.open_popup_stack.size; popup_count_to_keep += 1)
        {
            ImGuiPopupData& popup = g.open_popup_stack[popup_count_to_keep];
            if (!popup.Window)
                continue;
            // IM_ASSERT((popup.Window.flags & WindowFlags::Popup) != 0);
            if (popup.Window.flags & WindowFlags::ChildWindow)
                continue;

            // Trim the stack unless the popup is a direct parent of the reference window (the reference window is often the nav_window)
            // - With this stack of window, clicking/focusing Popup1 will close Popup2 and Popup3:
            //     window -> Popup1 -> Popup2 -> Popup3
            // - Each popups may contain child windows, which is why we compare ->root_window_dock_tree!
            //     window -> Popup1 -> Popup1_Child -> Popup2 -> Popup2_Child
            bool ref_window_is_descendent_of_popup = false;
            for (int n = popup_count_to_keep; n < g.open_popup_stack.size; n += 1)
                if (ImGuiWindow* popup_window = g.open_popup_stack[n].Window)
                    //if (popup_window->root_window_dock_tree == ref_window->root_window_dock_tree) // FIXME-MERGE
                    if (is_window_within_begin_stack_of(ref_window, popup_window))
                    {
                        ref_window_is_descendent_of_popup = true;
                        break;
                    }
            if (!ref_window_is_descendent_of_popup)
                break;
        }
    }
    if (popup_count_to_keep < g.open_popup_stack.size) // This test is not required but it allows to set a convenient breakpoint on the statement below
    {
        IMGUI_DEBUG_LOG_POPUP("[popup] close_popups_over_window(\"%s\")\n", ref_window ? ref_window.Name : "<NULL>");
        ClosePopupToLevel(popup_count_to_keep, restore_focus_to_window_under_popup);
    }
}

// void ClosePopupsExceptModals()
pub fn close_popups_except_modals(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;

    int popup_count_to_keep;
    for (popup_count_to_keep = g.open_popup_stack.size; popup_count_to_keep > 0; popup_count_to_keep--)
    {
        ImGuiWindow* window = g.open_popup_stack[popup_count_to_keep - 1].Window;
        if (!window || window.flags & WindowFlags::Modal)
            break;
    }
    if (popup_count_to_keep < g.open_popup_stack.size) // This test is not required but it allows to set a convenient breakpoint on the statement below
        ClosePopupToLevel(popup_count_to_keep, true);
}

// void ClosePopupToLevel(int remaining, bool restore_focus_to_window_under_popup)
pub fn close_popup_to_level(g: &mut Context, remaining: i32, restore_focus_to_window_under_popup: bool)
{
    // ImGuiContext& g = *GImGui;
    IMGUI_DEBUG_LOG_POPUP("[popup] ClosePopupToLevel(%d), restore_focus_to_window_under_popup=%d\n", remaining, restore_focus_to_window_under_popup);
    // IM_ASSERT(remaining >= 0 && remaining < g.open_popup_stack.size);

    // Trim open popup stack
    ImGuiWindow* focus_window = g.open_popup_stack[remaining].SourceWindow;
    ImGuiWindow* popup_window = g.open_popup_stack[remaining].Window;
    g.open_popup_stack.resize(remaining);

    if (restore_focus_to_window_under_popup)
    {
        if (focus_window && !focus_window.was_active && popup_window)
        {
            // Fallback
            FocusTopMostWindowUnderOne(popup_window, NULL);
        }
        else
        {
            if (g.NavLayer == NavLayer::Main && focus_window)
                focus_window = NavRestoreLastChildNavWindow(focus_window);
            focus_window(focus_window);
        }
    }
}

// Close the popup we have begin-ed into.
// void CloseCurrentPopup()
pub fn close_current_popup(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    int popup_idx = g.begin_popup_stack.size - 1;
    if (popup_idx < 0 || popup_idx >= g.open_popup_stack.size || g.begin_popup_stack[popup_idx].PopupId != g.open_popup_stack[popup_idx].PopupId)
        return;

    // Closing a menu closes its top-most parent popup (unless a modal)
    while (popup_idx > 0)
    {
        ImGuiWindow* popup_window = g.open_popup_stack[popup_idx].Window;
        ImGuiWindow* parent_popup_window = g.open_popup_stack[popup_idx - 1].Window;
        bool close_parent = false;
        if (popup_window && (popup_window.flags & WindowFlags::ChildMenu))
            if (parent_popup_window && !(parent_popup_window.flags & WindowFlags::MenuBar))
                close_parent = true;
        if (!close_parent)
            break;
        popup_idx--;
    }
    IMGUI_DEBUG_LOG_POPUP("[popup] CloseCurrentPopup %d -> %d\n", g.begin_popup_stack.size - 1, popup_idx);
    ClosePopupToLevel(popup_idx, true);

    // A common pattern is to close a popup when selecting a menu item/selectable that will open another window.
    // To improve this usage pattern, we avoid nav highlight for a single frame in the parent window.
    // Similarly, we could avoid mouse hover highlight in this window but it is less visually problematic.
    if (ImGuiWindow* window = g.nav_window)
        window.dc.NavHideHighlightOneFrame = true;
}

// Attention! BeginPopup() adds default flags which BeginPopupEx()!
// bool BeginPopupEx(ImGuiID id, ImGuiWindowFlags flags)
pub fn begin_popup_ex(g: &mut Context, id: Id32, flags: &HashSet<WindowFlags>) -> bool
{
    // ImGuiContext& g = *GImGui;
    if (!IsPopupOpen(id, ImGuiPopupFlags_None))
    {
        g.next_window_data.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }

    char name[20];
    if (flags & WindowFlags::ChildMenu)
        ImFormatString(name, IM_ARRAYSIZE(name), "##Menu_%02d", g.BeginMenuCount); // Recycle windows based on depth
    else
        ImFormatString(name, IM_ARRAYSIZE(name), "##Popup_%08x", id); // Not recycling, so we can close/open during the same frame

    flags |= WindowFlags::Popup | WindowFlags::NoDocking;
    bool is_open = begin(name, NULL, flags);
    if (!is_open) // NB: Begin can return false when the popup is completely clipped (e.g. zero size display)
        EndPopup();

    return is_open;
}

// bool BeginPopup(const char* str_id, ImGuiWindowFlags flags)
pub fn begin_popup(g: &mut Context, str_id: &str, flags: &HashSet<WindowFlags>) -> bool
{
    // ImGuiContext& g = *GImGui;
    if (g.open_popup_stack.size <= g.begin_popup_stack.size) // Early out for performance
    {
        g.next_window_data.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }
    flags |= WindowFlags::AlwaysAutoResize | WindowFlags::NoTitleBar | WindowFlags::NoSavedSettings;
    ImGuiID id = g.current_window.get_id(str_id);
    return BeginPopupEx(id, flags);
}

// If 'p_open' is specified for a modal popup window, the popup will have a regular close button which will close the popup.
// Note that popup visibility status is owned by Dear ImGui (and manipulated with e.g. OpenPopup) so the actual value of *p_open is meaningless here.
// bool BeginPopupModal(const char* name, bool* p_open, ImGuiWindowFlags flags)
pub fn begin_popup_modal(g: &mut Context, name: &str, p_open: &mut bool, flags: &HashSet<WindowFlags>) -> bool
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    const ImGuiID id = window.get_id(name);
    if (!IsPopupOpen(id, ImGuiPopupFlags_None))
    {
        g.next_window_data.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }

    // Center modal windows by default for increased visibility
    // (this won't really last as settings will kick in, and is mostly for backward compatibility. user may do the same themselves)
    // FIXME: Should test for (PosCond & window->set_window_pos_allow_flags) with the upcoming window.
    if ((g.next_window_data.flags & NextWindowDataFlags::HasPos) == 0)
    {
        const ImGuiViewport* viewport = window.was_active ? window.viewport : GetMainViewport(); // FIXME-VIEWPORT: What may be our reference viewport?
        SetNextWindowPos(viewport.GetCenter(), Cond::FirstUseEver, Vector2D::new(0.5, 0.5));
    }

    flags |= WindowFlags::Popup | WindowFlags::Modal | WindowFlags::NoCollapse | WindowFlags::NoDocking;
    const bool is_open = begin(name, p_open, flags);
    if (!is_open || (p_open && !*p_open)) // NB: is_open can be 'false' when the popup is completely clipped (e.g. zero size display)
    {
        EndPopup();
        if (is_open)
            ClosePopupToLevel(g.begin_popup_stack.size, true);
        return false;
    }
    return is_open;
}

// void EndPopup()
pub fn end_popup(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    // IM_ASSERT(window.flags & WindowFlags::Popup);  // Mismatched BeginPopup()/EndPopup() calls
    // IM_ASSERT(g.begin_popup_stack.size > 0);

    // Make all menus and popups wrap around for now, may need to expose that policy (e.g. focus scope could include wrap/loop policy flags used by new move requests)
    if (g.nav_window == window)
        NavMoveRequestTryWrapping(window, ImGuiNavMoveFlags_LoopY);

    // Child-popups don't need to be laid out
    // IM_ASSERT(g.within_end_child == false);
    if (window.flags & WindowFlags::ChildWindow)
        g.within_end_child = true;
    end();
    g.within_end_child = false;
}

// Helper to open a popup if mouse button is released over the item
// - This is essentially the same as BeginPopupContextItem() but without the trailing BeginPopup()
// void OpenPopupOnItemClick(const char* str_id, ImGuiPopupFlags popup_flags)
pub fn open_popup_on_item_click(g: &mut Context, str_id: &str, popup_flags: &HashSet<PopupFlags>)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
    {
        ImGuiID id = str_id ? window.get_id(str_id) : g.last_item_data.id;    // If user hasn't passed an id, we can use the LastItemID. Using LastItemID as a Popup id won't conflict!
        // IM_ASSERT(id != 0);                                             // You cannot pass a NULL str_id if the last item has no identifier (e.g. a Text() item)
        OpenPopupEx(id, popup_flags);
    }
}

// This is a helper to handle the simplest case of associating one named popup to one given widget.
// - To create a popup associated to the last item, you generally want to pass a NULL value to str_id.
// - To create a popup with a specific identifier, pass it in str_id.
//    - This is useful when using using BeginPopupContextItem() on an item which doesn't have an identifier, e.g. a Text() call.
//    - This is useful when multiple code locations may want to manipulate/open the same popup, given an explicit id.
// - You may want to handle the whole on user side if you have specific needs (e.g. tweaking IsItemHovered() parameters).
//   This is essentially the same as:
//       id = str_id ? GetID(str_id) : GetItemID();
//       OpenPopupOnItemClick(str_id, ImGuiPopupFlags_MouseButtonRight);
//       return BeginPopup(id);
//   Which is essentially the same as:
//       id = str_id ? GetID(str_id) : GetItemID();
//       if (IsItemHovered() && IsMouseReleased(ImGuiMouseButton_Right))
//           OpenPopup(id);
//       return BeginPopup(id);
//   The main difference being that this is tweaked to avoid computing the id twice.
// bool BeginPopupContextItem(const char* str_id, ImGuiPopupFlags popup_flags)
pub fn begin_popup_context_item(g: &mut Context, str_id: &str, popup_flags: &HashSet<PopupFlags>) -> bool
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    if (window.skip_items)
        return false;
    ImGuiID id = str_id ? window.get_id(str_id) : g.last_item_data.id;    // If user hasn't passed an id, we can use the LastItemID. Using LastItemID as a Popup id won't conflict!
    // IM_ASSERT(id != 0);                                             // You cannot pass a NULL str_id if the last item has no identifier (e.g. a Text() item)
    int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
        OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, WindowFlags::AlwaysAutoResize | WindowFlags::NoTitleBar | WindowFlags::NoSavedSettings);
}

// bool BeginPopupContextWindow(const char* str_id, ImGuiPopupFlags popup_flags)
pub fn begin_popup_context_window(g: &mut Context, str_id: &str, popup_flags: &HashSet<PopupFlags>) -> bool
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    if (!str_id)
        str_id = "window_context";
    ImGuiID id = window.get_id(str_id);
    int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsWindowHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
        if (!(popup_flags & ImGuiPopupFlags_NoOpenOverItems) || !IsAnyItemHovered())
            OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, WindowFlags::AlwaysAutoResize | WindowFlags::NoTitleBar | WindowFlags::NoSavedSettings);
}

// bool BeginPopupContextVoid(const char* str_id, ImGuiPopupFlags popup_flags)
pub fn begin_popup_context_void(g: &mut Context, str_id: &str, popup_flags: &HashSet<PopupFlags>) -> bool
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    if (!str_id)
        str_id = "void_context";
    ImGuiID id = window.get_id(str_id);
    int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && !IsWindowHovered(ImGuiHoveredFlags_AnyWindow))
        if (get_top_most_popup_modal() == NULL)
            OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, WindowFlags::AlwaysAutoResize | WindowFlags::NoTitleBar | WindowFlags::NoSavedSettings);
}

// r_avoid = the rectangle to avoid (e.g. for tooltip it is a rectangle around the mouse cursor which we want to avoid. for popups it's a small point around the cursor.)
// r_outer = the visible area rectangle, minus safe area padding. If our popup size won't fit because of safe area padding we ignore it.
// (r_outer is usually equivalent to the viewport rectangle minus padding, but when multi-viewports are enabled and monitor
//  information are available, it may represent the entire platform monitor from the frame of reference of the current viewport.
//  this allows us to have tooltips/popups displayed out of the parent viewport.)
// Vector2D FindBestWindowPosForPopupEx(const Vector2D& ref_pos, const Vector2D& size, ImGuiDir* last_dir, const Rect& r_outer, const Rect& r_avoid, ImGuiPopupPositionPolicy policy)
pub fn find_best_window_pos_for_popup_ex(g: &mut Context, ref_pos: &Vector2D, size: &Vector2D, last_dir: &Direction, r_outer: &Rect, r_avoid: &Rect, policy: PopupPositionPolicy) -> Vector2D
{
    Vector2D base_pos_clamped = ImClamp(ref_pos, r_outer.min, r_outer.max - size);
    //GetForegroundDrawList()->add_rect(r_avoid.min, r_avoid.max, IM_COL32(255,0,0,255));
    //GetForegroundDrawList()->add_rect(r_outer.min, r_outer.max, IM_COL32(0,255,0,255));

    // Combo Box policy (we want a connecting edge)
    if (policy == ImGuiPopupPositionPolicy_ComboBox)
    {
        const ImGuiDir dir_prefered_order[Dir::COUNT] = { Dir::Down, Dir::Right, Dir::Left, Dir::Up };
        for (int n = (*last_dir != Dir::None) ? -1 : 0; n < Dir::COUNT; n += 1)
        {
            const ImGuiDir dir = (n == -1) ? *last_dir : dir_prefered_order[n];
            if (n != -1 && dir == *last_dir) // Already tried this direction?
                continue;
            Vector2D pos;
            if (dir == Dir::Down)  pos = Vector2D::new(r_avoid.min.x, r_avoid.max.y);          // Below, Toward Right (default)
            if (dir == Dir::Right) pos = Vector2D::new(r_avoid.min.x, r_avoid.min.y - size.y); // Above, Toward Right
            if (dir == Dir::Left)  pos = Vector2D::new(r_avoid.max.x - size.x, r_avoid.max.y); // Below, Toward Left
            if (dir == Dir::Up)    pos = Vector2D::new(r_avoid.max.x - size.x, r_avoid.min.y - size.y); // Above, Toward Left
            if (!r_outer.Contains(Rect(pos, pos + size)))
                continue;
            *last_dir = dir;
            return pos;
        }
    }

    // Tooltip and Default popup policy
    // (Always first try the direction we used on the last frame, if any)
    if (policy == ImGuiPopupPositionPolicy_Tooltip || policy == ImGuiPopupPositionPolicy_Default)
    {
        const ImGuiDir dir_prefered_order[Dir::COUNT] = { Dir::Right, Dir::Down, Dir::Up, Dir::Left };
        for (int n = (*last_dir != Dir::None) ? -1 : 0; n < Dir::COUNT; n += 1)
        {
            const ImGuiDir dir = (n == -1) ? *last_dir : dir_prefered_order[n];
            if (n != -1 && dir == *last_dir) // Already tried this direction?
                continue;

            const float avail_w = (dir == Dir::Left ? r_avoid.min.x : r_outer.max.x) - (dir == Dir::Right ? r_avoid.max.x : r_outer.min.x);
            const float avail_h = (dir == Dir::Up ? r_avoid.min.y : r_outer.max.y) - (dir == Dir::Down ? r_avoid.max.y : r_outer.min.y);

            // If there not enough room on one axis, there's no point in positioning on a side on this axis (e.g. when not enough width, use a top/bottom position to maximize available width)
            if (avail_w < size.x && (dir == Dir::Left || dir == Dir::Right))
                continue;
            if (avail_h < size.y && (dir == Dir::Up || dir == Dir::Down))
                continue;

            Vector2D pos;
            pos.x = (dir == Dir::Left) ? r_avoid.min.x - size.x : (dir == Dir::Right) ? r_avoid.max.x : base_pos_clamped.x;
            pos.y = (dir == Dir::Up) ? r_avoid.min.y - size.y : (dir == Dir::Down) ? r_avoid.max.y : base_pos_clamped.y;

            // Clamp top-left corner of popup
            pos.x = ImMax(pos.x, r_outer.min.x);
            pos.y = ImMax(pos.y, r_outer.min.y);

            *last_dir = dir;
            return pos;
        }
    }

    // Fallback when not enough room:
    *last_dir = Dir::None;

    // For tooltip we prefer avoiding the cursor at all cost even if it means that part of the tooltip won't be visible.
    if (policy == ImGuiPopupPositionPolicy_Tooltip)
        return ref_pos + Vector2D::new(2, 2);

    // Otherwise try to keep within display
    Vector2D pos = ref_pos;
    pos.x = ImMax(ImMin(pos.x + size.x, r_outer.max.x) - size.x, r_outer.min.x);
    pos.y = ImMax(ImMin(pos.y + size.y, r_outer.max.y) - size.y, r_outer.min.y);
    return pos;
}

// Note that this is used for popups, which can overlap the non work-area of individual viewports.
// Rect GetPopupAllowedExtentRect(ImGuiWindow* window)
pub fn get_popup_allowed_extent_rect(g: &mut Context, window: &mut Window) -> Rect
{
    // ImGuiContext& g = *GImGui;
    Rect r_screen;
    if (window.ViewportAllowPlatformMonitorExtend >= 0)
    {
        // Extent with be in the frame of reference of the given viewport (so min is likely to be negative here)
        const ImGuiPlatformMonitor& monitor = g.platform_io.monitors[window.ViewportAllowPlatformMonitorExtend];
        r_screen.min = monitor.WorkPos;
        r_screen.max = monitor.WorkPos + monitor.work_size;
    }
    else
    {
        // Use the full viewport area (not work area) for popups
        r_screen = window.viewport.get_main_rect();
    }
    Vector2D padding = g.style.DisplaySafeAreaPadding;
    r_screen.Expand(Vector2D::new((r_screen.get_width() > padding.x * 2) ? -padding.x : 0.0, (r_screen.get_height() > padding.y * 2) ? -padding.y : 0.0));
    return r_screen;
}

// Vector2D FindBestWindowPosForPopup(ImGuiWindow* window)
pub fn find_best_window_pos_for_popup(g: &mut Context, window: &mut Window) -> Vector2D
{
    // ImGuiContext& g = *GImGui;

    Rect r_outer = GetPopupAllowedExtentRect(window);
    if (window.flags & WindowFlags::ChildMenu)
    {
        // Child menus typically request _any_ position within the parent menu item, and then we move the new menu outside the parent bounds.
        // This is how we end up with child menus appearing (most-commonly) on the right of the parent menu.
        ImGuiWindow* parent_window = window.parent_window;
        float horizontal_overlap = g.style.item_inner_spacing.x; // We want some overlap to convey the relative depth of each menu (currently the amount of overlap is hard-coded to style.ItemSpacing.x).
        Rect r_avoid;
        if (parent_window.dc.MenuBarAppending)
            r_avoid = Rect(-f32::MAX, parent_window.clip_rect.min.y, f32::MAX, parent_window.clip_rect.max.y); // Avoid parent menu-bar. If we wanted multi-line menu-bar, we may instead want to have the calling window setup e.g. a next_window_data.PosConstraintAvoidRect field
        else
            r_avoid = Rect(parent_window.Pos.x + horizontal_overlap, -f32::MAX, parent_window.Pos.x + parent_window.size.x - horizontal_overlap - parent_window.scrollbar_sizes.x, f32::MAX);
        return FindBestWindowPosForPopupEx(window.pos, window.size, &window.AutoPosLastDirection, r_outer, r_avoid, ImGuiPopupPositionPolicy_Default);
    }
    if (window.flags & WindowFlags::Popup)
    {
        return FindBestWindowPosForPopupEx(window.pos, window.size, &window.AutoPosLastDirection, r_outer, Rect(window.pos, window.pos), ImGuiPopupPositionPolicy_Default); // Ideally we'd disable r_avoid here
    }
    if (window.flags & WindowFlags::Tooltip)
    {
        // Position tooltip (always follows mouse)
        float sc = g.style.MouseCursorScale;
        Vector2D ref_pos = NavCalcPreferredRefPos();
        Rect r_avoid;
        if (!g.nav_disable_highlight && g.nav_disable_mouse_hover && !(g.io.config_flags & ImGuiConfigFlags_NavEnableSetMousePos))
            r_avoid = Rect(ref_pos.x - 16, ref_pos.y - 8, ref_pos.x + 16, ref_pos.y + 8);
        else
            r_avoid = Rect(ref_pos.x - 16, ref_pos.y - 8, ref_pos.x + 24 * sc, ref_pos.y + 24 * sc); // FIXME: Hard-coded based on mouse cursor shape expectation. Exact dimension not very important.
        return FindBestWindowPosForPopupEx(ref_pos, window.size, &window.AutoPosLastDirection, r_outer, r_avoid, ImGuiPopupPositionPolicy_Tooltip);
    }
    // IM_ASSERT(0);
    return window.pos;
}
