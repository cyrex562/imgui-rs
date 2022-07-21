
use crate::imgui_color::{ColorConvertFloat4ToU32, IM_COL32_A_MASK, IM_COL32_A_SHIFT, ColorMod};
use crate::imgui_globals::GImGui;
use crate::imgui_h::{Color, ImGuiDataType, ImGuiDir, ImGuiStyleVar};
use crate::imgui_math::ImLerpF32;
use crate::imgui_vec::{Vector2D, Vector4D};

#[allow(non_snake_)]
pub struct Style {
    pub Alpha: f32,
    // Global alpha applies to everything in Dear ImGui.
    pub DisabledAlpha: f32,
    // Additional alpha multiplier applied by BeginDisabled(). Multiply over current value of Alpha.
    pub WindowPadding: Vector2D,
    // Padding within a window.
    pub WindowRounding: f32,
    // Radius of window corners rounding. Set to 0.0 to have rectangular windows. Large values tend to lead to variety of artifacts and are not recommended.
    pub WindowBorderSize: f32,
    // Thickness of border around windows. Generally set to 0.0 or 1.0. (Other values are not well tested and more CPU/GPU costly).
    pub WindowMinSize: Vector2D,
    // Minimum window size. This is a global setting. If you want to constraint individual windows, use SetNextWindowSizeConstraints().
    pub window_title_align: Vector2D,
    // Alignment for title bar text. Defaults to (0.0,0.5) for left-aligned,vertically centered.
    pub window_menu_button_position: ImGuiDir,
    // Side of the collapsing/docking button in the title bar (None/Left/Right). Defaults to ImGuiDir_Left.
    pub ChildRounding: f32,
    // Radius of child window corners rounding. Set to 0.0 to have rectangular windows.
    pub ChildBorderSize: f32,
    // Thickness of border around child windows. Generally set to 0.0 or 1.0. (Other values are not well tested and more CPU/GPU costly).
    pub PopupRounding: f32,
    // Radius of popup window corners rounding. (Note that tooltip windows use window_rounding)
    pub PopupBorderSize: f32,
    // Thickness of border around popup/tooltip windows. Generally set to 0.0 or 1.0. (Other values are not well tested and more CPU/GPU costly).
    pub frame_padding: Vector2D,
    // Padding within a framed rectangle (used by most widgets).
    pub FrameRounding: f32,
    // Radius of frame corners rounding. Set to 0.0 to have rectangular frame (used by most widgets).
    pub FrameBorderSize: f32,
    // Thickness of border around frames. Generally set to 0.0 or 1.0. (Other values are not well tested and more CPU/GPU costly).
    pub ItemSpacing: Vector2D,
    // Horizontal and vertical spacing between widgets/lines.
    pub item_inner_spacing: Vector2D,
    // Horizontal and vertical spacing between within elements of a composed widget (e.g. a slider and its label).
    pub CellPadding: Vector2D,
    // Padding within a table cell
    pub touch_extra_padding: Vector2D,
    // Expand reactive bounding box for touch-based system where touch position is not accurate enough. Unfortunately we don't sort widgets so priority on overlap will always be given to the first widget. So don't grow this too much!
    pub IndentSpacing: f32,
    // Horizontal indentation when e.g. entering a tree node. Generally == (font_size + FramePadding.x*2).
    pub ColumnsMinSpacing: f32,
    // Minimum horizontal spacing between two columns. Preferably > (FramePadding.x + 1).
    pub ScrollbarSize: f32,
    // width of the vertical scrollbar, height of the horizontal scrollbar.
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
    pub ButtonTextAlign: Vector2D,
    // Alignment of button text when button is larger than text. Defaults to (0.5, 0.5) (centered).
    pub SelectableTextAlign: Vector2D,
    // Alignment of selectable text. Defaults to (0.0, 0.0) (top-left aligned). It's generally important to keep this left-aligned if you want to lay multiple items on a same line.
    pub DisplayWindowPadding: Vector2D,
    // window position are clamped to be visible within the display area or monitors by at least this amount. Only applies to regular windows.
    pub DisplaySafeAreaPadding: Vector2D,
    // If you cannot see the edges of your screen (e.g. on a TV) increase the safe area padding. Apply to popups/tooltips as well regular windows. NB: Prefer configuring your TV sets correctly!
    pub MouseCursorScale: f32,
    // scale software rendered mouse cursor (when io.mouse_draw_cursor is enabled). We apply per-monitor DPI scaling over this scale. May be removed later.
    pub anti_aliased_lines: bool,
    // Enable anti-aliased lines/borders. Disable if you are really tight on CPU/GPU. Latched at the beginning of the frame (copied to ImDrawList).
    pub anti_aliased_lines_use_tex: bool,
    // Enable anti-aliased lines/borders using textures where possible. Require backend to render with bilinear filtering (NOT point/nearest filtering). Latched at the beginning of the frame (copied to ImDrawList).
    pub anti_aliased_fill: bool,
    // Enable anti-aliased edges around filled shapes (rounded rectangles, circles, etc.). Disable if you are really tight on CPU/GPU. Latched at the beginning of the frame (copied to ImDrawList).
    pub curve_tessellation_tol: f32,
    // Tessellation tolerance when using PathBezierCurveTo() without a specific number of segments. Decrease for highly tessellated curves (higher quality, more polygons), increase to reduce quality.
    pub circle_tessellation_max_error: f32,
    // Maximum error (in pixels) allowed when using add_circle()/add_circle_filled() or drawing rounded corner rectangles with no explicit segment count specified. Decrease for higher quality but more geometry.
    // Vector4D      colors[ImGuiColor::COUNT];
    pub Colors: Vec<Color>,

    //  ImGuiStyle();
    //  void ScaleAllSizes(float scale_factor);
}

