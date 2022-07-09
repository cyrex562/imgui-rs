use crate::draw_cmd::DimgDrawCmd;

// [Internal] For use by ImDrawListSplitter
#[derive(Debug,Clone,Default)]
pub struct DimgDrawChannel
{
    // ImVector<ImDrawCmd>         _cmd_buffer;
    pub cmd_buffer: Vec<DimgDrawCmd>,
    // ImVector<ImDrawIdx>         _idx_buffer;
    pub idx_buffer: Vec<u32>,
}
