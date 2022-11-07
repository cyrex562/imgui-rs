use crate::mouse_cursor::ImGuiMouseCursor_COUNT;
use crate::vec2::ImVec2;

pub const FONT_ATLAS_DEFAULT_TEX_CURSOR_DATA: [[ImVec2; 3]; ImGuiMouseCursor_COUNT as usize] = [
    // Pos ........ Size ......... Offset ......
    [ImVec2::new2(0.0, 3.0), ImVec2::new2(12.0, 19.0), ImVec2::new2(0.0, 0.0)], // ImGuiMouseCursor_Arrow
    [ImVec2::new2(13.0, 0.0), ImVec2::new2(7.0, 16.0), ImVec2::new2(1.0, 8.0)], // ImGuiMouseCursor_TextInput
    [ImVec2::new2(31.0, 0.0), ImVec2::new2(23.0, 23.0), ImVec2::new2(11.0, 11.0)], // ImGuiMouseCursor_ResizeAll
    [ImVec2::new2(21.0, 0.0), ImVec2::new2(9.0, 23.0), ImVec2::new2(4.0, 11.0)], // ImGuiMouseCursor_ResizeNS
    [ImVec2::new2(55.0, 18.0), ImVec2::new2(23.0, 9.0), ImVec2::new2(11.0, 4.0)], // ImGuiMouseCursor_ResizeEW
    [ImVec2::new2(73.0, 0.0), ImVec2::new2(17.0, 17.0), ImVec2::new2(8.0, 8.0)], // ImGuiMouseCursor_ResizeNESW
    [ImVec2::new2(55.0, 0.0), ImVec2::new2(17.0, 17.0), ImVec2::new2(8.0, 8.0)], // ImGuiMouseCursor_ResizeNWSE
    [ImVec2::new2(91.0, 0.0), ImVec2::new2(17.0, 22.0), ImVec2::new2(5.0, 0.0)], // ImGuiMouseCursor_Hand
    [ImVec2::new2(109.0, 0.0), ImVec2::new2(13.0, 15.0), ImVec2::new2(6.0, 7.0)], // ImGuiMouseCursor_NotAllowed
];
