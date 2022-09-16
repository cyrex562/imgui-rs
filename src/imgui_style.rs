#![allow(non_snake_case)]

impl ImGuiStyle {
    // ImGuiStyle::ImGuiStyle()
    pub fn new() -> Self {
        let mut out = Self { ..Default::default() };
        out.Alpha = 1f32;             // Global alpha applies to everything in Dear ImGui.
        out.DisabledAlpha = 0.60f32;            // Additional alpha multiplier applied by BeginDisabled(). Multiply over current value of Alpha.
        out.WindowPadding = ImVec2(8, 8);      // Padding within a window
        out.WindowRounding = 0f32;             // Radius of window corners rounding. Set to 0f32 to have rectangular windows. Large values tend to lead to variety of artifacts and are not recommended.
        out.WindowBorderSize = 1f32;             // Thickness of border around windows. Generally set to 0f32 or 1.0f. Other values not well tested.
        out.WindowMinSize = ImVec2(32, 32);    // Minimum window size
        out.WindowTitleAlign = ImVec2(0f32, 0.5f32);// Alignment for title bar text
        out.WindowMenuButtonPosition = ImGuiDir_Left;    // Position of the collapsing/docking button in the title bar (left/right). Defaults to ImGuiDir_Left.
        out.ChildRounding = 0f32;             // Radius of child window corners rounding. Set to 0f32 to have rectangular child windows
        out.ChildBorderSize = 1f32;             // Thickness of border around child windows. Generally set to 0f32 or 1.0f. Other values not well tested.
        out.PopupRounding = 0f32;             // Radius of popup window corners rounding. Set to 0f32 to have rectangular child windows
        out.PopupBorderSize = 1f32;             // Thickness of border around popup or tooltip windows. Generally set to 0f32 or 1.0f. Other values not well tested.
        out.FramePadding = ImVec2(4, 3);      // Padding within a framed rectangle (used by most widgets)
        out.FrameRounding = 0f32;             // Radius of frame corners rounding. Set to 0f32 to have rectangular frames (used by most widgets).
        out.FrameBorderSize = 0f32;             // Thickness of border around frames. Generally set to 0f32 or 1.0f. Other values not well tested.
        out.ItemSpacing = ImVec2(8, 4);      // Horizontal and vertical spacing between widgets/lines
        out.ItemInnerSpacing = ImVec2(4, 4);      // Horizontal and vertical spacing between within elements of a composed widget (e.g. a slider and its label)
        out.CellPadding = ImVec2(4, 2);      // Padding within a table cell
        out.TouchExtraPadding = ImVec2(0, 0);      // Expand reactive bounding box for touch-based system where touch position is not accurate enough. Unfortunately we don't sort widgets so priority on overlap will always be given to the first widget. So don't grow this too much!
        out.IndentSpacing = 21f32;            // Horizontal spacing when e.g. entering a tree node. Generally == (FontSize + FramePadding.x*2).
        out.ColumnsMinSpacing = 6f32;             // Minimum horizontal spacing between two columns. Preferably > (FramePadding.x + 1).
        out.crollbarSize = 14.0f32;            // Width of the vertical scrollbar, Height of the horizontal scrollbar
        out.ScrollbarRounding = 9.0f32;             // Radius of grab corners rounding for scrollbar
        out.GrabMinSize = 12.0f32;            // Minimum width/height of a grab box for slider/scrollbar
        out.GrabRounding = 0f32;             // Radius of grabs corners rounding. Set to 0f32 to have rectangular slider grabs.
        out.LogSliderDeadzone = 4.0f32;             // The size in pixels of the dead-zone around zero on logarithmic sliders that cross zero.
        out.TabRounding = 4.0f32;             // Radius of upper corners of a tab. Set to 0f32 to have rectangular tabs.
        out.TabBorderSize = 0f32;             // Thickness of border around tabs.
        out.TabMinWidthForCloseButton = 0f32;           // Minimum width for close button to appears on an unselected tab when hovered. Set to 0f32 to always show when hovering, set to f32::MAX to never show close button unless selected.
        out.ColorButtonPosition = ImGuiDir_Right;   // Side of the color button in the ColorEdit4 widget (left/right). Defaults to ImGuiDir_Right.
        out.ButtonTextAlign = ImVec2(0.5f32, 0.5f32);// Alignment of button text when button is larger than text.
        out.SelectableTextAlign = ImVec2(0f32, 0f32);// Alignment of selectable text. Defaults to (0f32, 0f32) (top-left aligned). It's generally important to keep this left-aligned if you want to lay multiple items on a same line.
        out.DisplayWindowPadding = ImVec2(19, 19);    // Window position are clamped to be visible within the display area or monitors by at least this amount. Only applies to regular windows.
        out.DisplaySafeAreaPadding = ImVec2(3, 3);      // If you cannot see the edge of your screen (e.g. on a TV) increase the safe area padding. Covers popups/tooltips as well regular windows.
        out.MouseCursorScale = 1f32;             // Scale software rendered mouse cursor (when io.MouseDrawCursor is enabled). May be removed later.
        out.AntiAliasedLines = true;             // Enable anti-aliased lines/borders. Disable if you are really tight on CPU/GPU.
        out.AntiAliasedLinesUseTex = true;             // Enable anti-aliased lines/borders using textures where possible. Require backend to render with bilinear filtering (NOT point/nearest filtering).
        out.AntiAliasedFill = true;             // Enable anti-aliased filled shapes (rounded rectangles, circles, etc.).
        out.CurveTessellationTol = 1.25f32;            // Tessellation tolerance when using PathBezierCurveTo() without a specific number of segments. Decrease for highly tessellated curves (higher quality, more polygons), increase to reduce quality.
        out.CircleTessellationMaxError = 0.3f32;         // Maximum error (in pixels) allowed when using AddCircle()/AddCircleFilled() or drawing rounded corner rectangles with no explicit segment count specified. Decrease for higher quality but more geometry.

        // Default theme
        ImGui::StyleColorsDark(this);

        return out;
    }

