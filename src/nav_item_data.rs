#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::c_float;
use crate::item_flags::ImGuiItemFlags;
use crate::rect::ImRect;
use crate::window::ImGuiWindow;
use crate::type_defs::ImGuiID;

pub struct ImGuiNavItemData {
    pub window: &mut ImGuiWindow,
    // Init,Move    // Best candidate window (result->Itemwindow.RootWindowForNav == request.Window)
    pub ID: ImGuiID,
    // Init,Move    // Best candidate item ID
    pub FocusScopeId: ImGuiID,
    // Init,Move    // Best candidate focus scope ID
    pub RectRel: ImRect,
    // Init,Move    // Best candidate bounding box in window relative space
    pub InFlags: ImGuiItemFlags,
    // ????,Move    // Best candidate item flags
    pub DistBox: c_float,
    //      Move    // Best candidate box distance to current NavId
    pub DistCenter: c_float,
    //      Move    // Best candidate center distance to current NavId
    pub DistAxial: c_float,      //      Move    // Best candidate axial distance to current NavId
}

impl ImGuiNavItemData {
    // ImGuiNavItemData()  { Clear(); }

    // void Clear()        { Window = None; ID = FocusScopeId = 0; InFlags = 0; DistBox = DistCenter = DistAxial = f32::MAX; }
    pub fn Clear(&mut self) {
        self.Window = None;
        self.ID = 0;
        self.FocusScopeId = 0;
        self.InFlags = 0;
        self.DistBox = f32::MAX;
        self.DistCenter = f32::MAX;
        self.DistAxial = f32::MAX;
    }
}
