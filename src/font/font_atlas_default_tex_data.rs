use crate::vectors::Vector2D;

// A work of art lies ahead! (. = white layer, x = black layer, others are blank)
// The 2x2 white texels on the top left are the ones we'll use everywhere in Dear ImGui to render filled shapes.
// (This is used when io.mouse_draw_cursor = true)
pub const FONT_ATLAS_DEFAULT_TEX_DATA_W: usize = 122;
// Actual texture will be 2 times that + 1 spacing.
pub const FONT_ATLAS_DEFAULT_TEX_DATA_H: usize = 27;
pub const FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS: &'static str =

    "..-         -XXXXXXX-    x    -           x           -XXXXXXX          -          XXXXXXX-     XX          - XX       XX " +
    "..-         -x.....x-   x.x   -          x.x          -x.....x          -          x.....x-    x..x         -x..x     x..x" +
    "---         -XXX.XXX-  x...x  -         x...x         -x....x           -           x....x-    x..x         -x...x   x...x" +
    "x           -  x.x  - x.....x -        x.....x        -x...x            -            x...x-    x..x         - x...x x...x " +
    "XX          -  x.x  -x.......x-       x.......x       -x..x.x           -           x.x..x-    x..x         -  x...x...x  " +
    "x.x         -  x.x  -XXXX.XXXX-       XXXX.XXXX       -x.x x.x          -          x.x x.x-    x..XXX       -   x.....x   " +
    "x..x        -  x.x  -   x.x   -          x.x          -XX   x.x         -         x.x   XX-    x..x..XXX    -    x...x    " +
    "x...x       -  x.x  -   x.x   -    XX    x.x    XX    -      x.x        -        x.x      -    x..x..x..XX  -     x.x     " +
    "x....x      -  x.x  -   x.x   -   x.x    x.x    x.x   -       x.x       -       x.x       -    x..x..x..x.x -    x...x    " +
    "x.....x     -  x.x  -   x.x   -  x..x    x.x    x..x  -        x.x      -      x.x        -XXX x..x..x..x..x-   x.....x   " +
    "x......x    -  x.x  -   x.x   - x...XXXXXX.XXXXXX...x -         x.x   XX-XX   x.x         -x..XX........x..x-  x...x...x  " +
    "x.......x   -  x.x  -   x.x   -x.....................x-          x.x x.x-x.x x.x          -x...x...........x- x...x x...x " +
    "x........x  -  x.x  -   x.x   - x...XXXXXX.XXXXXX...x -           x.x..x-x..x.x           - x..............x-x...x   x...x" +
    "x.........x -XXX.XXX-   x.x   -  x..x    x.x    x..x  -            x...x-x...x            -  x.............x-x..x     x..x" +
    "x..........x-x.....x-   x.x   -   x.x    x.x    x.x   -           x....x-x....x           -  x.............x- XX       XX " +
    "x......XXXXX-XXXXXXX-   x.x   -    XX    x.x    XX    -          x.....x-x.....x          -   x............x--------------" +
    "x...x..x    ---------   x.x   -          x.x          -          XXXXXXX-XXXXXXX          -   x...........x -             " +
    "x..x x..x   -       -XXXX.XXXX-       XXXX.XXXX       -------------------------------------    x..........x -             " +
    "x.x  x..x   -       -x.......x-       x.......x       -    XX           XX    -           -    x..........x -             " +
    "XX    x..x  -       - x.....x -        x.....x        -   x.x           x.x   -           -     x........x  -             " +
    "      x..x  -       -  x...x  -         x...x         -  x..x           x..x  -           -     x........x  -             " +
    "       XX   -       -   x.x   -          x.x          - x...XXXXXXXXXXXXX...x -           -     XXXXXXXXXX  -             " +
    "-------------       -    x    -           x           -x.....................x-           -------------------             " +
    "                    ----------------------------------- x...XXXXXXXXXXXXX...x -                                           " +
    "                                                      -  x..x           x..x  -                                           " +
    "                                                      -   x.x           x.x   -                                           " +
    "                                                      -    XX           XX    -                                           ";


pub const FONT_ATLAS_DEFAULT_TEX_CURSOR_DATA: [[Vector2D;3];9] =
[
    // pos ........ size ......... Offset ......
    [ Vector2D::new( 0f32,3f32), Vector2D::new(12f32,19f32), Vector2D::new( 0f32, 0f32) ], // ImGuiMouseCursor_Arrow
    [ Vector2D::new(13f32,0f32), Vector2D::new( 7f32,16f32), Vector2D::new( 1f32, 8f32) ], // ImGuiMouseCursor_TextInput
    [ Vector2D::new(31f32,0f32), Vector2D::new(23f32,23f32), Vector2D::new(11f32,11f32) ], // ImGuiMouseCursor_ResizeAll
    [ Vector2D::new(21f32,0f32), Vector2D::new( 9f32,23f32), Vector2D::new( 4f32,11f32) ], // ImGuiMouseCursor_ResizeNS
    [ Vector2D::new(55f32,18f32),Vector2D::new(23f32, 9f32), Vector2D::new(11f32, 4f32) ], // ImGuiMouseCursor_ResizeEW
    [ Vector2D::new(73f32,0f32), Vector2D::new(17f32,17f32), Vector2D::new( 8f32, 8f32) ], // ImGuiMouseCursor_ResizeNESW
    [ Vector2D::new(55f32,0f32), Vector2D::new(17f32,17f32), Vector2D::new( 8f32, 8f32) ], // ImGuiMouseCursor_ResizeNWSE
    [ Vector2D::new(91f32,0f32), Vector2D::new(17f32,22f32), Vector2D::new( 5f32, 0f32) ], // ImGuiMouseCursor_Hand
    [ Vector2D::new(109f32,0f32),Vector2D::new(13f32,15f32), Vector2D::new( 6f32, 7f32) ], // ImGuiMouseCursor_NotAllowed
];
