
// Supported flags: ImGuiPopupFlags_AnyPopupId, ImGuiPopupFlags_AnyPopupLevel
bool IsPopupOpen(id: ImGuiID, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (popup_flags & ImGuiPopupFlags_AnyPopupId)
    {
        // Return true if any popup is open at the current BeginPopup() level of the popup stack
        // This may be used to e.g. test for another popups already opened to handle popups priorities at the same level.
        // IM_ASSERT(id == 0);
        if (popup_flags & ImGuiPopupFlags_AnyPopupLevel)
            return g.OpenPopupStack.Size > 0;
        else
            return g.OpenPopupStack.Size > g.BeginPopupStack.Size;
    }
    else
    {
        if (popup_flags & ImGuiPopupFlags_AnyPopupLevel)
        {
            // Return true if the popup is open anywhere in the popup stack
            for (let n: c_int = 0; n < g.OpenPopupStack.Size; n++)
                if (g.OpenPopupStack[n].PopupId == id)
                    return true;
            return false;
        }
        else
        {
            // Return true if the popup is open at the current BeginPopup() level of the popup stack (this is the most-common query)
            return g.OpenPopupStack.Size > g.BeginPopupStack.Size && g.OpenPopupStack[g.BeginPopupStack.Size].PopupId == id;
        }
    }
}

bool IsPopupOpen(*const char str_id, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut id: ImGuiID =  (popup_flags & ImGuiPopupFlags_AnyPopupId) ? 0 : g.Currentwindow.GetID(str_id);
    if ((popup_flags & ImGuiPopupFlags_AnyPopupLevel) && id != 0)
        // IM_ASSERT(0 && "Cannot use IsPopupOpen() with a string id and ImGuiPopupFlags_AnyPopupLevel."); // But non-string version is legal and used internally
    return IsPopupOpen(id, popup_flags);
}

ImGuiWindow* GetTopMostPopupModal()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (let n: c_int = g.OpenPopupStack.Size - 1; n >= 0; n--)
        if (let mut popup: *mut ImGuiWindow =  g.OpenPopupStack.Data[n].Window)
            if (popup.Flags & ImGuiWindowFlags_Modal)
                return popup;
    return null_mut();
}

ImGuiWindow* GetTopMostAndVisiblePopupModal()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (let n: c_int = g.OpenPopupStack.Size - 1; n >= 0; n--)
        if (let mut popup: *mut ImGuiWindow =  g.OpenPopupStack.Data[n].Window)
            if ((popup.Flags & ImGuiWindowFlags_Modal) && IsWindowActiveAndVisible(popup))
                return popup;
    return null_mut();
}

c_void OpenPopup(*const char str_id, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut id: ImGuiID =  g.Currentwindow.GetID(str_id);
    IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopup(\"%s\" -> 0x%08X\n", str_id, id);
    OpenPopupEx(id, popup_flags);
}

c_void OpenPopup(id: ImGuiID, ImGuiPopupFlags popup_flags)
{
    OpenPopupEx(id, popup_flags);
}

// Mark popup as open (toggle toward open state).
// Popups are closed when user click outside, or activate a pressable item, or CloseCurrentPopup() is called within a BeginPopup()/EndPopup() block.
// Popup identifiers are relative to the current ID-stack (so OpenPopup and BeginPopup needs to be at the same level).
// One open popup per level of the popup hierarchy (NB: when assigning we reset the Window member of ImGuiPopupRef to NULL)
c_void OpenPopupEx(id: ImGuiID, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut parent_window: *mut ImGuiWindow =  g.CurrentWindow;
    let current_stack_size: c_int = g.BeginPopupStack.Size;

    if (popup_flags & ImGuiPopupFlags_NoOpenOverExistingPopup)
        if (IsPopupOpen(0u, ImGuiPopupFlags_AnyPopupId))
            return;

    ImGuiPopupData popup_ref; // Tagged as new ref as Window will be set back to NULL if we write this into OpenPopupStack.
    popup_ref.PopupId = id;
    popup_ref.Window= null_mut();
    popup_ref.BackupNavWindow = g.NavWindow;            // When popup closes focus may be restored to NavWindow (depend on window type).
    popup_ref.OpenFrameCount = g.FrameCount;
    popup_ref.OpenParentId = parent_window.IDStack.last().unwrap();
    popup_ref.OpenPopupPos = NavCalcPreferredRefPos();
    popup_ref.OpenMousePos = IsMousePosValid(&g.IO.MousePos) ? g.IO.MousePos : popup_ref.OpenPopupPos;

    IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopupEx(0x%08X)\n", id);
    if (g.OpenPopupStack.Size < current_stack_size + 1)
    {
        g.OpenPopupStack.push(popup_ref);
    }
    else
    {
        // Gently handle the user mistakenly calling OpenPopup() every frame. It is a programming mistake! However, if we were to run the regular code path, the ui
        // would become completely unusable because the popup will always be in hidden-while-calculating-size state _while_ claiming focus. Which would be a very confusing
        // situation for the programmer. Instead, we silently allow the popup to proceed, it will keep reappearing and the programming error will be more obvious to understand.
        if (g.OpenPopupStack[current_stack_size].PopupId == id && g.OpenPopupStack[current_stack_size].OpenFrameCount == g.FrameCount - 1)
        {
            g.OpenPopupStack[current_stack_size].OpenFrameCount = popup_ref.OpenFrameCount;
        }
        else
        {
            // Close child popups if any, then flag popup for open/reopen
            ClosePopupToLevel(current_stack_size, false);
            g.OpenPopupStack.push(popup_ref);
        }

        // When reopening a popup we first refocus its parent, otherwise if its parent is itself a popup it would get closed by ClosePopupsOverWindow().
        // This is equivalent to what ClosePopupToLevel() does.
        //if (g.OpenPopupStack[current_stack_size].PopupId == id)
        //    FocusWindow(parent_window);
    }
}

