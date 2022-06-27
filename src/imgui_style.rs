use std::intrinsics::floorf32;
use crate::img_h::{ImGuiCol, ImGuiDir, ImVec2};

#[allow(non_snake_case)]
pub struct ImGuiStyle {
    pub Alpha: f32,
    // Global alpha applies to everything in Dear ImGui.
    pub DisabledAlpha: f32,
    // Additional alpha multiplier applied by BeginDisabled(). Multiply over current value of Alpha.
    pub WindowPadding: ImVec2,
    // Padding within a window.
    pub WindowRounding: f32,
    // Radius of window corners rounding. Set to 0.0 to have rectangular windows. Large values tend to lead to variety of artifacts and are not recommended.
    pub WindowBorderSize: f32,
    // Thickness of border around windows. Generally set to 0.0 or 1.0. (Other values are not well tested and more CPU/GPU costly).
    pub WindowMinSize: ImVec2,
    // Minimum window size. This is a global setting. If you want to constraint individual windows, use SetNextWindowSizeConstraints().
    pub WindowTitleAlign: ImVec2,
    // Alignment for title bar text. Defaults to (0.0,0.5) for left-aligned,vertically centered.
    pub WindowMenuButtonPosition: ImGuiDir,
    // Side of the collapsing/docking button in the title bar (None/Left/Right). Defaults to ImGuiDir_Left.
    pub ChildRounding: f32,
    // Radius of child window corners rounding. Set to 0.0 to have rectangular windows.
    pub ChildBorderSize: f32,
    // Thickness of border around child windows. Generally set to 0.0 or 1.0. (Other values are not well tested and more CPU/GPU costly).
    pub PopupRounding: f32,
    // Radius of popup window corners rounding. (Note that tooltip windows use WindowRounding)
    pub PopupBorderSize: f32,
    // Thickness of border around popup/tooltip windows. Generally set to 0.0 or 1.0. (Other values are not well tested and more CPU/GPU costly).
    pub FramePadding: ImVec2,
    // Padding within a framed rectangle (used by most widgets).
    pub FrameRounding: f32,
    // Radius of frame corners rounding. Set to 0.0 to have rectangular frame (used by most widgets).
    pub FrameBorderSize: f32,
    // Thickness of border around frames. Generally set to 0.0 or 1.0. (Other values are not well tested and more CPU/GPU costly).
    pub ItemSpacing: ImVec2,
    // Horizontal and vertical spacing between widgets/lines.
    pub ItemInnerSpacing: ImVec2,
    // Horizontal and vertical spacing between within elements of a composed widget (e.g. a slider and its label).
    pub CellPadding: ImVec2,
    // Padding within a table cell
    pub TouchExtraPadding: ImVec2,
    // Expand reactive bounding box for touch-based system where touch position is not accurate enough. Unfortunately we don't sort widgets so priority on overlap will always be given to the first widget. So don't grow this too much!
    pub IndentSpacing: f32,
    // Horizontal indentation when e.g. entering a tree node. Generally == (FontSize + FramePadding.x*2).
    pub ColumnsMinSpacing: f32,
    // Minimum horizontal spacing between two columns. Preferably > (FramePadding.x + 1).
    pub ScrollbarSize: f32,
    // Width of the vertical scrollbar, Height of the horizontal scrollbar.
    pub ScrollbarRounding: f32,
    // Radius of grab corners for scrollbar.
    pub GrabMinSize: f32,
    // Minimum width/height of a grab box for slider/scrollbar.
    pub GrabRounding: f32,
    // Radius of grabs corners rounding. Set to 0.0 to have rectangular slider grabs.
    pub LogSliderDeadzone: f32,
    // The size in pixels of the dead-zone around zero on logarithmic sliders that cross zero.
    pub TabRounding: f32,
    // Radius of upper corners of a tab. Set to 0.0 to have rectangular tabs.
    pub TabBorderSize: f32,
    // Thickness of border around tabs.
    pub TabMinWidthForCloseButton: f32,
    // Minimum width for close button to appears on an unselected tab when hovered. Set to 0.0 to always show when hovering, set to FLT_MAX to never show close button unless selected.
    pub ColorButtonPosition: ImGuiDir,
    // Side of the color button in the ColorEdit4 widget (left/right). Defaults to ImGuiDir_Right.
    pub ButtonTextAlign: ImVec2,
    // Alignment of button text when button is larger than text. Defaults to (0.5, 0.5) (centered).
    pub SelectableTextAlign: ImVec2,
    // Alignment of selectable text. Defaults to (0.0, 0.0) (top-left aligned). It's generally important to keep this left-aligned if you want to lay multiple items on a same line.
    pub DisplayWindowPadding: ImVec2,
    // Window position are clamped to be visible within the display area or monitors by at least this amount. Only applies to regular windows.
    pub DisplaySafeAreaPadding: ImVec2,
    // If you cannot see the edges of your screen (e.g. on a TV) increase the safe area padding. Apply to popups/tooltips as well regular windows. NB: Prefer configuring your TV sets correctly!
    pub MouseCursorScale: f32,
    // Scale software rendered mouse cursor (when io.MouseDrawCursor is enabled). We apply per-monitor DPI scaling over this scale. May be removed later.
    pub AntiAliasedLines: bool,
    // Enable anti-aliased lines/borders. Disable if you are really tight on CPU/GPU. Latched at the beginning of the frame (copied to ImDrawList).
    pub AntiAliasedLinesUseTex: bool,
    // Enable anti-aliased lines/borders using textures where possible. Require backend to render with bilinear filtering (NOT point/nearest filtering). Latched at the beginning of the frame (copied to ImDrawList).
    pub AntiAliasedFill: bool,
    // Enable anti-aliased edges around filled shapes (rounded rectangles, circles, etc.). Disable if you are really tight on CPU/GPU. Latched at the beginning of the frame (copied to ImDrawList).
    pub CurveTessellationTol: f32,
    // Tessellation tolerance when using PathBezierCurveTo() without a specific number of segments. Decrease for highly tessellated curves (higher quality, more polygons), increase to reduce quality.
    pub CircleTessellationMaxError: f32,
    // Maximum error (in pixels) allowed when using AddCircle()/AddCircleFilled() or drawing rounded corner rectangles with no explicit segment count specified. Decrease for higher quality but more geometry.
    // ImVec4      Colors[ImGuiCol_COUNT];
    pub Colors: Vec<ImGuiCol>,