impl Style {
    pub fn new() -> Self {
        let mut out = Self {..Default::default()};
        out.alpha = 1.0;             // Global alpha applies to everything in Dear ImGui.
        out.DisabledAlpha = 0.60;            // Additional alpha multiplier applied by BeginDisabled(). Multiply over current value of Alpha.
        out.WindowPadding = Vector2D::new(8.0, 8.0);      // Padding within a window
        out.WindowRounding = 0.0;             // Radius of window corners rounding. Set to 0.0 to have rectangular windows. Large values tend to lead to variety of artifacts and are not recommended.
        out.WindowBorderSize = 1.0;             // Thickness of border around windows. Generally set to 0.0 or 1.0. Other values not well tested.
        out.window_min_size = Vector2D::new(32.0, 32.0);    // Minimum window size
        out.window_title_align = Vector2D::new(0.0, 0.5);// Alignment for title bar text
        out.window_menu_button_position = ImGuiDir::Dir::Left;    // Position of the collapsing/docking button in the title bar (left/right). Defaults to ImGuiDir_Left.
        out.ChildRounding = 0.0;             // Radius of child window corners rounding. Set to 0.0 to have rectangular child windows
        out.ChildBorderSize = 1.0;             // Thickness of border around child windows. Generally set to 0.0 or 1.0. Other values not well tested.
        out.PopupRounding = 0.0;             // Radius of popup window corners rounding. Set to 0.0 to have rectangular child windows
        out.PopupBorderSize = 1.0;             // Thickness of border around popup or tooltip windows. Generally set to 0.0 or 1.0. Other values not well tested.
        out.frame_padding = Vector2D::new(4.0, 3.0);      // Padding within a framed rectangle (used by most widgets)
        out.FrameRounding = 0.0;             // Radius of frame corners rounding. Set to 0.0 to have rectangular frames (used by most widgets).
        out.frame_border_size = 0.0;             // Thickness of border around frames. Generally set to 0.0 or 1.0. Other values not well tested.
        out.ItemSpacing = Vector2D::new(8.0, 4.0);      // Horizontal and vertical spacing between widgets/lines
        out.item_inner_spacing = Vector2D::new(4.0, 4.0);      // Horizontal and vertical spacing between within elements of a composed widget (e.g. a slider and its label)
        out.CellPadding = Vector2D::new(4.0, 2.0);      // Padding within a table cell
        out.touch_extra_padding = Vector2D::new(0.0, 0.0);      // Expand reactive bounding box for touch-based system where touch position is not accurate enough. Unfortunately we don't sort widgets so priority on overlap will always be given to the first widget. So don't grow this too much!
        out.IndentSpacing = 21.0;            // Horizontal spacing when e.g. entering a tree node. Generally == (font_size + FramePadding.x*2).
        out.ColumnsMinSpacing = 6.0;             // Minimum horizontal spacing between two columns. Preferably > (FramePadding.x + 1).
        out.scrollbar_size = 14.0;            // width of the vertical scrollbar, height of the horizontal scrollbar
        out.ScrollbarRounding = 9.0;             // Radius of grab corners rounding for scrollbar
        out.GrabMinSize = 12.0;            // Minimum width/height of a grab box for slider/scrollbar
        out.GrabRounding = 0.0;             // Radius of grabs corners rounding. Set to 0.0 to have rectangular slider grabs.
        out.LogSliderDeadzone = 4.0;             // The size in pixels of the dead-zone around zero on logarithmic sliders that cross zero.
        out.TabRounding = 4.0;             // Radius of upper corners of a tab. Set to 0.0 to have rectangular tabs.
        out.TabBorderSize = 0.0;             // Thickness of border around tabs.
        out.TabMinWidthForCloseButton = 0.0;           // Minimum width for close button to appears on an unselected tab when hovered. Set to 0.0 to always show when hovering, set to FLT_MAX to never show close button unless selected.
        out.ColorButtonPosition = ImGuiDir::Right;   // Side of the color button in the ColorEdit4 widget (left/right). Defaults to ImGuiDir_Right.
        out.ButtonTextAlign = Vector2D::new(0.5, 0.5);// Alignment of button text when button is larger than text.
        out.SelectableTextAlign = Vector2D::new(0.0, 0.0);// Alignment of selectable text. Defaults to (0.0, 0.0) (top-left aligned). It's generally important to keep this left-aligned if you want to lay multiple items on a same line.
        out.DisplayWindowPadding = Vector2D::new(19.0, 19.0);    // window position are clamped to be visible within the display area or monitors by at least this amount. Only applies to regular windows.
        out.DisplaySafeAreaPadding = Vector2D::new(3.0, 3.0);      // If you cannot see the edge of your screen (e.g. on a TV) increase the safe area padding. Covers popups/tooltips as well regular windows.
        out.MouseCursorScale = 1.0;             // scale software rendered mouse cursor (when io.mouse_draw_cursor is enabled). May be removed later.
        out.anti_aliased_lines = true;             // Enable anti-aliased lines/borders. Disable if you are really tight on CPU/GPU.
        out.anti_aliased_lines_use_tex = true;             // Enable anti-aliased lines/borders using textures where possible. Require backend to render with bilinear filtering (NOT point/nearest filtering).
        out.anti_aliased_fill = true;             // Enable anti-aliased filled shapes (rounded rectangles, circles, etc.).
        out.curve_tessellation_tol = 1.25;            // Tessellation tolerance when using PathBezierCurveTo() without a specific number of segments. Decrease for highly tessellated curves (higher quality, more polygons), increase to reduce quality.
        out.circle_tessellation_max_error = 0.30;         // Maximum error (in pixels) allowed when using add_circle()/add_circle_filled() or drawing rounded corner rectangles with no explicit segment count specified. Decrease for higher quality but more geometry.

        // Default theme
        StyleColorsDark(&mut out);
        out
    }