// When popups are stacked, clicking on a lower level popups puts focus back to it and close popups above it.
// This function closes any popups that are over 'ref_window'.
c_void ClosePopupsOverWindow(ref_window: *mut ImGuiWindow, restore_focus_to_window_under_popup: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.OpenPopupStack.Size == 0)
        return;

    // Don't close our own child popup windows.
    let popup_count_to_keep: c_int = 0;
    if (ref_window)
    {
        // Find the highest popup which is a descendant of the reference window (generally reference window = NavWindow)
        for (; popup_count_to_keep < g.OpenPopupStack.Size; popup_count_to_keep++)
        {
            ImGuiPopupData& popup = g.OpenPopupStack[popup_count_to_keep];
            if (!popup.Window)
                continue;
            // IM_ASSERT((popup.window.Flags & ImGuiWindowFlags_Popup) != 0);
            if (popup.window.Flags & ImGuiWindowFlags_ChildWindow)
                continue;

            // Trim the stack unless the popup is a direct parent of the reference window (the reference window is often the NavWindow)
            // - With this stack of window, clicking/focusing Popup1 will close Popup2 and Popup3:
            //     Window -> Popup1 -> Popup2 -> Popup3
            // - Each popups may contain child windows, which is why we compare .RootWindowDockTree!
            //     Window -> Popup1 -> Popup1_Child -> Popup2 -> Popup2_Child
            let mut ref_window_is_descendent_of_popup: bool =  false;
            for (let n: c_int = popup_count_to_keep; n < g.OpenPopupStack.Size; n++)
                if (let mut popup_window: *mut ImGuiWindow =  g.OpenPopupStack[n].Window)
                    //if (popup_window.RootWindowDockTree == ref_window.RootWindowDockTree) // FIXME-MERGE
                    if (IsWindowWithinBeginStackOf(ref_window, popup_window))
                    {
                        ref_window_is_descendent_of_popup = true;
                        break;
                    }
            if (!ref_window_is_descendent_of_popup)
                break;
        }
    }
    if (popup_count_to_keep < g.OpenPopupStack.Size) // This test is not required but it allows to set a convenient breakpoint on the statement below
    {
        IMGUI_DEBUG_LOG_POPUP("[popup] ClosePopupsOverWindow(\"%s\")\n", ref_window ? ref_window.Name : "<NULL>");
        ClosePopupToLevel(popup_count_to_keep, restore_focus_to_window_under_popup);
    }
}

c_void ClosePopupsExceptModals()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let mut popup_count_to_keep: c_int = 0;
    for (popup_count_to_keep = g.OpenPopupStack.Size; popup_count_to_keep > 0; popup_count_to_keep--)
    {
        let mut window: *mut ImGuiWindow =  g.OpenPopupStack[popup_count_to_keep - 1].Window;
        if (!window || window.Flags & ImGuiWindowFlags_Modal)
            break;
    }
    if (popup_count_to_keep < g.OpenPopupStack.Size) // This test is not required but it allows to set a convenient breakpoint on the statement below
        ClosePopupToLevel(popup_count_to_keep, true);
}

