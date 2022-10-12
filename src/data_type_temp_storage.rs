#[derive(Default,Debug,Clone,Copy)]
pub struct ImGuiDataTypeTempStorage
{
    // u8        Data[8];        // Can fit any data up to ImGuiDataType_COUNT
    pub Data: [u8;8],
}