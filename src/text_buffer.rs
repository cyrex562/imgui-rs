#![allow(non_snake_case)]

use std::borrow::{Borrow, BorrowMut};
use std::ops::{Index, IndexMut};
use std::ptr::null;
use libc::{c_char, c_int};

// Helper: Growable text buffer for logging/accumulating text
// (this could be called 'ImGuiTextBuilder' / 'ImGuiStringBuilder')
#[derive(Default,Clone,Debug)]
pub struct ImGuiTextBuffer
{
    // ImVector<char>      Buf;
    pub Buf: Vec<c_char>,

    // IMGUI_API static char EmptyString[1];
    pub EmptyString: [c_char;1]
}

impl ImGuiTextBuffer {
    // ImGuiTextBuffer()   { }
    pub fn new() -> Self {
        Self {
            Buf: vec![],
            EmptyString: [0;1]
        }
    }

    // inline char         operator[](int i) const { IM_ASSERT(Buf.Data != NULL); return Buf.Data[i]; }

    // const char*         begin() const           { return Buf.Data ? &Buf.front() : EmptyString; }
    pub fn begin(&self) -> *const c_char {
        return if self.Buf.is_empty() {
            null()
        } else {
            self.Buf.first().unwrap()
        }
    }

    // const char*         end() const             { return Buf.Data ? &Buf.back() : EmptyString; }   // Buf is zero-terminated, so end() will point on the zero-terminator
    pub fn end(&self) -> *const c_char {
        return if self.Buf.is_empty() {
            null()
        } else {
            self.Buf.last().unwrap()
        }
    }

    // int                 size() const            { return Buf.Size ? Buf.Size - 1 : 0; }
    pub fn size(&self) -> usize {
        self.Buf.len()
    }


    // bool                empty() const           { return Buf.Size <= 1; }
    pub fn empty(&self) -> bool {
        self.Buf.is_empty()
    }


    // void                clear()                 { Buf.clear(); }
    pub fn clear(&mut self) {
        self.Buf.clear()
    }


    // void                reserve(int capacity)   { Buf.reserve(capacity); }
    pub fn reserve(&mut self, capacity: usize) {
        self.Buf.reserve(capacity)
    }


    // const char*         c_str() const           { return Buf.Data ? Buf.Data : EmptyString; }
    pub fn c_str(&self) -> *const c_char {
        if self.Buf.is_empty() {
            return null()
        } else {
            self.Buf.as_ptr()
        }
    }


    // IMGUI_API void      append(const char* str, const char* str_end = NULL);
    pub unsafe fn append(&mut self,begin: *const c_char, end: *const c_char) {
        let len = if end.is_null() == false { (str_end - str) } else { libc::strlen(str) };

        // Add zero-terminator the first time
        let write_off = if self.Buf.len() != 0 { Buf.Size } else { 1 };
        let needed_sz = write_off + len;
        if write_off.clone() + len.clone() >= self.Buf.capacity()
        {
            let new_capacity = Buf.Capacity * 2;
            self.Buf.reserve(if needed_sz > new_capacity { needed_sz } else { new_capacity });
        }

        self.Buf.resize(needed_sz);
        libc::memcpy(&mut self.Buf[write_off.clone() - 1..], begin, len.clone());
        self.Buf[write_off.clone() - 1 + len.clone()] = 0;
    }


    // IMGUI_API void      appendf(const char* fmt, ...) IM_FMTARGS(2);

    // IMGUI_API void      appendfv(const char* fmt, va_list args) IM_FMTLIST(2);
}

impl Index<usize> for ImGuiTextBuffer  {
    type Output = c_char;

    fn index(&self, index: usize) -> &Self::Output {
        self.Buf[index].borrow()
    }
}

impl IndexMut<usize> for ImGuiTextBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.Buf[index].borrow_mut()
    }
}
