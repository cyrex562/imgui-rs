pub type WindowHandle = u32;

pub const INVALID_ID: u32 = u32::MAX;

// Scalar data types
// typedef unsigned int        Id32;// A unique id used by widgets (typically the result of hashing a stack of string)
pub type Id32 = u32;

// Character types
// (we generally use UTF-8 encoded string in the API. This is storage specifically for a decoded character used for keyboard input and display)
// typedef unsigned short ImWchar16;   // A single decoded U16 character/code point. We encode them as multi bytes UTF-8 when used in strings.
// pub type ImWchar16 = u16;
// typedef unsigned pub ImWchar32: i32,   // A single decoded U32 character/code point. We encode them as multi bytes UTF-8 when used in strings.
// pub type ImWchar32 = u32;
// #ifdef IMGUI_USE_WCHAR32            // ImWchar [configurable type: override in imconfig.h with '#define IMGUI_USE_WCHAR32' to support Unicode planes 1-16]
// typedef ImWchar32 ImWchar;
// #else
// typedef ImWchar16 ImWchar;
// #endif
// pub type DimgWchar = ImWchar32;

// #[derive(Debug, Default, Clone)]
// pub struct PtrOrIndex {
//     // void*       Ptr;            // Either field can be set, not both. e.g. Dock node tab bars are loose while BeginTabBar() ones are in a pool.
//     Ptr: *mut c_void,
//     // int         index;          // Usually index in a main pool.
//     Index: i32,
// }
//
// impl PtrOrIndex {
//     // ImGuiPtrOrIndex(void* ptr)  { Ptr = ptr; index = -1; }
//     pub fn new(ptr: *mut c_void) -> Self {
//         Self {
//             Ptr: ptr,
//             Index: -1,
//         }
//     }
//     //     ImGuiPtrOrIndex(int index)  { Ptr = None; index = index; }
//     pub fn new2(index: i32) -> Self {
//         Self {
//             Ptr: null_mut(),
//             Index: index,
//         }
//     }
// }

// Extend DataType::
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DataType {
    String,
    Pointer,
    ID,
}

//#define ImDrawIdx unsigned int
pub type DrawIndex = usize;

/// Store the source authority (dock node vs window) of a field
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DataAuthority {
    None,
    Auto,
    DockNode,
    Window,
}

impl Default for DataAuthority {
    fn default() -> Self {
        Self::None
    }
}

// #[derive(Default, Debug, Clone)]
// pub struct DataTypeTempStorage {
//     // ImU8        Data[8];        // Can fit any data up to DataType::COUNT
//     pub data: [u8; 8],
// }

/// Type information associated to one DataType. Retrieve with DataTypeGetInfo().
// #[derive(Default, Debug, Clone)]
// pub struct DataTypeInfo {
//     // size_t      size;           // size in bytes
//     pub size: usize,
//     // const char* name;           // Short descriptive name for the type, for debugging
//     pub name: String,
//     // const char* PrintFmt;       // Default printf format for the type
//     pub print_fmt: String,
//     // const char* ScanFmt;        // Default scanf format for the type
//     pub scan_fmt: String,
// }

/// A cardinal direction
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    None,
    Left,
    Right,
    Up,
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        Self::None
    }
}

/// A sorting direction
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SortDirection {
    None,
    Ascending,
    // Ascending = 0->9, A->Z etc.
    Descending, // Descending = 9->0, Z->A etc.
}

impl Default for SortDirection {
    fn default() -> Self {
        Self::None
    }
}

pub const DIRECTIONS: [Direction; 5] = [
    Direction::None,
    Direction::Left,
    Direction::Right,
    Direction::Up,
    Direction::Down,
];
