use std::collections::HashSet;
use crate::condition::Condition;
use crate::{Context, INVALID_ID};
use crate::globals::GImGui;
use crate::rect::Rect;
use crate::types::Id32;
use crate::vectors::vector_2d::Vector2D;
use crate::window::class::WindowClass;
use crate::window::ImGuiSizeCallback;

// void ImGui::SetNextWindowSizeConstraints(const Vector2D& size_min, const Vector2D& size_max, ImGuiSizeCallback custom_callback, void* custom_callback_user_data)
pub fn set_next_window_size_constraints(g: &mut Context, size_min: &Vector2D, size_max: &Vector2D, custom_callback: ImGuiSizeCallback, custom_callback_user_data: &mut Vec<u8>)
{
    // ImGuiContext& g = *GImGui;
    g.next_window_data.flags |= NextWindowDataFlags::HasSizeConstraint;
    g.next_window_data.sizeConstraintRect = Rect(size_min, size_max);
    g.next_window_data.sizeCallback = custom_callback;
    g.next_window_data.size_callback_user_data = custom_callback_user_data;
}

// void ImGui::set_next_window_pos(const Vector2D& pos, ImGuiCond cond, const Vector2D& pivot)
pub fn set_next_window_pos(g: &mut Context, pos: &Vector2D, cond: Condition, pivot: Option<Vector2D>)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.next_window_data.flags |= NextWindowDataFlags::HasPos;
    g.next_window_data.pos_val = pos;
    g.next_window_data.pos_pivot_val = pivot;
    g.next_window_data.pos_cond = cond ? cond : Condition::Always;
    g.next_window_data.pos_undock = true;
}

// void ImGui::set_next_window_size(const Vector2D& size, ImGuiCond cond)
pub fn set_next_window_size(g: &mut Context, size: &Vector2D, cond: Condition)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.next_window_data.flags |= NextWindowDataFlags::HasSize;
    g.next_window_data.sizeVal = size;
    g.next_window_data.sizeCond = cond ? cond : Condition::Always;
}

// Content size = inner scrollable rectangle, padded with window_padding.
// SetNextWindowContentSize(Vector2D(100,100) + WindowFlags_AlwaysAutoResize will always allow submitting a 100x100 item.
// void ImGui::SetNextWindowContentSize(const Vector2D& size)
pub fn set_next_window_content_size(g: &mut Context, size: &Vector2D)
{
    // ImGuiContext& g = *GImGui;
    g.next_window_data.flags |= NextWindowDataFlags::HasContentSize;
    g.next_window_data.content_size_val = f32::floor(size);
}

// void ImGui::SetNextWindowScroll(const Vector2D& scroll)
pub fn set_next_window_scroll(g: &mut Context, scroll: &Vector2D)
{
    // ImGuiContext& g = *GImGui;
    g.next_window_data.flags |= NextWindowDataFlags::HasScroll;
    g.next_window_data.scroll_val = scroll;
}

// void ImGui::set_next_window_collapsed(bool collapsed, ImGuiCond cond)
pub fn set_next_window_collapsed(g: &mut Context, collapsed, cond:Condition)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.next_window_data.flags |= NextWindowDataFlags::HasCollapsed;
    g.next_window_data.collapsed_val = collapsed;
    g.next_window_data.CollapsedCond = cond ? cond : Condition::Always;
}

// void ImGui::SetNextWindowFocus()
pub fn set_next_window_focus(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    g.next_window_data.flags |= NextWindowDataFlags::HasFocus;
}

// void ImGui::set_netxt_window_bg_alpha(float alpha)
pub fn set_next_window_bg_alpha(g: &mut Context, alpha: f32)
{
    // ImGuiContext& g = *GImGui;
    g.next_window_data.flags |= NextWindowDataFlags::HasBgAlpha;
    g.next_window_data.bg_alpha_val = alpha;
}

// void ImGui::set_next_window_viewport(Id32 id)
pub fn set_next_window_viewport(g: &mut Context, id: Id32)
{
    // ImGuiContext& g = *GImGui;
    g.next_window_data.flags |= NextWindowDataFlags::HasViewport;
    g.next_window_data.viewport_id = id;
}

