use crate::a_imgui_cpp::NavMoveRequestTryWrapping;
use crate::condition::ImGuiCond_FirstUseEver;
use crate::config_flags::ImGuiConfigFlags_NavEnableSetMousePos;
use crate::context::ImguiContext;
use crate::direction::{
    ImGuiDir, ImGuiDir_COUNT, ImGuiDir_Down, ImGuiDir_Left, ImGuiDir_None, ImGuiDir_Right,
    ImGuiDir_Up,
};
use crate::hovered_flags::{
    ImGuiHoveredFlags_AllowWhenBlockedByPopup, ImGuiHoveredFlags_AnyWindow,
};
use crate::input_ops::IsMousePosValid;
use crate::item_ops::{IsAnyItemHovered, IsItemHovered};
use crate::math_ops::{ImClamp, ImMax, ImMin};
use crate::nav_layer::ImGuiNavLayer_Main;
use crate::nav_move_flags::ImGuiNavMoveFlags_LoopY;
use crate::nav_ops::{
    NavCalcPreferredRefPos, NavMoveRequestTryWrapping, NavRestoreLastChildNavWindow,
};
use crate::next_window_data_flags::ImGuiNextWindowDataFlags_HasPos;
use crate::popup_data::ImGuiPopupData;
use crate::popup_flags::{
    ImGuiPopupFlags, ImGuiPopupFlags_AnyPopupId, ImGuiPopupFlags_AnyPopupLevel,
    ImGuiPopupFlags_MouseButtonMask_, ImGuiPopupFlags_NoOpenOverExistingPopup,
    ImGuiPopupFlags_NoOpenOverItems, ImGuiPopupFlags_None,
};
use crate::popup_position_policy::{
    ImGuiPopupPositionPolicy, ImGuiPopupPositionPolicy_ComboBox, ImGuiPopupPositionPolicy_Default,
    ImGuiPopupPositionPolicy_Tooltip,
};
use crate::rect::ImRect;
use crate::string_ops::{str_to_const_c_char_ptr, ImFormatString};
use crate::type_defs::ImguiHandle;
use crate::utils::{flag_clear, flag_set, is_not_null};
use crate::vec2::ImVec2;
use crate::viewport_ops::GetMainViewport;
use crate::window::find::IsWindowWithinBeginStackOf;
use crate::window::focus::{FocusTopMostWindowUnderOne, FocusWindow};
use crate::window::ops::{Begin, End, IsWindowActiveAndVisible};
use crate::window::props::{IsWindowHovered, SetNextWindowPos};
use crate::window::window_flags::{
    ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_ChildMenu,
    ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_MenuBar, ImGuiWindowFlags_Modal,
    ImGuiWindowFlags_NoCollapse, ImGuiWindowFlags_NoDocking, ImGuiWindowFlags_NoSavedSettings,
    ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_Popup, ImGuiWindowFlags_Tooltip,
};
use crate::window::ImguiWindow;
use crate::{GImGui, ImguiViewport};
use libc::{c_char, c_float, c_int};
use std::ptr::{null, null_mut};

// Supported flags: ImGuiPopupFlags_AnyPopupId, ImGuiPopupFlags_AnyPopupLevel
// IsPopupOpen: bool(id: ImguiHandle, ImGuiPopupFlags popup_flags)
pub fn IsPopupOpen(g: &mut ImguiContext, id: ImguiHandle, popup_flags: ImGuiPopupFlags) -> bool {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    if flag_set(popup_flags, ImGuiPopupFlags_AnyPopupId) {
        // Return true if any popup is open at the current BeginPopup() level of the popup stack
        // This may be used to e.g. test for another popups already opened to handle popups priorities at the same level.
        // IM_ASSERT(id == 0);
        if flag_set(popup_flags, ImGuiPopupFlags_AnyPopupLevel) {
            return g.OpenPopupStack.len() > 0;
        } else {
            return g.OpenPopupStack.len() > g.BeginPopupStack.len();
        }
    } else {
        if flag_set(popup_flags, ImGuiPopupFlags_AnyPopupLevel) {
            // Return true if the popup is open anywhere in the popup stack
            // for (let n: c_int = 0; n < g.OpenPopupStack.len(); n++)
            for n in 0..g.OpenPopupStack.len() {
                if g.OpenPopupStack[n].PopupId == id {
                    return true;
                }
            }
            return false;
        } else {
            // Return true if the popup is open at the current BeginPopup() level of the popup stack (this is the most-common query)
            return g.OpenPopupStack.len() > g.BeginPopupStack.len()
                && g.OpenPopupStack[g.BeginPopupStack.len()].PopupId == id;
        }
    }
}