c_void ClosePopupToLevel(c_int remaining, restore_focus_to_window_under_popup: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    IMGUI_DEBUG_LOG_POPUP("[popup] ClosePopupToLevel(%d), restore_focus_to_window_under_popup=%d\n", remaining, restore_focus_to_window_under_popup);
    // IM_ASSERT(remaining >= 0 && remaining < g.OpenPopupStack.Size);

    // Trim open popup stack
    let mut popup_window: *mut ImGuiWindow =  g.OpenPopupStack[remaining].Window;
    let mut popup_backup_nav_window: *mut ImGuiWindow =  g.OpenPopupStack[remaining].BackupNavWindow;
    g.OpenPopupStack.resize(remaining);

    if (restore_focus_to_window_under_popup)
    {
        let mut focus_window: *mut ImGuiWindow =  (popup_window && popup_window.Flags & ImGuiWindowFlags_ChildMenu) ? popup_window.ParentWindow : popup_backup_nav_window;
        if (focus_window && !focus_window.WasActive && popup_window)
        {
            // Fallback
            FocusTopMostWindowUnderOne(popup_window, null_mut());
        }
        else
        {
            if (g.NavLayer == ImGuiNavLayer_Main && focus_window)
                focus_window = NavRestoreLastChildNavWindow(focus_window);
            FocusWindow(focus_window);
        }
    }
}

// Close the popup we have begin-ed into.
c_void CloseCurrentPopup()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let popup_idx: c_int = g.BeginPopupStack.Size - 1;
    if (popup_idx < 0 || popup_idx >= g.OpenPopupStack.Size || g.BeginPopupStack[popup_idx].PopupId != g.OpenPopupStack[popup_idx].PopupId)
        return;

    // Closing a menu closes its top-most parent popup (unless a modal)
    while (popup_idx > 0)
    {
        let mut popup_window: *mut ImGuiWindow =  g.OpenPopupStack[popup_idx].Window;
        let mut parent_popup_window: *mut ImGuiWindow =  g.OpenPopupStack[popup_idx - 1].Window;
        let mut close_parent: bool =  false;
        if (popup_window && (popup_window.Flags & ImGuiWindowFlags_ChildMenu))
            if (parent_popup_window && !(parent_popup_window.Flags & ImGuiWindowFlags_MenuBar))
                close_parent = true;
        if (!close_parent)
            break;
        popup_idx-= 1;
    }
    IMGUI_DEBUG_LOG_POPUP("[popup] CloseCurrentPopup %d -> %d\n", g.BeginPopupStack.Size - 1, popup_idx);
    ClosePopupToLevel(popup_idx, true);

    // A common pattern is to close a popup when selecting a menu item/selectable that will open another window.
    // To improve this usage pattern, we avoid nav highlight for a single frame in the parent window.
    // Similarly, we could avoid mouse hover highlight in this window but it is less visually problematic.
    if (let mut window: *mut ImGuiWindow =  g.NavWindow)
        window.DC.NavHideHighlightOneFrame = true;
}

// Attention! BeginPopup() adds default flags which BeginPopupEx()!
bool BeginPopupEx(id: ImGuiID, ImGuiWindowFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!IsPopupOpen(id, ImGuiPopupFlags_None))
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }

    name: [c_char;20];
    if (flags & ImGuiWindowFlags_ChildMenu)
        ImFormatString(name, IM_ARRAYSIZE(name), "##Menu_%02d", g.BeginMenuCount); // Recycle windows based on depth
    else
        ImFormatString(name, IM_ARRAYSIZE(name), "##Popup_%08x", id); // Not recycling, so we can close/open during the same frame

    flags |= ImGuiWindowFlags_Popup | ImGuiWindowFlags_NoDocking;
    let mut is_open: bool =  Begin(name, null_mut(), flags);
    if (!is_open) // NB: Begin can return false when the popup is completely clipped (e.g. zero size display)
        EndPopup();

    return is_open;
}

bool BeginPopup(*const char str_id, ImGuiWindowFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.OpenPopupStack.Size <= g.BeginPopupStack.Size) // Early out for performance
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }
    flags |= ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings;
    let mut id: ImGuiID =  g.Currentwindow.GetID(str_id);
    return BeginPopupEx(id, flags);
}