    // IMGUI_API ImGuiStyle();
    // IMGUI_API void ScaleAllSizes(float scale_factor);
}

impl ImGuiStyle {
    pub fn new() -> Self {
        let mut out = Self {..Default()};
        out.Alpha = 1.0;             // Global alpha applies to everything in Dear ImGui.
        out.DisabledAlpha = 0.60;            // Additional alpha multiplier applied by BeginDisabled(). Multiply over current value of Alpha.
        out.WindowPadding = ImVec2(8, 8);      // Padding within a window
        out.WindowRounding = 0.0;             // Radius of window corners rounding. Set to 0.0 to have rectangular windows. Large values tend to lead to variety of artifacts and are not recommended.
        out.WindowBorderSize = 1.0;             // Thickness of border around windows. Generally set to 0.0 or 1.0. Other values not well tested.
        out.WindowMinSize = ImVec2(32, 32);    // Minimum window size
        out.WindowTitleAlign = ImVec2(0.0, 0.5);// Alignment for title bar text
        out.WindowMenuButtonPosition = ImGuiDir::ImGuiDir_Left;    // Position of the collapsing/docking button in the title bar (left/right). Defaults to ImGuiDir_Left.
        out.ChildRounding = 0.0;             // Radius of child window corners rounding. Set to 0.0 to have rectangular child windows
        out.ChildBorderSize = 1.0;             // Thickness of border around child windows. Generally set to 0.0 or 1.0. Other values not well tested.
        out.PopupRounding = 0.0;             // Radius of popup window corners rounding. Set to 0.0 to have rectangular child windows
        out.PopupBorderSize = 1.0;             // Thickness of border around popup or tooltip windows. Generally set to 0.0 or 1.0. Other values not well tested.
        out.FramePadding = ImVec2(4, 3);      // Padding within a framed rectangle (used by most widgets)
        out.FrameRounding = 0.0;             // Radius of frame corners rounding. Set to 0.0 to have rectangular frames (used by most widgets).
        out.FrameBorderSize = 0.0;             // Thickness of border around frames. Generally set to 0.0 or 1.0. Other values not well tested.
        out.ItemSpacing = ImVec2(8, 4);      // Horizontal and vertical spacing between widgets/lines
        out.ItemInnerSpacing = ImVec2(4, 4);      // Horizontal and vertical spacing between within elements of a composed widget (e.g. a slider and its label)
        out.CellPadding = ImVec2(4, 2);      // Padding within a table cell
        out.TouchExtraPadding = ImVec2(0, 0);      // Expand reactive bounding box for touch-based system where touch position is not accurate enough. Unfortunately we don't sort widgets so priority on overlap will always be given to the first widget. So don't grow this too much!
        out.IndentSpacing = 21.0;            // Horizontal spacing when e.g. entering a tree node. Generally == (FontSize + FramePadding.x*2).
        out.ColumnsMinSpacing = 6.0;             // Minimum horizontal spacing between two columns. Preferably > (FramePadding.x + 1).
        out.ScrollbarSize = 14.0;            // Width of the vertical scrollbar, Height of the horizontal scrollbar
        out.ScrollbarRounding = 9.0;             // Radius of grab corners rounding for scrollbar
        out.GrabMinSize = 12.0;            // Minimum width/height of a grab box for slider/scrollbar
        out.GrabRounding = 0.0;             // Radius of grabs corners rounding. Set to 0.0 to have rectangular slider grabs.
        out.LogSliderDeadzone = 4.0;             // The size in pixels of the dead-zone around zero on logarithmic sliders that cross zero.
        out.TabRounding = 4.0;             // Radius of upper corners of a tab. Set to 0.0 to have rectangular tabs.
        out.TabBorderSize = 0.0;             // Thickness of border around tabs.
        out.TabMinWidthForCloseButton = 0.0;           // Minimum width for close button to appears on an unselected tab when hovered. Set to 0.0 to always show when hovering, set to FLT_MAX to never show close button unless selected.
        out.ColorButtonPosition = ImGuiDir::ImGuiDir_Right;   // Side of the color button in the ColorEdit4 widget (left/right). Defaults to ImGuiDir_Right.
        out.ButtonTextAlign = ImVec2(0.5, 0.5);// Alignment of button text when button is larger than text.
        out.SelectableTextAlign = ImVec2(0.0, 0.0);// Alignment of selectable text. Defaults to (0.0, 0.0) (top-left aligned). It's generally important to keep this left-aligned if you want to lay multiple items on a same line.
        out.DisplayWindowPadding = ImVec2(19, 19);    // Window position are clamped to be visible within the display area or monitors by at least this amount. Only applies to regular windows.
        out.DisplaySafeAreaPadding = ImVec2(3, 3);      // If you cannot see the edge of your screen (e.g. on a TV) increase the safe area padding. Covers popups/tooltips as well regular windows.
        out.MouseCursorScale = 1.0;             // Scale software rendered mouse cursor (when io.MouseDrawCursor is enabled). May be removed later.
        out.AntiAliasedLines = true;             // Enable anti-aliased lines/borders. Disable if you are really tight on CPU/GPU.
        out.AntiAliasedLinesUseTex = true;             // Enable anti-aliased lines/borders using textures where possible. Require backend to render with bilinear filtering (NOT point/nearest filtering).
        out.AntiAliasedFill = true;             // Enable anti-aliased filled shapes (rounded rectangles, circles, etc.).
        out.CurveTessellationTol = 1.25;            // Tessellation tolerance when using PathBezierCurveTo() without a specific number of segments. Decrease for highly tessellated curves (higher quality, more polygons), increase to reduce quality.
        out.CircleTessellationMaxError = 0.30;         // Maximum error (in pixels) allowed when using AddCircle()/AddCircleFilled() or drawing rounded corner rectangles with no explicit segment count specified. Decrease for higher quality but more geometry.

        // Default theme
        StyleColorsDark(&mut out);
        out
    }