// IsPopupOpen: bool(str_id: *const c_char, ImGuiPopupFlags popup_flags)
pub fn IsPopupOpenWithStrId(
    g: &mut ImguiContext,
    str_id: &String,
    popup_flags: ImGuiPopupFlags,
) -> bool {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut id: ImguiHandle = if flag_set(popup_flags, ImGuiPopupFlags_AnyPopupId) {
        0
    } else {
        g.Currentwindow.GetID(str_id)
    };
    if flag_set(popup_flags, ImGuiPopupFlags_AnyPopupLevel) && id != 0 {}
    // IM_ASSERT(0 && "Cannot use IsPopupOpen() with a string id and ImGuiPopupFlags_AnyPopupLevel."); // But non-string version is legal and used internally
    return IsPopupOpen(g, id, popup_flags);
}

// GetTopMostPopupModal: *mut ImGuiWindow()
pub fn GetTopMostPopupModal(g: &mut ImguiContext) -> Option<ImguiWindow> {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    // for (let n: c_int = g.OpenPopupStack.len() - 1; n >= 0; n--)
    for n in g.OpenPopupStack.len() - 1..0 {
        let mut popup = &mut g.OpenPopupStack[n].Window;
        if popup.is_some() {
            if flag_set(popup.unwrap().Flags, ImGuiWindowFlags_Modal) {
                return popup.clone();
            }
        }
    }
    return None;
}

// GetTopMostAndVisiblePopupModal: *mut ImGuiWindow()
pub fn GetTopMostAndVisiblePopupModal(g: &mut ImguiContext) -> Option<&mut ImguiWindow> {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    // for (let n: c_int = g.OpenPopupStack.len() - 1; n >= 0; n--)
    for n in g.OpenPopupStack.len() - 1..0 {
        let mut popup = g.OpenPopupStack.Data[n].Window;
        if is_not_null(popup) {
            if flag_set(popup.Flags, ImGuiWindowFlags_Modal) && IsWindowActiveAndVisible(popup) {
                return popup;
            }
        }
    }
    return None;
}

pub unsafe fn OpenPopup(str_id: &str, popup_flags: ImGuiPopupFlags) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut id: ImguiHandle = g.Currentwindow.GetID(str_id);
    IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopup(\"{}\" -> 0x{}\n", str_id, id);
    OpenPopupEx(g, id, popup_flags);
}

pub unsafe fn OpenPopup2(id: ImguiHandle, popup_flags: ImGuiPopupFlags) {
    OpenPopupEx(g, id, popup_flags);
}

// Mark popup as open (toggle toward open state).
// Popups are closed when user click outside, or activate a pressable item, or CloseCurrentPopup() is called within a BeginPopup()/EndPopup() block.
// Popup identifiers are relative to the current ID-stack (so OpenPopup and BeginPopup needs to be at the same level).
// One open popup per level of the popup hierarchy (NB: when assigning we reset the Window member of ImGuiPopupRef to NULL)
pub  fn OpenPopupEx(g: &mut ImguiContext, id: ImguiHandle, popup_flags: ImGuiPopupFlags) {
    let mut parent_window = g.current_window_mut().unwrap();
    let current_stack_size: usize = g.BeginPopupStack.len();

    if flag_set(popup_flags , ImGuiPopupFlags_NoOpenOverExistingPopup) {
        if IsPopupOpen(g, 0, ImGuiPopupFlags_AnyPopupId) {
            return;
        }
    }

    // ImGuiPopupData popup_ref; // Tagged as new ref as Window will be set back to NULL if we write this into OpenPopupStack.
    let mut popup_ref = ImGuiPopupData::default();
    popup_ref.PopupId = id;
    popup_ref.Window = None;
    popup_ref.BackupNavWindow = g.NavWindow; // When popup closes focus may be restored to NavWindow (depend on window type).
    popup_ref.OpenFrameCount = g.FrameCount;
    popup_ref.OpenParentId = parent_window.id_stack.last().unwrap().clone();
    popup_ref.OpenPopupPos = NavCalcPreferredRefPos(g);
    popup_ref.OpenMousePos = if IsMousePosValid(&g.IO.MousePos) {
        g.IO.MousePos
    } else {
        popup_ref.OpenPopupPos
    };

    IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopupEx(0x{})\n", id);
    if (g.OpenPopupStack.len() < current_stack_size + 1) {
        g.OpenPopupStack.push(popup_re0f32);
    } else {
        // Gently handle the user mistakenly calling OpenPopup() every frame. It is a programming mistake! However, if we were to run the regular code path, the ui
        // would become completely unusable because the popup will always be in hidden-while-calculating-size state _while_ claiming focus. Which would be a very confusing
        // situation for the programmer. Instead, we silently allow the popup to proceed, it will keep reappearing and the programming error will be more obvious to understand.
        if (g.OpenPopupStack[current_stack_size].PopupId == id
            && g.OpenPopupStack[current_stack_size].OpenFrameCount == g.FrameCount - 1)
        {
            g.OpenPopupStack[current_stack_size].OpenFrameCount = popup_ref.OpenFrameCount;
        } else {
            // Close child popups if any, then flag popup for open/reopen
            ClosePopupToLevel(current_stack_size, false);
            g.OpenPopupStack.push(popup_re0f32);
        }

        // When reopening a popup we first refocus its parent, otherwise if its parent is itself a popup it would get closed by ClosePopupsOverWindow().
        // This is equivalent to what ClosePopupToLevel() does.
        //if (g.OpenPopupStack[current_stack_size].PopupId == id)
        //    FocusWindow(parent_window);
    }
}

