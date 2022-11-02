#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::{c_char, c_int, c_void};
use crate::type_defs::ImGuiID;

// Data payload for Drag and Drop operations: AcceptDragDropPayload(), GetDragDropPayload()
#[derive(Default, Debug, Clone)]
pub struct ImGuiPayload {
    // Members
    pub Data: Vec<u8>,
    // Data (copied and owned by dear imgui)
    pub DataSize: usize,           // Data size

    // [Internal]
    pub SourceId: ImGuiID,
    // Source item id
    pub SourceParentId: ImGuiID,
    // Source parent id (if available)
    pub DataFrameCount: c_int,
    // Data timestamp
    pub DataType: String,
    // Data type tag (short user-supplied string, 32 characters max)
    pub Preview: bool,
    // Set when AcceptDragDropPayload() was called and mouse has been hovering the target item (nb: handle overlapping drag targets)
    pub Delivery: bool,           // Set when AcceptDragDropPayload() was called and mouse button is released over the target item.
}

impl ImGuiPayload {
    // ImGuiPayload()  { Clear(); }


    // void Clear()    { SourceId = SourceParentId = 0; Data = None; DataSize = 0; memset(DataType, 0, sizeof(DataType)); DataFrameCount = - 1; Preview = Delivery = false; }
    pub fn Clear(&mut self) {
        self.SourceId = 0;
        self.SourceParentId = 0;
        self.Data = vec![];
        self.DataSize = 0;
        self.DataType = String::with_capacity(33);
        self.Preview = false;
        self.Delivery = false;
    }

    // IsDataType: bool( * const char type ) const { return DataFrameCount != - 1 & & strcmp( type, DataType) == 0; }
    pub unsafe fn IsDataType(&mut self, data_type: &str) -> bool {
        self.DataFrameCount != -1 && data_type == self.DataType
    }


    // IsPreview: bool() const { return Preview; }
    pub fn IsPreview(&mut self) -> bool {
        self.Preview
    }


    // IsDelivery: bool() const { return Delivery; }
    pub fn IsDelivery(&mut self) -> bool {
        self.Delivery
    }
}
