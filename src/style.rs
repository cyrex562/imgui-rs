use crate::color::{COLOR32_A_MASK, IM_COL32_A_SHIFT, StyleColor};
use crate::imgui_color::{
    color_convert_float4_to_u32, ImGuiColorMod, COLOR32_A_MASK, IM_COL32_A_SHIFT,
};
use crate::imgui_globals::GImGui;
use crate::imgui_h::{DataType, StyleColor, Direction, ImGuiStyleVar};
use crate::imgui_math::ImLerpF32;
use crate::imgui_vec::{Vector2D, Vector4D};
use crate::Context;
use crate::types::{DataType, Direction};
use crate::vectors::{Vector2D, Vector4D};

#[allow(non_snake_)]
pub struct Style {
    pub alpha: f32,
    // Global alpha applies to everything in Dear ImGui.
    pub disabled_alpha: f32,
    // Additional alpha multiplier applied by BeginDisabled(). Multiply over current value of alpha.
    pub window_padding: Vector2D,
    // Padding within a window.
    pub window_rounding: f32,
    // Radius of window corners rounding. Set to 0.0 to have rectangular windows. Large values tend to lead to variety of artifacts and are not recommended.
    pub window_border_size: f32,
    // Thickness of border around windows. Generally set to 0.0 or 1.0. (Other values are not well tested and more CPU/GPU costly).
    pub window_min_size: Vector2D,
    // Minimum window size. This is a global setting. If you want to constraint individual windows, use SetNextWindowSizeConstraints().
    pub window_title_align: Vector2D,
    // Alignment for title bar text. Defaults to (0.0,0.5) for left-aligned,vertically centered.
    pub window_menu_button_position: Direction,
    // Side of the collapsing/docking button in the title bar (None/Left/Right). Defaults to ImGuiDir_Left.
    pub child_rounding: f32,
    // Radius of child window corners rounding. Set to 0.0 to have rectangular windows.
    pub child_border_size: f32,
    // Thickness of border around child windows. Generally set to 0.0 or 1.0. (Other values are not well tested and more CPU/GPU costly).
    pub popup_rounding: f32,
    // Radius of popup window corners rounding. (Note that tooltip windows use window_rounding)
    pub popup_border_size: f32,
    // Thickness of border around popup/tooltip windows. Generally set to 0.0 or 1.0. (Other values are not well tested and more CPU/GPU costly).
    pub frame_padding: Vector2D,
    // Padding within a framed rectangle (used by most widgets).
    pub frame_rounding: f32,
    // Radius of frame corners rounding. Set to 0.0 to have rectangular frame (used by most widgets).
    pub frame_border_size: f32,
    // Thickness of border around frames. Generally set to 0.0 or 1.0. (Other values are not well tested and more CPU/GPU costly).
    pub item_spacing: Vector2D,
    // Horizontal and vertical spacing between widgets/lines.
    pub item_inner_spacing: Vector2D,
    // Horizontal and vertical spacing between within elements of a composed widget (e.g. a slider and its label).
    pub cell_padding: Vector2D,
    // Padding within a table cell
    pub touch_extra_padding: Vector2D,
    // Expand reactive bounding box for touch-based system where touch position is not accurate enough. Unfortunately we don't sort widgets so priority on overlap will always be given to the first widget. So don't grow this too much!
    pub indent_spacing: f32,
    // Horizontal indentation when e.g. entering a tree node. Generally == (font_size + FramePadding.x*2).
    pub columns_min_spacing: f32,
    // Minimum horizontal spacing between two columns. Preferably > (FramePadding.x + 1).
    pub scrollbar_size: f32,
    // width of the vertical scrollbar, height of the horizontal scrollbar.
    pub scrollbar_rounding: f32,
    // Radius of grab corners for scrollbar.
    pub grab_min_size: f32,
    // Minimum width/height of a grab box for slider/scrollbar.
    pub grab_rounding: f32,
    // Radius of grabs corners rounding. Set to 0.0 to have rectangular slider grabs.
    pub log_slider_deadzone: f32,
    // The size in pixels of the dead-zone around zero on logarithmic sliders that cross zero.
    pub tab_rounding: f32,
    // Radius of upper corners of a tab. Set to 0.0 to have rectangular tabs.
    pub tab_border_size: f32,
    // Thickness of border around tabs.
    pub tab_min_width_for_close_button: f32,
    // Minimum width for close button to appears on an unselected tab when hovered. Set to 0.0 to always show when hovering, set to FLT_MAX to never show close button unless selected.
    pub color_button_position: Direction,
    // Side of the color button in the ColorEdit4 widget (left/right). Defaults to ImGuiDir_Right.
    pub button_text_align: Vector2D,
    // Alignment of button text when button is larger than text. Defaults to (0.5, 0.5) (centered).
    pub selectable_text_align: Vector2D,
    // Alignment of selectable text. Defaults to (0.0, 0.0) (top-left aligned). It's generally important to keep this left-aligned if you want to lay multiple items on a same line.
    pub display_window_padding: Vector2D,
    // window position are clamped to be visible within the display area or monitors by at least this amount. Only applies to regular windows.
    pub display_safe_area_padding: Vector2D,
    // If you cannot see the edges of your screen (e.g. on a TV) increase the safe area padding. Apply to popups/tooltips as well regular windows. NB: Prefer configuring your TV sets correctly!
    pub mouse_cursor_scale: f32,
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
    pub colors: Vec<StyleColor>,
    //  ImGuiStyle();
    //  void ScaleAllSizes(float scale_factor);
}

