#![allow(non_snake_case)]

use libc::c_int;
use crate::imgui_vec2::ImVec2;
use crate::imgui_window::ImGuiWindow;
use crate::type_defs::ImGuiID;

// Storage for current popup stack
#[derive(Default, Debug, Clone)]
pub struct ImGuiPopupData {
    pub PopupId: ImGuiID,
    // Set on OpenPopup()
    pub Window: *mut ImGuiWindow,
    // Resolved on BeginPopup() - may stay unresolved if user never calls OpenPopup()
    pub BackupNavWindow: *mut ImGuiWindow,
    // Set on OpenPopup(), a NavWindow that will be restored on popup close
    pub ParentNavLayer: c_int,
    // Resolved on BeginPopup(). Actually a ImGuiNavLayer type (declared down below), initialized to -1 which is not part of an enum, but serves well-enough as "not any of layers" value
    pub OpenFrameCount: c_int,
    // Set on OpenPopup()
    pub OpenParentId: ImGuiID,
    // Set on OpenPopup(), we need this to differentiate multiple menu sets from each others (e.g. inside menu bar vs loose menu items)
    pub OpenPopupPos: ImVec2,
    // Set on OpenPopup(), preferred popup position (typically == OpenMousePos when using mouse)
    pub OpenMousePos: ImVec2,   // Set on OpenPopup(), copy of mouse position at the time of opening popup
}

impl ImGuiPopupData {
    // ImGuiPopupData()    { memset(this, 0, sizeof(*this)); ParentNavLayer = OpenFrameCount = -1; }
    pub fn new() -> Self {
        Self {
            ParentNavLayer: -1,
            OpenFrameCount: -1,
            ..Default::default()
        }
    }
}