// void ImGui::SetNextWindowDockID(Id32 id, ImGuiCond cond)
pub fn set_next_window_dock_id(g: &mut Context, id: Id32, cond:Condition)
{
    // ImGuiContext& g = *GImGui;
    g.next_window_data.flags |= NextWindowDataFlags::HasDock;
    g.next_window_data.dock_cond = cond ? cond : Condition::Always;
    g.next_window_data.dock_id = id;
}

// void ImGui::set_next_window_class(const window_class* window_class)
pub fn set_next_window_class(g: &mut Context, window_class: &mut WindowClass)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT((window_class.viewportFlagsOverrideSet & window_class.viewport_flags_override_clear) == 0); // Cannot set both set and clear for the same bit
    g.next_window_data.flags |= NextWindowDataFlags::HasWindowClass;
    g.next_window_data.window_class = *window_class;
}

// Storage for SetNexWindow** functions
#[derive(Debug, Clone, Default)]
pub struct NextWindowData {
    // ImGuiNextWindowDataFlags    flags;
    pub flags: HashSet<NextWindowDataFlags>,
    // ImGuiCond                   pos_cond;
    pub pos_cond: Condition,
    // ImGuiCond                   size_cond;
    pub size_cond: Condition,
    // ImGuiCond                   CollapsedCond;
    pub collapse_cond: Condition,
    // ImGuiCond                   dock_cond;
    pub dock_cond: Condition,
    // Vector2D                      pos_val;
    pub pos_val: Vector2D,
    // Vector2D                      pos_pivot_val;
    pub pos_pivot_val: Vector2D,
    // Vector2D                      size_val;
    pub size_val: Vector2D,
    // Vector2D                      content_size_val; 
    pub content_size_val: Vector2D,
    // Vector2D                      scroll_val;
    pub scroll_val: Vector2D,
    // bool                        pos_undock;
    pub pos_undock: bool,
    // bool                        collapsed_val;
    pub collapsed_val: bool,
    // ImRect                      size_constraint_rect;
    pub size_constraint_rect: Rect,
    // ImGuiSizeCallback           size_callback;
    pub size_callback: Option<SizeCallback>,
    // void*                       size_callback_user_data;
    pub size_callback_user_data: Vec<u8>,
    // float                       bg_alpha_val;             // Override background alpha
    pub bg_alpha_val: f32,
    // Id32                     viewport_id;
    pub viewport_id: Id32,
    // Id32                     dock_id;
    pub dock_id: Id32,
    // window_class            window_class;
    pub window_class: WindowClass,
    // Vector2D                      menu_bar_offset_min_val;    // (Always on) This is not exposed publicly, so we don't clear it and it doesn't have a corresponding flag (could we? for consistency?)
    pub menu_bar_offset_min_val: Vector2D,

}

impl Default for NextWindowData {
    fn default() -> Self {
        Self {
            flags: HashSet::new(),
            pos_cond: Condition::None,
            size_cond: Condition::None,
            collapse_cond: Condition::None,
            dock_cond: Condition::None,
            pos_val: Vector2D::default(),
            pos_pivot_val: Vector2D::default(),
            size_val: Vector2D::default(),
            content_size_val: Vector2D::default(),
            scroll_val: Vector2D::default(),
            pos_undock: false,
            collapsed_val: false,
            size_constraint_rect: Rect::default(),
            size_callback: None,
            size_callback_user_data: vec![],
            bg_alpha_val: 0.0,
            viewport_id: INVALID_ID,
            dock_id: INVALID_ID,
            window_class: WindowClass::default(),
            menu_bar_offset_min_val: Vector2D::default()
        }
    }
}

impl NextWindowData {
    // ImGuiNextWindowData()       { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     inline void ClearFlags()    { flags = ImGuiNextWindowDataFlags_None; }
    pub fn clear_flags(&mut self) {
        self.flags.clear()
    }
}


pub enum NextWindowDataFlags {
    None = 0,
    HasPos,
    HasSize,
    HasContentSize,
    HasCollapsed,
    HasSizeConstraint,
    HasFocus,
    HasBgAlpha,
    HasScroll,
    HasViewport,
    HasDock,
    HasWindowClass,
}
