#![allow(non_snake_case)]


// You may modify the GetStyle() main instance during initialization and before NewFrame().
// During the frame, use PushStyleVar(ImGuiStyleVar_XXXX)/PopStyleVar() to alter the main style values,
// and PushStyleColor(ImGuiCol_XXX)/PopStyleColor() for colors.
//-----------------------------------------------------------------------------

use libc::c_float;
use crate::color::ImGuiCol_COUNT;
use crate::direction::{ImGuiDir, ImGuiDir_Left, ImGuiDir_Right};
use crate::style_ops::StyleColorsDark;
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::type_defs::ImGuiDir;

#[derive(Default,Debug,Clone)]
pub struct ImGuiStyle {
    pub Alpha: c_float,
    // Global alpha applies to everything in Dear ImGui.
    pub DisabledAlpha: c_float,
    // Additional alpha multiplier applied by BeginDisabled(). Multiply over current value of Alpha.
    pub WindowPadding: ImVec2,
    // Padding within a window.
    pub WindowRounding: c_float,
    // Radius of window corners rounding. Set to 0.0 to have rectangular windows. Large values tend to lead to variety of artifacts and are not recommended.
    pub WindowBorderSize: c_float,
    // Thickness of border around windows. Generally set to 0.0 or 1.0f. (Other values are not well tested and more CPU/GPU costly).
    pub WindowMinSize: ImVec2,
    // Minimum window size. This is a global setting. If you want to constraint individual windows, use SetNextWindowSizeConstraints().
    pub WindowTitleAlign: ImVec2,
    // Alignment for title bar text. Defaults to (0.0,0.5) for left-aligned,vertically centered.
    pub WindowMenuButtonPosition: ImGuiDir,
    // Side of the collapsing/docking button in the title bar (None/Left/Right). Defaults to ImGuiDir_Left.
    pub ChildRounding: c_float,
    // Radius of child window corners rounding. Set to 0.0 to have rectangular windows.
    pub ChildBorderSize: c_float,
    // Thickness of border around child windows. Generally set to 0.0 or 1.0f. (Other values are not well tested and more CPU/GPU costly).
    pub PopupRounding: c_float,
    // Radius of popup window corners rounding. (Note that tooltip windows use WindowRounding)
    pub PopupBorderSize: c_float,
    // Thickness of border around popup/tooltip windows. Generally set to 0.0 or 1.0f. (Other values are not well tested and more CPU/GPU costly).
    pub FramePadding: ImVec2,
    // Padding within a framed rectangle (used by most widgets).
    pub FrameRounding: c_float,
    // Radius of frame corners rounding. Set to 0.0 to have rectangular frame (used by most widgets).
    pub FrameBorderSize: c_float,
    // Thickness of border around frames. Generally set to 0.0 or 1.0f. (Other values are not well tested and more CPU/GPU costly).
    pub ItemSpacing: ImVec2,
    // Horizontal and vertical spacing between widgets/lines.
    pub ItemInnerSpacing: ImVec2,
    // Horizontal and vertical spacing between within elements of a composed widget (e.g. a slider and its label).
    pub CellPadding: ImVec2,
    // Padding within a table cell
    pub TouchExtraPadding: ImVec2,
    // Expand reactive bounding box for touch-based system where touch position is not accurate enough. Unfortunately we don't sort widgets so priority on overlap will always be given to the first widget. So don't grow this too much!
    pub IndentSpacing: c_float,
    // Horizontal indentation when e.g. entering a tree node. Generally == (FontSize + FramePadding.x*2).
    pub ColumnsMinSpacing: c_float,
    // Minimum horizontal spacing between two columns. Preferably > (FramePadding.x + 1).
    pub ScrollbarSize: c_float,
    // Width of the vertical scrollbar, Height of the horizontal scrollbar.
    pub ScrollbarRounding: c_float,
    // Radius of grab corners for scrollbar.
    pub GrabMinSize: c_float,
    // Minimum width/height of a grab box for slider/scrollbar.
    pub GrabRounding: c_float,
    // Radius of grabs corners rounding. Set to 0.0 to have rectangular slider grabs.
    pub LogSliderDeadzone: c_float,
    // The size in pixels of the dead-zone around zero on logarithmic sliders that cross zero.
    pub TabRounding: c_float,
    // Radius of upper corners of a tab. Set to 0.0 to have rectangular tabs.
    pub TabBorderSize: c_float,
    // Thickness of border around tabs.
    pub TabMinWidthForCloseButton: c_float,
    // Minimum width for close button to appears on an unselected tab when hovered. Set to 0.0 to always show when hovering, set to f32::MAX to never show close button unless selected.
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
    pub MouseCursorScale: c_float,
    // Scale software rendered mouse cursor (when io.MouseDrawCursor is enabled). We apply per-monitor DPI scaling over this scale. May be removed later.
    pub AntiAliasedLines: bool,
    // Enable anti-aliased lines/borders. Disable if you are really tight on CPU/GPU. Latched at the beginning of the frame (copied to ImDrawList).
    pub AntiAliasedLinesUseTex: bool,
    // Enable anti-aliased lines/borders using textures where possible. Require backend to render with bilinear filtering (NOT point/nearest filtering). Latched at the beginning of the frame (copied to ImDrawList).
    pub AntiAliasedFill: bool,
    // Enable anti-aliased edges around filled shapes (rounded rectangles, circles, etc.). Disable if you are really tight on CPU/GPU. Latched at the beginning of the frame (copied to ImDrawList).
    pub CurveTessellationTol: c_float,
    // Tessellation tolerance when using PathBezierCurveTo() without a specific number of segments. Decrease for highly tessellated curves (higher quality, more polygons), increase to reduce quality.
    pub CircleTessellationMaxError: c_float,
    // Maximum error (in pixels) allowed when using AddCircle()/AddCircleFilled() or drawing rounded corner rectangles with no explicit segment count specified. Decrease for higher quality but more geometry.
// ImVec4      Colors[ImGuiCol_COUNT];
    pub Colors: [ImVec4; ImGuiCol_COUNT],