// If 'p_open' is specified for a modal popup window, the popup will have a regular close button which will close the popup.
// Note that popup visibility status is owned by Dear ImGui (and manipulated with e.g. OpenPopup) so the actual value of *p_open is meaningless here.
bool BeginPopupModal(*const char name, bool* p_open, ImGuiWindowFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut id: ImGuiID =  window.GetID(name);
    if (!IsPopupOpen(id, ImGuiPopupFlags_None))
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }

    // Center modal windows by default for increased visibility
    // (this won't really last as settings will kick in, and is mostly for backward compatibility. user may do the same themselves)
    // FIXME: Should test for (PosCond & window.SetWindowPosAllowFlags) with the upcoming window.
    if ((g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasPos) == 0)
    {
        let viewport: *const ImGuiViewport = window.WasActive ? window.Viewport : GetMainViewport(); // FIXME-VIEWPORT: What may be our reference viewport?
        SetNextWindowPos(viewport.GetCenter(), ImGuiCond_FirstUseEver, ImVec2::new2(0.5f32, 0.5f32));
    }

    flags |= ImGuiWindowFlags_Popup | ImGuiWindowFlags_Modal | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoDocking;
    let is_open: bool = Begin(name, p_open, flags);
    if (!is_open || (p_open && !*p_open)) // NB: is_open can be 'false' when the popup is completely clipped (e.g. zero size display)
    {
        EndPopup();
        if (is_open)
            ClosePopupToLevel(g.BeginPopupStack.Size, true);
        return false;
    }
    return is_open;
}

c_void EndPopup()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(window.Flags & ImGuiWindowFlags_Popup);  // Mismatched BeginPopup()/EndPopup() calls
    // IM_ASSERT(g.BeginPopupStack.Size > 0);

    // Make all menus and popups wrap around for now, may need to expose that policy (e.g. focus scope could include wrap/loop policy flags used by new move requests)
    if (g.NavWindow == window)
        NavMoveRequestTryWrapping(window, ImGuiNavMoveFlags_LoopY);

    // Child-popups don't need to be laid out
    // IM_ASSERT(g.WithinEndChild == false);
    if (window.Flags & ImGuiWindowFlags_ChildWindow)
        g.WithinEndChild = true;
    End();
    g.WithinEndChild = false;
}

// Helper to open a popup if mouse button is released over the item
// - This is essentially the same as BeginPopupContextItem() but without the trailing BeginPopup()
c_void OpenPopupOnItemClick(*const char str_id, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mouse_button: c_int = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
    {
        let mut id: ImGuiID =  str_id ? window.GetID(str_id) : g.LastItemData.ID;    // If user hasn't passed an ID, we can use the LastItemID. Using LastItemID as a Popup ID won't conflict!
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
//   The main difference being that this is tweaked to avoid computing the ID twice.
bool BeginPopupContextItem(*const char str_id, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (window.SkipItems)
        return false;
    let mut id: ImGuiID =  str_id ? window.GetID(str_id) : g.LastItemData.ID;    // If user hasn't passed an ID, we can use the LastItemID. Using LastItemID as a Popup ID won't conflict!
    // IM_ASSERT(id != 0);                                             // You cannot pass a NULL str_id if the last item has no identifier (e.g. a Text() item)
    let mouse_button: c_int = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
        OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings);
}

bool BeginPopupContextWindow(*const char str_id, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (!str_id)
        str_id = "window_context";
    let mut id: ImGuiID =  window.GetID(str_id);
    let mouse_button: c_int = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsWindowHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
        if (!(popup_flags & ImGuiPopupFlags_NoOpenOverItems) || !IsAnyItemHovered())
            OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings);
}

bool BeginPopupContextVoid(*const char str_id, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (!str_id)
        str_id = "void_context";
    let mut id: ImGuiID =  window.GetID(str_id);
    let mouse_button: c_int = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && !IsWindowHovered(ImGuiHoveredFlags_AnyWindow))
        if (GetTopMostPopupModal() == null_mut())
            OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings);
}

