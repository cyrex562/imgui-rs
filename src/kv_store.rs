use std::borrow::BorrowMut;
use std::ffi::c_void;
use std::intrinsics::size_of;
use std::mem;
use crate::imgui_h::ImGuiInputTextCallbackData;
use crate::imgui_sort::ImQsort;

pub enum ImGuiStoragePairValTypes {
    Int(i32),
    Float(f32),
    VoidPtr(*mut c_void),
    Bool(bool)
}

union  ImGuiStoragePairVal {
    pub val_i: i32,
    pub val_f: f32,
    pub val_p: *mut c_void,
    pub val_b: bool
}

#[derive(Debug,Clone,Default)]
pub struct ImGuiStoragePair {
    // ImGuiID key;
    pub key: ImGuiID,
    // union {
    // int val_i; pub val_f: f32,
    // void * val_p; };
    pub val: ImGuiStoragePairVal,

}

impl ImGuiStoragePair {
    //     ImGuiStoragePair(ImGuiID _key, int _val_i)      { key = _key; val_i = _val_i; }
    pub fn new(key: ImGuiID, val_in: ImGuiStoragePairValTypes) -> Self {
        let mut val: ImGuiStoragePairVal = match val_in {
             ImGuiStoragePairValTypes::Int(i) => ImGuiStoragePairVal{val_i:i},
            ImGuiStoragePairValTypes::Float(f) => ImGuiStoragePairVal{val_f:f},
            ImGuiStoragePairValTypes::VoidPtr(x) =>  ImGuiStoragePairVal{val_p: x},
            ImGuiStoragePairValTypes::Bool(b) => ImGuiStoragePairVal{val_b: b}
        };
        Self {
            key,
            val,
        }
    }
    // ImGuiStoragePair(ImGuiID _key, float _val_f)    { key = _key; val_f = _val_f; }
    // ImGuiStoragePair(ImGuiID _key, void* _val_p)    { key = _key; val_p = _val_p; }
}

// Helper: Key->value storage
// Typically you don't have to worry about this since a storage is held within each window.
// We use it to e.g. store collapse state for a tree (Int 0/1)
// This is optimized for efficient lookup (dichotomy into a contiguous buffer) and rare insertion (typically tied to user interactions aka max once a frame)
// You can use it as custom user storage for temporary values. Declare your own storage if, for example:
// - You want to manipulate the open/close state of a particular sub-tree in your interface (tree node uses Int 0/1 to store their state).
// - You want to store custom debug data easily without adding or editing structures in your code (probably not efficient, but convenient)
// Types are NOT stored, so it is up to you to make sure your Key don't collide with different types.
#[derive(Debug,Clone,Default)]
pub struct Storage
{
    pub Data: Vec<ImGuiStoragePair>,

    // ImVector<ImGuiStoragePair>      data;

    // - Get***() functions find pair, never add/allocate. Pairs are sorted so a query is O(log N)
    // - Set***() functions find pair, insertion on demand if missing.
    // - Sorted insertion is costly, paid once. A typical frame shouldn't need to insert any new pair.

}

