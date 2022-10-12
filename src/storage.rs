#![allow(non_snake_case)]

use std::borrow::BorrowMut;
use std::ptr::{null, null_mut};
use libc::{c_int, c_void};
use crate::imgui_cpp::LowerBound;
use crate::type_defs::ImGuiID;
use crate::utils::ImQsort;

// [Internal]
pub struct ImGuiStoragePair {
    // ImGuiID key;
    pub key: ImGuiID,
    // union { int val_i; float val_f; void* val_p; };
    pub val_i: i32,
    pub val_f: f32,
    pub val_p: *mut c_void,
}

impl ImGuiStoragePair {
    // ImGuiStoragePair(ImGuiID _key, int _val_i)      { key = _key; val_i = _val_i; }
    pub fn new(_key: ImGuiID, _val_i: Option<i32>, _val_f: Option<f32>, _val_p: Option<*mut c_void>) -> Self {
        Self {
            key: _key,
            val_i: _val_i.unwrap_or(0),
            val_f: _val_f.unwrap_or(0.0),
            val_p: _val_p.unwrap_or(null_mut()),
        }
    }
    // ImGuiStoragePair(ImGuiID _key, float _val_0f32)    { key = _key; val_f = _val_f; }
    // ImGuiStoragePair(ImGuiID _key, void* _val_p)    { key = _key; val_p = _val_p; }
}

// For quicker full rebuild of a storage (instead of an incremental one), you may add all your contents and then sort once.
// void ImGuiStorage::BuildSortByKey()

pub fn PairComparerByID(lhs: *const c_void, rhs: *const c_void) -> c_int {
    // We can't just do a subtraction because qsort uses signed integers and subtracting our ID doesn't play well with that.
    if (lhs).key > (rhs).key { return 1; };
    if (lhs).key < (rhs).key { return -1; };
    return 0;
}

// Helper: Key->Value storage
// Typically you don't have to worry about this since a storage is held within each Window.
// We use it to e.g. store collapse state for a tree (Int 0/1)
// This is optimized for efficient lookup (dichotomy into a contiguous buffer) and rare insertion (typically tied to user interactions aka max once a frame)
// You can use it as custom user storage for temporary values. Declare your own storage if, for example:
// - You want to manipulate the open/close state of a particular sub-tree in your interface (tree node uses Int 0/1 to store their state).
// - You want to store custom debug data easily without adding or editing structures in your code (probably not efficient, but convenient)
// Types are NOT stored, so it is up to you to make sure your Key don't collide with different types.
pub struct ImGuiStorage {
    // ImVector<ImGuiStoragePair>      Data;
    pub Data: Vec<ImGuiStoragePair>,

}

impl ImGuiStorage {
    // - Get***() functions find pair, never add/allocate. Pairs are sorted so a query is O(log N)
    // - Set***() functions find pair, insertion on demand if missing.
    // - Sorted insertion is costly, paid once. A typical frame shouldn't need to insert any new pair.
    // void                Clear() { Data.clear(); }
    pub fn Clear(&mut self) {
        self.Data.clear();
    }


    // IMGUI_API int       GetInt(ImGuiID key, int default_val = 0) const;
    pub fn GetInt(&mut self, key: ImGuiID, default_val: i32) -> i32 {
        let it = self.LowerBound(Data, key);
        if it == self.Data.end() || it.key != key {
            return default_val;
        }
        return it.val_i;
    }
    // IMGUI_API void      SetInt(ImGuiID key, int val);
    pub fn SetInt(&mut self, key: ImGuiID, val: i32) {
        let mut it = self.LowerBound(&mut self.Data, key);
        if it == self.Data.last_mut().unwrap() || it.key != key {
            self.Data.push(ImGuiStoragePair(key, val));
            return;
        }
        it.val_i = val;
    }


    // IMGUI_API bool      GetBool(ImGuiID key, default_val: bool = false) const;
    pub fn GetBool(&mut self, key: ImGuiID, default_val: bool) -> bool {
        return self.GetInt(key, if default_val { 1 } else { 0 }) != 0;
    }

    // IMGUI_API void      SetBool(ImGuiID key, val: bool);
    pub fn SetBool(&mut self, key: ImGuiID, val: bool) {
        self.SetInt(key, if val { 1 } else { 0 });
    }

    // IMGUI_API float     GetFloat(ImGuiID key, float default_val = 0.0) const;
    pub fn GetFloat(&mut self, key: ImGuiID, default_val: f32) -> f32 {
        let it = self.LowerBound(&mut self.Data, key);
        if it == self.Data.last_mut().unwrap() || it.key != key {
            return default_val;
        }
        return it.val_f;
    }