impl Style {
    pub fn new() -> Self {
        let mut out = Self {
            ..Default::default()
        };
        out.alpha = 1.0; // Global alpha applies to everything in Dear ImGui.
        out.disabled_alpha = 0.60; // Additional alpha multiplier applied by BeginDisabled(). Multiply over current value of alpha.
        out.window_padding = Vector2D::new(8.0, 8.0); // Padding within a window
        out.window_rounding = 0.0; // Radius of window corners rounding. Set to 0.0 to have rectangular windows. Large values tend to lead to variety of artifacts and are not recommended.
        out.window_border_size = 1.0; // Thickness of border around windows. Generally set to 0.0 or 1.0. Other values not well tested.
        out.window_min_size = Vector2D::new(32.0, 32.0); // Minimum window size
        out.window_title_align = Vector2D::new(0.0, 0.5); // Alignment for title bar text
        out.window_menu_button_position = ImGuiDirection::Direction::Left; // Position of the collapsing/docking button in the title bar (left/right). Defaults to ImGuiDir_Left.
        out.child_rounding = 0.0; // Radius of child window corners rounding. Set to 0.0 to have rectangular child windows
        out.child_border_size = 1.0; // Thickness of border around child windows. Generally set to 0.0 or 1.0. Other values not well tested.
        out.popup_rounding = 0.0; // Radius of popup window corners rounding. Set to 0.0 to have rectangular child windows
        out.popup_border_size = 1.0; // Thickness of border around popup or tooltip windows. Generally set to 0.0 or 1.0. Other values not well tested.
        out.frame_padding = Vector2D::new(4.0, 3.0); // Padding within a framed rectangle (used by most widgets)
        out.frame_rounding = 0.0; // Radius of frame corners rounding. Set to 0.0 to have rectangular frames (used by most widgets).
        out.frame_border_size = 0.0; // Thickness of border around frames. Generally set to 0.0 or 1.0. Other values not well tested.
        out.item_spacing = Vector2D::new(8.0, 4.0); // Horizontal and vertical spacing between widgets/lines
        out.item_inner_spacing = Vector2D::new(4.0, 4.0); // Horizontal and vertical spacing between within elements of a composed widget (e.g. a slider and its label)
        out.cell_padding = Vector2D::new(4.0, 2.0); // Padding within a table cell
        out.touch_extra_padding = Vector2D::new(0.0, 0.0); // Expand reactive bounding box for touch-based system where touch position is not accurate enough. Unfortunately we don't sort widgets so priority on overlap will always be given to the first widget. So don't grow this too much!
        out.indent_spacing = 21.0; // Horizontal spacing when e.g. entering a tree node. Generally == (font_size + FramePadding.x*2).
        out.columns_min_spacing = 6.0; // Minimum horizontal spacing between two columns. Preferably > (FramePadding.x + 1).
        out.scrollbar_size = 14.0; // width of the vertical scrollbar, height of the horizontal scrollbar
        out.scrollbar_rounding = 9.0; // Radius of grab corners rounding for scrollbar
        out.grab_min_size = 12.0; // Minimum width/height of a grab box for slider/scrollbar
        out.grab_rounding = 0.0; // Radius of grabs corners rounding. Set to 0.0 to have rectangular slider grabs.
        out.log_slider_deadzone = 4.0; // The size in pixels of the dead-zone around zero on logarithmic sliders that cross zero.
        out.tab_rounding = 4.0; // Radius of upper corners of a tab. Set to 0.0 to have rectangular tabs.
        out.tab_border_size = 0.0; // Thickness of border around tabs.
        out.tab_min_width_for_close_button = 0.0; // Minimum width for close button to appears on an unselected tab when hovered. Set to 0.0 to always show when hovering, set to FLT_MAX to never show close button unless selected.
        out.color_button_position = ImGuiDirection::Right; // Side of the color button in the ColorEdit4 widget (left/right). Defaults to ImGuiDir_Right.
        out.button_text_align = Vector2D::new(0.5, 0.5); // Alignment of button text when button is larger than text.
        out.selectable_text_align = Vector2D::new(0.0, 0.0); // Alignment of selectable text. Defaults to (0.0, 0.0) (top-left aligned). It's generally important to keep this left-aligned if you want to lay multiple items on a same line.
        out.display_window_padding = Vector2D::new(19.0, 19.0); // window position are clamped to be visible within the display area or monitors by at least this amount. Only applies to regular windows.
        out.display_safe_area_padding = Vector2D::new(3.0, 3.0); // If you cannot see the edge of your screen (e.g. on a TV) increase the safe area padding. Covers popups/tooltips as well regular windows.
        out.mouse_cursor_scale = 1.0; // scale software rendered mouse cursor (when io.mouse_draw_cursor is enabled). May be removed later.
        out.anti_aliased_lines = true; // Enable anti-aliased lines/borders. Disable if you are really tight on CPU/GPU.
        out.anti_aliased_lines_use_tex = true; // Enable anti-aliased lines/borders using textures where possible. Require backend to render with bilinear filtering (NOT point/nearest filtering).
        out.anti_aliased_fill = true; // Enable anti-aliased filled shapes (rounded rectangles, circles, etc.).
        out.curve_tessellation_tol = 1.25; // Tessellation tolerance when using PathBezierCurveTo() without a specific number of segments. Decrease for highly tessellated curves (higher quality, more polygons), increase to reduce quality.
        out.circle_tessellation_max_error = 0.30; // Maximum error (in pixels) allowed when using add_circle()/add_circle_filled() or drawing rounded corner rectangles with no explicit segment count specified. Decrease for higher quality but more geometry.

        // Default theme
        StyleColorsDark(&mut out);
        out
    }

    // To scale your entire UI (e.g. if you want your app to use High DPI or generally be DPI aware) you may use this helper function. Scaling the fonts is done separately and is up to you.
    // Important: This operation is lossy because we round all sizes to integer. If you need to change your scale multiples, call this over a freshly initialized ImGuiStyle structure rather than scaling multiple times.
    pub fn scale_all_sizes(&mut self, scale_factor: f32) {
        self.window_padding = Vector2D::floor(&self.window_padding * scale_factor);
        self.window_rounding = f32::floor(&self.window_rounding * scale_factor);
        self.window_min_size = Vector2D::floor(&self.window_min_size * scale_factor);
        self.child_rounding = f32::floor(&self.child_rounding * scale_factor);
        self.popup_rounding = f32::floor(&self.popup_rounding * scale_factor);
        self.frame_padding = Vector2D::floor(&self.frame_padding * scale_factor);
        self.frame_rounding = f32::floor(&self.frame_rounding * scale_factor);
        self.item_spacing = Vector2D::floor(&self.item_spacing * scale_factor);
        self.item_inner_spacing = Vector2D::floor(&self.item_inner_spacing * scale_factor);
        self.cell_padding = Vector2D::floor(&self.cell_padding * scale_factor);
        self.touch_extra_padding = Vector2D::floor(&self.touch_extra_padding * scale_factor);
        self.indent_spacing = f32::floor(self.indent_spacing * scale_factor);
        self.columns_min_spacing = f32::floor(self.columns_min_spacing * scale_factor);
        self.scrollbar_size = f32::floor(self.scrollbar_size * scale_factor);
        self.scrollbar_rounding = f32::floor(self.scrollbar_rounding * scale_factor);
        self.grab_min_size = f32::floor(self.grab_min_size * scale_factor);
        self.grab_rounding = f32::floor(self.grab_rounding * scale_factor);
        self.log_slider_deadzone = f32::floor(self.log_slider_deadzone * scale_factor);
        self.tab_rounding = f32::floor(self.tab_rounding * scale_factor);
        self.tab_min_width_for_close_button = if Self.tab_min_width_for_close_button != f32::MAX {
            (self.tab_min_width_for_close_button * scale_factor).floor()
        } else {
            f32::MAX
        };
        self.display_window_padding = Vector2D::floor(&self.display_window_padding * scale_factor);
        self.display_safe_area_padding = Vector2D::floor(&self.display_safe_area_padding * scale_factor);
        self.mouse_cursor_scale = f32::floor(self.mouse_cursor_scale * scale_factor);
    }
}

