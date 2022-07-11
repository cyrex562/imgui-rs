use std::ptr::{null, null_mut};
use crate::imgui_h::ImGuiID;
use crate::imgui_kv_store::ImGuiStorage;

// Helper: ImPool<>
// Basic keyed storage for contiguous instances, slow/amortized insertion, O(1) indexable, O(Log N) queries by id over a dense/hot buffer,
// Honor constructor/destructor. Add/remove invalidate all pointers. Indexes have the same lifetime as the associated object.
// typedef int ImPoolIdx;
pub type ImGuiPoolIdx = isize;
// template<typename T>
pub struct ImGuiPool<T>
{
    // ImVector<T>     Buf;        // Contiguous data
    pub Buf: Vec<T>,
    // ImGuiStorage    Map;        // id->index
    pub Map: ImGuiStorage,
    // ImPoolIdx       FreeIdx;    // Next free idx to use
    pub FreeIdx: ImGuiPoolIdx,
    // ImPoolIdx       AliveCount; // Number of active/alive items (for display purpose)
    pub AliveCount: ImGuiPoolIdx,
}

impl<T> ImGuiPool<T> {
    // ImPool()    { FreeIdx = AliveCount = 0; }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     ~ImPool()   { clear(); }
    //     T*          GetByKey(ImGuiID key)               { int idx = Map.GetInt(key, -1); return (idx != -1) ? &Buf[idx] : NULL; }
    pub fn GetByKey(&self, key: ImGuiID) -> *mut T {
        let idx = self.Map.GetInt(key, -1);
        if idx != -1 {
            return &mut self.Buf[idx];
        } else {
            return null_mut();
        }
    }
    //     T*          GetByIndex(ImPoolIdx n)             { return &Buf[n]; }
    pub fn GetByIndex(&self, n: usize) -> T {
        self.Buf[n]
    }
    //     ImPoolIdx   GetIndex(const T* p) const          { IM_ASSERT(p >= Buf.data && p < Buf.data + Buf.size); return (ImPoolIdx)(p - Buf.data); }
    pub fn GetIndex(&self, p: *const T) -> ImGuiPoolIdx {
        let mut out_idx: isize = -1;
        for x in self.Buf.iter() {
            if p == x {
                break;
            }
            out_idx += 1;
        }
        out_idx
    }
    //     T*          GetOrAddByKey(ImGuiID key)          { int* p_idx = Map.GetIntRef(key, -1); if (*p_idx != -1) return &Buf[*p_idx]; *p_idx = FreeIdx; return Add(); }
    pub unsafe fn GetOrAddByKey(&mut self, key: ImGuiID) -> *mut T {
        let mut p_idx: *mut i32 = self.Map.GetIntRef(key, -1);
        if *p_idx != -1 {
            return &mut self.Buf[*p_idx];
        }
        *p_idx = self.FreeIdx as i32;
        self.Add()
    }


    //     bool        contains(const T* p) const          { return (p >= Buf.data && p < Buf.data + Buf.size); }
    pub fn Contains(&self, p: *const T) -> bool {
        for x in self.Buf.iter() {
            if p == x {
                return true;
            }
        }
        return false;
    }
    //     void        clear()                             { for (int n = 0; n < Map.data.size; n++) { int idx = Map.data[n].val_i; if (idx != -1) Buf[idx].~T(); } Map.clear(); Buf.clear(); FreeIdx = AliveCount = 0; }
    pub fn Clear(&mut self) {
        self.Buf.clear();
        self.FreeIdx = 0;
        self.AliveCount = 0;
    }

    //     T*          Add()                               { int idx = FreeIdx; if (idx == Buf.size) { Buf.resize(Buf.size + 1); FreeIdx++; } else { FreeIdx = *(int*)&Buf[idx]; } IM_PLACEMENT_NEW(&Buf[idx]) T(); AliveCount++; return &Buf[idx]; }
    pub fn  Add(&mut self) -> *mut T {
    let mut idx = self.FreeIdx;
        if idx == self.Buf.len() as ImGuiPoolIdx {
            // self.Buf.resize(self.Buf.len() + 1);
            self.Buf.reserve(1);
            self.FreeIdx += 1;
        } else {
            // self.FreeIdx = &Buf[idx];
        }
        self.AliveCount += 1;
        return &mut self.Buf[idx];
}

    //     void        Remove(ImGuiID key, const T* p)     { Remove(key, GetIndex(p)); }
    pub fn Remove(&mut self, key: ImGuiID, p: *const T) {
        self.Remove2(key, self.GetIndex(p))
    }
    //     void        Remove(ImGuiID key, ImPoolIdx idx)  { Buf[idx].~T(); *(int*)&Buf[idx] = FreeIdx; FreeIdx = idx; Map.SetInt(key, -1); AliveCount--; }
    pub fn Remove2(&mut self, key: ImGuiID, idx: ImGuiPoolIdx) {
        self.Buf.remove(idx as usize);
        self.AliveCount -= 1;
        self.Map.SetInt(key, -1);
    }

    //     void        Reserve(int capacity)               { Buf.reserve(capacity); Map.data.reserve(capacity); }
    pub fn Reserve(&mut self, capacity: i32) {
        self.Buf.reserve(capacity as usize);
        self.Map.Data.reserve(capacity as usize)
    }
    //
    //     // To iterate a ImPool: for (int n = 0; n < pool.GetMapSize(); n++) if (T* t = pool.TryGetMapData(n)) { ... }
    //     // Can be avoided if you know .Remove() has never been called on the pool, or AliveCount == GetMapSize()
    //     int         GetAliveCount() const               { return AliveCount; }      // Number of active/alive items in the pool (for display purpose)
    //     int         GetBufSize() const                  { return Buf.size; }
    //     int         GetMapSize() const                  { return Map.data.size; }   // It is the map we need iterate to find valid items, since we don't have "alive" storage anywhere
    //     T*          TryGetMapData(ImPoolIdx n)          { int idx = Map.data[n].val_i; if (idx == -1) return NULL; return GetByIndex(idx); }
    pub fn TryGetMapData(&mut self, n: ImGuiPoolIdx) -> *mut T {
        let mut idx = self.Map.Data[n].val.val_i;
        if idx == -1 {
            return null_mut();
        }
        return self.GetByIndex(idx);
    }
}
