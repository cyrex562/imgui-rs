// Helper: Parse and apply text filters. In format "aaaaa[,bbbb][,ccccc]"
#![allow(non_snake_case)]

use std::borrow::BorrowMut;
use std::ptr::null;
use libc::c_char;
use crate::imgui_cpp::{ImStristr, ImStrncpy};

// [Internal]
#[derive(Default,Debug,Clone)]
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
    pub fn split(&mut self, separator: c_char, out: &mut Vec<ImGuiTextRange>) {
        // out->resize(0);
        let mut wb = b;
        let mut we = wb;
        while we < e
        {
            if *we == separator
            {
                out.push(ImGuiTextRange::new2(wb, we));
                wb = we + 1;
            }
            we+= 1;
        }
        if wb != we {
            out.push(ImGuiTextRange::new2(wb, we));
        }
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
    pub unsafe fn new(default_filter: *const c_char) -> Self {
        // InputBuf[0] = 0;
        // CountGrep = 0;
        let mut out = Self {
            InputBuf: [0;256],
            CountGrep: 0,
            Filters: vec![]
        };
        if default_filter.is_null() == false
        {
            ImStrncpy(out.InputBuf.as_mut_ptr(), default_filter, IM_ARRAYSIZE(InputBuf));
            Build();
        }
        out
    }

    // IMGUI_API bool      Draw(const char* label = "Filter (inc,-exc)", float width = 0f32);  // Helper calling InputText+Build
    pub unsafe fn Draw(&mut self, label: *const c_char, width: f32) -> bool {
        if width != 0f32 {
            SetNextItemWidth(width);
        }
        let value_changed = InputText::new(label, InputBuf, IM_ARRAYSIZE(InputBu0f32));
        if value_changed {
            self.Build();
        }
        return value_changed;
    }


    // IMGUI_API bool      PassFilter(const char* text, const char* text_end = NULL) const;
    pub unsafe fn PassFilter(&mut self, mut text: *const c_char, text_end: *const c_char) -> bool {
        if self.Filters.empty() {
            return true;
        }

        if text.is_null() {
            text = String::from("").into();
        }

        // for (int i = 0; i != Filters.Size; i++)
        for i in 0 .. self.Filters.len()
        {
            let f = self.Filters[i].borrow_mut();
            if f.empty() {
                continue;
            }
            if f.b[0] == '-'
            {
                // Subtract
                if ImStristr(text, text_end, f.b + 1, f.e) != NULL {
                    return false;
                }
            }
            else
            {
                // Grep
                if ImStristr(text, text_end, f.b, f.e) != NULL {
                    return true;
                }
            }
        }

        // Implicit * grep
        if self.CountGrep == 0 {
            return true;
        }

        return false;
    }

    // IMGUI_API void      Build();
    pub unsafe fn Build(&mut self) {
        // Filters.resize(0);
        self.Filters.clear();
        let  mut input_range = ImGuiTextRange::new2(InputBuf, InputBuf + libc::strlen(self.InputBuf.as_ptr()));
        input_range.split(',' as c_char, &mut self.Filters);

        self.CountGrep = 0;
        // for (int i = 0; i != Filters.Size; i++)
        for i in 0 .. self.Filters.len()
        {
            let f = Filters[i];
            while f.b < f.e && ImCharIsBlankA(f.b[0]) {
                f.b += 1;
            }
            while f.e > f.b && ImCharIsBlankA(f.e[-1]) {
                f.e -= 1;
            }
            if f.empty() {
                continue;
            }
            if Filters[i].b[0] != '-' {
                CountGrep += 1;
            }
        }
    }

    // void                Clear()          { InputBuf[0] = 0; Build(); }
    pub unsafe fn Clear(&mut self) {
        self.InputBuf[0] = 0;
        self.Build();
    }

    // bool                IsActive() const { return !Filters.empty(); }
    pub unsafe fn IsActive(&mut self) -> bool {
        !self.Filters.is_empty()
    }
}
