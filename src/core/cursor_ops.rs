use crate::a_imgui_cpp::GImGui;
use crate::core::context::ImguiContext;
use crate::imgui::GImGui;
use crate::core::math_ops::ImMax;
use crate::core::vec2::ImVec2;
use crate::window::ops::GetCurrentWindow;
use crate::window::ImguiWindow;
use libc::c_float;

// Until 1.89 (IMGUI_VERSION_NUM < 18814) it was legal to use SetCursorPos() to extend the boundary of a parent (e.g. window or table cell)
// This is causing issues and ambiguity and we need to retire that.
// See https://github.com/ocornut/imgui/issues/5548 for more details.
// [Scenario 1]
//  Previously this would make the window content size ~200x200:
//    Begin(...) + SetCursorScreenPos(GetCursorScreenPos() + ImVec2::new(200,200)) + End();  // NOT OK
//  Instead, please submit an item:
//    Begin(...) + SetCursorScreenPos(GetCursorScreenPos() + ImVec2::new(200,200)) + Dummy(ImVec2::new(0,0)) + End(); // OK
//  Alternative:
//    Begin(...) + Dummy(ImVec2::new(200,200)) + End(); // OK
// [Scenario 2]
//  For reference this is one of the issue what we aim to fix with this change:
//    BeginGroup() + SomeItem("foobar") + SetCursorScreenPos(GetCursorScreenPos()) + EndGroup()
//  The previous logic made SetCursorScreenPos(GetCursorScreenPos()) have a side-effect! It would erroneously incorporate ItemSpacing.y after the item into content size, making the group taller!
//  While this code is a little twisted, no-one would expect SetXXX(GetXXX()) to have a side-effect. Using vertical alignment patterns could trigger this issue.
// c_void ErrorCheckUsingSetCursorPosToExtendParentBoundaries()
pub fn ErrorCheckUsingSetCursorPosToExtendParentBoundaries(g: &mut ImguiContext) {
    let mut window = g.current_window_mut().unwrap();
    // IM_ASSERT(window.dc.is_set_pos);
    window.dc.is_set_pos = false;
    if window.dc.cursor_pos.x <= window.dc.CursorMaxPos.x
        && window.dc.cursor_pos.y <= window.dc.CursorMaxPos.y
    {
        return;
    }
    // IM_ASSERT(0 && "Code uses SetCursorPos()/SetCursorScreenPos() to extend window/parent boundaries. Please submit an item e.g. Dummy() to validate extent.");
}

pub fn cursor_screen_pos(g: &mut ImguiContext) -> ImVec2 {
    let mut window = g.current_window_mut().unwrap();
    return window.dc.cursor_pos;
}

// 2022/08/05: Setting cursor position also extend boundaries (via modifying CursorMaxPos)
// used to compute window size, group size etc.
// I believe this was is a judicious choice but it's probably being relied upon (it has
// been the case since 1.31 and 1.50)
// It would be sane if we requested user to use SetCursorPos() + Dummy(ImVec2::new(0,0))
// to extend CursorMaxPos...
pub fn set_cursor_screen_pos(g: &mut ImguiContext, pos: &ImVec2) {
    let mut window = g.current_window_mut().unwrap();
    window.dc.cursor_pos = pos.clone();
    //window.dc.CursorMaxPos = ImMax(window.dc.CursorMaxPos, window.dc.cursor_pos);
    window.dc.is_set_pos = true;
}

// User generally sees positions in window coordinates. Internally we store CursorPos in absolute screen coordinates because it is more convenient.
// Conversion happens as we pass the value to user, but it makes our naming convention confusing because GetCursorPos() == (dc.cursor_pos - window.position). May want to rename 'dc.cursor_pos'.
// GetCursorPos: ImVec2()
pub fn cursor_pos(g: &mut ImguiContext) -> ImVec2 {
    let mut window = g.current_window_mut().unwrap();
    return window.dc.cursor_pos - window.position + window.scroll;
}

// GetCursorPosX: c_float()
pub fn cursor_pos_x(g: &mut ImguiContext) -> f32 {
    let mut window = g.current_window_mut().unwrap();
    return window.dc.cursor_pos.x - window.position.x + window.scroll.x;
}

// GetCursorPosY: c_float()
pub fn cursor_pos_y(g: &mut ImguiContext) -> f32 {
    let mut window = g.current_window_mut().unwrap();
    return window.dc.cursor_pos.y - window.position.y + window.scroll.y;
}

// c_void SetCursorPos(local_pos: &ImVec2)
pub fn set_cursor_pos(g: &mut ImguiContext, local_pos: &ImVec2) {
    let mut window = g.current_window_mut().unwrap();
    window.dc.cursor_pos = window.position - window.scroll + local_pos;
    //window.dc.CursorMaxPos = ImMax(window.dc.CursorMaxPos, window.dc.cursor_pos);
    window.dc.is_set_pos = true;
}

pub fn set_cursor_x(g: &mut ImguiContext, x: c_float) {
    let mut window = g.current_window_mut().unwrap();
    window.dc.cursor_pos.x = window.position.x - window.scroll.x + x;
    //window.dc.CursorMaxPos.x = ImMax(window.dc.CursorMaxPos.x, window.dc.cursor_pos.x);
    window.dc.is_set_pos = true;
}

pub fn set_cursor_pos_y(g: &mut ImguiContext, y: c_float) {
    let mut window = g.current_window_mut().unwrap();
    window.dc.cursor_pos.y = window.position.y - window.scroll.y + y;
    //window.dc.CursorMaxPos.y = ImMax(window.dc.CursorMaxPos.y, window.dc.cursor_pos.y);
    window.dc.is_set_pos = true;
}

pub fn cursor_start_pos(g: &mut ImguiContext) -> ImVec2 {
    let mut window = g.current_window_mut().unwrap();
    return window.dc.cursor_start_pos - window.position;
}

// c_void Indent(indent_w: c_float)
pub fn indent(indent_w: c_float, g: &mut ImguiContext) {
    let mut window = g.current_window_mut().unwrap();
    window.dc.indent.x += if indent_w != 0.0 {
        indent_w
    } else {
        g.style.indent_spacing
    };
    window.dc.cursor_pos.x = window.position.x + window.dc.indent.x + window.dc.columns_offset.x;
}

pub fn unindent(g: &mut ImguiContext, indent_w: c_float) {
    let mut window = g.current_window_mut().unwrap();
    window.dc.indent.x -= if indent_w != 0.0 {
        indent_w
    } else {
        g.style.indent_spacing
    };
    window.dc.cursor_pos.x = window.position.x + window.dc.indent.x + window.dc.columns_offset.x;
}
