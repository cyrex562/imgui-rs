// Helpers: Sorting
// #ifndef ImQsort
// static inline void      ImQsort(void* base, size_t count, size_t size_of_element, int(IMGUI_CDECL *compare_func)(void const*, void const*)) { if (count > 1) qsort(base, count, size_of_element, compare_func); }

use std::ffi::c_void;

// TODO

// pub unsafe fn partition(items: *mut c_void, item_size: usize, low: usize, high: usize) {
//     let mut pivot: *mut c_void = items.add(high * item_size);
//     let mut i = low - 1;
//     for j in low .. high - 1 {
//         if *(base.add(j * item_size) < *pivot
//     }
// }

// https://www.geeksforgeeks.org/quick-sort/

// TODO
pub unsafe fn ImQsort(base: *mut c_void, count: usize, size_of_element: usize, compare_fn: fn(*const c_void, *const c_void)->i32) {
    // TODO
    todo!()
}

// #endif
