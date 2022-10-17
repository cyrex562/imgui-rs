#![allow(non_snake_case)]

use std::mem;
use std::ptr::null_mut;
use libc::{c_int, size_t};
use crate::draw_channel::ImDrawChannel;
use crate::draw_cmd::ImDrawCmd;
use crate::draw_list::ImDrawList;
use crate::type_defs::ImDrawIdx;

// Split/Merge functions are used to split the draw list into different layers which can be drawn into out of order.
// This is used by the Columns/Tables API, so items of each column can be batched together in a same draw call.
#[derive(Default, Debug, Clone,Copy)]
pub struct ImDrawListSplitter {
    pub _Current: size_t,
    // Current channel number (0)
    pub _Count: size_t,
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
    pub unsafe fn ClearFreeMemory(&mut self) {
        // for (let i: c_int = 0; i < self._Channels.len(); i++)
        for i in 0 .. self._Channels.len()
        {
            if i == self._Current {
                libc::memset(&mut self._Channels[i], 0, mem::size_of::<ImDrawChannel>());
            }  // Current channel is a copy of CmdBuffer/IdxBuffer, don't destruct again
            self._Channels[i]._CmdBuffer.clear();
            self._Channels[i]._IdxBuffer.clear();
        }
        self.Current = 0;
        self._Count = 1;
        self._Channels.clear();

    }

    // c_void              Split(draw_list: *mut ImDrawList, count: c_int);
    pub unsafe fn Split(&mut self, draw_list: *mut ImDrawList, channels_count: size_t) {
        // IM_ASSERT(_Current == 0 && _Count <= 1 && "Nested channel splitting is not supported. Please use separate instances of ImDrawListSplitter.");
        let old_channels_count: size_t = self._Channels.len();
        if old_channels_count < channels_count {
            self._Channels.reserve(channels_count); // Avoid over reserving since this is likely to stay stable
            self._Channels.resize_with(channels_count, ImDrawChannel::default());
        }
        self._Count = channels_count;

        // Channels[] (24/32 bytes each) hold storage that we'll swap with draw_list._CmdBuffer/_IdxBuffer
        // The content of Channels[0] at this point doesn't matter. We clear it to make state tidy in a debugger but we don't strictly need to.
        // When we switch to the next channel, we'll copy draw_list._CmdBuffer/_IdxBuffer into Channels[0] and then Channels[1] into draw_list.CmdBuffer/_IdxBuffer
        libc::memset(&mut self._Channels[0], 0, mem::sizeof::<ImDrawChannel>());
        // for (let i: c_int = 1; i < channels_count; i++)
        for i in 1..channels_count {
            if i >= old_channels_count {
                // IM_PLACEMENT_NEW(&_Channels[i]) ImDrawChannel();
                self._Channels[i] = ImDrawChannel::default();
            } else {
                self._Channels[i]._CmdBuffer.clear();
                self._Channels[i]._IdxBuffer.clear();
            }
        }
    }


