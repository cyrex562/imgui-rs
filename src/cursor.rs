use crate::Context;
use crate::globals::GImGui;
use crate::vectors::vector_2d::Vector2D;

// Vector2D GetCursorScreenPos()
pub fn get_cursor_screen_pos(g: &mut Context) -> Vector2D {
    // Window* window = GetCurrentWindowRead();
    let window = g.current_window_mut();
    return window.dc.cursor_pos.clone();
}

// void SetCursorScreenPos(const Vector2D& pos)
pub fn set_cursor_screen_pos(g: &mut Context, pos: &Vector2D) {
    // Window* window = GetCurrentWindow();
    let window = g.current_window_mut();
    window.dc.cursor_pos.clone_from(pos);
    window.dc.cursor_max_pos = Vector2D::max(&window.dc.cursor_max_pos, &window.dc.cursor_pos);
}

// User generally sees positions in window coordinates. Internally we store CursorPos in absolute screen coordinates because it is more convenient.
// Conversion happens as we pass the value to user, but it makes our naming convention confusing because get_cursor_pos() == (dc.cursor_pos - window.pos). May want to rename 'dc.cursor_pos'.
// Vector2D get_cursor_pos()
pub fn get_cursor_pos(g: &mut Context) -> Vector2D {
    // Window* window = GetCurrentWindowRead();
    let window = g.current_window_mut();
    return &window.dc.cursor_pos - &window.pos + &window.scroll;
}

// float GetCursorPosX()
pub fn get_cursor_pos_x(g: &mut Context) -> f32 {
    // Window* window = GetCurrentWindowRead();
    let window = g.current_window_mut();
    return &window.dc.cursor_pos.x - &window.pos.x + &window.scroll.x;
}

// float GetCursorPosY()
pub fn get_cursor_pos_y(g: &mut Context) -> f32 {
    // Window* window = GetCurrentWindowRead();
    let window = g.current_window_mut();
    return &window.dc.cursor_pos.y - &window.pos.y + &window.scroll.y;
}

// void SetCursorPos(const Vector2D& local_pos)
pub fn set_cursor_pos(g: &mut Context, local_pos: &Vector2D) {
    // Window* window = GetCurrentWindow();
    let window = g.current_window_mut();
    window.dc.cursor_pos = &window.pos - &window.scroll + local_pos;
    window.dc.cursor_max_pos = Vector2D::max(&window.dc.cursor_max_pos, &window.dc.cursor_pos);
}

// void SetCursorPosX(float x)
pub fn set_cursor_pos_x(g: &mut Context, x: f32) {
    // Window* window = GetCurrentWindow();
    let window = g.current_window_mut();
    window.dc.cursor_pos.x = window.pos.x - window.scroll.x + x;
    window.dc.cursor_max_pos.x = f32::max(window.dc.cursor_max_pos.x, window.dc.cursor_pos.x);
}

// void SetCursorPosY(float y)
pub fn set_cursor_pos_y(g: &mut Context, y: f32) {
    // Window* window = GetCurrentWindow();
    let window = g.current_window_mut();
    window.dc.cursor_pos.y = window.pos.y - window.scroll.y + y;
    window.dc.cursor_max_pos.y = f32::max(window.dc.cursor_max_pos.y, window.dc.cursor_pos.y);
}

// Vector2D GetCursorStartPos()
pub fn get_cursor_start_pos(g: &mut Context) -> Vector2D {
    // Window* window = GetCurrentWindowRead();
    let window = g.current_window_mut();
    return &window.dc.cursor_start_pos - &window.pos;
}

// void Indent(float indent_w)
pub fn indent(g: &mut Context, indent_w: f32) {
    // ImGuiContext& g = *GImGui;
    // Window* window = GetCurrentWindow();
    let window = g.current_window_mut();
    window.dc.indent.x += if indent_w != 0.0 { indent_w } else { g.style.indent_spacing };
    window.dc.cursor_pos.x = window.pos.x + window.dc.indent.x + window.dc.columns_offset.x;
}

// void Unindent(float indent_w)
pub fn unindent(g: &mut Context, indent_w: f32) {
    // ImGuiContext& g = *GImGui;
    // Window* window = GetCurrentWindow();
    let window = g.current_window_mut();
    window.dc.indent.x -= if indent_w != 0.0 { indent_w } else { g.style.indent_spacing };
    window.dc.cursor_pos.x = window.pos.x + window.dc.indent.x + window.dc.columns_offset.x;
}
