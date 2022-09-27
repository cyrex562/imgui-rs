// Helper: ImSpan<>
// Pointing to a span of data we don't own.
// template<typename T>

use std::mem;
use std::ops::{Index, IndexMut};
use std::ptr::null_mut;
use libc::c_int;

#[derive(Default, Debug, Clone)]
pub struct ImSpan<T> {
    // *mut T                  Data;
    pub Data: *mut T,
    // *mut T                  DataEnd;
    pub DataEnd: *mut T,

}

impl ImSpan<T> {
    // Constructors, destructor
    // inline ImSpan()                                 { Data = DataEnd = None; }
    pub fn new() -> Self {
        Self {
            Data: null_mut(),
            DataEnd: null_mut(),
        }
    }
    // inline ImSpan(*mut T data, c_int size)                { Data = data; DataEnd = data + size; }
    pub fn new2(data: *mut T, size: c_int) -> Self {
        Self {
            Data: data,
            DataEnd: data + size,
        }
    }
    // inline ImSpan(*mut T data, *mut T data_end)             { Data = data; DataEnd = data_end; }
    pub fn new3(data: *mut T, data_end: *mut T) -> Self {
        Self {
            Data: data,
            DataEnd: data_end,
        }
    }

    // inline c_void         set(*mut T data, c_int size)      { Data = data; DataEnd = data + size; }
    pub fn set(&mut self, data: *mut T, size: c_int) {
        self.Data = data;
        self.DataEnd = data + size;
    }

    // inline c_void         set(*mut T data, *mut T data_end)   { Data = data; DataEnd = data_end; }
    pub fn set2(&mut self, data: *mut T, data_end: *mut T) {
        self.Data = data;
        self.DataEnd = data_end;
    }
    // inline c_int          size() const                { return (ptrdiff_t)(DataEnd - Data); }
    pub fn size(&self) -> c_int {
        self.DataEnd - self.Data
    }
    // inline c_int          size_in_bytes() const       { return (ptrdiff_t)(DataEnd - Data) * sizeof(T); }
    pub fn size_in_bytes(&self) -> c_int {
        (self.DataEnd - self.Data) * mem::size_of::<T>()
    }
    // inline T&           operator[](c_int i)           { *mut T p = Data + i; IM_ASSERT(p >= Data && p < DataEnd); return *p; }

    // inline const T&     operator[](c_int i) const     { *const T p = Data + i; IM_ASSERT(p >= Data && p < DataEnd); return *p; }

    // inline *mut T           begin()                     { return Data; }
    pub fn begin_mut(&mut self) -> *mut T {
        self.Data
    }
    // inline *const T     begin() const               { return Data; }
    pub fn begin(&self) -> *const T {
        self.Data
    }
    // inline *mut T           end()                       { return DataEnd; }
    pub fn end_mut(&mut self) -> *mut T {
        self.DataEnd
    }

    // inline *const T     end() const                 { return DataEnd; }
    pub fn end(&self) -> *const T {
        self.DataEnd
    }

    // Utilities
    // inline c_int  index_from_ptr(*const T it) const   { IM_ASSERT(it >= Data && it < DataEnd); const ptrdiff_t off = it - Data; return off; }
    pub fn index_from_ptr(&self, it: *const T) -> c_int {
        let off = it - self.Data;
        return off;
    }
}

impl Index<usize> for ImSpan<T> {
    type Output = *const T;

    fn index(&self, index: usize) -> &Self::Output {
        self.Data + index
    }
}

impl IndexMut<usize> for ImSpan<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.Data + index
    }
}
