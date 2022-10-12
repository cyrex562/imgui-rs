use libc::{c_char, size_t};

// Type information associated to one ImGuiDataType. Retrieve with DataTypeGetInfo().
#[derive(Default,Debug,Clone,Copy)]
pub struct ImGuiDataTypeInfo
{
    // size_t      Size;           // Size in bytes
    pub Size: size_t,
    // let Name: *const c_char;           // Short descriptive name for the type, for debugging
    pub Name: *const c_char,
    // let PrintFmt: *const c_char;       // Default printf format for the type
    pub PrintFmt: *const c_char,
    // let ScanFmt: *const c_char;        // Default scanf format for the type
    pub ScanFmt: *const c_char,
}
