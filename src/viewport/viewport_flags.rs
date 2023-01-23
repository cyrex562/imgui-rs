use libc::c_int;

// typedef int ImguiViewportFlags;     // -> enum ImguiViewportFlags_   // Flags: for ImguiViewport
pub type ImguiViewportFlags = c_int;

// enum ImguiViewportFlags_
// {
    pub const ImguiViewportFlags_None: ImguiViewportFlags = 0;
    pub const ImguiViewportFlags_IsPlatformWindow: ImguiViewportFlags = 1 << 0;   // Represent a Platform Window
    pub const ImguiViewportFlags_IsPlatformMonitor: ImguiViewportFlags = 1 << 1;   // Represent a Platform Monitor (unused yet)
    pub const ImguiViewportFlags_OwnedByApp: ImguiViewportFlags = 1 << 2;   // Platform Window: is created/managed by the application (rather than a dear imgui backend)
    pub const ImguiViewportFlags_NoDecoration: ImguiViewportFlags = 1 << 3;   // Platform Window: Disable platform decorations: title bar; borders; etc. (generally set all windows; but if ImGuiConfigFlags_ViewportsDecoration is set we only set this on popups/tooltips)
    pub const ImguiViewportFlags_NoTaskBarIcon: ImguiViewportFlags = 1 << 4;   // Platform Window: Disable platform task bar icon (generally set on popups/tooltips; or all windows if ImGuiConfigFlags_ViewportsNoTaskBarIcon is set)
    pub const ImguiViewportFlags_NoFocusOnAppearing: ImguiViewportFlags = 1 << 5;   // Platform Window: Don't take focus when created.
    pub const ImguiViewportFlags_NoFocusOnClick: ImguiViewportFlags = 1 << 6;   // Platform Window: Don't take focus when clicked on.
    pub const ImguiViewportFlags_NoInputs: ImguiViewportFlags = 1 << 7;   // Platform Window: Make mouse pass through so we can drag this window while peaking behind it.
    pub const ImguiViewportFlags_NoRendererClear: ImguiViewportFlags = 1 << 8;   // Platform Window: Renderer doesn't need to clear the framebuffer ahead (because we will fill it entirely).
    pub const ImguiViewportFlags_TopMost: ImguiViewportFlags = 1 << 9;   // Platform Window: Display on top (for tooltips only).
    pub const ImguiViewportFlags_Minimized: ImguiViewportFlags = 1 << 10;  // Platform Window: Window is minimized; can skip render. When minimized we tend to avoid using the viewport pos/size for clipping window or testing if they are contained in the viewport.
    pub const ImguiViewportFlags_NoAutoMerge: ImguiViewportFlags = 1 << 11;  // Platform Window: Avoid merging this window into another host window. This can only be set via ImGuiWindowClass viewport flags override (because we need to now ahead if we are going to create a viewport in the first place!).
    pub const ImguiViewportFlags_CanHostOtherWindows: ImguiViewportFlags = 1 << 12;  // Main viewport: can host multiple imgui windows (secondary viewports are associated to a single window).
// };