    // To scale your entire UI (e.g. if you want your app to use High DPI or generally be DPI aware) you may use this helper function. Scaling the fonts is done separately and is up to you.
// Important: This operation is lossy because we round all sizes to integer. If you need to change your scale multiples, call this over a freshly initialized ImGuiStyle structure rather than scaling multiple times.
    pub fn scale_all_sizes(&mut self, scale_factor: f32) {
        self.WindowPadding = Vector2D::floor(&self.WindowPadding * scale_factor);
        self.WindowRounding = f32::floor(&self.WindowRounding * scale_factor);
        self.window_min_size = Vector2D::floor(&self.window_min_size * scale_factor);
        self.ChildRounding = f32::floor(&self.ChildRounding * scale_factor);
        self.PopupRounding = f32::floor(&self.PopupRounding * scale_factor);
        self.frame_padding = Vector2D::floor(&self.frame_padding * scale_factor);
        self.FrameRounding = f32::floor(&self.FrameRounding * scale_factor);
        self.ItemSpacing = Vector2D::floor(&self.ItemSpacing * scale_factor);
        self.item_inner_spacing = Vector2D::floor(&self.item_inner_spacing * scale_factor);
        self.CellPadding = Vector2D::floor(&self.CellPadding * scale_factor);
        self.touch_extra_padding = Vector2D::floor(&self.touch_extra_padding * scale_factor);
        self.IndentSpacing = f32::floor(self.IndentSpacing * scale_factor);
        self.ColumnsMinSpacing = f32::floor(self.ColumnsMinSpacing * scale_factor);
        self.scrollbar_size = f32::floor(self.scrollbar_size * scale_factor);
        self.ScrollbarRounding = f32::floor(self.ScrollbarRounding * scale_factor);
        self.GrabMinSize = f32::floor(self.GrabMinSize * scale_factor);
        self.GrabRounding = f32::floor(self.GrabRounding * scale_factor);
        self.LogSliderDeadzone = f32::floor(self.LogSliderDeadzone * scale_factor);
        self.TabRounding = f32::floor(self.TabRounding * scale_factor);
        self.TabMinWidthForCloseButton =
            if Self.TabMinWidthForCloseButton != f32::MAX {
                (self.TabMinWidthForCloseButton * scale_factor).floor()
            } else {
                f32::MAX
            };
        self.DisplayWindowPadding = Vector2D::floor(&self.DisplayWindowPadding * scale_factor);
        self.DisplaySafeAreaPadding = Vector2D::floor(&self.DisplaySafeAreaPadding * scale_factor);
        self.MouseCursorScale = f32::floor(self.MouseCursorScale * scale_factor);
    }
}

pub union ImGuiStyleModUnion1 {
    // union           { int BackupInt[2]; float BackupFloat[2]; };
    pub BackupInt: [i32;2],
    pub BackupFloat: [f32;2]
}

// Stacked style modifier, backup of modified data so we can restore it. data type inferred from the variable.
#[derive(Debug,Default,Clone)]
pub struct StyleMod
{
    // ImGuiStyleVar   VarIdx;
    pub VarIdx: ImGuiStyleVar,
    pub Backup: ImGuiStyleModUnion1,
}

impl StyleMod {
    // ImGuiStyleMod(ImGuiStyleVar idx, int v)     { VarIdx = idx; BackupInt[0] = v; }
    pub fn new(idx: ImGuiStyleVar, v: i32) -> Self {
        Self {
            VarIdx: idx,
            Backup: ImGuiStyleModUnion1{BackupInt: [v,0]}
        }
    }
    //     ImGuiStyleMod(ImGuiStyleVar idx, float v)   { VarIdx = idx; BackupFloat[0] = v; }
    pub fn new2(idx: ImGuiStyleVar, v: f32) -> Self {
        Self {
            VarIdx: idx,
            Backup: ImGuiStyleModUnion1{BackupFloat: [v,0]}
        }
    }
    //     ImGuiStyleMod(ImGuiStyleVar idx, Vector2D v)  { VarIdx = idx; BackupFloat[0] = v.x; BackupFloat[1] = v.y; }
    pub fn new3(idx: ImGuiStyleVar, v: Vector2D) -> Self {
        Self {
            VarIdx: idx,
            Backup: ImGuiStyleModUnion1{BackupFloat: [v.x,v.y]}
        }
    }
}


// pub fn GetStyle() -> &mut ImGuiStyle
// {
//     // IM_ASSERT(GImGui != NULL && "No current context. Did you call ImGui::CreateContext() and ImGui::SetCurrentContext() ?");
//     return &mut GImGui.style;
// }

// ImU32 ImGui::get_color_u32(ImGuiCol idx, float alpha_mul)
pub fn get_color_u32(idx: Color, alpha_mul: f32) -> u32
{
    let style = &g.style;
    let c = style.colors[idx];
    c.w *= style.alpha * alpha_mul;
    return ColorConvertFloat4ToU32(c);
}

pub fn get_color_u32_no_alpha(idx: Color) -> u32 {
    get_color_u32(color, 0.0)
}

// ImU32 ImGui::get_color_u32(const Vector4D& col)
pub fn GetColorU32_2(col: &mut Vector4D) -> u32
{
    let style = &mut g.style;
    let mut c = col;
    *c.w *= style.alpha;
    return ColorConvertFloat4ToU32(c);
}

// const Vector4D& ImGui::GetStyleColorVec4(ImGuiCol idx)
pub fn GetStyleColorVec4(idx: Color) -> Vector4D
{
    // ImGuiStyle& style = GImGui.style;
    let style = &g.style;
    style.colors[idx]
}

// ImU32 ImGui::get_color_u32(ImU32 col)
pub fn GetColorU32_3(col: u32) -> u32
{
    let style = &g.style;
    if style.alpha >= 1.0 {
        return col;
    }
    let mut a = (col & IM_COL32_A_MASK) >> IM_COL32_A_SHIFT;
    a = (a * style.alpha); // We don't need to clamp 0..255 because style.Alpha is in 0..1 range.
    (col & !IM_COL32_A_MASK) | (a << IM_COL32_A_SHIFT)
}

pub fn ColorConvertU32ToFloat4(col: u32) -> Vector4D {
    todo!()
}

// FIXME: This may incur a round-trip (if the end user got their data from a float4) but eventually we aim to store the in-flight colors as ImU32
// void ImGui::PushStyleColor(ImGuiCol idx, ImU32 col)
pub fn push_style_color(idx: &Color, col: u32)
{
    // ImGuiContext& g = *GImGui;
    let g = &GImGui;
    // ImGuiColorMod backup;
    let mut backup = ColorMod::default();
    backup.Col = idx.clone();
    backup.BackupValue = g.style.colors[idx];
    g.color_stack.push_back(backup);
    g.style.colors[idx] = ColorConvertU32ToFloat4(col);
}

// void ImGui::PushStyleColor(ImGuiCol idx, const Vector4D& col)
pub fn PushStyleColor2(idx: &Color, col: &mut Vector4D)
{
    // ImGuiContext& g = *GImGui;
    let g = &GImGui;
    // ImGuiColorMod backup;
    let mut backup = ColorMod::default();
    backup.Col = idx.clone();
    backup.BackupValue = g.style.colors[idx];
    g.color_stack.push_back(backup);
    g.style.colors[idx] = col;
}