    // ImGuiStyle();
    // void ScaleAllSizes(c_float scale_factor);
}


impl ImGuiStyle {
    // ImGuiStyle::ImGuiStyle()
    pub fn new() -> Self {
        let mut out = Self { ..Default::default() };
        out.Alpha = 1.0;             // Global alpha applies to everything in Dear ImGui.
        out.DisabledAlpha = 0.60;            // Additional alpha multiplier applied by BeginDisabled(). Multiply over current value of Alpha.
        out.WindowPadding = ImVec2::new(8, 8);      // Padding within a window
        out.WindowRounding = 0.0;             // Radius of window corners rounding. Set to 0.0 to have rectangular windows. Large values tend to lead to variety of artifacts and are not recommended.
        out.WindowBorderSize = 1.0;             // Thickness of border around windows. Generally set to 0.0 or 1.0f. Other values not well tested.
        out.WindowMinSize = ImVec2::new(32, 32);    // Minimum window size
        out.WindowTitleAlign = ImVec2::new(0.0, 0.5);// Alignment for title bar text
        out.WindowMenuButtonPosition = ImGuiDir_Left;    // Position of the collapsing/docking button in the title bar (left/right). Defaults to ImGuiDir_Left.
        out.ChildRounding = 0.0;             // Radius of child window corners rounding. Set to 0.0 to have rectangular child windows
        out.ChildBorderSize = 1.0;             // Thickness of border around child windows. Generally set to 0.0 or 1.0f. Other values not well tested.
        out.PopupRounding = 0.0;             // Radius of popup window corners rounding. Set to 0.0 to have rectangular child windows
        out.PopupBorderSize = 1.0;             // Thickness of border around popup or tooltip windows. Generally set to 0.0 or 1.0f. Other values not well tested.
        out.FramePadding = ImVec2::new(4, 3);      // Padding within a framed rectangle (used by most widgets)
        out.FrameRounding = 0.0;             // Radius of frame corners rounding. Set to 0.0 to have rectangular frames (used by most widgets).
        out.FrameBorderSize = 0.0;             // Thickness of border around frames. Generally set to 0.0 or 1.0f. Other values not well tested.
        out.ItemSpacing = ImVec2::new(8, 4);      // Horizontal and vertical spacing between widgets/lines
        out.ItemInnerSpacing = ImVec2::new(4, 4);      // Horizontal and vertical spacing between within elements of a composed widget (e.g. a slider and its label)
        out.CellPadding = ImVec2::new(4, 2);      // Padding within a table cell
        out.TouchExtraPadding = ImVec2::new(0, 0);      // Expand reactive bounding box for touch-based system where touch position is not accurate enough. Unfortunately we don't sort widgets so priority on overlap will always be given to the first widget. So don't grow this too much!
        out.IndentSpacing = 21f32;            // Horizontal spacing when e.g. entering a tree node. Generally == (FontSize + FramePadding.x*2).
        out.ColumnsMinSpacing = 6f32;             // Minimum horizontal spacing between two columns. Preferably > (FramePadding.x + 1).
        out.crollbarSize = 14.0;            // Width of the vertical scrollbar, Height of the horizontal scrollbar
        out.ScrollbarRounding = 9.0;             // Radius of grab corners rounding for scrollbar
        out.GrabMinSize = 12.0;            // Minimum width/height of a grab box for slider/scrollbar
        out.GrabRounding = 0.0;             // Radius of grabs corners rounding. Set to 0.0 to have rectangular slider grabs.
        out.LogSliderDeadzone = 4.0;             // The size in pixels of the dead-zone around zero on logarithmic sliders that cross zero.
        out.TabRounding = 4.0;             // Radius of upper corners of a tab. Set to 0.0 to have rectangular tabs.
        out.TabBorderSize = 0.0;             // Thickness of border around tabs.
        out.TabMinWidthForCloseButton = 0.0;           // Minimum width for close button to appears on an unselected tab when hovered. Set to 0.0 to always show when hovering, set to f32::MAX to never show close button unless selected.
        out.ColorButtonPosition = ImGuiDir_Right;   // Side of the color button in the ColorEdit4 widget (left/right). Defaults to ImGuiDir_Right.
        out.ButtonTextAlign = ImVec2::new(0.5, 0.5);// Alignment of button text when button is larger than text.
        out.SelectableTextAlign = ImVec2::new(0.0, 0.0);// Alignment of selectable text. Defaults to (0.0, 0.0) (top-left aligned). It's generally important to keep this left-aligned if you want to lay multiple items on a same line.
        out.DisplayWindowPadding = ImVec2::new(19, 19);    // Window position are clamped to be visible within the display area or monitors by at least this amount. Only applies to regular windows.
        out.DisplaySafeAreaPadding = ImVec2::new(3, 3);      // If you cannot see the edge of your screen (e.g. on a TV) increase the safe area padding. Covers popups/tooltips as well regular windows.
        out.MouseCursorScale = 1.0;             // Scale software rendered mouse cursor (when io.MouseDrawCursor is enabled). May be removed later.
        out.AntiAliasedLines = true;             // Enable anti-aliased lines/borders. Disable if you are really tight on CPU/GPU.
        out.AntiAliasedLinesUseTex = true;             // Enable anti-aliased lines/borders using textures where possible. Require backend to render with bilinear filtering (NOT point/nearest filtering).
        out.AntiAliasedFill = true;             // Enable anti-aliased filled shapes (rounded rectangles, circles, etc.).
        out.CurveTessellationTol = 1.25f32;            // Tessellation tolerance when using PathBezierCurveTo() without a specific number of segments. Decrease for highly tessellated curves (higher quality, more polygons), increase to reduce quality.
        out.CircleTessellationMaxError = 0.3f32;         // Maximum error (in pixels) allowed when using AddCircle()/AddCircleFilled() or drawing rounded corner rectangles with no explicit segment count specified. Decrease for higher quality but more geometry.

        // Default theme
        StyleColorsDark(this);

        return out;
    }