// When popups are stacked, clicking on a lower level popups puts focus back to it and close popups above it.
// This function closes any popups that are over 'ref_window'.
pub unsafe fn ClosePopupsOverWindow(
    ref_window: &mut ImguiWindow,
    restore_focus_to_window_under_popup: bool,
) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.OpenPopupStack.len() == 0) {
        return;
    }

    // Don't close our own child popup windows.
    let mut popup_count_to_keep: usize = 0;
    if (ref_window) {
        // Find the highest popup which is a descendant of the reference window (generally reference window = NavWindow)
        // for (; popup_count_to_keep < g.OpenPopupStack.len(); popup_count_to_keep++)
        while popup_count_to_keep < g.OpenPopupStack.len() {
            let popup = &g.OpenPopupStack[popup_count_to_keep];
            if (!popup.Window) {
                continue;
            }
            // IM_ASSERT((popup.window.Flags & ImGuiWindowFlags_Popup) != 0);
            if (popup.window.Flags & ImGuiWindowFlags_ChildWindow) {
                continue;
            }

            // Trim the stack unless the popup is a direct parent of the reference window (the reference window is often the NavWindow)
            // - With this stack of window, clicking/focusing Popup1 will close Popup2 and Popup3:
            //     Window -> Popup1 -> Popup2 -> Popup3
            // - Each popups may contain child windows, which is why we compare ->RootWindowDockTree!
            //     Window -> Popup1 -> Popup1_Child -> Popup2 -> Popup2_Child
            let mut ref_window_is_descendent_of_popup: bool = false;
            // for (let n: c_int = popup_count_to_keep; n < g.OpenPopupStack.len(); n++)
            for n in popup_count_to_keep..g.OpenPopupStack.len() {
                let mut popup_window: &mut ImguiWindow = g.OpenPopupStack[n].Window;
                if is_not_null(popup_window) {
                    //if (popup_window.RootWindowDockTree == ref_window.RootWindowDockTree) // FIXME-MERGE
                    if IsWindowWithinBeginStackOf(ref_window, popup_window) {
                        ref_window_is_descendent_of_popup = true;
                        break;
                    }
                }
            }
            if !ref_window_is_descendent_of_popup {
                break;
            }
            popup_count_to_keep += 1;
        }
    }
    if popup_count_to_keep < g.OpenPopupStack.len()
    // This test is not required but it allows to set a convenient breakpoint on the statement below
    {
        // IMGUI_DEBUG_LOG_POPUP("[popup] ClosePopupsOverWindow(\"{}\")\n", ref_window ? ref_window.Name : "<NULL>");
        ClosePopupToLevel(popup_count_to_keep, restore_focus_to_window_under_popup);
    }
}

pub unsafe fn ClosePopupsExceptModals() {
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let mut popup_count_to_keep: usize = 0;
    // for (popup_count_to_keep = g.OpenPopupStack.len(); popup_count_to_keep > 0; popup_count_to_keep--)
    for popup_count_to_keep in g.OpenPopupStack.len()..0 {
        let mut window: &mut ImguiWindow = g.OpenPopupStack[popup_count_to_keep - 1].Window;
        if !is_not_null(window) || flag_set(window.Flags, ImGuiWindowFlags_Modal) {
            break;
        }
    }
    if popup_count_to_keep < g.OpenPopupStack.len() {
        // This test is not required but it allows to set a convenient breakpoint on the statement below
        ClosePopupToLevel(popup_count_to_keep, true);
    }
}

