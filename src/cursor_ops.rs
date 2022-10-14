#![allow(non_snake_case)]

use libc::c_float;
use crate::imgui::GImGui;
use crate::imgui_cpp::GImGui;
use crate::math_ops::ImMax;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;

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
pub unsafe fn ErrorCheckUsingSetCursorPosToExtendParentBoundaries() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(window.DC.IsSetPos);
    window.DC.IsSetPos = false;
    // #ifdef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    if window.DC.CursorPos.x <= window.DC.CursorMaxPos.x
        && window.DC.CursorPos.y <= window.DC.CursorMaxPos.y
    {
        return;
    }
    // IM_ASSERT(0 && "Code uses SetCursorPos()/SetCursorScreenPos() to extend window/parent boundaries. Please submit an item e.g. Dummy() to validate extent.");
    // #else
    window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, window.DC.CursorPos);
    // #endif
}



// GetCursorScreenPos: ImVec2()
pub unsafe fn GetCursorScreenPos() -> ImVec2
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.DC.CursorPos;
}

// 2022/08/05: Setting cursor position also extend boundaries (via modifying CursorMaxPos) used to compute window size, group size etc.
// I believe this was is a judicious choice but it's probably being relied upon (it has been the case since 1.31 and 1.50)
// It would be sane if we requested user to use SetCursorPos() + Dummy(ImVec2::new(0,0)) to extend CursorMaxPos...
pub unsafe fn SetCursorScreenPos(pos: &ImVec2)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.CursorPos = pos.clone();
    //window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, window.DC.CursorPos);
    window.DC.IsSetPos = true;
}

// User generally sees positions in window coordinates. Internally we store CursorPos in absolute screen coordinates because it is more convenient.
// Conversion happens as we pass the value to user, but it makes our naming convention confusing because GetCursorPos() == (DC.CursorPos - window.Pos). May want to rename 'DC.CursorPos'.
// GetCursorPos: ImVec2()
pub unsafe fn GetCursorPos() -> ImVec2
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.DC.CursorPos - window.Pos + window.Scroll;
}

// GetCursorPosX: c_float()
pub unsafe fn GetCursorPosX() -> c_float
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.DC.CursorPos.x - window.Pos.x + window.Scroll.x;
}

// GetCursorPosY: c_float()
pub unsafe fn GetCursorPosY() -> c_float
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.DC.CursorPos.y - window.Pos.y + window.Scroll.y;
}

// c_void SetCursorPos(local_pos: &ImVec2)
pub unsafe fn SetCursorPos(local_pos: &ImVec2)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.CursorPos = window.Pos - window.Scroll + local_pos;
    //window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, window.DC.CursorPos);
    window.DC.IsSetPos = true;
}

// c_void SetCursorPosX(x: c_float)
pub unsafe fn SetCursorPosX(x: c_float)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.CursorPos.x = window.Pos.x - window.Scroll.x + x;
    //window.DC.CursorMaxPos.x = ImMax(window.DC.CursorMaxPos.x, window.DC.CursorPos.x);
    window.DC.IsSetPos = true;
}

pub unsafe fn SetCursorPosY(y: c_float)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.CursorPos.y = window.Pos.y - window.Scroll.y + y;
    //window.DC.CursorMaxPos.y = ImMax(window.DC.CursorMaxPos.y, window.DC.CursorPos.y);
    window.DC.IsSetPos = true;
}

// GetCursorStartPos: ImVec2()
pub unsafe fn GetCursorStartPos() -> ImVec2
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.DC.CursorStartPos - window.Pos;
}

// c_void Indent(indent_w: c_float)
pub unsafe fn Indent(indent_w: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.Indent.x += if indent_w != 0.0 { indent_w } else { g.Style.IndentSpacing };
    window.DC.CursorPos.x = window.Pos.x + window.DC.Indent.x + window.DC.ColumnsOffset.x;
}

pub unsafe fn Unindent(indent_w: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.Indent.x -= if (indent_w != 0.0) { indent_w } else { g.Style.IndentSpacing };
    window.DC.CursorPos.x = window.Pos.x + window.DC.Indent.x + window.DC.ColumnsOffset.x;
}