// void ImGui::PopStyleColor(int count)
pub fn pop_style_color(mut count: i32)
{
    // ImGuiContext& g = *GImGui;
    let g = &GImGui;
    while count > 0
    {
        // ImGuiColorMod& backup = g.color_stack.back();
        let backup = g.color_stack.last().unwrap();
        g.style.colors[backup.Col.clone()] = backup.BackupValue.clone();
        g.color_stack.pop_back();
        count -= 1;
    }
}

#[derive(Default,Debug,Clone)]
pub struct ImGuiStyleVarInfo
{
    // ImGuiDataType   Type;
    pub data_type: ImGuiDataType,
    // ImU32           count;
    pub count: u32,
    // ImU32           Offset;
    pub offset: u32,
    // void*           GetVarPtr(ImGuiStyle* style) const { return (void*)((unsigned char*)style + Offset); }
}

// impl ImGuiStyleVarInfo {
//     pub fn GetVarPtr(&self, style: *mut ImGuiStyle) -> *mut c_void {
//         style + self.offset
//     }
//
//     pub fn new(data_type: ImGuiDataType, count: u32, offset: u32) -> Self {
//         Self {
//             data_type,
//             count,
//             offset
//         }
//     }
// }

// static const ImGuiCol GWindowDockStyleColors[ImGuiWindowDockStyleCol_COUNT] =
pub const GWindowDockStyleColors: [Color; 6] = [
    Color::Text, Color::Tab, Color::TabHovered, Color::TabActive, Color::TabUnfocused, Color::TabUnfocusedActive
];
//
// pub const GStyleVarInfo: [ImGuiStyleVarInfo;25] =
// [
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, Alpha) ),               // ImGuiStyleVar_Alpha
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, DisabledAlpha) ),       // ImGuiStyleVar_DisabledAlpha
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, window_padding) ),       // ImGuiStyleVar_WindowPadding
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, window_rounding) ),      // ImGuiStyleVar_WindowRounding
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, WindowBorderSize) ),    // ImGuiStyleVar_WindowBorderSize
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, WindowMinSize) ),       // ImGuiStyleVar_WindowMinSize
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, WindowTitleAlign) ),    // ImGuiStyleVar_WindowTitleAlign
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, ChildRounding) ),       // ImGuiStyleVar_ChildRounding
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, ChildBorderSize) ),     // ImGuiStyleVar_ChildBorderSize
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, PopupRounding) ),       // ImGuiStyleVar_PopupRounding
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, PopupBorderSize) ),     // ImGuiStyleVar_PopupBorderSize
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, FramePadding) ),        // ImGuiStyleVar_FramePadding
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, FrameRounding) ),       // ImGuiStyleVar_FrameRounding
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, FrameBorderSize) ),     // ImGuiStyleVar_FrameBorderSize
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, ItemSpacing) ),         // ImGuiStyleVar_ItemSpacing
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, ItemInnerSpacing) ),    // ImGuiStyleVar_ItemInnerSpacing
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, IndentSpacing) ),       // ImGuiStyleVar_IndentSpacing
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, CellPadding) ),         // ImGuiStyleVar_CellPadding
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, ScrollbarSize) ),       // ImGuiStyleVar_ScrollbarSize
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, ScrollbarRounding) ),   // ImGuiStyleVar_ScrollbarRounding
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, GrabMinSize) ),         // ImGuiStyleVar_GrabMinSize
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, GrabRounding) ),        // ImGuiStyleVar_GrabRounding
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, TabRounding) ),         // ImGuiStyleVar_TabRounding
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, ButtonTextAlign) ),     // ImGuiStyleVar_ButtonTextAlign
//     ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, SelectableTextAlign) ), // ImGuiStyleVar_SelectableTextAlign
// ];

// static const ImGuiStyleVarInfo* GetStyleVarInfo(ImGuiStyleVar idx)
// {
//     IM_ASSERT(idx >= 0 && idx < ImGuiStyleVar_COUNT);
//     IM_ASSERT(IM_ARRAYSIZE(GStyleVarInfo) == ImGuiStyleVar_COUNT);
//     return &GStyleVarInfo[idx];
// }

// void ImGui::PushStyleVar(ImGuiStyleVar idx, float val)
// {
//     const ImGuiStyleVarInfo* var_info = GetStyleVarInfo(idx);
//     if (var_info->Type == ImGuiDataType_Float && var_info->count == 1)
//     {
//         ImGuiContext& g = *GImGui;
//         float* pvar = (float*)var_info->GetVarPtr(&g.style);
//         g.style_var_stack.push_back(ImGuiStyleMod(idx, *pvar));
//         *pvar = val;
//         return;
//     }
//     IM_ASSERT(0 && "Called PushStyleVar() float variant but variable is not a float!");
// }

// void ImGui::PushStyleVar(ImGuiStyleVar idx, const Vector2D& val)
// {
//     const ImGuiStyleVarInfo* var_info = GetStyleVarInfo(idx);
//     if (var_info->Type == ImGuiDataType_Float && var_info->count == 2)
//     {
//         ImGuiContext& g = *GImGui;
//         Vector2D* pvar = (Vector2D*)var_info->GetVarPtr(&g.style);
//         g.style_var_stack.push_back(ImGuiStyleMod(idx, *pvar));
//         *pvar = val;
//         return;
//     }
//     IM_ASSERT(0 && "Called PushStyleVar() Vector2D variant but variable is not a Vector2D!");
// }

// void ImGui::PopStyleVar(int count)
// {
//     ImGuiContext& g = *GImGui;
//     while (count > 0)
//     {
//         // We avoid a generic memcpy(data, &backup.Backup.., GDataTypeSize[info->Type] * info->count), the overhead in Debug is not worth it.
//         ImGuiStyleMod& backup = g.style_var_stack.back();
//         const ImGuiStyleVarInfo* info = GetStyleVarInfo(backup.VarIdx);
//         void* data = info->GetVarPtr(&g.style);
//         if (info->Type == ImGuiDataType_Float && info->count == 1)      { ((float*)data)[0] = backup.BackupFloat[0]; }
//         else if (info->Type == ImGuiDataType_Float && info->count == 2) { ((float*)data)[0] = backup.BackupFloat[0]; ((float*)data)[1] = backup.BackupFloat[1]; }
//         g.style_var_stack.pop_back();
//         count--;
//     }
// }