pub unsafe fn ClosePopupToLevel(remaining: usize, restore_focus_to_window_under_popup: bool) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
                    // IMGUI_DEBUG_LOG_POPUP("[popup] ClosePopupToLevel({}), restore_focus_to_window_under_popup={}\n", remaining, restore_focus_to_window_under_popup);
                    // IM_ASSERT(remaining >= 0 && remaining < g.OpenPopupStack.Size);

    // Trim open popup stack
    let mut popup_window: &mut ImguiWindow = g.OpenPopupStack[remaining].Window;
    let mut popup_backup_nav_window: &mut ImguiWindow = g.OpenPopupStack[remaining].BackupNavWindow;
    g.OpenPopupStack
        .resize_with(remaining, ImGuiPopupData::default());

    if restore_focus_to_window_under_popup {
        let mut focus_window: &mut ImguiWindow = if is_not_null(popup_window)
            && flag_set(popup_window.Flags, ImGuiWindowFlags_ChildMenu)
        {
            popup_window.ParentWindow
        } else {
            popup_backup_nav_window
        };
        if is_not_null(focus_window) && focus_window.WasActive && is_not_null(popup_window) {
            // Fallback
            FocusTopMostWindowUnderOne(popup_window, null_mut());
        } else {
            if g.NavLayer == ImGuiNavLayer_Main && is_not_null(focus_window) {
                focus_window = NavRestoreLastChildNavWindow(focus_window);
            }
            FocusWindow(focus_window);
        }
    }
}

// Close the popup we have begin-ed into.
pub unsafe fn CloseCurrentPopup() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut popup_idx: usize = g.BeginPopupStack.len() - 1;
    if popup_idx < 0
        || popup_idx >= g.OpenPopupStack.len()
        || g.BeginPopupStack[popup_idx].PopupId != g.OpenPopupStack[popup_idx].PopupId
    {
        return;
    }

    // Closing a menu closes its top-most parent popup (unless a modal)
    while (popup_idx > 0) {
        let mut popup_window: &mut ImguiWindow = g.OpenPopupStack[popup_idx].Window;
        let mut parent_popup_window: &mut ImguiWindow = g.OpenPopupStack[popup_idx - 1].Window;
        let mut close_parent: bool = false;
        if (is_not_null(popup_window) && flag_set(popup_window.Flags, ImGuiWindowFlags_ChildMenu)) {
            if (is_not_null(parent_popup_window)
                && flag_clear(parent_popup_window.Flags, ImGuiWindowFlags_MenuBar))
            {
                close_parent = true;
            }
        }
        if !close_parent {
            break;
        }
        popup_idx -= 1;
    }
    IMGUI_DEBUG_LOG_POPUP(
        "[popup] CloseCurrentPopup {} -> {}\n",
        g.BeginPopupStack.len() - 1,
        popup_idx,
    );
    ClosePopupToLevel(popup_idx, true);

    // A common pattern is to close a popup when selecting a menu item/selectable that will open another window.
    // To improve this usage pattern, we avoid nav highlight for a single frame in the parent window.
    // Similarly, we could avoid mouse hover highlight in this window but it is less visually problematic.
    let mut window: &mut ImguiWindow = g.NavWindow;
    if is_not_null(window) {
        window.dc.NavHideHighlightOneFrame = true;
    }
}

// Attention! BeginPopup() adds default flags which BeginPopupEx()!
// BeginPopupEx: bool(id: ImguiHandle, flags: ImGuiWindowFlags)
pub unsafe fn BeginPopupEx(id: ImguiHandle, mut flags: ImGuiWindowFlags) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if !IsPopupOpen(id, ImGuiPopupFlags_None) {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }

    name: [c_char; 20];
    if flag_set(flags, ImGuiWindowFlags_ChildMenu) {
        // ImFormatString(name, name.len(), "##Menu_{}", g.BeginMenuCount);
    }
    // Recycle windows based on depth
    else {
        // ImFormatString(name, name.len(), "##Popup_{}", id);
    } // Not recycling, so we can close/open during the same frame

    flags |= ImGuiWindowFlags_Popup | ImGuiWindowFlags_NoDocking;
    let mut is_open: bool = Begin(g, name, null_mut());
    if (!is_open) {
        // NB: Begin can return false when the popup is completely clipped (e.g. zero size display)
        EndPopup(g);
    }

    return is_open;
}

