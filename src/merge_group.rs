use libc::c_int;
use crate::bit_array::ImBitArray;
use crate::rect::ImRect;

#[derive(Default, Debug, Copy, Clone)]
pub struct MergeGroup {
    // ImRect  ClipRect;
    pub ClipRect: ImRect,
    // c_int     ChannelsCount;
    pub ChannelsCount: c_int,
    // ImBitArray<IMGUI_TABLE_MAX_DRAW_CHANNELS> ChannelsMask;
    pub ChannelsMask: ImBitArray,
    // MergeGroup() { ChannelsCount = 0; }
}
