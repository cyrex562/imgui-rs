use crate::draw::channel::DrawChannel;
use crate::draw::command::DrawCommand;
use crate::draw_channel::DrawChannel;
use crate::draw::list::DrawList;

// split/merge functions are used to split the draw list into different layers which can be drawn into out of order.
// This is used by the columns/tables API, so items of each column can be batched together in a same draw call.
#[derive(Debug, Clone, Default)]
pub struct DrawListSplitter {
    pub current: usize,
    // current channel number (0)
    pub count: usize,
    // Number of active channels (1+)
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
        // for (int i = 0; i < channels.size; i += 1)
        for i in 0..self.channels.len() {
            if i == self.current {
                // memset(&channels[i], 0, sizeof(channels[i]));
                self.channels[i] = DrawChannel::default();
            } // current channel is a copy of cmd_buffer/idx_buffer, don't destruct again
            self.channels[i].cmd_buffer.clear();
            self.channels[i].idx_buffer.clear();
        }
        self.current = 0;
        self.count = 1;
        self.channels.clear();
    }
    //      void              split(ImDrawList* draw_list, int count);
    pub fn split(&mut self, draw_list: &DrawList, channels_count: usize) {
        // IM_UNUSED(draw_list);
        // IM_ASSERT(_Current == 0 && _Count <= 1 && "Nested channel splitting is not supported. Please use separate instances of ImDrawListSplitter.");
        let old_channels_count = self.channels.len();
        if old_channels_count < channels_count {
            channels.reserve(channels_count); // Avoid over reserving since this is likely to stay stable
            channels.resize(channels_count);
        }
        self.count = channels_count;

        // Channels[] (24/32 bytes each) hold storage that we'll swap with draw_list->_cmd_buffer/_idx_buffer
        // The content of Channels[0] at this point doesn't matter. We clear it to make state tidy in a debugger but we don't strictly need to.
        // When we switch to the next channel, we'll copy draw_list->_cmd_buffer/_idx_buffer into Channels[0] and then Channels[1] into draw_list->cmd_buffer/_idx_buffer
        // memset(&channels[0], 0, sizeof(ImDrawChannel));
        self.channels[0] = DrawChannel::default();
        // for (int i = 1; i < channels_count; i += 1)
        for i in 1..channels_count {
            if i >= old_channels_count {
                // IM_PLACEMENT_NEW(&channels[i]) ImDrawChannel();
                self.channels[i] = DrawChannel::new();
            } else {
                self.channels[i].cmd_buffer.resize(0, DrawCommand::default());
                self.channels[i].idx_buffer.resize(0, 0u32);
            }
        }
    }
    //      void              merge(ImDrawList* draw_list);
    pub fn merge(&mut self, draw_list: &DrawList) {
        // // Note that we never use or rely on _channels.size because it is merely a buffer that we never shrink back to 0 to keep all sub-buffers ready for use.
        //     if (_Count <= 1)
        //         return;
        //
        //     SetCurrentChannel(draw_list, 0);
        //     draw_list->_PopUnusedDrawCmd();
        //
        //     // Calculate our final buffer sizes. Also fix the incorrect idx_offset values in each command.
        //     int new_cmd_buffer_count = 0;
        //     int new_idx_buffer_count = 0;
        //     ImDrawCmd* last_cmd = (_Count > 0 && draw_list.cmd_buffer.size > 0) ? &draw_list.cmd_buffer.back() : None;
        //     int idx_offset = last_cmd ? last_cmd.IdxOffset + last_cmd.ElemCount : 0;
        //     for (int i = 1; i < _Count; i += 1)
        //     {
        //         ImDrawChannel& ch = channels[i];
        //         if (ch._CmdBuffer.size > 0 && ch._CmdBuffer.back().elem_count == 0 && ch._CmdBuffer.back().user_callback == None) // Equivalent of PopUnusedDrawCmd()
        //             ch._CmdBuffer.pop_back();
        //
        //         if (ch._CmdBuffer.size > 0 && last_cmd != None)
        //         {
        //             // Do not include ImDrawCmd_AreSequentialIdxOffset() in the compare as we rebuild idx_offset values ourselves.
        //             // Manipulating idx_offset (e.g. by reordering draw commands like done by RenderDimmedBackgroundBehindWindow()) is not supported within a splitter.
        //             ImDrawCmd* next_cmd = &ch._CmdBuffer[0];
        //             if (ImDrawCmd_HeaderCompare(last_cmd, next_cmd) == 0 && last_cmd.UserCallback == None && next_cmd.UserCallback == None)
        //             {
        //                 // merge previous channel last draw command with current channel first draw command if matching.
        //                 last_cmd.ElemCount += next_cmd.ElemCount;
        //                 idx_offset += next_cmd.ElemCount;
        //                 ch._CmdBuffer.erase(ch._CmdBuffer.data); // FIXME-OPT: Improve for multiple merges.
        //             }
        //         }
        //         if (ch._CmdBuffer.size > 0)
        //             last_cmd = &ch._CmdBuffer.back();
        //         new_cmd_buffer_count += ch._CmdBuffer.size;
        //         new_idx_buffer_count += ch._IdxBuffer.size;
        //         for (int cmd_n = 0; cmd_n < ch._CmdBuffer.size; cmd_n += 1)
        //         {
        //             ch._CmdBuffer.data[cmd_n].IdxOffset = idx_offset;
        //             idx_offset += ch._CmdBuffer.data[cmd_n].elem_count;
        //         }
        //     }
        //     draw_list.cmd_buffer.resize(draw_list.cmd_buffer.size + new_cmd_buffer_count);
        //     draw_list.IdxBuffer.resize(draw_list.IdxBuffer.size + new_idx_buffer_count);
        //
        //     // Write commands and indices in order (they are fairly small structures, we don't copy vertices only indices)
        //     ImDrawCmd* cmd_write = draw_list.cmd_buffer.data + draw_list.cmd_buffer.size - new_cmd_buffer_count;
        //     ImDrawIdx* idx_write = draw_list.IdxBuffer.data + draw_list.IdxBuffer.size - new_idx_buffer_count;
        //     for (int i = 1; i < _Count; i += 1)
        //     {
        //         ImDrawChannel& ch = channels[i];
        //         if (int sz = ch._CmdBuffer.size) { memcpy(cmd_write, ch._CmdBuffer.data, sz * sizeof(ImDrawCmd)); cmd_write += sz; }
        //         if (int sz = ch._IdxBuffer.size) { memcpy(idx_write, ch._IdxBuffer.data, sz * sizeof(ImDrawIdx)); idx_write += sz; }
        //     }
        //     draw_list->_IdxWritePtr = idx_write;
        //
        //     // Ensure there's always a non-callback draw command trailing the command-buffer
        //     if (draw_list.cmd_buffer.size == 0 || draw_list.cmd_buffer.back().user_callback != None)
        //         draw_list.add_draw_cmd();
        //
        //     // If current command is used with different settings we need to add a new command
        //     ImDrawCmd* curr_cmd = &draw_list.cmd_buffer.data[draw_list.cmd_buffer.size - 1];
        //     if (curr_cmd.ElemCount == 0)
        //         ImDrawCmd_HeaderCopy(curr_cmd, &draw_list->_CmdHeader); // Copy clip_rect, texture_id, vtx_offset
        //     else if (ImDrawCmd_HeaderCompare(curr_cmd, &draw_list->_CmdHeader) != 0)
        //         draw_list.add_draw_cmd();
        //
        //     _Count = 1;
    }
    //      void              SetCurrentChannel(ImDrawList* draw_list, int channel_idx);
    pub fn set_current_channel(&mut self, draw_list: &DrawList, channel_idx: i32) {
        //
        // IM_ASSERT(idx >= 0 && idx < _Count);
        if self.current == idx {
            return;
        }

        // TODO:
        // Overwrite ImVector (12/16 bytes), four times. This is merely a silly optimization instead of doing .swap()
        // memcpy(&channels.data[_Current]._cmd_buffer, &draw_list.cmd_buffer, sizeof(draw_list.cmd_buffer));
        // memcpy(&channels.data[_Current]._idx_buffer, &draw_list.idx_buffer, sizeof(draw_list.idx_buffer));
        // _Current = idx;
        // memcpy(&draw_list.cmd_buffer, &channels.data[idx]._cmd_buffer, sizeof(draw_list.cmd_buffer));
        // memcpy(&draw_list.idx_buffer, &channels.data[idx]._idx_buffer, sizeof(draw_list.idx_buffer));
        // draw_list->idx_write_ptr = draw_list.idx_buffer.data + draw_list.idx_buffer.size;
        //
        // // If current command is used with different settings we need to add a new command
        // ImDrawCmd* curr_cmd = (draw_list.cmd_buffer.size == 0) ? None : &draw_list.cmd_buffer.data[draw_list.cmd_buffer.size - 1];
        // if (curr_cmd == None)
        //     draw_list.add_draw_cmd();
        // else if (curr_cmd.elem_count == 0)
        //     ImDrawCmd_HeaderCopy(curr_cmd, &draw_list->command_header); // Copy clip_rect, texture_id, vtx_offset
        // else if (ImDrawCmd_HeaderCompare(curr_cmd, &draw_list->command_header) != 0)
        //     draw_list.add_draw_cmd();
        // }
    }
}
