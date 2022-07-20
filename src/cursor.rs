use crate::Context;
use crate::globals::GImGui;
use crate::vectors::two_d::Vector2D;

// Vector2D GetCursorScreenPos()
pub fn get_cursor_screen_pos(g: &mut Context) -> Vector2D
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.dc.cursor_pos;
}

// void SetCursorScreenPos(const Vector2D& pos)
pub fn set_cursor_screen_pos(g: &mut Context, pos: &Vector2D)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.cursor_pos = pos;
    window.dc.cursor_max_pos = ImMax(window.dc.cursor_max_pos, window.dc.cursor_pos);
}

// User generally sees positions in window coordinates. Internally we store CursorPos in absolute screen coordinates because it is more convenient.
// Conversion happens as we pass the value to user, but it makes our naming convention confusing because GetCursorPos() == (dc.cursor_pos - window.pos). May want to rename 'dc.cursor_pos'.
// Vector2D GetCursorPos()
pub fn get_cursor_pos(g: &mut Context) -> Vector2D
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.dc.cursor_pos - window.pos + window.scroll;
}

// float GetCursorPosX()
pub fn get_cursor_pos_x(g: &mut Context) -> f32
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.dc.cursor_pos.x - window.pos.x + window.scroll.x;
}

// float GetCursorPosY()
pub fn get_cursor_pos_y(g: &mut Context) -> f32
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.dc.cursor_pos.y - window.pos.y + window.scroll.y;
}

// void SetCursorPos(const Vector2D& local_pos)
pub fn set_cursor_pos(g: &mut Context, local_pos: &Vector2D)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.cursor_pos = window.pos - window.scroll + local_pos;
    window.dc.cursor_max_pos = ImMax(window.dc.cursor_max_pos, window.dc.cursor_pos);
}

// void SetCursorPosX(float x)
pub fn set_cursor_pos_x(g: &mut Context, x: f32)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.cursor_pos.x = window.pos.x - window.scroll.x + x;
    window.dc.cursor_max_pos.x = ImMax(window.dc.cursor_max_pos.x, window.dc.cursor_pos.x);
}

// void SetCursorPosY(float y)
pub fn set_cursor_pos_y(g: &mut Context, y: f32)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.cursor_pos.y = window.pos.y - window.scroll.y + y;
    window.dc.cursor_max_pos.y = ImMax(window.dc.cursor_max_pos.y, window.dc.cursor_pos.y);
}

// Vector2D GetCursorStartPos()
pub fn get_cursor_start_pos(g: &mut Context) -> Vector2D
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.dc.cursor_start_pos - window.pos;
}

// void Indent(float indent_w)
pub fn indent(g: &mut Context, indent_w: f32)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.Indent.x += (indent_w != 0.0) ? indent_w : g.style.IndentSpacing;
    window.dc.cursor_pos.x = window.pos.x + window.dc.Indent.x + window.dc.ColumnsOffset.x;
}

// void Unindent(float indent_w)
pub fn unindent(g: &mut Context, indent_w: f32)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.Indent.x -= (indent_w != 0.0) ? indent_w : g.style.IndentSpacing;
    window.dc.cursor_pos.x = window.pos.x + window.dc.Indent.x + window.dc.ColumnsOffset.x;
}