// BeginPopup: bool(str_id: *const c_char, flags: ImGuiWindowFlags)
pub unsafe fn BeginPopup(str_id: &str, mut flags: ImGuiWindowFlags) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.OpenPopupStack.len() <= g.BeginPopupStack.len()
    // Early out for performance
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }
    flags |= ImGuiWindowFlags_AlwaysAutoResize
        | ImGuiWindowFlags_NoTitleBar
        | ImGuiWindowFlags_NoSavedSettings;
    let mut id: ImguiHandle = g.Currentwindow.GetID(str_id);
    return BeginPopupEx(id, flags);
}

// If 'p_open' is specified for a modal popup window, the popup will have a regular close button which will close the popup.
// Note that popup visibility status is owned by Dear ImGui (and manipulated with e.g. OpenPopup) so the actual value of *p_open is meaningless here.
// BeginPopupModal: bool(name: *const c_char,p_open: *mut bool, flags: ImGuiWindowFlags)
pub unsafe fn BeginPopupModal(
    name: *const c_char,
    p_open: *mut bool,
    mut flags: ImGuiWindowFlags,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    let mut id: ImguiHandle = window.id_from_str(name, null());
    if !IsPopupOpen(id, ImGuiPopupFlags_None) {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }

    // Center modal windows by default for increased visibility
    // (this won't really last as settings will kick in, and is mostly for backward compatibility. user may do the same themselves)
    // FIXME: Should test for (PosCond & window.SetWindowPosAllowFlags) with the upcoming window.
    if ((g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasPos) == 0) {
        let viewport: *const ImguiViewport = if window.WasActive {
            window.Viewport
        } else {
            GetMainViewport()
        }; // FIXME-VIEWPORT: What may be our reference viewport?
        SetNextWindowPos(,
                         &viewport.GetCenter(),
                         ImGuiCond_FirstUseEver,
                         &ImVec2::from_floats(0.5, 0.5),
        );
    }

    flags |= ImGuiWindowFlags_Popup
        | ImGuiWindowFlags_Modal
        | ImGuiWindowFlags_NoCollapse
        | ImGuiWindowFlags_NoDocking;
    let is_open: bool = Begin(g, name, p_open);
    if !is_open || (is_not_null(p_open) && !*p_open)
    // NB: is_open can be 'false' when the popup is completely clipped (e.g. zero size display)
    {
        EndPopup(g);
        if is_open {
            ClosePopupToLevel(g.BeginPopupStack.len(), true);
        }
        return false;
    }
    return is_open;
}

pub fn EndPopup(g: &mut ImguiContext) {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    // IM_ASSERT(window.Flags & ImGuiWindowFlags_Popup);  // Mismatched BeginPopup()/EndPopup() calls
    // IM_ASSERT(g.BeginPopupStack.Size > 0);

    // Make all menus and popups wrap around for now, may need to expose that policy (e.g. focus scope could include wrap/loop policy flags used by new move requests)
    if g.NavWindow == window {
        NavMoveRequestTryWrapping(window, ImGuiNavMoveFlags_LoopY);
    }

    // Child-popups don't need to be laid out
    // IM_ASSERT(g.WithinEndChild == false);
    if window.Flags & ImGuiWindowFlags_ChildWindow {
        g.WithinEndChild = true;
    }
    End();
    g.WithinEndChild = false;
}

// Helper to open a popup if mouse button is released over the item
// - This is essentially the same as BeginPopupContextItem() but without the trailing BeginPopup()
pub unsafe fn OpenPopupOnItemClick(str_id: &str, popup_flags: ImGuiPopupFlags) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    let mouse_button: c_int = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup) {
        let mut id: ImguiHandle = if str_id {
            window.id_from_str(str_id, null())
        } else {
            g.last_item_data.ID
        }; // If user hasn't passed an ID, we can use the LastItemID. Using LastItemID as a Popup ID won't conflict!
           // IM_ASSERT(id != 0);                                             // You cannot pass a NULL str_id if the last item has no identifier (e.g. a Text() item)
        OpenPopupEx(g, id, popup_flags);
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
//   The main difference being that this is tweaked to avoid computing the ID twice.
pub unsafe fn BeginPopupContextItem(str_id: *const c_char, popup_flags: ImGuiPopupFlags) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    if (window.skip_items) {
        return false;
    }
    let mut id: ImguiHandle = if str_id {
        window.id_from_str(str_id, null())
    } else {
        g.last_item_data.ID
    }; // If user hasn't passed an ID, we can use the LastItemID. Using LastItemID as a Popup ID won't conflict!
       // IM_ASSERT(id != 0);                                             // You cannot pass a NULL str_id if the last item has no identifier (e.g. a Text() item)
    let mouse_button: c_int = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup)) {
        OpenPopupEx(g, id, popup_flags);
    }
    return BeginPopupEx(
        id,
        ImGuiWindowFlags_AlwaysAutoResize
            | ImGuiWindowFlags_NoTitleBar
            | ImGuiWindowFlags_NoSavedSettings,
    );
}

