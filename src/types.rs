pub type DimgWindowHandle = u32;

pub const DIMG_WINDOW_HANDLE_INVALID: u32 = u32::MAX;
pub const DIMG_ID_INVALID: u32 = u32::MAX;


// Scalar data types
// typedef unsigned int        ImGuiID;// A unique id used by widgets (typically the result of hashing a stack of string)
pub type DimgId = u32;

// Character types
// (we generally use UTF-8 encoded string in the API. This is storage specifically for a decoded character used for keyboard input and display)
// typedef unsigned short ImWchar16;   // A single decoded U16 character/code point. We encode them as multi bytes UTF-8 when used in strings.
pub type ImWchar16 = u16;
// typedef unsigned pub ImWchar32: i32,   // A single decoded U32 character/code point. We encode them as multi bytes UTF-8 when used in strings.
pub type ImWchar32 = u32;
// #ifdef IMGUI_USE_WCHAR32            // ImWchar [configurable type: override in imconfig.h with '#define IMGUI_USE_WCHAR32' to support Unicode planes 1-16]
// typedef ImWchar32 ImWchar;
// #else
// typedef ImWchar16 ImWchar;
// #endif
pub type DimgWchar = ImWchar32;


#[derive(Debug,Default,Clone)]
pub struct DimgPtrOrIndex
{
    // void*       Ptr;            // Either field can be set, not both. e.g. Dock node tab bars are loose while BeginTabBar() ones are in a pool.
    Ptr: *mut c_void,
    // int         Index;          // Usually index in a main pool.
    Index: i32,

}

impl DimgPtrOrIndex {
    // ImGuiPtrOrIndex(void* ptr)  { Ptr = ptr; Index = -1; }
    pub fn new(ptr:*mut c_void) -> Self {
        Self {
            Ptr: ptr,
            Index: -1,
        }
    }
    //     ImGuiPtrOrIndex(int index)  { Ptr = NULL; Index = index; }
    pub fn new2(index: i32) -> Self {
        Self {
            Ptr: null_mut(),
            Index: index
        }
    }
}


// Extend ImGuiDataType_
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum ImGuiDataType
{
    String,
    Pointer,
    ID
}
