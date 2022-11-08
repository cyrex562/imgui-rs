
#[derive(Default,Debug,Clone,Copy)]
pub struct ImGuiDataTypeTempStorage
{
    // u8        Data[8];        // Can fit any data up to IM_GUI_DATA_TYPE_COUNT
    pub Data: [u8;8]
}
