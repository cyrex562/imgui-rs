use crate::draw::command::DrawCommand;

// [Internal] For use by ImDrawListSplitter
#[derive(Debug,Clone,Default)]
pub struct DrawChannel
{
    pub cmd_buffer: Vec<DrawCommand>,
    pub idx_buffer: Vec<u32>,
}