    // To scale your entire UI (e.g. if you want your app to use High DPI or generally be DPI aware) you may use this helper function. Scaling the fonts is done separately and is up to you.
// Important: This operation is lossy because we round all sizes to integer. If you need to change your scale multiples, call this over a freshly initialized ImGuiStyle structure rather than scaling multiple times.
// void ImGuiStyle::ScaleAllSizes(float scale_factor)
    pub fn ScaleAllSizes(&mut self, scale_factor: f32) {
        self.WindowPadding = ImFloor(self.WindowPadding * scale_factor);
        self.WindowRounding = ImFloor(self.WindowRounding * scale_factor);
        self.WindowMinSize = ImFloor(self.WindowMinSize * scale_factor);
        self.ChildRounding = ImFloor(self.ChildRounding * scale_factor);
        self.PopupRounding = ImFloor(self.PopupRounding * scale_factor);
        self.FramePadding = ImFloor(self.FramePadding * scale_factor);
        self.FrameRounding = ImFloor(self.FrameRounding * scale_factor);
        self.ItemSpacing = ImFloor(self.ItemSpacing * scale_factor);
        self.ItemInnerSpacing = ImFloor(self.ItemInnerSpacing * scale_factor);
        self.CellPadding = ImFloor(self.CellPadding * scale_factor);
        self.TouchExtraPadding = ImFloor(self.TouchExtraPadding * scale_factor);
        self.IndentSpacing = ImFloor(self.IndentSpacing * scale_factor);
        self.ColumnsMinSpacing = ImFloor(self.ColumnsMinSpacing * scale_factor);
        self.ScrollbarSize = ImFloor(self.ScrollbarSize * scale_factor);
        self.ScrollbarRounding = ImFloor(self.ScrollbarRounding * scale_factor);
        self.GrabMinSize = ImFloor(self.GrabMinSize * scale_factor);
        self.GrabRounding = ImFloor(self.GrabRounding * scale_factor);
        self.LogSliderDeadzone = ImFloor(self.LogSliderDeadzone * scale_factor);
        self.TabRounding = ImFloor(TabRounding * scale_factor);
        self.TabMinWidthForCloseButton = if self.TabMinWidthForCloseButton != f32::MAX { ImFloor(self.TabMinWidthForCloseButton * scale_factor) } else { f32::MAX };
        self.DisplayWindowPadding = ImFloor(self.DisplayWindowPadding * scale_factor);
        self.DisplaySafeAreaPadding = ImFloor(self.DisplaySafeAreaPadding * scale_factor);
        self.MouseCursorScale = ImFloor(self.MouseCursorScale * scale_factor);
    }
}
