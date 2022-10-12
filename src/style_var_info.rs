#![allow(non_snake_case)]

use libc::c_void;
use crate::data_type::ImGuiDataType;
use crate::style::ImGuiStyle;

#[derive(Default,Clone,Debug)]
pub struct ImGuiStyleVarInfo
{
pub Type:  ImGuiDataType,
pub Count:  u32,
pub Offset:  u32,
    
}

impl ImGuiStyleVarInfo {
    // *mut c_void           GetVarPtr(ImGuiStyle* style) const { return (style + Offset); }
    pub fn GetVarPtr(&mut self, style: *mut ImGuiStyle) -> *mut c_void {
        style + self.Offset.clone()
    }
    
    pub fn new(data_type: ImGuiDataType, count: u32, offset: u32) -> Self {
        Self {
            Type: data_type,
            Count: count,
            Offset: offset
        }
    }
}