pub unsafe fn BeginPopupContextWindow(
    mut str_id: *const c_char,
    popup_flags: ImGuiPopupFlags,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    if !str_id {
        str_id = str_to_const_c_char_ptr("window_context");
    }
    let mut id: ImguiHandle = window.id_from_str(str_id, null());
    let mouse_button: c_int = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if IsMouseReleased(mouse_button) && IsWindowHovered(g, ImGuiHoveredFlags_AllowWhenBlockedByPopup) {
        if flag_clear(popup_flags, ImGuiPopupFlags_NoOpenOverItems) || !IsAnyItemHovered() {
            OpenPopupEx(g, id, popup_flags);
        }
    }
    return BeginPopupEx(
        id,
        ImGuiWindowFlags_AlwaysAutoResize
            | ImGuiWindowFlags_NoTitleBar
            | ImGuiWindowFlags_NoSavedSettings,
    );
}

pub unsafe fn BeginPopupContextVoid(
    mut str_id: *const c_char,
    popup_flags: ImGuiPopupFlags,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    if (!str_id) {
        str_id = str_to_const_c_char_ptr("void_context");
    }
    let mut id: ImguiHandle = window.id_from_str(str_id, null());
    let mouse_button: c_int = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if IsMouseReleased(mouse_button) && !IsWindowHovered(g, ImGuiHoveredFlags_AnyWindow) {
        if GetTopMostPopupModal() == None {
            OpenPopupEx(g, id, popup_flags);
        }
    }
    return BeginPopupEx(
        id,
        ImGuiWindowFlags_AlwaysAutoResize
            | ImGuiWindowFlags_NoTitleBar
            | ImGuiWindowFlags_NoSavedSettings,
    );
}

