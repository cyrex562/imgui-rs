use crate::draw::command::DrawCommand;

// [Internal] For use by ImDrawListSplitter
#[derive(Debug,Clone,Default)]
pub struct DrawChannel
{
    // ImVector<ImDrawCmd>         _cmd_buffer;
    pub cmd_buffer: Vec<DrawCommand>,
    // ImVector<ImDrawIdx>         _idx_buffer;
    pub idx_buffer: Vec<u32>,
}