pub fn GetStyleColorName(idx: &Color) -> String
{
    // Create switch- from enum with regexp: ImGuiColor::{.*}, -->  ImGuiColor::\1=> "\1";
    match idx
    {
     Color::Text=> String::from("Text"),
     Color::TextDisabled=> String::from("TextDisabled"),
     Color::WindowBg=> String::from("WindowBg"),
     Color::ChildBg=> String::from("ChildBg"),
     Color::PopupBg=> String::from("PopupBg"),
     Color::Border=> String::from("Border"),
     Color::BorderShadow=> String::from("BorderShadow"),
     Color::FrameBg=> String::from("FrameBg"),
     Color::FrameBgHovered=> String::from("FrameBgHovered"),
     Color::FrameBgActive=> String::from("FrameBgActive"),
     Color::TitleBg=> String::from("TitleBg"),
     Color::TitleBgActive=> String::from("TitleBgActive"),
     Color::TitleBgCollapsed=> String::from("TitleBgCollapsed"),
     Color::MenuBarBg=> String::from("MenuBarBg"),
     Color::ScrollbarBg=> String::from("ScrollbarBg"),
     Color::ScrollbarGrab=> String::from("ScrollbarGrab"),
     Color::ScrollbarGrabHovered=> String::from("ScrollbarGrabHovered"),
     Color::ScrollbarGrabActive=> String::from("ScrollbarGrabActive"),
     Color::CheckMark=> String::from("CheckMark"),
     Color::SliderGrab=> String::from("SliderGrab"),
     Color::SliderGrabActive=> String::from("SliderGrabActive"),
     Color::Button=> String::from("Button"),
     Color::ButtonHovered=> String::from("ButtonHovered"),
     Color::ButtonActive=> String::from("ButtonActive"),
     Color::Header=> String::from("Header"),
     Color::HeaderHovered=> String::from("HeaderHovered"),
     Color::HeaderActive=> String::from("HeaderActive"),
     Color::Separator=> String::from("Separator"),
     Color::SeparatorHovered=> String::from("SeparatorHovered"),
     Color::SeparatorActive=> String::from("SeparatorActive"),
     Color::ResizeGrip=> String::from("ResizeGrip"),
     Color::ResizeGripHovered=> String::from("ResizeGripHovered"),
     Color::ResizeGripActive=> String::from("ResizeGripActive"),
     Color::Tab=> String::from("Tab"),
     Color::TabHovered=> String::from("TabHovered"),
     Color::TabActive=> String::from("TabActive"),
     Color::TabUnfocused=> String::from("TabUnfocused"),
     Color::TabUnfocusedActive=> String::from("TabUnfocusedActive"),
     Color::DockingPreview=> String::from("DockingPreview"),
     Color::DockingEmptyBg=> String::from("DockingEmptyBg"),
     Color::PlotLines=> String::from("PlotLines"),
     Color::PlotLinesHovered=> String::from("PlotLinesHovered"),
     Color::PlotHistogram=> String::from("PlotHistogram"),
     Color::PlotHistogramHovered=> String::from("PlotHistogramHovered"),
     Color::TableHeaderBg=> String::from("TableHeaderBg"),
     Color::TableBorderStrong=> String::from("TableBorderStrong"),
     Color::TableBorderLight=> String::from("TableBorderLight"),
     Color::TableRowBg=> String::from("TableRowBg"),
     Color::TableRowBgAlt=> String::from("TableRowBgAlt"),
     Color::TextSelectedBg=> String::from("TextSelectedBg"),
     Color::DragDropTarget=> String::from("DragDropTarget"),
     Color::NavHighlight=> String::from("NavHighlight"),
     Color::NavWindowingHighlight=> String::from("NavWindowingHighlight"),
     Color::NavWindowingDimBg=> String::from("NavWindowingDimBg"),
     Color::ModalWindowDimBg=> String::from("ModalWindowDimBg"),
    }
    // String::from("Unknown")
}