// r_avoid = the rectangle to avoid (e.g. for tooltip it is a rectangle around the mouse cursor which we want to avoid. for popups it's a small point around the cursor.)
// r_outer = the visible area rectangle, minus safe area padding. If our popup size won't fit because of safe area padding we ignore it.
// (r_outer is usually equivalent to the viewport rectangle minus padding, but when multi-viewports are enabled and monitor
//  information are available, it may represent the entire platform monitor from the frame of reference of the current viewport.
//  this allows us to have tooltips/popups displayed out of the parent viewport.)
pub fn FindBestWindowPosForPopupEx(
    ref_pos: &ImVec2,
    size: &ImVec2,
    last_dir: &mut ImGuiDir,
    r_outer: &mut ImRect,
    r_avoid: &ImRect,
    policy: ImGuiPopupPositionPolicy,
) -> ImVec2 {
    let base_pos_clamped: ImVec2 = ImClamp(ref_pos.clone(), r_outer.min, r_outer.max - size);
    //GetForegroundDrawList().AddRect(r_avoid.Min, r_avoid.Max, IM_COL32(255,0,0,255));
    //GetForegroundDrawList().AddRect(r_outer.Min, r_outer.Max, IM_COL32(0,255,0,255));

    // Combo Box policy (we want a connecting edge)
    if policy == ImGuiPopupPositionPolicy_ComboBox {
        let dir_prefered_order: [ImGuiDir; 4] =
            [ImGuiDir_Down, ImGuiDir_Right, ImGuiDir_Left, ImGuiDir_Up];
        // for (let n: c_int = (*last_dir != ImGuiDir_None) ? -1 : 0; n < ImGuiDir_COUNT; n++)
        for n in if *last_idr != ImGuiDir_None { -1 } else { 0 }..ImGuiDir_COUNT {
            const dir: ImGuiDir = if n == -1 {
                *last_dir
            } else {
                dir_prefered_order[n]
            };
            if n != -1 && dir == *last_dir {
                // Already tried this direction?
                continue;
            }
            pos: ImVec2;
            if dir == ImGuiDir_Down {
                pos = ImVec2::from_floats(r_avoid.min.x, r_avoid.max.y);
            } // Below, Toward Right (default)
            if dir == ImGuiDir_Right {
                pos = ImVec2::from_floats(r_avoid.min.x, r_avoid.min.y - size.y);
            } // Above, Toward Right
            if dir == ImGuiDir_Left {
                pos = ImVec2::from_floats(r_avoid.max.x - size.x, r_avoid.max.y);
            } // Below, Toward Left
            if dir == ImGuiDir_Up {
                pos = ImVec2::from_floats(r_avoid.max.x - size.x, r_avoid.min.y - size.y);
            } // Above, Toward Left
            if !r_outer.Contains(ImRect(pos, pos + size)) {
                continue;
            }
            *last_dir = dir;
            return pos;
        }
    }

    // Tooltip and Default popup policy
    // (Always first try the direction we used on the last frame, if any)
    if policy == ImGuiPopupPositionPolicy_Tooltip || policy == ImGuiPopupPositionPolicy_Default {
        let dir_prefered_order: [ImGuiDir; 4] =
            [ImGuiDir_Right, ImGuiDir_Down, ImGuiDir_Up, ImGuiDir_Left];
        // for (let n: c_int = (*last_dir != ImGuiDir_None) ? -1 : 0; n < ImGuiDir_COUNT; n++)
        for n in if *last_idr != ImGuiDir_None { -1 } else { 0 }..ImGuiDir_COUNT {
            let dir: ImGuiDir = if n == -1 {
                *last_dir
            } else {
                dir_prefered_order[n]
            };
            if n != -1 && dir == *last_dir {
                // Already tried this direction?
                continue;
            }

            let avail_w: c_float = (if dir == ImGuiDir_Left {
                r_avoid.min.x
            } else {
                r_outer.max.x
            }) - (if dir == ImGuiDir_Right {
                r_avoid.max.x
            } else {
                r_outer.min.x
            });
            let avail_h: c_float = (if dir == ImGuiDir_Up {
                r_avoid.min.y
            } else {
                r_outer.max.y
            }) - (if dir == ImGuiDir_Down {
                r_avoid.max.y
            } else {
                r_outer.min.y
            });

            // If there not enough room on one axis, there's no point in positioning on a side on this axis (e.g. when not enough width, use a top/bottom position to maximize available width)
            if avail_w < size.x && (dir == ImGuiDir_Left || dir == ImGuiDir_Right) {
                continue;
            }
            if avail_h < size.y && (dir == ImGuiDir_Up || dir == ImGuiDir_Down) {
                continue;
            }

            let mut pos: ImVec2 = ImVec2::default();
            pos.x = if dir == ImGuiDir_Left {
                r_avoid.min.x - size.x
            } else {
                if dir == ImGuiDir_Right {
                    r_avoid.max.x
                } else {
                    base_pos_clamped.x
                }
            };
            pos.y = if dir == ImGuiDir_Up {
                r_avoid.min.y - size.y
            } else {
                if dir == ImGuiDir_Down {
                    r_avoid.max.y
                } else {
                    base_pos_clamped.y
                }
            };

            // Clamp top-left corner of popup
            pos.x = ImMax(pos.x, r_outer.min.x);
            pos.y = ImMax(pos.y, r_outer.min.y);

            *last_dir = dir;
            return pos;
        }
    }

    // Fallback when not enough room:
    *last_dir = ImGuiDir_None;

    // For tooltip we prefer avoiding the cursor at all cost even if it means that part of the tooltip won't be visible.
    if policy == ImGuiPopupPositionPolicy_Tooltip {
        return ref_pos + ImVec2::from_floats(2.0, 2.0);
    }

    // Otherwise try to keep within display
    let mut pos: ImVec2 = ref_pos.clone();
    pos.x = ImMax(ImMin(pos.x + size.x, r_outer.max.x) - size.x, r_outer.min.x);
    pos.y = ImMax(ImMin(pos.y + size.y, r_outer.max.y) - size.y, r_outer.min.y);
    return pos;
}

