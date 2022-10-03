

// When a modal popup is open, newly created windows that want focus (i.e. are not popups and do not specify ImGuiWindowFlags_NoFocusOnAppearing)
// should be positioned behind that modal window, unless the window was created inside the modal begin-stack.
// In case of multiple stacked modals newly created window honors begin stack order and does not go below its own modal parent.
// - Window             // FindBlockingModal() returns Modal1
//   - Window           //                  .. returns Modal1
//   - Modal1           //                  .. returns Modal2
//      - Window        //                  .. returns Modal2
//          - Window    //                  .. returns Modal2
//          - Modal2    //                  .. returns Modal2
static ImGuiWindow* FindBlockingModal(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.OpenPopupStack.Size <= 0)
        return null_mut();

    // Find a modal that has common parent with specified window. Specified window should be positioned behind that modal.
    for (let i: c_int = g.OpenPopupStack.Size - 1; i >= 0; i--)
    {
        let mut popup_window: *mut ImGuiWindow =  g.OpenPopupStack.Data[i].Window;
        if (popup_window == null_mut() || !(popup_window.Flags & ImGuiWindowFlags_Modal))
            continue;
        if (!popup_window.Active && !popup_window.WasActive)      // Check WasActive, because this code may run before popup renders on current frame, also check Active to handle newly created windows.
            continue;
        if (IsWindowWithinBeginStackOf(window, popup_window))       // Window is rendered over last modal, no render order change needed.
            break;
        for (let mut parent: *mut ImGuiWindow =  popup_window.ParentWindowInBeginStack.RootWindow; parent != null_mut(); parent = parent.ParentWindowInBeginStack.RootWindow)
            if (IsWindowWithinBeginStackOf(window, parent))
                return popup_window;                                // Place window above its begin stack parent.
    }
    return null_mut();
}