// r_avoid = the rectangle to avoid (e.g. for tooltip it is a rectangle around the mouse cursor which we want to avoid. for popups it's a small point around the cursor.)
// r_outer = the visible area rectangle, minus safe area padding. If our popup size won't fit because of safe area padding we ignore it.
// (r_outer is usually equivalent to the viewport rectangle minus padding, but when multi-viewports are enabled and monitor
//  information are available, it may represent the entire platform monitor from the frame of reference of the current viewport.
//  this allows us to have tooltips/popups displayed out of the parent viewport.)
ImVec2 FindBestWindowPosForPopupEx(const ImVec2& ref_pos, const ImVec2& size, ImGuiDir* last_dir, const ImRect& r_outer, const ImRect& r_avoid, ImGuiPopupPositionPolicy policy)
{
    let base_pos_clamped: ImVec2 = ImClamp(ref_pos, r_outer.Min, r_outer.Max - size);
    //GetForegroundDrawList().AddRect(r_avoid.Min, r_avoid.Max, IM_COL32(255,0,0,255));
    //GetForegroundDrawList().AddRect(r_outer.Min, r_outer.Max, IM_COL32(0,255,0,255));

    // Combo Box policy (we want a connecting edge)
    if (policy == ImGuiPopupPositionPolicy_ComboBox)
    {
        const ImGuiDir dir_prefered_order[ImGuiDir_COUNT] = { ImGuiDir_Down, ImGuiDir_Right, ImGuiDir_Left, ImGuiDir_Up };
        for (let n: c_int = (*last_dir != ImGuiDir_None) ? -1 : 0; n < ImGuiDir_COUNT; n++)
        {
            const ImGuiDir dir = (n == -1) ? *last_dir : dir_prefered_order[n];
            if (n != -1 && dir == *last_dir) // Already tried this direction?
                continue;
            let mut pos = ImVec2::default();
            if (dir == ImGuiDir_Down)  pos = ImVec2::new(r_avoid.Min.x, r_avoid.Max.y);          // Below, Toward Right (default)
            if (dir == ImGuiDir_Right) pos = ImVec2::new(r_avoid.Min.x, r_avoid.Min.y - size.y); // Above, Toward Right
            if (dir == ImGuiDir_Left)  pos = ImVec2::new(r_avoid.Max.x - size.x, r_avoid.Max.y); // Below, Toward Left
            if (dir == ImGuiDir_Up)    pos = ImVec2::new(r_avoid.Max.x - size.x, r_avoid.Min.y - size.y); // Above, Toward Left
            if (!r_outer.Contains(ImRect::new(pos, pos + size)))
                continue;
            *last_dir = dir;
            return pos;
        }
    }

    // Tooltip and Default popup policy
    // (Always first try the direction we used on the last frame, if any)
    if (policy == ImGuiPopupPositionPolicy_Tooltip || policy == ImGuiPopupPositionPolicy_Default)
    {
        const ImGuiDir dir_prefered_order[ImGuiDir_COUNT] = { ImGuiDir_Right, ImGuiDir_Down, ImGuiDir_Up, ImGuiDir_Left };
        for (let n: c_int = (*last_dir != ImGuiDir_None) ? -1 : 0; n < ImGuiDir_COUNT; n++)
        {
            const ImGuiDir dir = (n == -1) ? *last_dir : dir_prefered_order[n];
            if (n != -1 && dir == *last_dir) // Already tried this direction?
                continue;

            let avail_w: c_float =  (dir == ImGuiDir_Left ? r_avoid.Min.x : r_outer.Max.x) - (dir == ImGuiDir_Right ? r_avoid.Max.x : r_outer.Min.x);
            let avail_h: c_float =  (dir == ImGuiDir_Up ? r_avoid.Min.y : r_outer.Max.y) - (dir == ImGuiDir_Down ? r_avoid.Max.y : r_outer.Min.y);

            // If there not enough room on one axis, there's no point in positioning on a side on this axis (e.g. when not enough width, use a top/bottom position to maximize available width)
            if (avail_w < size.x && (dir == ImGuiDir_Left || dir == ImGuiDir_Right))
                continue;
            if (avail_h < size.y && (dir == ImGuiDir_Up || dir == ImGuiDir_Down))
                continue;

            let mut pos = ImVec2::default();
            pos.x = (dir == ImGuiDir_Left) ? r_avoid.Min.x - size.x : (dir == ImGuiDir_Right) ? r_avoid.Max.x : base_pos_clamped.x;
            pos.y = (dir == ImGuiDir_Up) ? r_avoid.Min.y - size.y : (dir == ImGuiDir_Down) ? r_avoid.Max.y : base_pos_clamped.y;

            // Clamp top-left corner of popup
            pos.x = ImMax(pos.x, r_outer.Min.x);
            pos.y = ImMax(pos.y, r_outer.Min.y);

            *last_dir = dir;
            return pos;
        }
    }

    // Fallback when not enough room:
    *last_dir = ImGuiDir_None;

    // For tooltip we prefer avoiding the cursor at all cost even if it means that part of the tooltip won't be visible.
    if (policy == ImGuiPopupPositionPolicy_Tooltip)
        return ref_pos + ImVec2::new2(2, 2);

    // Otherwise try to keep within display
    let pos: ImVec2 = ref_pos;
    pos.x = ImMax(ImMin(pos.x + size.x, r_outer.Max.x) - size.x, r_outer.Min.x);
    pos.y = ImMax(ImMin(pos.y + size.y, r_outer.Max.y) - size.y, r_outer.Min.y);
    return pos;
}