pub union ImGuiStyleModUnion1 {
    // union           { int BackupInt[2]; float BackupFloat[2]; };
    pub BackupInt: [i32; 2],
    pub BackupFloat: [f32; 2],
}

// Stacked style modifier, backup of modified data so we can restore it. data type inferred from the variable.
#[derive(Debug, Default, Clone)]
pub struct StyleMod {
    // ImGuiStyleVar   VarIdx;
    pub VarIdx: ImGuiStyleVar,
    pub Backup: ImGuiStyleModUnion1,
}

impl StyleMod {
    // ImGuiStyleMod(ImGuiStyleVar idx, int v)     { VarIdx = idx; BackupInt[0] = v; }
    pub fn new(idx: ImGuiStyleVar, v: i32) -> Self {
        Self {
            VarIdx: idx,
            Backup: ImGuiStyleModUnion1 { BackupInt: [v, 0] },
        }
    }
    //     ImGuiStyleMod(ImGuiStyleVar idx, float v)   { VarIdx = idx; BackupFloat[0] = v; }
    pub fn new2(idx: ImGuiStyleVar, v: f32) -> Self {
        Self {
            VarIdx: idx,
            Backup: ImGuiStyleModUnion1 {
                BackupFloat: [v, 0],
            },
        }
    }
    //     ImGuiStyleMod(ImGuiStyleVar idx, Vector2D v)  { VarIdx = idx; BackupFloat[0] = v.x; BackupFloat[1] = v.y; }
    pub fn new3(idx: ImGuiStyleVar, v: Vector2D) -> Self {
        Self {
            VarIdx: idx,
            Backup: ImGuiStyleModUnion1 {
                BackupFloat: [v.x, v.y],
            },
        }
    }
}

// pub fn GetStyle() -> &mut ImGuiStyle
// {
//     // IM_ASSERT(GImGui != None && "No current context. Did you call ImGui::CreateContext() and ImGui::SetCurrentContext() ?");
//     return &mut GImGui.style;
// }

// ImU32 ImGui::get_color_u32(ImGuiCol idx, float alpha_mul)
pub fn color_u32_from_style_color(g: &mut Context, idx: StyleColor) -> u32 {
    let style = &g.style;
    let c = style.colors[idx];
    c.w *= style.alpha * alpha_mul;
    return color_convert_float4_to_u32(c);
}



// pub fn get_color_u32_no_alpha(idx: StyleColor) -> u32 {
//     get_color_u32(color, 0.0)
// }

pub fn color_u32_from_style_color_with_alpha(g: &mut Context, idx: StyleColor, alpha_mul: f32) -> u32 {
    let style = &g.style;
    let c = style.colors[idx];
    c.w *= style.alpha * alpha_mul;
    return color_convert_float4_to_u32(c);
}

// ImU32 ImGui::get_color_u32(const Vector4D& col)
pub fn color_u32_from_vec4d(g: &mut Context, col: &mut Vector4D) -> u32 {
    let style = &mut g.style;
    let mut c = col;
    *c.w *= style.alpha;
    return color_convert_float4_to_u32(c);
}

// const Vector4D& ImGui::GetStyleColorVec4(ImGuiCol idx)
pub fn style_color_to_vec4d(g: &mut Context, idx: StyleColor) -> Vector4D {
    // ImGuiStyle& style = GImGui.style;
    let style = &g.style;
    style.colors[idx]
}

// ImU32 ImGui::get_color_u32(ImU32 col)
pub fn color_u32_from_u32(g: &mut Context, col: u32) -> u32 {
    let style = &g.style;
    if style.alpha >= 1.0 {
        return col;
    }
    let mut a = (col & COLOR32_A_MASK) >> IM_COL32_A_SHIFT;
    a = (a * style.alpha); // We don't need to clamp 0..255 because style.alpha is in 0..1 range.
    (col & !COLOR32_A_MASK) | (a << IM_COL32_A_SHIFT)
}

pub fn color_convert_u32_to_float4(col: u32) -> Vector4D {
    todo!()
}

// FIXME: This may incur a round-trip (if the end user got their data from a float4) but eventually we aim to store the in-flight colors as ImU32
// void ImGui::PushStyleColor(ImGuiCol idx, ImU32 col)
pub fn push_style_color(g: &mut Context, idx: StyleColor, color: u32) {
    // ImGuiContext& g = *GImGui;
    // let g = &GImGui;
    // ImGuiColorMod backup;
    let mut backup = ImGuiColorMod::default();
    backup.Col = idx.clone();
    backup.BackupValue = g.style.colors[idx];
    g.color_stack.push_back(backup);
    g.style.colors[&idx] = color_convert_u32_to_float4(color);
}

// void ImGui::PushStyleColor(ImGuiCol idx, const Vector4D& col)
pub fn push_style_color2(idx: &StyleColor, col: &mut Vector4D) {
    // ImGuiContext& g = *GImGui;
    let g = &GImGui;
    // ImGuiColorMod backup;
    let mut backup = ImGuiColorMod::default();
    backup.Col = idx.clone();
    backup.BackupValue = g.style.colors[idx];
    g.color_stack.push_back(backup);
    g.style.colors[idx] = col;
}

// void ImGui::PopStyleColor(int count)
pub fn pop_style_color(mut count: i32) {
    // ImGuiContext& g = *GImGui;
    let g = &GImGui;
    while count > 0 {
        // ImGuiColorMod& backup = g.color_stack.back();
        let backup = g.color_stack.last().unwrap();
        g.style.colors[backup.Col.clone()] = backup.BackupValue.clone();
        g.color_stack.pop_back();
        count -= 1;
    }
}

