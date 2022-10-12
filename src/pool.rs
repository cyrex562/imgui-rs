#![allow(non_snake_case)]
// Helper: ImPool<>
// Basic keyed storage for contiguous instances, slow/amortized insertion, O(1) indexable, O(Log N) queries by ID over a dense/hot buffer,
// Honor constructor/destructor. Add/remove invalidate all pointers. Indexes have the same lifetime as the associated object.

use std::ptr::null_mut;
use libc::c_int;
use crate::storage::ImGuiStorage;
use crate::nav_item_data::ImGuiNavItemData;
use crate::type_defs::{ImGuiID, ImPoolIdx};

// template<typename T>
#[derive(Default,Debug,Clone)]
pub struct ImPool<T>
{
pub Buf: Vec<T>,        // Contiguous data
pub Map: ImGuiStorage,        // ID->Index
pub FreeIdx: ImPoolIdx,    // Next free idx to use
pub AliveCount: ImPoolIdx, // Number of active/alive items (for display purpose)

}

impl ImPool<T> {
    // ImPool()    { FreeIdx = AliveCount = 0; }
    // ~ImPool()   { Clear(); }
    
    
    // *mut T          GetByKey(ImGuiID key)               { idx: c_int = Map.GetInt(key, -1); return (idx != -1) ? &Buf[idx] : NULL; }
    pub fn GetByKey(&mut self, key: ImGuiID) -> *mut T {
        let idx = self.Map.GetInt(key, -1);
        return if idx != -1 {
            &mut self.Buf[idx]
        } else {
            null_mut()
        }
    }
    
    // *mut T          GetByIndex(ImPoolIdx n)             { return &Buf[n]; }
    pub fn GetByIndex(&mut self, n: ImPoolIdx) -> *mut T {
        &mut self.Buf[n]
    }
    
    // ImPoolIdx   GetIndex(*const T p) const          { IM_ASSERT(p >= Buf.Data && p < Buf.Data + Buf.Size); return (ImPoolIdx)(p - Buf.Data); }
    pub fn GetIndex(&self, p: *const T) -> ImPoolIdx {
        // p - self.Buf
        todo!()
    }

    
    // *mut T          GetOrAddByKey(ImGuiID key)          { *mut p_idx: c_int = Map.GetIntRef(key, -1); if (*p_idx != -1) return &Buf[*p_idx]; *p_idx = FreeIdx; return Add(); }
    pub unsafe fn GetOrAddByKey(&mut self, key: ImGuiID) -> *mut T {
        let mut p_idx = self.Map.GetIntRef(key, -1);
        if *p_idx != -1 {
            return &mut Buf[*p_idx]
        } else {
            *p_idx = self.FreeIdx;
            return self.Add()
        }
    }

    
    // bool        Contains(*const T p) const          { return (p >= Buf.Data && p < Buf.Data + Buf.Size); }
    pub fn Contains(&mut self, p: *const T) -> bool {
        // if p >= self.Buf
        todo!()
    }

    
    // void        Clear()                             { for (n: c_int = 0; n < Map.Data.Size; n++) { idx: c_int = Map.Data[n].val_i; if (idx != -1) Buf[idx].~T(); } Map.Clear(); Buf.clear(); FreeIdx = AliveCount = 0; }
    pub fn Clear(&mut self) {
        for n in 0 .. self.Map.Data.len() {
            let idx = self.Map.Data[n].val_i;
            if idx != -1 {
                // self.Buf[idx]
            }
        }
        self.Map.Clear();
        self.Buf.clear();
        self.FreeIdx = 0;
        self.AliveCount = 0;
    }

    
    // *mut T          Add()                               { idx: c_int = FreeIdx; if (idx == Buf.Size) { Buf.resize(Buf.Size + 1); FreeIdx+= 1; } else { FreeIdx = *(*mut c_int)&Buf[idx]; } IM_PLACEMENT_NEW(&Buf[idx]) T(); AliveCount+= 1; return &Buf[idx]; }
    pub fn Add(&mut self) -> *mut T {
        let idx = self.FreeIdx;
        if idx == self.Buf.len() as ImPoolIdx {
            // self.Buf.resize(self.Buf.len() + 1);
        } else {
            self.FreeIdx = self.Buf[idx];
        }
        // IM_PLACEMENT_NEW(&self.Buf[idx])
        self.AliveCount += 1;
        return &mut self.Buf[idx]
    }

    
    // void        Remove(ImGuiID key, *const T p)     { Remove(key, GetIndex(p)); }
    pub fn Remove(&mut self, key: ImGuiID, p: *const  T) {
        self.Remove2(key, self.GetIndex(p));
    }

    
    // void        Remove(ImGuiID key, ImPoolIdx idx)  { Buf[idx].~T(); *(*mut c_int)&Buf[idx] = FreeIdx; FreeIdx = idx; Map.SetInt(key, -1); AliveCount-= 1; }
    pub fn Remove2(&mut self, key: ImGuiID, idx: ImPoolIdx) {
        todo!()
    }

    
    // void        Reserve(capacity: c_int)               { Buf.reserve(capacity); Map.Data.reserve(capacity); }
    pub fn Reserve(&mut self, capacity: c_int) {
        self.Buf.reserve(capacity as usize);
        self.Map.Data.reserve(capacity as usize);
    }

// To iterate a ImPool: for (int n = 0; n < pool.GetMapSize(); n++) if (T* t = pool.TryGetMapData(n)) { ... }
// Can be avoided if you know .Remove() has never been called on the pool, or AliveCount == GetMapSize()

    
    // c_int         GetAliveCount() const               { return AliveCount; }      // Number of active/alive items in the pool (for display purpose)
    pub fn GetAliveCount(&self) -> c_int {
        self.AliveCount
    }

    
    // c_int         GetBufSize() const                  { return Buf.Size; }\
    pub fn GetBufSize(&self) -> c_int {
        self.Buf.len() as c_int
    }

    
    // c_int         GetMapSize() const                  { return Map.Data.Size; }   // It is the map we need iterate to find valid items, since we don't have "alive" storage anywhere
    pub fn GetMapSize(&self) -> c_int {
        self.Map.Data.len() as c_int
    }

    
    // *mut T          TryGetMapData(ImPoolIdx n)          { idx: c_int = Map.Data[n].val_i; if (idx == -1) return NULL; return GetByIndex(idx); }
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    pub fn TryGetMapData(&mut self, n: ImPoolIdx) -> *mut T {
        let idx = self.Map.Data[n].val_i;
        return if idx == -1 {
            null_mut()
        } else {
            self.GetByIndex(idx)
        }
    }

    
    // c_int         GetSize()                           { return GetMapSize(); } // For ImPlot: should use GetMapSize() from (IMGUI_VERSION_NUM >= 18304)
    pub fn GetSize(&mut self) -> c_int {
        self.GetMapSize()
    }

// #endif
}
