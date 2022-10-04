#![allow(non_snake_case)]

use libc::c_int;
use crate::draw_channel::ImDrawChannel;
use crate::draw_list::ImDrawList;

// Split/Merge functions are used to split the draw list into different layers which can be drawn into out of order.
// This is used by the Columns/Tables API, so items of each column can be batched together in a same draw call.
#[derive(Default, Debug, Clone)]
pub struct ImDrawListSplitter {
    pub _Current: c_int,
    // Current channel number (0)
    pub _Count: c_int,
    // Number of active channels (1+)
    pub _Channels: Vec<ImDrawChannel>,   // Draw channels (not resized down so _Count might be < Channels.Size)
}


impl ImDrawListSplitter {
    // inline ImDrawListSplitter()  { memset(this, 0, sizeof(*this)); }


    // inline ~ImDrawListSplitter() { ClearFreeMemory(); }


    // inline c_void                 Clear() { _Current = 0; _Count = 1; } // Do not clear Channels[] so our allocations are reused next frame
    pub fn Clear(&mut self) {
        self._Current = 0;
        self._Count = 0;
    }


    // c_void              ClearFreeMemory();
    pub fn ClearFreeMemory(&mut self) {
        todo!()
    }

    // c_void              Split(ImDrawList* draw_list, count: c_int);
    pub fn Split(&mut self, draw_list: *mut ImDrawList, count: c_int) {
        todo!()
    }


    // c_void              Merge(ImDrawList* draw_list);
    pub fn Merge(&mut self, draw_list: *mut ImDrawList) {
        todo!()
    }


    // c_void              SetCurrentChannel(ImDrawList* draw_list, channel_idx: c_int);
    pub fn SetCurrentChannel(&mut self, draw_list: *mut ImDrawList, channel_idx: c_int) {
        todo!()
    }
}