#[derive(Default, Debug, Clone)]
pub struct ImGuiStyleVarInfo {
    // DataType   Type;
    pub data_type: DataType,
    // ImU32           Count;
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
//     pub fn new(data_type: DataType, count: u32, offset: u32) -> Self {
//         Self {
//             data_type,
//             count,
//             offset
//         }
//     }
// }

// static const ImGuiCol GWindowDockStyleColors[WindowDockStyleCol_COUNT] =
pub const WINDOW_DOCK_STYLE_COLORS: [StyleColor; 6] = [
    StyleColor::Text,
    StyleColor::Tab,
    StyleColor::TabHovered,
    StyleColor::TabActive,
    StyleColor::TabUnfocused,
    StyleColor::TabUnfocusedActive,
];
//
// pub const STYLE_VAR_INFO: [ImGuiStyleVarInfo;25] =
// [
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, alpha) ),               // ImGuiStyleVar_Alpha
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, DisabledAlpha) ),       // ImGuiStyleVar_DisabledAlpha
//     ImGuiStyleVarInfo::new( DataType::Float, 2, IM_OFFSETOF(ImGuiStyle, window_padding) ),       // ImGuiStyleVar_WindowPadding
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, window_rounding) ),      // ImGuiStyleVar_WindowRounding
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, window_border_size) ),    // ImGuiStyleVar_WindowBorderSize
//     ImGuiStyleVarInfo::new( DataType::Float, 2, IM_OFFSETOF(ImGuiStyle, window_min_size) ),       // ImGuiStyleVar_WindowMinSize
//     ImGuiStyleVarInfo::new( DataType::Float, 2, IM_OFFSETOF(ImGuiStyle, WindowTitleAlign) ),    // ImGuiStyleVar_WindowTitleAlign
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, child_rounding) ),       // ImGuiStyleVar_ChildRounding
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, child_border_size) ),     // ImGuiStyleVar_ChildBorderSize
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, popup_rounding) ),       // ImGuiStyleVar_PopupRounding
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, popup_border_size) ),     // ImGuiStyleVar_PopupBorderSize
//     ImGuiStyleVarInfo::new( DataType::Float, 2, IM_OFFSETOF(ImGuiStyle, FramePadding) ),        // ImGuiStyleVar_FramePadding
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, frame_rounding) ),       // ImGuiStyleVar_FrameRounding
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, frame_border_size) ),     // ImGuiStyleVar_FrameBorderSize
//     ImGuiStyleVarInfo::new( DataType::Float, 2, IM_OFFSETOF(ImGuiStyle, item_spacing) ),         // ImGuiStyleVar_ItemSpacing
//     ImGuiStyleVarInfo::new( DataType::Float, 2, IM_OFFSETOF(ImGuiStyle, ItemInnerSpacing) ),    // ImGuiStyleVar_ItemInnerSpacing
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, indent_spacing) ),       // ImGuiStyleVar_IndentSpacing
//     ImGuiStyleVarInfo::new( DataType::Float, 2, IM_OFFSETOF(ImGuiStyle, cell_padding) ),         // ImGuiStyleVar_CellPadding
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, scrollbar_size) ),       // ImGuiStyleVar_ScrollbarSize
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, scrollbar_rounding) ),   // ImGuiStyleVar_ScrollbarRounding
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, grab_min_size) ),         // ImGuiStyleVar_GrabMinSize
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, grab_rounding) ),        // ImGuiStyleVar_GrabRounding
//     ImGuiStyleVarInfo::new( DataType::Float, 1, IM_OFFSETOF(ImGuiStyle, tab_rounding) ),         // ImGuiStyleVar_TabRounding
//     ImGuiStyleVarInfo::new( DataType::Float, 2, IM_OFFSETOF(ImGuiStyle, button_text_align) ),     // ImGuiStyleVar_ButtonTextAlign
//     ImGuiStyleVarInfo::new( DataType::Float, 2, IM_OFFSETOF(ImGuiStyle, selectable_text_align) ), // ImGuiStyleVar_selectableTextAlign
// ];

// static const ImGuiStyleVarInfo* get_style_var_info(ImGuiStyleVar idx)
// {
//     IM_ASSERT(idx >= 0 && idx < ImGuiStyleVar_COUNT);
//     IM_ASSERT(IM_ARRAYSIZE(STYLE_VAR_INFO) == ImGuiStyleVar_COUNT);
//     return &STYLE_VAR_INFO[idx];
// }

// void ImGui::PushStyleVar(ImGuiStyleVar idx, float val)
// {
//     const ImGuiStyleVarInfo* var_info = get_style_var_info(idx);
//     if (var_info->Type == DataType::Float && var_info->Count == 1)
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
//     const ImGuiStyleVarInfo* var_info = get_style_var_info(idx);
//     if (var_info->Type == DataType::Float && var_info->Count == 2)
//     {
//         ImGuiContext& g = *GImGui;
//         Vector2D* pvar = var_info->GetVarPtr(&g.style);
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
//         // We avoid a generic memcpy(data, &backup.Backup.., GDataTypeSize[info->Type] * info->Count), the overhead in Debug is not worth it.
//         ImGuiStyleMod& backup = g.style_var_stack.back();
//         const ImGuiStyleVarInfo* info = get_style_var_info(backup.VarIdx);
//         void* data = info->GetVarPtr(&g.style);
//         if (info->Type == DataType::Float && info->Count == 1)      { ((float*)data)[0] = backup.BackupFloat[0]; }
//         else if (info->Type == DataType::Float && info->Count == 2) { ((float*)data)[0] = backup.BackupFloat[0]; ((float*)data)[1] = backup.BackupFloat[1]; }
//         g.style_var_stack.pop_back();
//         count--;
//     }
// }

