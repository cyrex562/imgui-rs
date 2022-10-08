use libc::c_int;

// A work of art lies ahead! (. = white layer, X = black layer, others are blank)
// The 2x2 white texels on the top left are the ones we'll use everywhere in Dear ImGui to render filled shapes.
// (This is used when io.MouseDrawCursor = true)
pub const FONT_ATLAS_DEFAULT_TEX_DATA_W: c_int = 122;
// Actual texture will be 2 times that + 1 spacing.
pub const FONT_ATLAS_DEFAULT_TEX_DATA_H: c_int = 27;
pub const FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS: [&'static str; (FONT_ATLAS_DEFAULT_TEX_DATA_W * FONT_ATLAS_DEFAULT_TEX_DATA_H + 1) as usize] = [
    "..-         -XXXXXXX-    X    -           X           -XXXXXXX          -          XXXXXXX-     XX          - XX       XX ",
    "..-         -X.....X-   X.X   -          X.X          -X.....X          -          X.....X-    X..X         -X..X     X..X",
    "---         -XXX.XXX-  X...X  -         X...X         -X....X           -           X....X-    X..X         -X...X   X...X",
    "X           -  X.X  - X.....X -        X.....X        -X...X            -            X...X-    X..X         - X...X X...X ",
    "XX          -  X.X  -X.......X-       X.......X       -X..X.X           -           X.X..X-    X..X         -  X...X...X  ",
    "X.X         -  X.X  -XXXX.XXXX-       XXXX.XXXX       -X.X X.X          -          X.X X.X-    X..XXX       -   X.....X   ",
    "X..X        -  X.X  -   X.X   -          X.X          -XX   X.X         -         X.X   XX-    X..X..XXX    -    X...X    ",
    "X...X       -  X.X  -   X.X   -    XX    X.X    XX    -      X.X        -        X.X      -    X..X..X..XX  -     X.X     ",
    "X....X      -  X.X  -   X.X   -   X.X    X.X    X.X   -       X.X       -       X.X       -    X..X..X..X.X -    X...X    ",
    "X.....X     -  X.X  -   X.X   -  X..X    X.X    X..X  -        X.X      -      X.X        -XXX X..X..X..X..X-   X.....X   ",
    "X......X    -  X.X  -   X.X   - X...XXXXXX.XXXXXX...X -         X.X   XX-XX   X.X         -X..XX........X..X-  X...X...X  ",
    "X.......X   -  X.X  -   X.X   -X.....................X-          X.X X.X-X.X X.X          -X...X...........X- X...X X...X ",
    "X........X  -  X.X  -   X.X   - X...XXXXXX.XXXXXX...X -           X.X..X-X..X.X           - X..............X-X...X   X...X",
    "X.........X -XXX.XXX-   X.X   -  X..X    X.X    X..X  -            X...X-X...X            -  X.............X-X..X     X..X",
    "X..........X-X.....X-   X.X   -   X.X    X.X    X.X   -           X....X-X....X           -  X.............X- XX       XX ",
    "X......XXXXX-XXXXXXX-   X.X   -    XX    X.X    XX    -          X.....X-X.....X          -   X............X--------------",
    "X...X..X    ---------   X.X   -          X.X          -          XXXXXXX-XXXXXXX          -   X...........X -             ",
    "X..X X..X   -       -XXXX.XXXX-       XXXX.XXXX       -------------------------------------    X..........X -             ",
    "X.X  X..X   -       -X.......X-       X.......X       -    XX           XX    -           -    X..........X -             ",
    "XX    X..X  -       - X.....X -        X.....X        -   X.X           X.X   -           -     X........X  -             ",
    "      X..X  -       -  X...X  -         X...X         -  X..X           X..X  -           -     X........X  -             ",
    "       XX   -       -   X.X   -          X.X          - X...XXXXXXXXXXXXX...X -           -     XXXXXXXXXX  -             ",
    "-------------       -    X    -           X           -X.....................X-           -------------------             ",
    "                    ----------------------------------- X...XXXXXXXXXXXXX...X -                                           ",
    "                                                      -  X..X           X..X  -                                           ",
    "                                                      -   X.X           X.X   -                                           ",
    "                                                      -    XX           XX    -                                           "
];