// void ImGui::StyleColorsDark(ImGuiStyle* dst)
pub fn StyleColorsDark(dst: *mut Style)
{
    // ImGuiStyle* style = dst ? dst : &ImGui::GetStyle();
    let style = if dst.is_null() == false { dst } else { &g.style };
    // Vector4D* colors = style->colors;
    let colors = &mut style.colors;

    colors[Color::Text]                   = Vector4D::new(1.00, 1.00, 1.00, 1.00);
    colors[Color::TextDisabled]           = Vector4D::new(0.50, 0.50, 0.50, 1.00);
    colors[Color::WindowBg]               = Vector4D::new(0.06, 0.06, 0.06, 0.94);
    colors[Color::ChildBg]                = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[Color::PopupBg]                = Vector4D::new(0.08, 0.08, 0.08, 0.94);
    colors[Color::Border]                 = Vector4D::new(0.43, 0.43, 0.50, 0.50);
    colors[Color::BorderShadow]           = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[Color::FrameBg]                = Vector4D::new(0.16, 0.29, 0.48, 0.54);
    colors[Color::FrameBgHovered]         = Vector4D::new(0.26, 0.59, 0.98, 0.40);
    colors[Color::FrameBgActive]          = Vector4D::new(0.26, 0.59, 0.98, 0.67);
    colors[Color::TitleBg]                = Vector4D::new(0.04, 0.04, 0.04, 1.00);
    colors[Color::TitleBgActive]          = Vector4D::new(0.16, 0.29, 0.48, 1.00);
    colors[Color::TitleBgCollapsed]       = Vector4D::new(0.00, 0.00, 0.00, 0.51);
    colors[Color::MenuBarBg]              = Vector4D::new(0.14, 0.14, 0.14, 1.00);
    colors[Color::ScrollbarBg]            = Vector4D::new(0.02, 0.02, 0.02, 0.53);
    colors[Color::ScrollbarGrab]          = Vector4D::new(0.31, 0.31, 0.31, 1.00);
    colors[Color::ScrollbarGrabHovered]   = Vector4D::new(0.41, 0.41, 0.41, 1.00);
    colors[Color::ScrollbarGrabActive]    = Vector4D::new(0.51, 0.51, 0.51, 1.00);
    colors[Color::CheckMark]              = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[Color::SliderGrab]             = Vector4D::new(0.24, 0.52, 0.88, 1.00);
    colors[Color::SliderGrabActive]       = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[Color::Button]                 = Vector4D::new(0.26, 0.59, 0.98, 0.40);
    colors[Color::ButtonHovered]          = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[Color::ButtonActive]           = Vector4D::new(0.06, 0.53, 0.98, 1.00);
    colors[Color::Header]                 = Vector4D::new(0.26, 0.59, 0.98, 0.31);
    colors[Color::HeaderHovered]          = Vector4D::new(0.26, 0.59, 0.98, 0.80);
    colors[Color::HeaderActive]           = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[Color::Separator]              = colors[Color::Border];
    colors[Color::SeparatorHovered]       = Vector4D::new(0.10, 0.40, 0.75, 0.78);
    colors[Color::SeparatorActive]        = Vector4D::new(0.10, 0.40, 0.75, 1.00);
    colors[Color::ResizeGrip]             = Vector4D::new(0.26, 0.59, 0.98, 0.20);
    colors[Color::ResizeGripHovered]      = Vector4D::new(0.26, 0.59, 0.98, 0.67);
    colors[Color::ResizeGripActive]       = Vector4D::new(0.26, 0.59, 0.98, 0.95);
    colors[Color::Tab]                    = ImLerpF32(colors[Color::Header],       colors[Color::TitleBgActive], 0.80);
    colors[Color::TabHovered]             = colors[Color::HeaderHovered];
    colors[Color::TabActive]              = ImLerpF32(colors[Color::HeaderActive], colors[Color::TitleBgActive], 0.60);
    colors[Color::TabUnfocused]           = ImLerpF32(colors[Color::Tab],          colors[Color::TitleBg], 0.80);
    colors[Color::TabUnfocusedActive]     = ImLerpF32(colors[Color::TabActive],    colors[Color::TitleBg], 0.40);
    colors[Color::DockingPreview]         = colors[Color::HeaderActive] * Vector4D::new(1.0, 1.0, 1.0, 0.7);
    colors[Color::DockingEmptyBg]         = Vector4D::new(0.20, 0.20, 0.20, 1.00);
    colors[Color::PlotLines]              = Vector4D::new(0.61, 0.61, 0.61, 1.00);
    colors[Color::PlotLinesHovered]       = Vector4D::new(1.00, 0.43, 0.35, 1.00);
    colors[Color::PlotHistogram]          = Vector4D::new(0.90, 0.70, 0.00, 1.00);
    colors[Color::PlotHistogramHovered]   = Vector4D::new(1.00, 0.60, 0.00, 1.00);
    colors[Color::TableHeaderBg]          = Vector4D::new(0.19, 0.19, 0.20, 1.00);
    colors[Color::TableBorderStrong]      = Vector4D::new(0.31, 0.31, 0.35, 1.00);   // Prefer using Alpha=1.0 here
    colors[Color::TableBorderLight]       = Vector4D::new(0.23, 0.23, 0.25, 1.00);   // Prefer using Alpha=1.0 here
    colors[Color::TableRowBg]             = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[Color::TableRowBgAlt]          = Vector4D::new(1.00, 1.00, 1.00, 0.06);
    colors[Color::TextSelectedBg]         = Vector4D::new(0.26, 0.59, 0.98, 0.35);
    colors[Color::DragDropTarget]         = Vector4D::new(1.00, 1.00, 0.00, 0.90);
    colors[Color::NavHighlight]           = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[Color::NavWindowingHighlight]  = Vector4D::new(1.00, 1.00, 1.00, 0.70);
    colors[Color::NavWindowingDimBg]      = Vector4D::new(0.80, 0.80, 0.80, 0.20);
    colors[Color::ModalWindowDimBg]       = Vector4D::new(0.80, 0.80, 0.80, 0.35);
}