pub fn GetStyleColorName(idx: &StyleColor) -> String {
    // Create switch- from enum with regexp: ImGuiColor::{.*}, -->  ImGuiColor::\1=> "\1";
    match idx {
        StyleColor::Text => String::from("Text"),
        StyleColor::TextDisabled => String::from("TextDisabled"),
        StyleColor::WindowBg => String::from("WindowBg"),
        StyleColor::ChildBg => String::from("ChildBg"),
        StyleColor::PopupBg => String::from("PopupBg"),
        StyleColor::Border => String::from("Border"),
        StyleColor::BorderShadow => String::from("BorderShadow"),
        StyleColor::FrameBg => String::from("FrameBg"),
        StyleColor::FrameBgHovered => String::from("FrameBgHovered"),
        StyleColor::FrameBgActive => String::from("FrameBgActive"),
        StyleColor::TitleBg => String::from("TitleBg"),
        StyleColor::TitleBgActive => String::from("TitleBgActive"),
        StyleColor::TitleBgCollapsed => String::from("TitleBgCollapsed"),
        StyleColor::MenuBarBg => String::from("MenuBarBg"),
        StyleColor::ScrollbarBg => String::from("ScrollbarBg"),
        StyleColor::ScrollbarGrab => String::from("ScrollbarGrab"),
        StyleColor::ScrollbarGrabHovered => String::from("ScrollbarGrabHovered"),
        StyleColor::ScrollbarGrabActive => String::from("ScrollbarGrabActive"),
        StyleColor::CheckMark => String::from("CheckMark"),
        StyleColor::SliderGrab => String::from("SliderGrab"),
        StyleColor::SliderGrabActive => String::from("SliderGrabActive"),
        StyleColor::Button => String::from("Button"),
        StyleColor::ButtonHovered => String::from("ButtonHovered"),
        StyleColor::ButtonActive => String::from("ButtonActive"),
        StyleColor::Header => String::from("Header"),
        StyleColor::HeaderHovered => String::from("HeaderHovered"),
        StyleColor::HeaderActive => String::from("HeaderActive"),
        StyleColor::Separator => String::from("Separator"),
        StyleColor::SeparatorHovered => String::from("SeparatorHovered"),
        StyleColor::SeparatorActive => String::from("SeparatorActive"),
        StyleColor::ResizeGrip => String::from("ResizeGrip"),
        StyleColor::ResizeGripHovered => String::from("ResizeGripHovered"),
        StyleColor::ResizeGripActive => String::from("ResizeGripActive"),
        StyleColor::Tab => String::from("Tab"),
        StyleColor::TabHovered => String::from("TabHovered"),
        StyleColor::TabActive => String::from("TabActive"),
        StyleColor::TabUnfocused => String::from("TabUnfocused"),
        StyleColor::TabUnfocusedActive => String::from("TabUnfocusedActive"),
        StyleColor::DockingPreview => String::from("DockingPreview"),
        StyleColor::DockingEmptyBg => String::from("DockingEmptyBg"),
        StyleColor::PlotLines => String::from("PlotLines"),
        StyleColor::PlotLinesHovered => String::from("PlotLinesHovered"),
        StyleColor::PlotHistogram => String::from("PlotHistogram"),
        StyleColor::PlotHistogramHovered => String::from("PlotHistogramHovered"),
        StyleColor::TableHeaderBg => String::from("TableHeaderBg"),
        StyleColor::TableBorderStrong => String::from("TableBorderStrong"),
        StyleColor::TableBorderLight => String::from("TableBorderLight"),
        StyleColor::TableRowBg => String::from("TableRowBg"),
        StyleColor::TableRowBgAlt => String::from("TableRowBgAlt"),
        StyleColor::TextSelectedBg => String::from("TextSelectedBg"),
        StyleColor::DragDropTarget => String::from("DragDropTarget"),
        StyleColor::NavHighlight => String::from("NavHighlight"),
        StyleColor::NavWindowingHighlight => String::from("NavWindowingHighlight"),
        StyleColor::NavWindowingDimBg => String::from("NavWindowingDimBg"),
        StyleColor::ModalWindowDimBg => String::from("ModalWindowDimBg"),
    }
    // String::from("Unknown")
}

