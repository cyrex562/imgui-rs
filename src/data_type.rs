pub struct DimgDataTypeTempStorage
{
    // ImU8        Data[8];        // Can fit any data up to ImGuiDataType_COUNT
    pub data: [u8;8],
}

// Type information associated to one ImGuiDataType. Retrieve with DataTypeGetInfo().
pub struct DimgDataTypeInfo
{
    // size_t      size;           // size in bytes
    pub size: usize,
    // const char* name;           // Short descriptive name for the type, for debugging
    pub name: String,
    // const char* PrintFmt;       // Default printf format for the type
    pub print_fmt: String,
    // const char* ScanFmt;        // Default scanf format for the type
    pub scan_fmt: String,
}