    // To scale your entire UI (e.g. if you want your app to use High DPI or generally be DPI aware) you may use this helper function. Scaling the fonts is done separately and is up to you.
// Important: This operation is lossy because we round all sizes to integer. If you need to change your scale multiples, call this over a freshly initialized ImGuiStyle structure rather than scaling multiple times.
// void ImGuiStyle::ScaleAllSizes(float scale_factor)
    pub fn ScaleAllSizes(&mut self, scale_factor: f32) {
        self.WindowPadding = ImFloor(&self.WindowPadding * scale_factor);
        self.WindowRounding = ImFloor(&self.WindowRounding * scale_factor);
        self.WindowMinSize = ImFloor(&self.WindowMinSize * scale_factor);
        self.ChildRounding = ImFloor(&self.ChildRounding * scale_factor);
        self.PopupRounding = ImFloor(self.PopupRounding * scale_factor);
        self.FramePadding = ImFloor(&self.FramePadding * scale_factor);
        self.FrameRounding = ImFloor(self.FrameRounding * scale_factor);
        self.ItemSpacing = ImFloor(&self.ItemSpacing * scale_factor);
        self.ItemInnerSpacing = ImFloor(&self.ItemInnerSpacing * scale_factor);
        self.CellPadding = ImFloor(&self.CellPadding * scale_factor);
        self.TouchExtraPadding = ImFloor(&self.TouchExtraPadding * scale_factor);
        self.IndentSpacing = ImFloor(self.IndentSpacing * scale_factor);
        self.ColumnsMinSpacing = ImFloor(self.ColumnsMinSpacing * scale_factor);
        self.ScrollbarSize = ImFloor(self.ScrollbarSize * scale_factor);
        self.ScrollbarRounding = ImFloor(self.ScrollbarRounding * scale_factor);
        self.GrabMinSize = ImFloor(self.GrabMinSize * scale_factor);
        self.GrabRounding = ImFloor(self.GrabRounding * scale_factor);
        self.LogSliderDeadzone = ImFloor(self.LogSliderDeadzone * scale_factor);
        self.TabRounding = ImFloor(TabRounding * scale_factor);
        self.TabMinWidthForCloseButton = if self.TabMinWidthForCloseButton != f32::MAX { ImFloor(self.TabMinWidthForCloseButton * scale_factor) } else { f32::MAX };
        self.DisplayWindowPadding = ImFloor(&self.DisplayWindowPadding * scale_factor);
        self.DisplaySafeAreaPadding = ImFloor(&self.DisplaySafeAreaPadding * scale_factor);
        self.MouseCursorScale = ImFloor(self.MouseCursorScale * scale_factor);
    }
}