// void ImGui::StyleColorsClassic(ImGuiStyle* dst)
pub fn StyleColorsClassic(dst: *mut Style)
{
    // ImGuiStyle* style = dst ? dst : &ImGui::GetStyle();
    let style = if dst.is_null() == false { dst } else { &g.style };
    // Vector4D* colors = style->colors;
    let colors = &mut style.colors;
    
    colors[Color::Text]                   = Vector4D::new(0.90, 0.90, 0.90, 1.00);
    colors[Color::TextDisabled]           = Vector4D::new(0.60, 0.60, 0.60, 1.00);
    colors[Color::WindowBg]               = Vector4D::new(0.00, 0.00, 0.00, 0.85);
    colors[Color::ChildBg]                = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[Color::PopupBg]                = Vector4D::new(0.11, 0.11, 0.14, 0.92);
    colors[Color::Border]                 = Vector4D::new(0.50, 0.50, 0.50, 0.50);
    colors[Color::BorderShadow]           = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[Color::FrameBg]                = Vector4D::new(0.43, 0.43, 0.43, 0.39);
    colors[Color::FrameBgHovered]         = Vector4D::new(0.47, 0.47, 0.69, 0.40);
    colors[Color::FrameBgActive]          = Vector4D::new(0.42, 0.41, 0.64, 0.69);
    colors[Color::TitleBg]                = Vector4D::new(0.27, 0.27, 0.54, 0.83);
    colors[Color::TitleBgActive]          = Vector4D::new(0.32, 0.32, 0.63, 0.87);
    colors[Color::TitleBgCollapsed]       = Vector4D::new(0.40, 0.40, 0.80, 0.20);
    colors[Color::MenuBarBg]              = Vector4D::new(0.40, 0.40, 0.55, 0.80);
    colors[Color::ScrollbarBg]            = Vector4D::new(0.20, 0.25, 0.30, 0.60);
    colors[Color::ScrollbarGrab]          = Vector4D::new(0.40, 0.40, 0.80, 0.30);
    colors[Color::ScrollbarGrabHovered]   = Vector4D::new(0.40, 0.40, 0.80, 0.40);
    colors[Color::ScrollbarGrabActive]    = Vector4D::new(0.41, 0.39, 0.80, 0.60);
    colors[Color::CheckMark]              = Vector4D::new(0.90, 0.90, 0.90, 0.50);
    colors[Color::SliderGrab]             = Vector4D::new(1.00, 1.00, 1.00, 0.30);
    colors[Color::SliderGrabActive]       = Vector4D::new(0.41, 0.39, 0.80, 0.60);
    colors[Color::Button]                 = Vector4D::new(0.35, 0.40, 0.61, 0.62);
    colors[Color::ButtonHovered]          = Vector4D::new(0.40, 0.48, 0.71, 0.79);
    colors[Color::ButtonActive]           = Vector4D::new(0.46, 0.54, 0.80, 1.00);
    colors[Color::Header]                 = Vector4D::new(0.40, 0.40, 0.90, 0.45);
    colors[Color::HeaderHovered]          = Vector4D::new(0.45, 0.45, 0.90, 0.80);
    colors[Color::HeaderActive]           = Vector4D::new(0.53, 0.53, 0.87, 0.80);
    colors[Color::Separator]              = Vector4D::new(0.50, 0.50, 0.50, 0.60);
    colors[Color::SeparatorHovered]       = Vector4D::new(0.60, 0.60, 0.70, 1.00);
    colors[Color::SeparatorActive]        = Vector4D::new(0.70, 0.70, 0.90, 1.00);
    colors[Color::ResizeGrip]             = Vector4D::new(1.00, 1.00, 1.00, 0.10);
    colors[Color::ResizeGripHovered]      = Vector4D::new(0.78, 0.82, 1.00, 0.60);
    colors[Color::ResizeGripActive]       = Vector4D::new(0.78, 0.82, 1.00, 0.90);
    colors[Color::Tab]                    = ImLerpF32(colors[Color::Header],       colors[Color::TitleBgActive], 0.80);
    colors[Color::TabHovered]             = colors[Color::HeaderHovered];
    colors[Color::TabActive]              = ImLerpF32(colors[Color::HeaderActive], colors[Color::TitleBgActive], 0.60);
    colors[Color::TabUnfocused]           = ImLerpF32(colors[Color::Tab],          colors[Color::TitleBg], 0.80);
    colors[Color::TabUnfocusedActive]     = ImLerpF32(colors[Color::TabActive],    colors[Color::TitleBg], 0.40);
    colors[Color::DockingPreview]         = colors[Color::Header] * Vector4D::new(1.0, 1.0, 1.0, 0.7);
    colors[Color::DockingEmptyBg]         = Vector4D::new(0.20, 0.20, 0.20, 1.00);
    colors[Color::PlotLines]              = Vector4D::new(1.00, 1.00, 1.00, 1.00);
    colors[Color::PlotLinesHovered]       = Vector4D::new(0.90, 0.70, 0.00, 1.00);
    colors[Color::PlotHistogram]          = Vector4D::new(0.90, 0.70, 0.00, 1.00);
    colors[Color::PlotHistogramHovered]   = Vector4D::new(1.00, 0.60, 0.00, 1.00);
    colors[Color::TableHeaderBg]          = Vector4D::new(0.27, 0.27, 0.38, 1.00);
    colors[Color::TableBorderStrong]      = Vector4D::new(0.31, 0.31, 0.45, 1.00);   // Prefer using Alpha=1.0 here
    colors[Color::TableBorderLight]       = Vector4D::new(0.26, 0.26, 0.28, 1.00);   // Prefer using Alpha=1.0 here
    colors[Color::TableRowBg]             = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[Color::TableRowBgAlt]          = Vector4D::new(1.00, 1.00, 1.00, 0.07);
    colors[Color::TextSelectedBg]         = Vector4D::new(0.00, 0.00, 1.00, 0.35);
    colors[Color::DragDropTarget]         = Vector4D::new(1.00, 1.00, 0.00, 0.90);
    colors[Color::NavHighlight]           = colors[Color::HeaderHovered];
    colors[Color::NavWindowingHighlight]  = Vector4D::new(1.00, 1.00, 1.00, 0.70);
    colors[Color::NavWindowingDimBg]      = Vector4D::new(0.80, 0.80, 0.80, 0.20);
    colors[Color::ModalWindowDimBg]       = Vector4D::new(0.20, 0.20, 0.20, 0.35);
}

