#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiNextWindowDataFlags;   // -> enum ImGuiNextWindowDataFlags_// Flags: for SetNextWindowXXX()
pub type ImGuiNextWindowDataFlags = c_int;

// enum ImGuiNextWindowDataFlags_
// {
    pub const ImGuiNextWindowDataFlags_None: ImGuiNextWindowDataFlags =  0;
    pub const ImGuiNextWindowDataFlags_HasPos: ImGuiNextWindowDataFlags =  1 << 0;
    pub const ImGuiNextWindowDataFlags_HasSize: ImGuiNextWindowDataFlags =  1 << 1;
    pub const ImGuiNextWindowDataFlags_HasContentSize: ImGuiNextWindowDataFlags =  1 << 2;
    pub const ImGuiNextWindowDataFlags_HasCollapsed: ImGuiNextWindowDataFlags =  1 << 3;
    pub const ImGuiNextWindowDataFlags_HasSizeConstraint: ImGuiNextWindowDataFlags =  1 << 4;
    pub const ImGuiNextWindowDataFlags_HasFocus: ImGuiNextWindowDataFlags =  1 << 5;
    pub const ImGuiNextWindowDataFlags_HasBgAlpha: ImGuiNextWindowDataFlags =  1 << 6;
    pub const ImGuiNextWindowDataFlags_HasScroll: ImGuiNextWindowDataFlags =  1 << 7;
    pub const ImGuiNextWindowDataFlags_HasViewport: ImGuiNextWindowDataFlags =  1 << 8;
    pub const ImGuiNextWindowDataFlags_HasDock: ImGuiNextWindowDataFlags =  1 << 9;
    pub const ImGuiNextWindowDataFlags_HasWindowClass: ImGuiNextWindowDataFlags =  1 << 10;
// };