    // IMGUI_API void      SetFloat(ImGuiID key, float val);
    pub fn SetFloat(&mut self, key: ImGuiID, val: f32) {
        let mut it = self.LowerBound(Data, key);
        if it == self.Data.last_mut().unwrap() || it.key != key {
            self.Data.push(ImGuiStoragePair(key, val));
            return;
        }
        it.val_f = val;
    }


    // IMGUI_API void*     GetVoidPtr(ImGuiID key) const; // default_val is NULL
    pub fn GetVoidPtr(&mut self, key: ImGuiID) -> *const c_void {
        let it = self.LowerBound(&mut self.Data, key);
        if it == self.Data.end() || it.key != key {
            return null();
        }
        return it.val_p;
    }


    // IMGUI_API void      SetVoidPtr(ImGuiID key, void* val);
    pub fn SetVoidPtr(&mut self, key: ImGuiID, val: *mut c_void) {
        let mut it = self.LowerBound(&mut self.Data, key);
        if it == self.Data.end() || it.key != key {
            self.Data.push(ImGuiStoragePair(key, val));
            return;
        }
        it.val_p = val;
    }

    // - Get***Ref() functions finds pair, insert on demand if missing, return pointer. Useful if you intend to do Get+Set.
    // - References are only valid until a new value is added to the storage. Calling a Set***() function or a Get***Ref() function invalidates the pointer.
    // - A typical use case where this is convenient for quick hacking (e.g. add storage during a live Edit&Continue session if you can't modify existing struct)
    //      float* pvar = GetFloatRef(key); SliderFloat("var", pvar, 0, 100f32); some_var += *pvar;
    // IMGUI_API int*      GetIntRef(ImGuiID key, int default_val = 0);
    pub fn GetIntRef(&mut self, key: ImGuiID, default_val: i32) -> *mut c_int {
        let mut it = self.LowerBound(&mut self.Data, key);
        if it == self.Data.last_mut().unwrap() || it.key != key {
            self.Data.push(ImGuiStoragePair(key, default_val));
        }
        return it.val_i.borrow_mut();
    }


    // IMGUI_API bool*     GetBoolRef(ImGuiID key, default_val: bool = false);
    pub fn GetBoolRef(&mut self, key: ImGUiID, default_val: bool) -> *mut bool {
        return self.GetIntRef(key, if default_val { 1 } else { 0 }) as *mut bool;
    }


    // IMGUI_API float*    GetFloatRef(ImGuiID key, float default_val = 0.0);
    pub fn GetFloatRef(&mut self, key: ImGuiID, default_val: f32) -> *mut f32 {
        let mut it = self.LowerBound(&mut self.Data, key);
        if it == self.Data.last_mut().unwrap() || it.key != key {
            self.Data.push(ImGuiStoragePair(key, default_val));
        }
        return &mut it.val_f;
    }


    // IMGUI_API void**    GetVoidPtrRef(ImGuiID key, void* default_val = NULL);
    pub fn GetVoidPtrRef(&mut self, key: ImGuiID, default_val: *mut c_void) -> *mut *mut c_void {
        let mut it = self.LowerBound(&mut self.Data, key);
        if it == self.Data.last_mut().unwrap() || it.key != key {
            self.Data.push(ImGuiStoragePair(key, default_val));
        }
        return &mut it.val_p;
    }


    // Use on your own storage if you know only integer are being stored (open/close all tree nodes)
    // IMGUI_API void      SetAllInt(int val);
    pub fn SetAllInt(&mut self, val: i32) {
        // for (int i = 0; i < Data.Size; i++)
        for i in 0..self.Data.len() {
            self.Data[i].val_i = v;
        }
    }

    // For quicker full rebuild of a storage (instead of an incremental one), you may add all your contents and then sort once.
    // IMGUI_API void      BuildSortByKey();
    pub fn BuildSortByKey(&mut self) {
        ImQsort(self.Data.as_mut_ptr(), self.Data.len(), libc::sizeof(ImGuiStoragePair), PairComparerByID);
    }

    pub fn LowerBound(&mut self, data: &mut Vec<ImGuiStoragePair>, key: ImGuiID) -> *mut ImGuiStoragePair {
        // ImGuiStorage::ImGuiStoragePair* first = data.Data;
        let mut first: *mut ImGuiStoragePair = data.first_mut().unwrap();
        // ImGuiStorage::ImGuiStoragePair* last = data.Data + data.Size;
        let mut last: *mut ImGuiStoragePair = data.Data + data.Size;
        let count = (last - first);
        while count > 0 {
            let mut count2 = count >> 1;
            // ImGuiStorage::ImGuiStoragePair* mid = first + count2;
            let mut mid: *mut ImGuiStoragePair = first + count2;
            if mid.key < key {
                mid += 1;
                first = mid;
                count -= count2 + 1;
            } else {
                count = count2;
            }
        }
        return first;
    }
}