// Those light colors are better suited with a thicker font than the default one + FrameBorder
// void ImGui::StyleColorsLight(ImGuiStyle* dst)
pub fn StyleColorsLight(dst: *mut Style)
{
    // ImGuiStyle* style = dst ? dst : &ImGui::GetStyle();
    let style = if dst.is_null() == false { dst } else { &g.style };
    // Vector4D* colors = style->colors;
    let colors = &mut style.colors;

    colors[Color::Text]                   = Vector4D::new(0.00, 0.00, 0.00, 1.00);
    colors[Color::TextDisabled]           = Vector4D::new(0.60, 0.60, 0.60, 1.00);
    colors[Color::WindowBg]               = Vector4D::new(0.94, 0.94, 0.94, 1.00);
    colors[Color::ChildBg]                = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[Color::PopupBg]                = Vector4D::new(1.00, 1.00, 1.00, 0.98);
    colors[Color::Border]                 = Vector4D::new(0.00, 0.00, 0.00, 0.30);
    colors[Color::BorderShadow]           = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[Color::FrameBg]                = Vector4D::new(1.00, 1.00, 1.00, 1.00);
    colors[Color::FrameBgHovered]         = Vector4D::new(0.26, 0.59, 0.98, 0.40);
    colors[Color::FrameBgActive]          = Vector4D::new(0.26, 0.59, 0.98, 0.67);
    colors[Color::TitleBg]                = Vector4D::new(0.96, 0.96, 0.96, 1.00);
    colors[Color::TitleBgActive]          = Vector4D::new(0.82, 0.82, 0.82, 1.00);
    colors[Color::TitleBgCollapsed]       = Vector4D::new(1.00, 1.00, 1.00, 0.51);
    colors[Color::MenuBarBg]              = Vector4D::new(0.86, 0.86, 0.86, 1.00);
    colors[Color::ScrollbarBg]            = Vector4D::new(0.98, 0.98, 0.98, 0.53);
    colors[Color::ScrollbarGrab]          = Vector4D::new(0.69, 0.69, 0.69, 0.80);
    colors[Color::ScrollbarGrabHovered]   = Vector4D::new(0.49, 0.49, 0.49, 0.80);
    colors[Color::ScrollbarGrabActive]    = Vector4D::new(0.49, 0.49, 0.49, 1.00);
    colors[Color::CheckMark]              = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[Color::SliderGrab]             = Vector4D::new(0.26, 0.59, 0.98, 0.78);
    colors[Color::SliderGrabActive]       = Vector4D::new(0.46, 0.54, 0.80, 0.60);
    colors[Color::Button]                 = Vector4D::new(0.26, 0.59, 0.98, 0.40);
    colors[Color::ButtonHovered]          = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[Color::ButtonActive]           = Vector4D::new(0.06, 0.53, 0.98, 1.00);
    colors[Color::Header]                 = Vector4D::new(0.26, 0.59, 0.98, 0.31);
    colors[Color::HeaderHovered]          = Vector4D::new(0.26, 0.59, 0.98, 0.80);
    colors[Color::HeaderActive]           = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[Color::Separator]              = Vector4D::new(0.39, 0.39, 0.39, 0.62);
    colors[Color::SeparatorHovered]       = Vector4D::new(0.14, 0.44, 0.80, 0.78);
    colors[Color::SeparatorActive]        = Vector4D::new(0.14, 0.44, 0.80, 1.00);
    colors[Color::ResizeGrip]             = Vector4D::new(0.35, 0.35, 0.35, 0.17);
    colors[Color::ResizeGripHovered]      = Vector4D::new(0.26, 0.59, 0.98, 0.67);
    colors[Color::ResizeGripActive]       = Vector4D::new(0.26, 0.59, 0.98, 0.95);
    colors[Color::Tab]                    = ImLerpF32(colors[Color::Header],       colors[Color::TitleBgActive], 0.90);
    colors[Color::TabHovered]             = colors[Color::HeaderHovered];
    colors[Color::TabActive]              = ImLerpF32(colors[Color::HeaderActive], colors[Color::TitleBgActive], 0.60);
    colors[Color::TabUnfocused]           = ImLerpF32(colors[Color::Tab],          colors[Color::TitleBg], 0.80);
    colors[Color::TabUnfocusedActive]     = ImLerpF32(colors[Color::TabActive],    colors[Color::TitleBg], 0.40);
    colors[Color::DockingPreview]         = colors[Color::Header] * Vector4D::new(1.0, 1.0, 1.0, 0.7);
    colors[Color::DockingEmptyBg]         = Vector4D::new(0.20, 0.20, 0.20, 1.00);
    colors[Color::PlotLines]              = Vector4D::new(0.39, 0.39, 0.39, 1.00);
    colors[Color::PlotLinesHovered]       = Vector4D::new(1.00, 0.43, 0.35, 1.00);
    colors[Color::PlotHistogram]          = Vector4D::new(0.90, 0.70, 0.00, 1.00);
    colors[Color::PlotHistogramHovered]   = Vector4D::new(1.00, 0.45, 0.00, 1.00);
    colors[Color::TableHeaderBg]          = Vector4D::new(0.78, 0.87, 0.98, 1.00);
    colors[Color::TableBorderStrong]      = Vector4D::new(0.57, 0.57, 0.64, 1.00);   // Prefer using Alpha=1.0 here
    colors[Color::TableBorderLight]       = Vector4D::new(0.68, 0.68, 0.74, 1.00);   // Prefer using Alpha=1.0 here
    colors[Color::TableRowBg]             = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[Color::TableRowBgAlt]          = Vector4D::new(0.30, 0.30, 0.30, 0.09);
    colors[Color::TextSelectedBg]         = Vector4D::new(0.26, 0.59, 0.98, 0.35);
    colors[Color::DragDropTarget]         = Vector4D::new(0.26, 0.59, 0.98, 0.95);
    colors[Color::NavHighlight]           = colors[Color::HeaderHovered];
    colors[Color::NavWindowingHighlight]  = Vector4D::new(0.70, 0.70, 0.70, 0.70);
    colors[Color::NavWindowingDimBg]      = Vector4D::new(0.20, 0.20, 0.20, 0.20);
    colors[Color::ModalWindowDimBg]       = Vector4D::new(0.20, 0.20, 0.20, 0.35);
}

// Enumeration for PushStyleVar() / PopStyleVar() to temporarily modify the ImGuiStyle structure.
// - The enum only refers to fields of ImGuiStyle which makes sense to be pushed/popped inside UI code.
//   During initialization or between frames, feel free to just poke into ImGuiStyle directly.
// - Tip: Use your programming IDE navigation facilities on the names in the _second column_ below to find the actual members and their description.
//   In Visual Studio IDE: CTRL+comma ("Edit.GoToAll") can follow symbols in comments, whereas CTRL+F12 ("Edit.GoToImplementation") cannot.
//   With Visual Assist installed: ALT+G ("VAssistX.GoToImplementation") can also follow symbols in comments.
// - When changing this enum, you need to update the associated internal table GStyleVarInfo[] accordingly. This is where we link enum values to members offset/type.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgStyleVar
{
    // Enum name --------------------- // Member in ImGuiStyle structure (see ImGuiStyle for descriptions)
    Alpha,               // float     Alpha
    DisabledAlpha,       // float     DisabledAlpha
    WindowPadding,       // Vector2D    window_padding
    WindowRounding,      // float     window_rounding
    WindowBorderSize,    // float     WindowBorderSize
    WindowMinSize,       // Vector2D    WindowMinSize
    window_title_align,    // Vector2D    WindowTitleAlign
    ChildRounding,       // float     ChildRounding
    ChildBorderSize,     // float     ChildBorderSize
    PopupRounding,       // float     PopupRounding
    PopupBorderSize,     // float     PopupBorderSize
    frame_padding,        // Vector2D    FramePadding
    FrameRounding,       // float     FrameRounding
    FrameBorderSize,     // float     FrameBorderSize
    ItemSpacing,         // Vector2D    ItemSpacing
    item_inner_spacing,    // Vector2D    ItemInnerSpacing
    IndentSpacing,       // float     IndentSpacing
    CellPadding,         // Vector2D    CellPadding
    ScrollbarSize,       // float     ScrollbarSize
    ScrollbarRounding,   // float     ScrollbarRounding
    GrabMinSize,         // float     GrabMinSize
    GrabRounding,        // float     GrabRounding
    TabRounding,         // float     TabRounding
    ButtonTextAlign,     // Vector2D    ButtonTextAlign
    SelectableTextAlign, // Vector2D    SelectableTextAlign
    COUNT
}
