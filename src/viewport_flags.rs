use libc::c_int;

// typedef int ImGuiViewportFlags;     // -> enum ImGuiViewportFlags_   // Flags: for ImGuiViewport
pub type ImGuiViewportFlags = c_int;

// enum ImGuiViewportFlags_
// {
    pub const ImGuiViewportFlags_None: ImGuiViewportFlags = 0;
    pub const ImGuiViewportFlags_IsPlatformWindow: ImGuiViewportFlags = 1 << 0;   // Represent a Platform Window
    pub const ImGuiViewportFlags_IsPlatformMonitor: ImGuiViewportFlags = 1 << 1;   // Represent a Platform Monitor (unused yet)
    pub const ImGuiViewportFlags_OwnedByApp: ImGuiViewportFlags = 1 << 2;   // Platform Window: is created/managed by the application (rather than a dear imgui backend)
    pub const ImGuiViewportFlags_NoDecoration: ImGuiViewportFlags = 1 << 3;   // Platform Window: Disable platform decorations: title bar; borders; etc. (generally set all windows; but if ImGuiConfigFlags_ViewportsDecoration is set we only set this on popups/tooltips)
    pub const ImGuiViewportFlags_NoTaskBarIcon: ImGuiViewportFlags = 1 << 4;   // Platform Window: Disable platform task bar icon (generally set on popups/tooltips; or all windows if ImGuiConfigFlags_ViewportsNoTaskBarIcon is set)
    pub const ImGuiViewportFlags_NoFocusOnAppearing: ImGuiViewportFlags = 1 << 5;   // Platform Window: Don't take focus when created.
    pub const ImGuiViewportFlags_NoFocusOnClick: ImGuiViewportFlags = 1 << 6;   // Platform Window: Don't take focus when clicked on.
    pub const ImGuiViewportFlags_NoInputs: ImGuiViewportFlags = 1 << 7;   // Platform Window: Make mouse pass through so we can drag this window while peaking behind it.
    pub const ImGuiViewportFlags_NoRendererClear: ImGuiViewportFlags = 1 << 8;   // Platform Window: Renderer doesn't need to clear the framebuffer ahead (because we will fill it entirely).
    pub const ImGuiViewportFlags_TopMost: ImGuiViewportFlags = 1 << 9;   // Platform Window: Display on top (for tooltips only).
    pub const ImGuiViewportFlags_Minimized: ImGuiViewportFlags = 1 << 10;  // Platform Window: Window is minimized; can skip render. When minimized we tend to avoid using the viewport pos/size for clipping window or testing if they are contained in the viewport.
    pub const ImGuiViewportFlags_NoAutoMerge: ImGuiViewportFlags = 1 << 11;  // Platform Window: Avoid merging this window into another host window. This can only be set via ImGuiWindowClass viewport flags override (because we need to now ahead if we are going to create a viewport in the first place!).
    pub const ImGuiViewportFlags_CanHostOtherWindows: ImGuiViewportFlags = 1 << 12;  // Main viewport: can host multiple imgui windows (secondary viewports are associated to a single window).
// };