// void ImGui::StyleColorsDark(ImGuiStyle* dst)
pub fn StyleColorsDark(dst: *mut Style) {
    // ImGuiStyle* style = dst ? dst : &ImGui::GetStyle();
    let style = if dst.is_null() == false {
        dst
    } else {
        &g.style
    };
    // Vector4D* colors = style->colors;
    let colors = &mut style.colors;

    colors[StyleColor::Text] = Vector4D::new(1.00, 1.00, 1.00, 1.00);
    colors[StyleColor::TextDisabled] = Vector4D::new(0.50, 0.50, 0.50, 1.00);
    colors[StyleColor::WindowBg] = Vector4D::new(0.06, 0.06, 0.06, 0.94);
    colors[StyleColor::ChildBg] = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[StyleColor::PopupBg] = Vector4D::new(0.08, 0.08, 0.08, 0.94);
    colors[StyleColor::Border] = Vector4D::new(0.43, 0.43, 0.50, 0.50);
    colors[StyleColor::BorderShadow] = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[StyleColor::FrameBg] = Vector4D::new(0.16, 0.29, 0.48, 0.54);
    colors[StyleColor::FrameBgHovered] = Vector4D::new(0.26, 0.59, 0.98, 0.40);
    colors[StyleColor::FrameBgActive] = Vector4D::new(0.26, 0.59, 0.98, 0.67);
    colors[StyleColor::TitleBg] = Vector4D::new(0.04, 0.04, 0.04, 1.00);
    colors[StyleColor::TitleBgActive] = Vector4D::new(0.16, 0.29, 0.48, 1.00);
    colors[StyleColor::TitleBgCollapsed] = Vector4D::new(0.00, 0.00, 0.00, 0.51);
    colors[StyleColor::MenuBarBg] = Vector4D::new(0.14, 0.14, 0.14, 1.00);
    colors[StyleColor::ScrollbarBg] = Vector4D::new(0.02, 0.02, 0.02, 0.53);
    colors[StyleColor::ScrollbarGrab] = Vector4D::new(0.31, 0.31, 0.31, 1.00);
    colors[StyleColor::ScrollbarGrabHovered] = Vector4D::new(0.41, 0.41, 0.41, 1.00);
    colors[StyleColor::ScrollbarGrabActive] = Vector4D::new(0.51, 0.51, 0.51, 1.00);
    colors[StyleColor::CheckMark] = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[StyleColor::SliderGrab] = Vector4D::new(0.24, 0.52, 0.88, 1.00);
    colors[StyleColor::SliderGrabActive] = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[StyleColor::Button] = Vector4D::new(0.26, 0.59, 0.98, 0.40);
    colors[StyleColor::ButtonHovered] = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[StyleColor::ButtonActive] = Vector4D::new(0.06, 0.53, 0.98, 1.00);
    colors[StyleColor::Header] = Vector4D::new(0.26, 0.59, 0.98, 0.31);
    colors[StyleColor::HeaderHovered] = Vector4D::new(0.26, 0.59, 0.98, 0.80);
    colors[StyleColor::HeaderActive] = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[StyleColor::Separator] = colors[StyleColor::Border];
    colors[StyleColor::SeparatorHovered] = Vector4D::new(0.10, 0.40, 0.75, 0.78);
    colors[StyleColor::SeparatorActive] = Vector4D::new(0.10, 0.40, 0.75, 1.00);
    colors[StyleColor::ResizeGrip] = Vector4D::new(0.26, 0.59, 0.98, 0.20);
    colors[StyleColor::ResizeGripHovered] = Vector4D::new(0.26, 0.59, 0.98, 0.67);
    colors[StyleColor::ResizeGripActive] = Vector4D::new(0.26, 0.59, 0.98, 0.95);
    colors[StyleColor::Tab] = ImLerpF32(
        colors[StyleColor::Header],
        colors[StyleColor::TitleBgActive],
        0.80,
    );
    colors[StyleColor::TabHovered] = colors[StyleColor::HeaderHovered];
    colors[StyleColor::TabActive] = ImLerpF32(
        colors[StyleColor::HeaderActive],
        colors[StyleColor::TitleBgActive],
        0.60,
    );
    colors[StyleColor::TabUnfocused] =
        ImLerpF32(colors[StyleColor::Tab], colors[StyleColor::TitleBg], 0.80);
    colors[StyleColor::TabUnfocusedActive] = ImLerpF32(
        colors[StyleColor::TabActive],
        colors[StyleColor::TitleBg],
        0.40,
    );
    colors[StyleColor::DockingPreview] =
        colors[StyleColor::HeaderActive] * Vector4D::new(1.0, 1.0, 1.0, 0.7);
    colors[StyleColor::DockingEmptyBg] = Vector4D::new(0.20, 0.20, 0.20, 1.00);
    colors[StyleColor::PlotLines] = Vector4D::new(0.61, 0.61, 0.61, 1.00);
    colors[StyleColor::PlotLinesHovered] = Vector4D::new(1.00, 0.43, 0.35, 1.00);
    colors[StyleColor::PlotHistogram] = Vector4D::new(0.90, 0.70, 0.00, 1.00);
    colors[StyleColor::PlotHistogramHovered] = Vector4D::new(1.00, 0.60, 0.00, 1.00);
    colors[StyleColor::TableHeaderBg] = Vector4D::new(0.19, 0.19, 0.20, 1.00);
    colors[StyleColor::TableBorderStrong] = Vector4D::new(0.31, 0.31, 0.35, 1.00); // Prefer using alpha=1.0 here
    colors[StyleColor::TableBorderLight] = Vector4D::new(0.23, 0.23, 0.25, 1.00); // Prefer using alpha=1.0 here
    colors[StyleColor::TableRowBg] = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[StyleColor::TableRowBgAlt] = Vector4D::new(1.00, 1.00, 1.00, 0.06);
    colors[StyleColor::TextSelectedBg] = Vector4D::new(0.26, 0.59, 0.98, 0.35);
    colors[StyleColor::DragDropTarget] = Vector4D::new(1.00, 1.00, 0.00, 0.90);
    colors[StyleColor::NavHighlight] = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[StyleColor::NavWindowingHighlight] = Vector4D::new(1.00, 1.00, 1.00, 0.70);
    colors[StyleColor::NavWindowingDimBg] = Vector4D::new(0.80, 0.80, 0.80, 0.20);
    colors[StyleColor::ModalWindowDimBg] = Vector4D::new(0.80, 0.80, 0.80, 0.35);
}