    // To scale your entire UI (e.g. if you want your app to use High DPI or generally be DPI aware) you may use this helper function. Scaling the fonts is done separately and is up to you.
// Important: This operation is lossy because we round all sizes to integer. If you need to change your scale multiples, call this over a freshly initialized ImGuiStyle structure rather than scaling multiple times.
    pub fn scale_all_sizes(&mut self, scale_factor: f32) {
        self.WindowPadding = ImFloor(WindowPadding * scale_factor);
        self.WindowRounding = ImFloor(WindowRounding * scale_factor);
        self.WindowMinSize = ImFloor(WindowMinSize * scale_factor);
        self.ChildRounding = ImFloor(ChildRounding * scale_factor);
        self.PopupRounding = ImFloor(PopupRounding * scale_factor);
        self.FramePadding = ImFloor(FramePadding * scale_factor);
        self.FrameRounding = ImFloor(FrameRounding * scale_factor);
        self.ItemSpacing = ImFloor(ItemSpacing * scale_factor);
        self.ItemInnerSpacing = ImFloor(ItemInnerSpacing * scale_factor);
        self.CellPadding = ImFloor(CellPadding * scale_factor);
        self.TouchExtraPadding = ImFloor(TouchExtraPadding * scale_factor);
        self.IndentSpacing = ImFloor(IndentSpacing * scale_factor);
        self.ColumnsMinSpacing = ImFloor(ColumnsMinSpacing * scale_factor);
        self.ScrollbarSize = ImFloor(ScrollbarSize * scale_factor);
        self.ScrollbarRounding = ImFloor(ScrollbarRounding * scale_factor);
        self.GrabMinSize = ImFloor(GrabMinSize * scale_factor);
        self.GrabRounding = ImFloor(GrabRounding * scale_factor);
        self.LogSliderDeadzone = ImFloor(LogSliderDeadzone * scale_factor);
        self.TabRounding = ImFloor(TabRounding * scale_factor);
        self.TabMinWidthForCloseButton =
            if Self.TabMinWidthForCloseButton != FLT_MAX {
                (self.TabMinWidthForCloseButton * scale_factor).floor()
            } else {
                FLT_MAX
            };
        self.DisplayWindowPadding = ImFloor(DisplayWindowPadding * scale_factor);
        self.DisplaySafeAreaPadding = ImFloor(DisplaySafeAreaPadding * scale_factor);
        self.MouseCursorScale = ImFloor(MouseCursorScale * scale_factor);
    }
}