// Note that this is used for popups, which can overlap the non work-area of individual viewports.
ImRect GetPopupAllowedExtentRect(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImRect r_screen;
    if (window.ViewportAllowPlatformMonitorExtend >= 0)
    {
        // Extent with be in the frame of reference of the given viewport (so Min is likely to be negative here)
        const ImGuiPlatformMonitor& monitor = g.PlatformIO.Monitors[window.ViewportAllowPlatformMonitorExtend];
        r_screen.Min = monitor.WorkPos;
        r_screen.Max = monitor.WorkPos + monitor.WorkSize;
    }
    else
    {
        // Use the full viewport area (not work area) for popups
        r_screen = window.Viewport.GetMainRect();
    }
    let padding: ImVec2 = g.Style.DisplaySafeAreaPadding;
    r_screen.Expand(ImVec2::new((r_screen.GetWidth() > padding.x * 2) ? -padding.x : 0f32, (r_screen.GetHeight() > padding.y * 2) ? -padding.y : 0f32));
    return r_screen;
}

ImVec2 FindBestWindowPosForPopup(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let r_outer: ImRect =  GetPopupAllowedExtentRect(window);
    if (window.Flags & ImGuiWindowFlags_ChildMenu)
    {
        // Child menus typically request _any_ position within the parent menu item, and then we move the new menu outside the parent bounds.
        // This is how we end up with child menus appearing (most-commonly) on the right of the parent menu.
        let mut parent_window: *mut ImGuiWindow =  window.ParentWindow;
        let horizontal_overlap: c_float =  g.Style.ItemInnerSpacing.x; // We want some overlap to convey the relative depth of each menu (currently the amount of overlap is hard-coded to style.ItemSpacing.x).
        ImRect r_avoid;
        if (parent_window.DC.MenuBarAppending)
            r_avoid = ImRect::new(-f32::MAX, parent_window.ClipRect.Min.y, f32::MAX, parent_window.ClipRect.Max.y); // Avoid parent menu-bar. If we wanted multi-line menu-bar, we may instead want to have the calling window setup e.g. a NextWindowData.PosConstraintAvoidRect field
        else
            r_avoid = ImRect::new(parent_window.Pos.x + horizontal_overlap, -f32::MAX, parent_window.Pos.x + parent_window.Size.x - horizontal_overlap - parent_window.ScrollbarSizes.x, f32::MAX);
        return FindBestWindowPosForPopupEx(window.Pos, window.Size, &window.AutoPosLastDirection, r_outer, r_avoid, ImGuiPopupPositionPolicy_Default);
    }
    if (window.Flags & ImGuiWindowFlags_Popup)
    {
        return FindBestWindowPosForPopupEx(window.Pos, window.Size, &window.AutoPosLastDirection, r_outer, ImRect::new(window.Pos, window.Pos), ImGuiPopupPositionPolicy_Default); // Ideally we'd disable r_avoid here
    }
    if (window.Flags & ImGuiWindowFlags_Tooltip)
    {
        // Position tooltip (always follows mouse)
        let sc: c_float =  g.Style.MouseCursorScale;
        let ref_pos: ImVec2 = NavCalcPreferredRefPos();
        ImRect r_avoid;
        if (!g.NavDisableHighlight && g.NavDisableMouseHover && !(g.IO.ConfigFlags & ImGuiConfigFlags_NavEnableSetMousePos))
            r_avoid = ImRect::new(ref_pos.x - 16, ref_pos.y - 8, ref_pos.x + 16, ref_pos.y + 8);
        else
            r_avoid = ImRect::new(ref_pos.x - 16, ref_pos.y - 8, ref_pos.x + 24 * sc, ref_pos.y + 24 * sc); // FIXME: Hard-coded based on mouse cursor shape expectation. Exact dimension not very important.
        return FindBestWindowPosForPopupEx(ref_pos, window.Size, &window.AutoPosLastDirection, r_outer, r_avoid, ImGuiPopupPositionPolicy_Tooltip);
    }
    // IM_ASSERT(0);
    return window.Pos;
}