    // c_void              Merge(draw_list: *mut ImDrawList);
    pub unsafe fn Merge(&mut self, mut draw_list: *mut ImDrawList) {
        // Note that we never use or rely on _Channels.Size because it is merely a buffer that we never shrink back to 0 to keep all sub-buffers ready for use.
        if self._Count <= 1 {
            return;
        }

        self.SetCurrentChannel(&mut *draw_list, 0);
        draw_list._PopUnusedDrawCmd();

        // Calculate our final buffer sizes. Also fix the incorrect IdxOffset values in each command.
        let mut new_cmd_buffer_count: size_t = 0;
        let mut new_idx_buffer_count: size_t = 0;
        let mut last_cmd = if self._Count > 0 && draw_list.CmdBuffer.len() > 0 { draw_list.CmdBuffer.last_mut().unwrap()} else { null_mut()};
        let mut idx_offset: size_t = if last_cmd { last_cmd.IdxOffset + last_cmd.ElemCount } else { 0 };
        // for (let i: c_int = 1; i < _Count; i++)
        for i in 1 ..self._Count
        {
            let mut ch = &mut self._Channels[i];
            if ch._CmdBuffer.len() > 0 && ch._CmdBuffer.last().unwrap().ElemCount == 0 && ch._CmdBuffer.last().unwrap().UserCallback == null_mut() { // Equivalent of PopUnusedDrawCmd()
                ch._CmdBuffer.pop_back();
            }

            if ch._CmdBuffer.len() > 0 && last_cmd != null_mut()
            {
                // Do not include ImDrawCmd_AreSequentialIdxOffset() in the compare as we rebuild IdxOffset values ourselves.
                // Manipulating IdxOffset (e.g. by reordering draw commands like done by RenderDimmedBackgroundBehindWindow()) is not supported within a splitter.
                let mut next_cmd: *mut ImDrawCmd = &mut ch._CmdBuffer[0];
                if ImDrawCmd_HeaderCompare(last_cmd, next_cmd) == 0 && last_cmd.UserCallback == null_mut() && next_cmd.UserCallback == null_mut()
                {
                    // Merge previous channel last draw command with current channel first draw command if matching.
                    last_cmd.ElemCount += next_cmd.ElemCount;
                    idx_offset += next_cmd.ElemCount;
                    ch._CmdBuffer.erase(&ch._CmdBuffer); // FIXME-OPT: Improve for multiple merges.
                }
            }
            if ch._CmdBuffer.len() > 0 {
                last_cmd = ch._CmdBuffer.last_mut().unwrap();
            }
            new_cmd_buffer_count += ch._CmdBuffer.len();
            new_idx_buffer_count += ch._IdxBuffer.len();
            // for (let cmd_n: c_int = 0; cmd_n < ch._CmdBuffer.len(); cmd_n++)
            for cmd_n in 0 .. ch._CmdBuffer.len()
            {
                ch._CmdBuffer[cmd_n].IdxOffset = idx_offset;
                idx_offset += ch._CmdBuffer[cmd_n].ElemCount;
            }
        }
        draw_list.CmdBuffer.resize_with(draw_list.CmdBuffer.len() + new_cmd_buffer_count, ImDrawCmd::default());
        draw_list.IdxBuffer.resize_with(draw_list.IdxBuffer.len() + new_idx_buffer_count, ImDrawIdx::default());

        // Write commands and indices in order (they are fairly small structures, we don't copy vertices only indices)
        let mut cmd_write: *mut ImDrawCmd = draw_list.CmdBuffer.as_mut_ptr() + draw_list.CmdBuffer.len() - new_cmd_buffer_count;
        let mut idx_write: *mut ImDrawIdx = draw_list.IdxBuffer.as_mut_ptr() + draw_list.IdxBuffer.len() - new_idx_buffer_count;
        // for (let i: c_int = 1; i < _Count; i++)
        for i in 1 .. self._Count
        {
            let mut ch: &mutImDrawChannel = &mut self._Channels[i];
            let sz: size_t = ch._CmdBuffer.len();
            if sz > 0 {
                libc::memcpy(cmd_write, ch._CmdBuffer, sz * mem::size_of::<ImDrawCmd>() ); cmd_write += sz; }
            let sz: size_t = ch._IdxBuffer.len();
            if sz > 0 { libc::memcpy(idx_write, ch._IdxBuffer, sz * sizeof);
                idx_write += sz; }
        }
        draw_list._IdxWritePtr = idx_write;

        // Ensure there's always a non-callback draw command trailing the command-buffer
        if draw_list.CmdBuffer.len() == 0 || draw_list.CmdBuffer.last().unwrap().UserCallback != null_mut() {
            draw_list.AddDrawCmd();
        }

        // If current command is used with different settings we need to add a new command
        let mut curr_cmd: *mut ImDrawCmd = &mut draw_list.CmdBuffer[draw_list.CmdBuffer.len() - 1];
        if curr_cmd.ElemCount == 0 {
            ImDrawCmd_HeaderCopy(curr_cmd, &draw_list._CmdHeader);
        } // Copy ClipRect, TextureId, VtxOffset
        else if (ImDrawCmd_HeaderCompare(curr_cmd, &draw_list._CmdHeader) != 0) {
            draw_list.AddDrawCmd();
        }

        self._Count = 1;



    }


    // c_void              SetCurrentChannel(draw_list: *mut ImDrawList, channel_idx: c_int);
    pub unsafe fn SetCurrentChannel(&mut self, draw_list: &mut ImDrawList, channel_idx: c_int) {
        // IM_ASSERT(idx >= 0 && idx < _Count);
        if self._Current == idx {
            return;
        }

        // Overwrite ImVector (12/16 bytes), four times. This is merely a silly optimization instead of doing .swap()
        libc::memcpy(&mut self._Channels[self._Current]._CmdBuffer, &draw_list.CmdBuffer, sizeof(draw_list.CmdBuffer.as_ptr()));
        libc::memcpy(&mut self._Channels[self._Current]._IdxBuffer, &draw_list.IdxBuffer, sizeof(draw_list.IdxBuffer.as_ptr()));
        _Current = idx;
        libc::memcpy(&mut draw_list.CmdBuffer, &self._Channels[idx]._CmdBuffer, sizeof(draw_list.CmdBuffer.as_ptr()));
        libc::memcpy(&mut draw_list.IdxBuffer, &self._Channels[idx]._IdxBuffer, sizeof(draw_list.IdxBuffer.as_ptr()));
        draw_list._IdxWritePtr = draw_list.IdxBuffer.as_ptr() + draw_list.IdxBuffer.len();

        // If current command is used with different settings we need to add a new command
        let mut curr_cmd: *mut ImDrawCmd = if (draw_list.CmdBuffer.len() == 0) { null_mut() } else { &mut draw_list.CmdBuffer[draw_list.CmdBuffer.len() - 1] };
        if (curr_cmd == null_mut()) {
            draw_list.AddDrawCmd();
        }
        else if curr_cmd.ElemCount == 0 {
            ImDrawCmd_HeaderCopy(curr_cmd, &draw_list._CmdHeader);
        } // Copy ClipRect, TextureId, VtxOffset
        else if (ImDrawCmd_HeaderCompare(curr_cmd, &draw_list._CmdHeader) != 0) {
            draw_list.AddDrawCmd();
        }

    }
}