// void ImGui::StyleColorsClassic(ImGuiStyle* dst)
pub fn StyleColorsClassic(dst: *mut Style) {
    // ImGuiStyle* style = dst ? dst : &ImGui::GetStyle();
    let style = if dst.is_null() == false {
        dst
    } else {
        &g.style
    };
    // Vector4D* colors = style->colors;
    let colors = &mut style.colors;

    colors[StyleColor::Text] = Vector4D::new(0.90, 0.90, 0.90, 1.00);
    colors[StyleColor::TextDisabled] = Vector4D::new(0.60, 0.60, 0.60, 1.00);
    colors[StyleColor::WindowBg] = Vector4D::new(0.00, 0.00, 0.00, 0.85);
    colors[StyleColor::ChildBg] = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[StyleColor::PopupBg] = Vector4D::new(0.11, 0.11, 0.14, 0.92);
    colors[StyleColor::Border] = Vector4D::new(0.50, 0.50, 0.50, 0.50);
    colors[StyleColor::BorderShadow] = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[StyleColor::FrameBg] = Vector4D::new(0.43, 0.43, 0.43, 0.39);
    colors[StyleColor::FrameBgHovered] = Vector4D::new(0.47, 0.47, 0.69, 0.40);
    colors[StyleColor::FrameBgActive] = Vector4D::new(0.42, 0.41, 0.64, 0.69);
    colors[StyleColor::TitleBg] = Vector4D::new(0.27, 0.27, 0.54, 0.83);
    colors[StyleColor::TitleBgActive] = Vector4D::new(0.32, 0.32, 0.63, 0.87);
    colors[StyleColor::TitleBgCollapsed] = Vector4D::new(0.40, 0.40, 0.80, 0.20);
    colors[StyleColor::MenuBarBg] = Vector4D::new(0.40, 0.40, 0.55, 0.80);
    colors[StyleColor::ScrollbarBg] = Vector4D::new(0.20, 0.25, 0.30, 0.60);
    colors[StyleColor::ScrollbarGrab] = Vector4D::new(0.40, 0.40, 0.80, 0.30);
    colors[StyleColor::ScrollbarGrabHovered] = Vector4D::new(0.40, 0.40, 0.80, 0.40);
    colors[StyleColor::ScrollbarGrabActive] = Vector4D::new(0.41, 0.39, 0.80, 0.60);
    colors[StyleColor::CheckMark] = Vector4D::new(0.90, 0.90, 0.90, 0.50);
    colors[StyleColor::SliderGrab] = Vector4D::new(1.00, 1.00, 1.00, 0.30);
    colors[StyleColor::SliderGrabActive] = Vector4D::new(0.41, 0.39, 0.80, 0.60);
    colors[StyleColor::Button] = Vector4D::new(0.35, 0.40, 0.61, 0.62);
    colors[StyleColor::ButtonHovered] = Vector4D::new(0.40, 0.48, 0.71, 0.79);
    colors[StyleColor::ButtonActive] = Vector4D::new(0.46, 0.54, 0.80, 1.00);
    colors[StyleColor::Header] = Vector4D::new(0.40, 0.40, 0.90, 0.45);
    colors[StyleColor::HeaderHovered] = Vector4D::new(0.45, 0.45, 0.90, 0.80);
    colors[StyleColor::HeaderActive] = Vector4D::new(0.53, 0.53, 0.87, 0.80);
    colors[StyleColor::Separator] = Vector4D::new(0.50, 0.50, 0.50, 0.60);
    colors[StyleColor::SeparatorHovered] = Vector4D::new(0.60, 0.60, 0.70, 1.00);
    colors[StyleColor::SeparatorActive] = Vector4D::new(0.70, 0.70, 0.90, 1.00);
    colors[StyleColor::ResizeGrip] = Vector4D::new(1.00, 1.00, 1.00, 0.10);
    colors[StyleColor::ResizeGripHovered] = Vector4D::new(0.78, 0.82, 1.00, 0.60);
    colors[StyleColor::ResizeGripActive] = Vector4D::new(0.78, 0.82, 1.00, 0.90);
    colors[StyleColor::Tab] = ImLerpF32(
        colors[StyleColor::Header],
        colors[StyleColor::TitleBgActive],
        0.80,
    );
    colors[StyleColor::TabHovered] = colors[StyleColor::HeaderHovered];
    colors[StyleColor::TabActive] = ImLerpF32(
        colors[StyleColor::HeaderActive],
        colors[StyleColor::TitleBgActive],
        0.60,
    );
    colors[StyleColor::TabUnfocused] =
        ImLerpF32(colors[StyleColor::Tab], colors[StyleColor::TitleBg], 0.80);
    colors[StyleColor::TabUnfocusedActive] = ImLerpF32(
        colors[StyleColor::TabActive],
        colors[StyleColor::TitleBg],
        0.40,
    );
    colors[StyleColor::DockingPreview] =
        colors[StyleColor::Header] * Vector4D::new(1.0, 1.0, 1.0, 0.7);
    colors[StyleColor::DockingEmptyBg] = Vector4D::new(0.20, 0.20, 0.20, 1.00);
    colors[StyleColor::PlotLines] = Vector4D::new(1.00, 1.00, 1.00, 1.00);
    colors[StyleColor::PlotLinesHovered] = Vector4D::new(0.90, 0.70, 0.00, 1.00);
    colors[StyleColor::PlotHistogram] = Vector4D::new(0.90, 0.70, 0.00, 1.00);
    colors[StyleColor::PlotHistogramHovered] = Vector4D::new(1.00, 0.60, 0.00, 1.00);
    colors[StyleColor::TableHeaderBg] = Vector4D::new(0.27, 0.27, 0.38, 1.00);
    colors[StyleColor::TableBorderStrong] = Vector4D::new(0.31, 0.31, 0.45, 1.00); // Prefer using alpha=1.0 here
    colors[StyleColor::TableBorderLight] = Vector4D::new(0.26, 0.26, 0.28, 1.00); // Prefer using alpha=1.0 here
    colors[StyleColor::TableRowBg] = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[StyleColor::TableRowBgAlt] = Vector4D::new(1.00, 1.00, 1.00, 0.07);
    colors[StyleColor::TextSelectedBg] = Vector4D::new(0.00, 0.00, 1.00, 0.35);
    colors[StyleColor::DragDropTarget] = Vector4D::new(1.00, 1.00, 0.00, 0.90);
    colors[StyleColor::NavHighlight] = colors[StyleColor::HeaderHovered];
    colors[StyleColor::NavWindowingHighlight] = Vector4D::new(1.00, 1.00, 1.00, 0.70);
    colors[StyleColor::NavWindowingDimBg] = Vector4D::new(0.80, 0.80, 0.80, 0.20);
    colors[StyleColor::ModalWindowDimBg] = Vector4D::new(0.20, 0.20, 0.20, 0.35);
}

