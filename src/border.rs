use crate::Context;
use crate::direction::Direction;
use crate::rect::Rect;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;
use crate::window::Window;

// data for resizing from borders
pub  struct ResizeBorderDef
{
    // Vector2D InnerDir;
    pub inner_dir: Vector2D,
    // Vector2D SegmentN1, SegmentN2;
    pub segment_n1: Vector2D,
    pub segment_n2: Vector2D,
    // float  OuterAngle;
    pub outer_angle: f32,
}

impl ResizeBorderDef {
    pub fn new(inner_dir: Vector2D, segment_n1: Vector2D, segment_n2: Vector2D, outer_angle:f32) -> Self {
        Self {
            inner_dir,
            segment_n1,
            segment_n2,
            outer_angle
        }
    }
}

// static const ImGuiResizeBorderDef resize_border_def[4] =
pub const RESIZE_BORDER_DEF: [ResizeBorderDef;4] =
[
    ImGuiResizeBorderDef::new( Vector2D::new(+1, 0), Vector2D::new(0, 1), Vector2D::new(0, 0), f32::PI * 1.00 ), // Left
    ImGuiResizeBorderDef::new( Vector2D::new(-1, 0), Vector2D::new(1, 0), Vector2D::new(1, 1), f32::PI * 0.00 ), // Right
    ImGuiResizeBorderDef::new( Vector2D::new(0, +1), Vector2D::new(0, 0), Vector2D::new(1, 0), f32::PI * 1.50 ), // Up
    ImGuiResizeBorderDef::new( Vector2D::new(0, -1), Vector2D::new(1, 1), Vector2D::new(0, 1), f32::PI * 0.50 )  // down
];

// static ImRect GetResizeBorderRect(ImGuiWindow* window, int border_n, float perp_padding, float thickness)
pub fn get_resize_border_rect(g: &mut Context, window: &mut Window, border_n: i32, perp_padding: f32, thickness: f32) -> Rect
{
    // ImRect rect = window.Rect();
    let mut rect: Rect = window.rect();
    if thickness == 0.0 {
        rect.max -= Vector2D::new(1.0, 1.0);
    }
    if border_n == Dir::Left { return Rect(rect.min.x - thickness, rect.min.y + perp_padding, rect.min.x + thickness, rect.max.y - perp_padding); }
    if border_n == Dir::Right { return Rect(rect.max.x - thickness, rect.min.y + perp_padding, rect.max.x + thickness, rect.max.y - perp_padding); }
    if border_n == Dir::Up { return Rect(rect.min.x + perp_padding, rect.min.y - thickness, rect.max.x - perp_padding, rect.min.y + thickness);    }
    if border_n == Dir::Down { return Rect(rect.min.x + perp_padding, rect.max.y - thickness, rect.max.x - perp_padding, rect.max.y + thickness);    }
    // IM_ASSERT(0);
    return Rect();
}

// Borders (Left, Right, Up, down)
// ImGuiID ImGui::GetWindowResizeBorderID(ImGuiWindow* window, ImGuiDir dir)
pub fn get_window_resize_border_id(g: &mut Context, window: &mut Window, dir: Direction) -> Id32
{
    // IM_ASSERT(dir >= 0 && dir < 4);
    // int n = dir + 4;
    // ImGuiID id = window.dock_is_active ? window.DockNode.HostWindow.ID : window.id;
    // id = ImHashStr("#RESIZE", 0, id);
    // id = ImHashData(&n, sizeof, id);
    // return id;
}

