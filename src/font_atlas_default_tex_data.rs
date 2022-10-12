use crate::mouse_cursor::ImGuiMouseCursor_COUNT;
use crate::vec2::ImVec2;
use libc::{c_char, size_t};
use std::ffi::CStr;

// A work of art lies ahead! (. = white layer, X = black layer, others are blank)
// The 2x2 white texels on the top left are the ones we'll use everywhere in Dear ImGui to render filled shapes.
// (This is used when io.MouseDrawCursor = true)
pub const FONT_ATLAS_DEFAULT_TEX_DATA_W: size_t = 122;
// Actual texture will be 2 times that + 1 spacing.
pub const FONT_ATLAS_DEFAULT_TEX_DATA_H: size_t = 27;

pub const FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS: &'static str =  "..-         -XXXXXXX-    X    -           X           -XXXXXXX          -          XXXXXXX-     XX          - XX       XX "+
    "..-         -X.....X-   X.X   -          X.X          -X.....X          -          X.....X-    X..X         -X..X     X..X"+
    "---         -XXX.XXX-  X...X  -         X...X         -X....X           -           X....X-    X..X         -X...X   X...X"+
    "X           -  X.X  - X.....X -        X.....X        -X...X            -            X...X-    X..X         - X...X X...X "+
    "XX          -  X.X  -X.......X-       X.......X       -X..X.X           -           X.X..X-    X..X         -  X...X...X  "+
    "X.X         -  X.X  -XXXX.XXXX-       XXXX.XXXX       -X.X X.X          -          X.X X.X-    X..XXX       -   X.....X   "+
    "X..X        -  X.X  -   X.X   -          X.X          -XX   X.X         -         X.X   XX-    X..X..XXX    -    X...X    "+
    "X...X       -  X.X  -   X.X   -    XX    X.X    XX    -      X.X        -        X.X      -    X..X..X..XX  -     X.X     "+
    "X....X      -  X.X  -   X.X   -   X.X    X.X    X.X   -       X.X       -       X.X       -    X..X..X..X.X -    X...X    "+
    "X.....X     -  X.X  -   X.X   -  X..X    X.X    X..X  -        X.X      -      X.X        -XXX X..X..X..X..X-   X.....X   "+
    "X......X    -  X.X  -   X.X   - X...XXXXXX.XXXXXX...X -         X.X   XX-XX   X.X         -X..XX........X..X-  X...X...X  "+
    "X.......X   -  X.X  -   X.X   -X.....................X-          X.X X.X-X.X X.X          -X...X...........X- X...X X...X "+
    "X........X  -  X.X  -   X.X   - X...XXXXXX.XXXXXX...X -           X.X..X-X..X.X           - X..............X-X...X   X...X"+
    "X.........X -XXX.XXX-   X.X   -  X..X    X.X    X..X  -            X...X-X...X            -  X.............X-X..X     X..X"+
    "X..........X-X.....X-   X.X   -   X.X    X.X    X.X   -           X....X-X....X           -  X.............X- XX       XX "+
    "X......XXXXX-XXXXXXX-   X.X   -    XX    X.X    XX    -          X.....X-X.....X          -   X............X--------------"+
    "X...X..X    ---------   X.X   -          X.X          -          XXXXXXX-XXXXXXX          -   X...........X -             "+
    "X..X X..X   -       -XXXX.XXXX-       XXXX.XXXX       -------------------------------------    X..........X -             "+
    "X.X  X..X   -       -X.......X-       X.......X       -    XX           XX    -           -    X..........X -             "+
    "XX    X..X  -       - X.....X -        X.....X        -   X.X           X.X   -           -     X........X  -             "+
    "      X..X  -       -  X...X  -         X...X         -  X..X           X..X  -           -     X........X  -             "+
    "       XX   -       -   X.X   -          X.X          - X...XXXXXXXXXXXXX...X -           -     XXXXXXXXXX  -             "+
    "-------------       -    X    -           X           -X.....................X-           -------------------             "+
    "                    ----------------------------------- X...XXXXXXXXXXXXX...X -                                           "+
    "                                                      -  X..X           X..X  -                                           "+
    "                                                      -   X.X           X.X   -                                           "+
    "                                                      -    XX           XX    -                                           ";

