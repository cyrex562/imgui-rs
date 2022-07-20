use crate::condition::Condition;
use crate::Context;
use crate::globals::GImGui;
use crate::rect::Rect;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;
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

// void ImGui::SetNextWindowPos(const Vector2D& pos, ImGuiCond cond, const Vector2D& pivot)
pub fn set_next_window_pos(g: &mut Context, pos: &Vector2D, cond: Condition, pivot: &Vector2D)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.next_window_data.flags |= NextWindowDataFlags::HasPos;
    g.next_window_data.PosVal = pos;
    g.next_window_data.PosPivotVal = pivot;
    g.next_window_data.PosCond = cond ? cond : Cond::Always;
    g.next_window_data.PosUndock = true;
}

// void ImGui::set_next_window_size(const Vector2D& size, ImGuiCond cond)
pub fn set_next_window_size(g: &mut Context, size: &Vector2D, cond: Condition)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.next_window_data.flags |= NextWindowDataFlags::HasSize;
    g.next_window_data.sizeVal = size;
    g.next_window_data.sizeCond = cond ? cond : Cond::Always;
}

// Content size = inner scrollable rectangle, padded with window_padding.
// SetNextWindowContentSize(Vector2D(100,100) + ImGuiWindowFlags_AlwaysAutoResize will always allow submitting a 100x100 item.
// void ImGui::SetNextWindowContentSize(const Vector2D& size)
pub fn set_next_window_content_size(g: &mut Context, size: &Vector2D)
{
    // ImGuiContext& g = *GImGui;
    g.next_window_data.flags |= NextWindowDataFlags::HasContentSize;
    g.next_window_data.ContentSizeVal = f32::floor(size);
}

// void ImGui::SetNextWindowScroll(const Vector2D& scroll)
pub fn set_next_window_scroll(g: &mut Context, scroll: &Vector2D)
{
    // ImGuiContext& g = *GImGui;
    g.next_window_data.flags |= NextWindowDataFlags::HasScroll;
    g.next_window_data.ScrollVal = scroll;
}

// void ImGui::SetNextWindowCollapsed(bool collapsed, ImGuiCond cond)
pub fn set_next_window_collapsed(g: &mut Context, collapsed, cond:Condition)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.next_window_data.flags |= NextWindowDataFlags::HasCollapsed;
    g.next_window_data.CollapsedVal = collapsed;
    g.next_window_data.CollapsedCond = cond ? cond : Cond::Always;
}

// void ImGui::SetNextWindowFocus()
pub fn set_next_window_focus(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    g.next_window_data.flags |= NextWindowDataFlags::HasFocus;
}

// void ImGui::SetNextWindowBgAlpha(float alpha)
pub fn set_next_window_bg_alpha(g: &mut Context, alpha: f32)
{
    // ImGuiContext& g = *GImGui;
    g.next_window_data.flags |= NextWindowDataFlags::HasBgAlpha;
    g.next_window_data.BgAlphaVal = alpha;
}

// void ImGui::SetNextWindowViewport(ImGuiID id)
pub fn set_next_window_viewport(g: &mut Context, id: Id32)
{
    // ImGuiContext& g = *GImGui;
    g.next_window_data.flags |= NextWindowDataFlags::HasViewport;
    g.next_window_data.viewport_id = id;
}

// void ImGui::SetNextWindowDockID(ImGuiID id, ImGuiCond cond)
pub fn set_next_window_dock_id(g: &mut Context, id: Id32, cond:Condition)
{
    // ImGuiContext& g = *GImGui;
    g.next_window_data.flags |= NextWindowDataFlags::HasDock;
    g.next_window_data.DockCond = cond ? cond : Cond::Always;
    g.next_window_data.DockId = id;
}

// void ImGui::SetNextWindowClass(const ImGuiWindowClass* window_class)
pub fn set_next_window_class(g: &mut Context, window_class: &mut WindowClass)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT((window_class.ViewportFlagsOverrideSet & window_class.ViewportFlagsOverrideClear) == 0); // Cannot set both set and clear for the same bit
    g.next_window_data.flags |= NextWindowDataFlags::HasWindowClass;
    g.next_window_data.WindowClass = *window_class;
}

// Storage for SetNexWindow** functions
#[derive(Debug, Clone, Default)]
pub struct NextWindowData {
    // ImGuiNextWindowDataFlags    flags;
    pub Flags: NextWindowDataFlags,
    // ImGuiCond                   PosCond;
    pub PosCond: Condition,
    // ImGuiCond                   SizeCond;
    pub SizeCond: Condition,
    // ImGuiCond                   CollapsedCond;
    pub CollapseCond: Condition,
    // ImGuiCond                   DockCond;
    pub DockCond: Condition,
    // Vector2D                      PosVal;
    pub PosVal: Vector2D,
    // Vector2D                      PosPivotVal;
    pub PosPivotVal: Vector2D,
    // Vector2D                      SizeVal;
    pub SizeVal: Vector2D,
    // Vector2D                      ContentSizeVal;
    pub ContentSizeVal: Vector2D,
    // Vector2D                      ScrollVal;
    pub ScrollVal: Vector2D,
    // bool                        PosUndock;
    pub PosUndock: bool,
    // bool                        CollapsedVal;
    pub CollapsedVal: bool,
    // ImRect                      SizeConstraintRect;
    pub SizeConstraintRect: Rect,
    // ImGuiSizeCallback           SizeCallback;
    pub SizeCallback: ImGuiSizeCallback,
    // void*                       SizeCallbackUserData;
    pub SizeCallbackUserData: Vec<u8>,
    // float                       BgAlphaVal;             // Override background alpha
    pub BgAlphaVal: f32,
    // ImGuiID                     viewport_id;
    pub ViewportId: Id32,
    // ImGuiID                     dock_id;
    pub DockId: Id32,
    // ImGuiWindowClass            window_class;
    pub WindowClass: WindowClass,
    // Vector2D                      MenuBarOffsetMinVal;    // (Always on) This is not exposed publicly, so we don't clear it and it doesn't have a corresponding flag (could we? for consistency?)
    pub MenuBarOffsetMinVal: Vector2D,

}

impl NextWindowData {
    // ImGuiNextWindowData()       { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     inline void ClearFlags()    { flags = ImGuiNextWindowDataFlags_None; }
    pub fn ClearFlags(&mut self) {
        self.flags = NextWindowDataFlags::None
    }
}


pub enum NextWindowDataFlags {
    None = 0,
    HasPos = 1 << 0,
    HasSize = 1 << 1,
    HasContentSize = 1 << 2,
    HasCollapsed = 1 << 3,
    HasSizeConstraint = 1 << 4,
    HasFocus = 1 << 5,
    HasBgAlpha = 1 << 6,
    HasScroll = 1 << 7,
    HasViewport = 1 << 8,
    HasDock = 1 << 9,
    HasWindowClass = 1 << 10,
}