// Those light colors are better suited with a thicker font than the default one + FrameBorder
// void ImGui::StyleColorsLight(ImGuiStyle* dst)
pub fn StyleColorsLight(dst: *mut Style) {
    // ImGuiStyle* style = dst ? dst : &ImGui::GetStyle();
    let style = if dst.is_null() == false {
        dst
    } else {
        &g.style
    };
    // Vector4D* colors = style->colors;
    let colors = &mut style.colors;

    colors[StyleColor::Text] = Vector4D::new(0.00, 0.00, 0.00, 1.00);
    colors[StyleColor::TextDisabled] = Vector4D::new(0.60, 0.60, 0.60, 1.00);
    colors[StyleColor::WindowBg] = Vector4D::new(0.94, 0.94, 0.94, 1.00);
    colors[StyleColor::ChildBg] = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[StyleColor::PopupBg] = Vector4D::new(1.00, 1.00, 1.00, 0.98);
    colors[StyleColor::Border] = Vector4D::new(0.00, 0.00, 0.00, 0.30);
    colors[StyleColor::BorderShadow] = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[StyleColor::FrameBg] = Vector4D::new(1.00, 1.00, 1.00, 1.00);
    colors[StyleColor::FrameBgHovered] = Vector4D::new(0.26, 0.59, 0.98, 0.40);
    colors[StyleColor::FrameBgActive] = Vector4D::new(0.26, 0.59, 0.98, 0.67);
    colors[StyleColor::TitleBg] = Vector4D::new(0.96, 0.96, 0.96, 1.00);
    colors[StyleColor::TitleBgActive] = Vector4D::new(0.82, 0.82, 0.82, 1.00);
    colors[StyleColor::TitleBgCollapsed] = Vector4D::new(1.00, 1.00, 1.00, 0.51);
    colors[StyleColor::MenuBarBg] = Vector4D::new(0.86, 0.86, 0.86, 1.00);
    colors[StyleColor::ScrollbarBg] = Vector4D::new(0.98, 0.98, 0.98, 0.53);
    colors[StyleColor::ScrollbarGrab] = Vector4D::new(0.69, 0.69, 0.69, 0.80);
    colors[StyleColor::ScrollbarGrabHovered] = Vector4D::new(0.49, 0.49, 0.49, 0.80);
    colors[StyleColor::ScrollbarGrabActive] = Vector4D::new(0.49, 0.49, 0.49, 1.00);
    colors[StyleColor::CheckMark] = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[StyleColor::SliderGrab] = Vector4D::new(0.26, 0.59, 0.98, 0.78);
    colors[StyleColor::SliderGrabActive] = Vector4D::new(0.46, 0.54, 0.80, 0.60);
    colors[StyleColor::Button] = Vector4D::new(0.26, 0.59, 0.98, 0.40);
    colors[StyleColor::ButtonHovered] = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[StyleColor::ButtonActive] = Vector4D::new(0.06, 0.53, 0.98, 1.00);
    colors[StyleColor::Header] = Vector4D::new(0.26, 0.59, 0.98, 0.31);
    colors[StyleColor::HeaderHovered] = Vector4D::new(0.26, 0.59, 0.98, 0.80);
    colors[StyleColor::HeaderActive] = Vector4D::new(0.26, 0.59, 0.98, 1.00);
    colors[StyleColor::Separator] = Vector4D::new(0.39, 0.39, 0.39, 0.62);
    colors[StyleColor::SeparatorHovered] = Vector4D::new(0.14, 0.44, 0.80, 0.78);
    colors[StyleColor::SeparatorActive] = Vector4D::new(0.14, 0.44, 0.80, 1.00);
    colors[StyleColor::ResizeGrip] = Vector4D::new(0.35, 0.35, 0.35, 0.17);
    colors[StyleColor::ResizeGripHovered] = Vector4D::new(0.26, 0.59, 0.98, 0.67);
    colors[StyleColor::ResizeGripActive] = Vector4D::new(0.26, 0.59, 0.98, 0.95);
    colors[StyleColor::Tab] = ImLerpF32(
        colors[StyleColor::Header],
        colors[StyleColor::TitleBgActive],
        0.90,
    );
    colors[StyleColor::TabHovered] = colors[StyleColor::HeaderHovered];
    colors[StyleColor::TabActive] = ImLerpF32(
        colors[StyleColor::HeaderActive],
        colors[StyleColor::TitleBgActive],
        0.60,
    );
    colors[StyleColor::TabUnfocused] =
        ImLerpF32(colors[StyleColor::Tab], colors[StyleColor::TitleBg], 0.80);
    colors[StyleColor::TabUnfocusedActive] = ImLerpF32(
        colors[StyleColor::TabActive],
        colors[StyleColor::TitleBg],
        0.40,
    );
    colors[StyleColor::DockingPreview] =
        colors[StyleColor::Header] * Vector4D::new(1.0, 1.0, 1.0, 0.7);
    colors[StyleColor::DockingEmptyBg] = Vector4D::new(0.20, 0.20, 0.20, 1.00);
    colors[StyleColor::PlotLines] = Vector4D::new(0.39, 0.39, 0.39, 1.00);
    colors[StyleColor::PlotLinesHovered] = Vector4D::new(1.00, 0.43, 0.35, 1.00);
    colors[StyleColor::PlotHistogram] = Vector4D::new(0.90, 0.70, 0.00, 1.00);
    colors[StyleColor::PlotHistogramHovered] = Vector4D::new(1.00, 0.45, 0.00, 1.00);
    colors[StyleColor::TableHeaderBg] = Vector4D::new(0.78, 0.87, 0.98, 1.00);
    colors[StyleColor::TableBorderStrong] = Vector4D::new(0.57, 0.57, 0.64, 1.00); // Prefer using alpha=1.0 here
    colors[StyleColor::TableBorderLight] = Vector4D::new(0.68, 0.68, 0.74, 1.00); // Prefer using alpha=1.0 here
    colors[StyleColor::TableRowBg] = Vector4D::new(0.00, 0.00, 0.00, 0.00);
    colors[StyleColor::TableRowBgAlt] = Vector4D::new(0.30, 0.30, 0.30, 0.09);
    colors[StyleColor::TextSelectedBg] = Vector4D::new(0.26, 0.59, 0.98, 0.35);
    colors[StyleColor::DragDropTarget] = Vector4D::new(0.26, 0.59, 0.98, 0.95);
    colors[StyleColor::NavHighlight] = colors[StyleColor::HeaderHovered];
    colors[StyleColor::NavWindowingHighlight] = Vector4D::new(0.70, 0.70, 0.70, 0.70);
    colors[StyleColor::NavWindowingDimBg] = Vector4D::new(0.20, 0.20, 0.20, 0.20);
    colors[StyleColor::ModalWindowDimBg] = Vector4D::new(0.20, 0.20, 0.20, 0.35);
}

// Enumeration for PushStyleVar() / PopStyleVar() to temporarily modify the ImGuiStyle structure.
// - The enum only refers to fields of ImGuiStyle which makes sense to be pushed/popped inside UI code.
//   During initialization or between frames, feel free to just poke into ImGuiStyle directly.
// - Tip: Use your programming IDE navigation facilities on the names in the _second column_ below to find the actual members and their description.
//   In Visual Studio IDE: CTRL+comma ("Edit.GoToAll") can follow symbols in comments, whereas CTRL+F12 ("Edit.GoToImplementation") cannot.
//   With Visual Assist installed: ALT+G ("VAssistX.GoToImplementation") can also follow symbols in comments.
// - When changing this enum, you need to update the associated internal table STYLE_VAR_INFO[] accordingly. This is where we link enum values to members offset/type.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DimgStyleVar {
    // Enum name --------------------- // Member in ImGuiStyle structure (see ImGuiStyle for descriptions)
    Alpha,               // float     alpha
    disabled_alpha,      // float     DisabledAlpha
    WindowPadding,       // Vector2D    window_padding
    WindowRounding,      // float     window_rounding
    WindowBorderSize,    // float     window_border_size
    WindowMinSize,       // Vector2D    window_min_size
    window_title_align,  // Vector2D    WindowTitleAlign
    ChildRounding,       // float     child_rounding
    ChildBorderSize,     // float     child_border_size
    PopupRounding,       // float     popup_rounding
    PopupBorderSize,     // float     popup_border_size
    frame_padding,       // Vector2D    FramePadding
    FrameRounding,       // float     frame_rounding
    FrameBorderSize,     // float     frame_border_size
    ItemSpacing,         // Vector2D    item_spacing
    item_inner_spacing,  // Vector2D    ItemInnerSpacing
    indent_spacing,      // float     indent_spacing
    CellPadding,         // Vector2D    cell_padding
    ScrollbarSize,       // float     scrollbar_size
    ScrollbarRounding,   // float     scrollbar_rounding
    GrabMinSize,         // float     grab_min_size
    GrabRounding,        // float     grab_rounding
    TabRounding,         // float     tab_rounding
    ButtonTextAlign,     // Vector2D    button_text_align
    selectableTextAlign, // Vector2D    selectable_text_align
    COUNT,
}