// pub const FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS_arr: [&'static str;FONT_ATLAS_DEFAULT_TEX_DATA_W * FONT_ATLAS_DEFAULT_TEX_DATA_H + 1] =
// [
//     "..-         -XXXXXXX-    X    -           X           -XXXXXXX          -          XXXXXXX-     XX          - XX       XX ",
//     "..-         -X.....X-   X.X   -          X.X          -X.....X          -          X.....X-    X..X         -X..X     X..X",
//     "---         -XXX.XXX-  X...X  -         X...X         -X....X           -           X....X-    X..X         -X...X   X...X",
//     "X           -  X.X  - X.....X -        X.....X        -X...X            -            X...X-    X..X         - X...X X...X ",
//     "XX          -  X.X  -X.......X-       X.......X       -X..X.X           -           X.X..X-    X..X         -  X...X...X  ",
//     "X.X         -  X.X  -XXXX.XXXX-       XXXX.XXXX       -X.X X.X          -          X.X X.X-    X..XXX       -   X.....X   ",
//     "X..X        -  X.X  -   X.X   -          X.X          -XX   X.X         -         X.X   XX-    X..X..XXX    -    X...X    ",
//     "X...X       -  X.X  -   X.X   -    XX    X.X    XX    -      X.X        -        X.X      -    X..X..X..XX  -     X.X     ",
//     "X....X      -  X.X  -   X.X   -   X.X    X.X    X.X   -       X.X       -       X.X       -    X..X..X..X.X -    X...X    ",
//     "X.....X     -  X.X  -   X.X   -  X..X    X.X    X..X  -        X.X      -      X.X        -XXX X..X..X..X..X-   X.....X   ",
//     "X......X    -  X.X  -   X.X   - X...XXXXXX.XXXXXX...X -         X.X   XX-XX   X.X         -X..XX........X..X-  X...X...X  ",
//     "X.......X   -  X.X  -   X.X   -X.....................X-          X.X X.X-X.X X.X          -X...X...........X- X...X X...X ",
//     "X........X  -  X.X  -   X.X   - X...XXXXXX.XXXXXX...X -           X.X..X-X..X.X           - X..............X-X...X   X...X",
//     "X.........X -XXX.XXX-   X.X   -  X..X    X.X    X..X  -            X...X-X...X            -  X.............X-X..X     X..X",
//     "X..........X-X.....X-   X.X   -   X.X    X.X    X.X   -           X....X-X....X           -  X.............X- XX       XX ",
//     "X......XXXXX-XXXXXXX-   X.X   -    XX    X.X    XX    -          X.....X-X.....X          -   X............X--------------",
//     "X...X..X    ---------   X.X   -          X.X          -          XXXXXXX-XXXXXXX          -   X...........X -             ",
//     "X..X X..X   -       -XXXX.XXXX-       XXXX.XXXX       -------------------------------------    X..........X -             ",
//     "X.X  X..X   -       -X.......X-       X.......X       -    XX           XX    -           -    X..........X -             ",
//     "XX    X..X  -       - X.....X -        X.....X        -   X.X           X.X   -           -     X........X  -             ",
//     "      X..X  -       -  X...X  -         X...X         -  X..X           X..X  -           -     X........X  -             ",
//     "       XX   -       -   X.X   -          X.X          - X...XXXXXXXXXXXXX...X -           -     XXXXXXXXXX  -             ",
//     "-------------       -    X    -           X           -X.....................X-           -------------------             ",
//     "                    ----------------------------------- X...XXXXXXXXXXXXX...X -                                           ",
//     "                                                      -  X..X           X..X  -                                           ",
//     "                                                      -   X.X           X.X   -                                           ",
//     "                                                      -    XX           XX    -                                           "
// ];

// pub unsafe fn FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS_as_vec() -> Vec<*const c_char> {
//     let mut out: Vec<*const c_char> = vec![];
//     for ele in FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS {
//         let item = CStr::from_bytes_with_nul_unchecked(ele.as_bytes());
//         out.push(item.as_ptr())
//     }
//     out
// }

pub unsafe fn FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS_as_const_char_ptr() -> *const c_char {
    CStr::from_bytes_with_nul_unchecked(FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS.as_bytes()).as_ptr()
}

pub const FONT_ATLAS_DEFAULT_TEX_CURSOR_DATA: [[ImVec2; 3]; ImGuiMouseCursor_COUNT as usize] = [
    // Pos ........ Size ......... Offset ......
    [
        ImVec2::new(0.0, 3.0),
        ImVec2::new(12.0, 19.0),
        ImVec2::new(0.0, 0.0),
    ], // ImGuiMouseCursor_Arrow
    [
        ImVec2::new(13.0, 0.0),
        ImVec2::new(7.0, 16.0),
        ImVec2::new(1.0, 8.0),
    ], // ImGuiMouseCursor_TextInput
    [
        ImVec2::new(31.0, 0.0),
        ImVec2::new(23.0, 23.0),
        ImVec2::new(11.0, 11.0),
    ], // ImGuiMouseCursor_ResizeAll
    [
        ImVec2::new(21.0, 0.0),
        ImVec2::new(9.0, 23.0),
        ImVec2::new(4.0, 11.0),
    ], // ImGuiMouseCursor_ResizeNS
    [
        ImVec2::new(55.0, 18.0),
        ImVec2::new(23.0, 9.0),
        ImVec2::new(11.0, 4.0),
    ], // ImGuiMouseCursor_ResizeEW
    [
        ImVec2::new(73.0, 0.0),
        ImVec2::new(17.0, 17.0),
        ImVec2::new(8.0, 8.0),
    ], // ImGuiMouseCursor_ResizeNESW
    [
        ImVec2::new(55.0, 0.0),
        ImVec2::new(17.0, 17.0),
        ImVec2::new(8.0, 8.0),
    ], // ImGuiMouseCursor_ResizeNWSE
    [
        ImVec2::new(91.0, 0.0),
        ImVec2::new(17.0, 22.0),
        ImVec2::new(5.0, 0.0),
    ], // ImGuiMouseCursor_Hand
    [
        ImVec2::new(109.0, 0.0),
        ImVec2::new(13.0, 15.0),
        ImVec2::new(6.0, 7.0),
    ], // ImGuiMouseCursor_NotAllowed
];