// Note that this is used for popups, which can overlap the non work-area of individual viewports.
pub fn GetPopupAllowedExtentRect(g: &mut ImguiContext, window: &mut ImguiWindow) -> ImRect {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut r_screen: ImRect = ImRect::default();
    if window.ViewportAllowPlatformMonitorExtend >= 0 {
        // Extent with be in the frame of reference of the given viewport (so Min is likely to be negative here)
        let monitor = g.PlatformIO.Monitors[window.ViewportAllowPlatformMonitorExtend];
        r_screen.min = monitor.WorkPos;
        r_screen.max = monitor.WorkPos + monitor.WorkSize;
    } else {
        // Use the full viewport area (not work area) for popups
        r_screen = window.Viewport.GetMainRect();
    }
    let padding: ImVec2 = g.style.DisplaySafeAreaPadding;
    r_screen.expand_from_vec(&ImVec2::from_floats(
        if r_screen.GetWidth() > padding.x * 2 {
            -padding.x
        } else {
            0.0
        },
        if r_screen.GetHeight() > padding.y * 2 {
            -padding.y
        } else {
            0.0
        },
    ));
    return r_screen;
}

pub unsafe fn FindBestWindowPosForPopup(window: &mut ImguiWindow) -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let mut r_outer: ImRect = GetPopupAllowedExtentRect(g, window);
    if window.Flags & ImGuiWindowFlags_ChildMenu {
        // Child menus typically request _any_ position within the parent menu item, and then we move the new menu outside the parent bounds.
        // This is how we end up with child menus appearing (most-commonly) on the right of the parent menu.
        let mut parent_window: &mut ImguiWindow = window.ParentWindow;
        let horizontal_overlap: c_float = g.style.ItemInnerSpacing.x; // We want some overlap to convey the relative depth of each menu (currently the amount of overlap is hard-coded to style.ItemSpacing.x).
        let mut r_avoid: ImRect = ImRect::default();
        if parent_window.dc.MenuBarAppending {
            r_avoid = ImRect(
                -f32::MAX,
                parent_window.ClipRect.Min.y,
                f32::MAX,
                parent_window.ClipRect.Max.y,
            );
        }
        // Avoid parent menu-bar. If we wanted multi-line menu-bar, we may instead want to have the calling window setup e.g. a NextWindowData.PosConstraintAvoidRect field
        else {
            r_avoid = ImRect(
                parent_window.position.x + horizontal_overlap,
                -f32::MAX,
                parent_window.position.x + parent_window.Size.x
                    - horizontal_overlap
                    - parent_window.scrollbarSizes.x,
                f32::MAX,
            );
        }
        return FindBestWindowPosForPopupEx(
            &window.position,
            &window.Size,
            &mut window.AutoPosLastDirection,
            &mut r_outer,
            &r_avoid,
            ImGuiPopupPositionPolicy_Default,
        );
    }
    if flag_set(window.Flags, ImGuiWindowFlags_Popup) {
        return FindBestWindowPosForPopupEx(
            &window.position,
            &window.Size,
            &mut window.AutoPosLastDirection,
            &mut r_outer,
            &ImRect::from_vec2(&window.position, &window.position),
            ImGuiPopupPositionPolicy_Default,
        ); // Ideally we'd disable r_avoid here
    }
    if flag_set(window.Flags, ImGuiWindowFlags_Tooltip) {
        // Position tooltip (always follows mouse)
        let sc: c_float = g.style.MouseCursorScale;
        let ref_pos: ImVec2 = NavCalcPreferredRefPos(g);
        let mut r_avoid: ImRect = ImRect::default();
        if !g.NavDisableHighlight
            && g.NavDisableMouseHover
            && flag_clear(g.IO.ConfigFlags, ImGuiConfigFlags_NavEnableSetMousePos)
        {
            r_avoid = ImRect(ref_pos.x - 16, ref_pos.y - 8, ref_pos.x + 16, ref_pos.y + 8);
        } else {
            r_avoid = ImRect(
                ref_pos.x - 16,
                ref_pos.y - 8,
                ref_pos.x + 24 * sc,
                ref_pos.y + 24 * sc,
            );
        } // FIXME: Hard-coded based on mouse cursor shape expectation. Exact dimension not very important.
        return FindBestWindowPosForPopupEx(
            &ref_pos,
            &window.Size,
            &mut window.AutoPosLastDirection,
            &mut r_outer,
            &r_avoid,
            ImGuiPopupPositionPolicy_Tooltip,
        );
    }
    // IM_ASSERT(0);
    return window.position;
}

pub fn CalcMaxPopupHeightFromItemCount(ctx: &mut ImguiContext, items_count: c_int) -> f32 {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    if items_count <= 0 {
        return f32::MAX;
    }
    return (g.FontSize + g.style.ItemSpacing.y) * items_count - g.style.ItemSpacing.y
        + (g.style.WindowPadding.y * 2);
}
