use crate::draw_channel::DrawChannel;
use crate::draw::draw_list::DrawList;

// split/merge functions are used to split the draw list into different layers which can be drawn into out of order.
// This is used by the columns/tables API, so items of each column can be batched together in a same draw call.
#[derive(Debug,Clone,Default)]
pub struct DrawListSplitter
{
    pub current: i32,  // current channel number (0)
    pub count: i32,    // Number of active channels (1+)
    // ImVector<ImDrawChannel>     _channels;   // Draw channels (not resized down so _count might be < Channels.size)
    pub channels: Vec<DrawChannel>,
}

impl DrawListSplitter {
    // inline ImDrawListSplitter()  { memset(this, 0, sizeof(*this)); }
    //     inline ~ImDrawListSplitter() { clear_free_memory(); }
    //     inline void                 clear() { _current = 0; _count = 1; } // Do not clear Channels[] so our allocations are reused next frame
    pub fn clear(&mut self) {
        self.current = 0;
        self.count = 1;
    }
    //      void              clear_free_memory();
    pub fn clear_free_memory(&mut self) {
        todo!()
    }
    //      void              split(ImDrawList* draw_list, int count);
    pub fn split(&mut self, draw_list: &DrawList, count: i32) {
        todo!()
    }
    //      void              merge(ImDrawList* draw_list);
    pub fn merge(&mut self, draw_list: &DrawList) {
        todo!()
    }
    //      void              SetCurrentChannel(ImDrawList* draw_list, int channel_idx);
    pub fn set_current_channel(&mut self, draw_list: &DrawList, channel_idx: i32) {
        todo!()
    }
}
