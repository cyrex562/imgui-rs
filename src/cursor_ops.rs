#![allow(non_snake_case)]

use crate::imgui::GImGui;
use crate::imgui_cpp::GImGui;
use crate::math_ops::ImMax;

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
