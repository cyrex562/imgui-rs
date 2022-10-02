// bool BeginChildEx(*const char name, ImGuiID id, const ImVec2& size_arg, bool border, ImGuiWindowFlags flags)
pub unsafe fn BeginChildEx(name: *const c_char, id: ImGuiID, size_arg: &ImVec2, border: bool, flags: ImGuiWindowFlags) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut parent_window: *mut ImGuiWindow = g.CurrentWindow;

    flags |= ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_NoDocking;
    flags |= (parent_window.Flags & ImGuiWindowFlags_NoMove);  // Inherit the NoMove flag

    // Size
    let content_avail: ImVec2 = GetContentRegionAvail();
    let size: ImVec2 = ImFloor(size_arg);
    let auto_fit_axises: c_int = (if size.x == 0f32 { (1 << ImGuiAxis_X) } else { 0x00 }) | (if size.y == 0f32 { (1 << ImGuiAxis_Y) } else { 0x00 });
    if size.x <= 0f32 {
        size.x = ImMax(content_avail.x + size.x, 4.00f32);
    }// Arbitrary minimum child size (0f32 causing too much issues)
    if size.y <= 0f32 {
        size.y = ImMax(content_avail.y + size.y, 4.00f32);
    }
    SetNextWindowSize(size);

    // Build up name. If you need to append to a same child from multiple location in the ID stack, use BeginChild(ImGuiID id) with a stable value.
    let temp_window_name: *const c_char;
    if name {
        ImFormatStringToTempBuffer(&temp_window_name, null_mut(), "%s/%s_%08X", parent_window.Name, name, id);
    } else {
        ImFormatStringToTempBuffer(&temp_window_name, null_mut(), "%s/%08X", parent_window.Name, id);
    }

    let backup_border_size: c_float = g.Style.ChildBorderSize;
    if (!border) {
        g.Style.ChildBorderSize = 0f32;
    }
    let mut ret: bool = Begin(temp_window_name, null_mut(), flags);
    g.Style.ChildBorderSize = backup_border_size;

    let mut child_window: *mut ImGuiWindow = g.CurrentWindow;
    child_window.ChildId = id;
    child_window.AutoFitChildAxises = auto_fit_axises;

    // Set the cursor to handle case where the user called SetNextWindowPos()+BeginChild() manually.
    // While this is not really documented/defined, it seems that the expected thing to do.
    if (child_window.BeginCount == 1) {
        parent_window.DC.CursorPos = child_window.Pos;
    }

    // Process navigation-in immediately so NavInit can run on first frame
    if g.NavActivateId == id && !(flags & ImGuiWindowFlags_NavFlattened) && (child_window.DC.NavLayersActiveMask != 0 || child_window.DC.NavHasScroll) {
        FocusWindow(child_window);
        NavInitWindow(child_window, false);
        SetActiveID(id + 1, child_window); // Steal ActiveId with another arbitrary id so that key-press won't activate child item
        g.ActiveIdSource = ImGuiInputSource_Nav;
    }
    return ret;
}

// bool BeginChild(*const char str_id, const ImVec2& size_arg, bool border, ImGuiWindowFlags extra_flags)
pub fn BeginChild(str_id: *const c_char, size_arg: &ImVec2, border: bool, extra_flags: ImGuiWindowFlags) -> bool {
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    return BeginChildEx(str_id, window.GetID(str_id), size_arg, border, extra_flags);
}

// bool BeginChild(ImGuiID id, const ImVec2& size_arg, bool border, ImGuiWindowFlags extra_flags)
pub fn BeginChild2(id: ImGuiID, size_arg: &ImVec2, border: bool, extra_flags: ImGuiWindowFlags) -> bool {
    // IM_ASSERT(id != 0);
    return BeginChildEx(null_mut(), id, size_arg, border, extra_flags);
}

// c_void EndChild()
pub unsafe fn EndChild() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // IM_ASSERT(g.WithinEndChild == false);
    // IM_ASSERT(window.Flags & ImGuiWindowFlags_ChildWindow);   // Mismatched BeginChild()/EndChild() calls

    g.WithinEndChild = true;
    if window.BeginCount > 1 {
        End();
    } else {
        let sz: ImVec2 = window.Size;
        if window.AutoFitChildAxises & (1 << ImGuiAxis_X) {// Arbitrary minimum zero-ish child size of 4.0f32 causes less trouble than a 0f32
            sz.x = ImMax(4.0f32, sz.x);
        }
        if window.AutoFitChildAxises & (1 << ImGuiAxis_Y) {
            sz.y = ImMax(4.0f32, sz.y);
        }
        End();

        let mut parent_window: *mut ImGuiWindow = g.CurrentWindow;
        let mut bb: ImRect = ImRect::new(parent_window.DC.CursorPos, parent_window.DC.CursorPos + sz);
        ItemSize(sz);
        if (window.DC.NavLayersActiveMask != 0 || window.DC.NavHasScroll) && !(window.Flags & ImGuiWindowFlags_NavFlattened) {
            ItemAdd(bb, window.ChildId);
            RenderNavHighlight(bb, window.ChildId);

            // When browsing a window that has no activable items (scroll only) we keep a highlight on the child (pass g.NavId to trick into always displaying)
            if window.DC.NavLayersActiveMask == 0 && window == g.NavWindow {
                RenderNavHighlight(ImRect(bb.Min - ImVec2(2, 2), bb.Max + ImVec2(2, 2)), g.NavId, ImGuiNavHighlightFlags_TypeThin);
            }
        } else {
            // Not navigable into
            ItemAdd(bb, 0);
        }
        if g.HoveredWindow == window {
            g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HoveredWindow;
        }
    }
    g.WithinEndChild = false;
    g.LogLinePosY = f32::MIN; // To enforce a carriage return
}

// Helper to create a child window / scrolling region that looks like a normal widget frame.
// bool BeginChildFrame(ImGuiID id, const ImVec2& size, ImGuiWindowFlags extra_flags)
pub unsafe fn BeginChildFrame(id: ImGuiID, size: &ImVec2, extra_flags: ImGuiWindowFlagss) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = g.Style;
    PushStyleColor(ImGuiCol_ChildBg, style.Colors[ImGuiCol_FrameBg]);
    PushStyleVar(ImGuiStyleVar_ChildRounding, style.FrameRounding);
    PushStyleVar(ImGuiStyleVar_ChildBorderSize, style.FrameBorderSize);
    PushStyleVar(ImGuiStyleVar_WindowPadding, style.FramePadding);
    let mut ret: bool = BeginChild(id, size, true, ImGuiWindowFlags_NoMove | ImGuiWindowFlags_AlwaysUseWindowPadding | extra_flags);
    PopStyleVar(3);
    PopStyleColor();
    return ret;
}

// c_void EndChildFrame()
pub fn EndChildFrame() {
    EndChild();
}
