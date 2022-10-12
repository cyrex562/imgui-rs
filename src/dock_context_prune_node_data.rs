#![allow(non_snake_case)]

use libc::c_int;
use crate::type_defs::ImGuiID;

// Pre C++0x doesn't allow us to use a function-local type (without linkage) as template parameter, so we moved this here.
#[derive(Default, Debug, Copy, Clone)]
pub struct ImGuiDockContextPruneNodeData {
    // c_int         CountWindows, CountChildWindows, CountChildNodes;
    pub CountWindows: c_int,
    pub CountChildWindows: c_int,
    pub CountChildNodes: c_int,
    // ImGuiID     RootId;
    pub RootId: ImGuiID,

}

impl ImGuiDockContextPruneNodeData {
    // ImGuiDockContextPruneNodeData()
    // pub fn new() -> Self
    // {
    // // CountWindows = CountChildWindows = CountChildNodes = 0; RootId = 0;
    //     Self {
    //         Coun
    //     }
    // }
}
