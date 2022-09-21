// Helper: Parse and apply text filters. In format "aaaaa[,bbbb][,ccccc]"
#![allow(non_snake_case)]

use std::ptr::null;
use libc::c_char;

// [Internal]
pub struct ImGuiTextRange
{
    // const char*     b;
    // const char*     e;
    pub b: *const c_char,
    pub e: *const c_char
}

impl ImGuiTextRange {
    // ImGuiTextRange()                                { b = e = None; }
    pub fn new() -> Self {
        Self {
            b: null(),
            e: null()
        }
    }

    // ImGuiTextRange(const char* _b, const char* _e)  { b = _b; e = _e; }
    pub fn new2(b: *const c_char, e: *const c_char) -> Self {
        Self {
            b,
            e,
        }
    }

    // bool            empty() const                   { return b == e; }
    pub fn empty(&mut self) -> bool {
        self.b == self.e
    }

    // IMGUI_API void  split(char separator, ImVector<ImGuiTextRange>* out) const;
    pub fn split(&mut self, separator, c_char, out: &mut Vec<ImGuiTextRange>) {

    }
}

#[derive(Default,Debug,Clone)]
pub struct ImGuiTextFilter
{
    // char                    InputBuf[256];
    pub InputBuf: [c_char;256],
    // ImVector<ImGuiTextRange>Filters;
    pub Filters: Vec<ImGuiTextRange>,
    // int                     CountGrep;
    pub CountGrep: i32
}

impl ImGuiTextFilter {
    // IMGUI_API           ImGuiTextFilter(const char* default_filter = "");

    // IMGUI_API bool      Draw(const char* label = "Filter (inc,-exc)", float width = 0f32);  // Helper calling InputText+Build

    // IMGUI_API bool      PassFilter(const char* text, const char* text_end = NULL) const;

    // IMGUI_API void      Build();

    // void                Clear()          { InputBuf[0] = 0; Build(); }

    // bool                IsActive() const { return !Filters.empty(); }
}
