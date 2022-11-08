use libc::{c_char, size_t};
use std::mem;

// Type information associated to one ImGuiDataType. Retrieve with DataTypeGetInfo().
#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiDataTypeInfo {
    // size_t      Size;           // Size in bytes
    pub Size: usize,
    // let Name: *const c_char;           // Short descriptive name for the type, for debugging
    pub Name: String,
    // let PrintFmt: *const c_char;       // Default printf format for the type
    pub PrintFmt: String,
    // let ScanFmt: *const c_char;        // Default scanf format for the type
    pub ScanFmt: String,
}

pub const GDATA_TYPE_INFO: [ImGuiDataTypeInfo; 10] = [
    ImGuiDataTypeInfo {
        Size: mem::size_of::<i8>(),
        Name: String::from("i8"),
        PrintFmt: String::from("{}"),
        ScanFmt: String::from("{}"),
    }, // IM_GUI_DATA_TYPE_S8
    ImGuiDataTypeInfo {
        Size: mem::size_of::<u8>(),
        Name: String::from("u8"),
        PrintFmt: String::from("%u"),
        ScanFmt: String::from("%u"),
    },
    // ImGuiDataTypeInfo{ sizeof,            "S16",  "{}",   "{}"    },  // IM_GUI_DATA_TYPE_S16
    ImGuiDataTypeInfo {
        Size: mem::size_of::<i16>(),
        Name: String::from("i16"),
        PrintFmt: String::from("{}"),
        ScanFmt: String::from("{}"),
    },
    // ImGuiDataTypeInfo{ sizeof,   "U16",  "%u",   "%u"    },
    ImGuiDataTypeInfo {
        Size: mem::size_of::<u16>(),
        Name: String::from("u16"),
        PrintFmt: String::from("%u"),
        ScanFmt: String::from("%u"),
    },
    // ImGuiDataTypeInfo{ sizeof,              "S32",  "{}",   "{}"    },  // IM_GUI_DATA_TYPE_S32
    ImGuiDataTypeInfo {
        Size: mem::size_of::<i32>(),
        Name: String::from("i32"),
        PrintFmt: String::from("{}"),
        ScanFmt: String::from("{}"),
    },
    // ImGuiDataTypeInfo{ sizeof,     "U32",  "%u",   "%u"    },
    ImGuiDataTypeInfo {
        Size: mem::size_of::<u32>(),
        Name: String::from("u32"),
        PrintFmt: String::from("%u"),
        ScanFmt: String::from("%u"),
    },
    ImGuiDataTypeInfo {
        Size: mem::size_of::<i64>(),
        Name: String::from("i64"),
        PrintFmt: String::from("%lld"),
        ScanFmt: String::from("%lld"),
    },
    ImGuiDataTypeInfo {
        Size: mem::size_of::<u64>(),
        Name: String::from("u8"),
        PrintFmt: String::from("%llu"),
        ScanFmt: String::from("%llu"),
    },
    ImGuiDataTypeInfo {
        Size: mem::size_of::<f32>(),
        Name: String::from("f32"),
        PrintFmt: String::from("{}"),
        ScanFmt: String::from("{}"),
    },
    ImGuiDataTypeInfo {
        Size: mem::size_of::<f64>(),
        Name: String::from("f64"),
        PrintFmt: String::from("{}"),
        ScanFmt: String::from("%lf"),
    },
];