impl Storage {
    // void                clear() { data.clear(); }
    pub fn Clear(&mut self) {
        self.data.clear()
    }
    //  int       GetInt(ImGuiID key, int default_val = 0) const;
    pub fn GetInt(&self, key: ImGuiID, default_val: i32) -> i32 {
        for x in self.data.iter() {
            if x.key == key {
                return x.val.val_i
            }
        }
        return default_val
    }
    //  void      SetInt(ImGuiID key, int val);
    pub fn SetInt(&mut self, key: ImGuiID, val: i32) {
        for x in self.data.iter_mut() {
            if x.key == key {
                x.val.val_i = val
            }
        }
    }
    //  bool      GetBool(ImGuiID key, bool default_val = false) const;
    pub fn GetBool(&self, key: ImGuiID, default_val: bool) -> bool {
        for x in self.data.iter() {
            if x.key == key {
                return x.val.val_b
            }
        }
        return default_val
    }
    //  void      SetBool(ImGuiID key, bool val);
    pub fn SetBool(&mut self, key: ImGuiID, val: bool) {
        for x in self.data.iter_mut() {
            if x.key == key {
                x.val.val_b = val
            }
        }
    }
    //  float     GetFloat(ImGuiID key, float default_val = 0.0) const;
    pub fn GetFloat(&self, key: ImGuiID, default_val: f32) -> f32 {
        for x in self.data.iter() {
            if x.key == key {
                return x.val.val_f
            }
        }
        return default_val
    }
    //  void      SetFloat(ImGuiID key, float val);
    pub fn SetFloat(&mut self, key: ImGuiID, val: f32) {
        for x in self.data.iter_mut() {
            if x.key == key {
                x.val.val_f = val
            }
        }
    }
    //  void*     GetVoidPtr(ImGuiID key) const; // default_val is NULL
    //  void      SetVoidPtr(ImGuiID key, void* val);
    //
    // // - Get***Ref() functions finds pair, insert on demand if missing, return pointer. Useful if you intend to do Get+Set.
    // // - References are only valid until a new value is added to the storage. Calling a Set***() function or a Get***Ref() function invalidates the pointer.
    // // - A typical use case where this is convenient for quick hacking (e.g. add storage during a live Edit&Continue session if you can't modify existing struct)
    // //      float* pvar = ImGui::GetFloatRef(key); ImGui::SliderFloat("var", pvar, 0, 100.0); some_var += *pvar;
    //  int*      GetIntRef(ImGuiID key, int default_val = 0);
    //  bool*     GetBoolRef(ImGuiID key, bool default_val = false);
    //  float*    GetFloatRef(ImGuiID key, float default_val = 0.0);
    //  void**    GetVoidPtrRef(ImGuiID key, void* default_val = NULL);
    //
    // // Use on your own storage if you know only integer are being stored (open/close all tree nodes)
    //  void      SetAllInt(int val);
    //
    // // For quicker full rebuild of a storage (instead of an incremental one), you may add all your contents and then sort once.
    //  void      BuildSortByKey();


// std::lower_bound but without the bullshit
// static ImGuiStorage::ImGuiStoragePair* LowerBound(ImVector<ImGuiStorage::ImGuiStoragePair>& data, ImGuiID key)
pub fn LowerBound(data: &mut Vec<ImGuiStoragePair>, key: ImGuiID) -> *mut ImGuiStoragePair
    {
    // ImGuiStorage::ImGuiStoragePair* first = data.data;
    let mut first: *mut ImGuiStoragePair = data.first_mut().unwrap();
        // ImGuiStorage::ImGuiStoragePair* last = data.data + data.size;
        let mut last: *mut ImGuiStoragePair = data.last_mut().unwrap();
    // size_t count = (size_t)(last - first);
    let mut count: usize = data.len();
        while count > 0
    {
        let mut count2 = count >> 1;
        // ImGuiStorage::ImGuiStoragePair* mid = first + count2;
        let mut mid = first + count2;
        if mid.key < key
        {
            first = mid;
            mid += 1;
            count -= count2 + 1;
        }
        else
        {
            count = count2;
        }
    }
    return first;
}

// For quicker full rebuild of a storage (instead of an incremental one), you may add all your contents and then sort once.
// void ImGuiStorage::BuildSortByKey()

pub fn PairComparerByID(lhs: *mut c_void, rhs: *mut c_void) -> i32 {
    if ((lhs as *mut ImGuiStoragePair).key > (rhs as *mut ImGuiStoragePair).key) { return 1; };
            if ((lhs as *mut ImGuiStoragePair).key < (rhs as *mut ImGuiStoragePair).key) { return -1; }
            return 0;
}

pub unsafe fn BuildSortByKey(&mut self)
{
    // struct StaticFunc
    // {
    //     static int IMGUI_CDECL PairComparerByID(const void* lhs, const void* rhs)
    //     {
    //         // We can't just do a subtraction because qsort uses signed integers and subtracting our id doesn't play well with that.
    //         if (((const ImGuiStoragePair*)lhs)->key > ((const ImGuiStoragePair*)rhs)->key) return +1;
    //         if (((const ImGuiStoragePair*)lhs)->key < ((const ImGuiStoragePair*)rhs)->key) return -1;
    //         return 0;
    //     }
    // };
    ImQsort(&mut self.data as *mut c_void, self.data.len(), mem::size_of::<ImGuiStoragePair>(), PairComparerByID);
}

// int ImGuiStorage::GetInt(ImGuiID key, int default_val) const
// {
//     ImGuiStoragePair* it = LowerBound(const_cast<ImVector<ImGuiStoragePair>&>(data), key);
//     if (it == data.end() || it->key != key)
//         return default_val;
//     return it->val_i;
// }

// bool ImGuiStorage::GetBool(ImGuiID key, bool default_val) const
// {
//     return GetInt(key, default_val ? 1 : 0) != 0;
// }

// float ImGuiStorage::GetFloat(ImGuiID key, float default_val) const
// {
//     ImGuiStoragePair* it = LowerBound(const_cast<ImVector<ImGuiStoragePair>&>(data), key);
//     if (it == data.end() || it->key != key)
//         return default_val;
//     return it->val_f;
// }

// void* ImGuiStorage::GetVoidPtr(ImGuiID key) const
// {
//     ImGuiStoragePair* it = LowerBound(const_cast<ImVector<ImGuiStoragePair>&>(data), key);
//     if (it == data.end() || it->key != key)
//         return NULL;
//     return it->val_p;
// }

// References are only valid until a new value is added to the storage. Calling a Set***() function or a Get***Ref() function invalidates the pointer.
// int* ImGuiStorage::GetIntRef(ImGuiID key, int default_val)
pub fn GetIntRef(&mut self, key: ImGuiID, default_val: i32) -> *mut i32
    {
    // ImGuiStoragePair* it = LowerBound(data, key);
    // if (it == data.end() || it->key != key)
    //     it = data.insert(it, ImGuiStoragePair(key, default_val));
    // return &it->val_i;
        for x in self.data.iter_mut() {
            if x.Key == key {
                return x.as_mut_ref();
            }
        }
        let mut new_pair = ImGuiStoragePair::new(key, ImGuiStoragePairValTypes::Int(default_val));
        self.data.push(new_pair);
        self.data.last_mut().unwrap().val.val_i.borrow_mut()
}

// bool* ImGuiStorage::GetBoolRef(ImGuiID key, bool default_val)
pub fn GetBoolRef(&mut self, key: ImGuiID, default_val: bool) -> *mut bool
    {
    // return (bool*)GetIntRef(key, default_val ? 1 : 0);
        for x in self.data.iter_mut() {
            if x.Key == key {
                return x.as_mut_ref();
            }
        }
        let mut new_pair = ImGuiStoragePair::new(key, ImGuiStoragePairValTypes::Bool(default_val));
        self.data.push(new_pair);
        self.data.last_mut().unwrap().val.val_b.borrow_mut()
}

// float* ImGuiStorage::GetFloatRef(ImGuiID key, float default_val)
pub fn GetFloatRef(&mut self, key: ImGuiID, default_val: f32) -> *mut f32
    {
    // ImGuiStoragePair* it = LowerBound(data, key);
    // if (it == data.end() || it->key != key)
    //     it = data.insert(it, ImGuiStoragePair(key, default_val));
    // return &it->val_f;
        for x in self.data.iter_mut() {
            if x.Key == key {
                return x.as_mut_ref();
            }
        }
        let mut new_pair = ImGuiStoragePair::new(key, ImGuiStoragePairValTypes::Float(default_val));
        self.data.push(new_pair);
        self.data.last_mut().unwrap().val.val_f.borrow_mut()
}

// void** ImGuiStorage::GetVoidPtrRef(ImGuiID key, void* default_val)
pub fn GetVoidPtrRef(&mut self, key: ImGuiID, default_val: *mut c_void) -> *mut *mut c_void
    {
    // ImGuiStoragePair* it = LowerBound(data, key);
    // if (it == data.end() || it->key != key)
    //     it = data.insert(it, ImGuiStoragePair(key, default_val));
    // return &it->val_p;
        for x in self.data.iter_mut() {
            if x.Key == key {
                return x.as_mut_ref();
            }
        }
        let mut new_pair = ImGuiStoragePair::new(key, ImGuiStoragePairValTypes::VoidPtr(default_val));
        self.data.push(new_pair);
        self.data.last_mut().unwrap().val.val_p.borrow_mut()
}

// FIXME-OPT: Need a way to reuse the result of lower_bound when doing GetInt()/SetInt() - not too bad because it only happens on explicit interaction (maximum one a frame)
// void ImGuiStorage::SetInt(ImGuiID key, int val)
// {
//     ImGuiStoragePair* it = LowerBound(data, key);
//     if (it == data.end() || it->key != key)
//     {
//         data.insert(it, ImGuiStoragePair(key, val));
//         return;
//     }
//     it->val_i = val;
// }

// void ImGuiStorage::SetBool(ImGuiID key, bool val)
// {
//     SetInt(key, val ? 1 : 0);
// }

// void ImGuiStorage::SetFloat(ImGuiID key, float val)
// {
//     ImGuiStoragePair* it = LowerBound(data, key);
//     if (it == data.end() || it->key != key)
//     {
//         data.insert(it, ImGuiStoragePair(key, val));
//         return;
//     }
//     it->val_f = val;
// }

// void ImGuiStorage::SetVoidPtr(ImGuiID key, void* val)
// {
//     ImGuiStoragePair* it = LowerBound(data, key);
//     if (it == data.end() || it->key != key)
//     {
//         data.insert(it, ImGuiStoragePair(key, val));
//         return;
//     }
//     it->val_p = val;
// }

// void ImGuiStorage::SetAllInt(int v)
// {
//     for (int i = 0; i < data.size; i++)
//         data[i].val_i = v;
// }
}


