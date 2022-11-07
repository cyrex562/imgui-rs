#![allow(non_snake_case)]

use crate::core::type_defs::ImguiHandle;
use crate::core::vec2::ImVec2;
use crate::window::ImguiWindow;
use libc::c_int;

// Storage for current popup stack
#[derive(Default, Debug, Clone)]
pub struct ImGuiPopupData {
    pub PopupId: ImguiHandle,
    // Set on OpenPopup()
    pub Window: ImguiHandle,
    // Resolved on BeginPopup() - may stay unresolved if user never calls OpenPopup()
    pub BackupNavWindow: ImguiHandle,
    // Set on OpenPopup(), a NavWindow that will be restored on popup close
    pub ParentNavLayer: c_int,
    // Resolved on BeginPopup(). Actually a ImGuiNavLayer type (declared down below), initialized to -1 which is not part of an enum, but serves well-enough as "not any of layers" value
    pub OpenFrameCount: usize,
    // Set on OpenPopup()
    pub OpenParentId: ImguiHandle,
    // Set on OpenPopup(), we need this to differentiate multiple menu sets from each others (e.g. inside menu bar vs loose menu items)
    pub OpenPopupPos: ImVec2,
    // Set on OpenPopup(), preferred popup position (typically == OpenMousePos when using mouse)
    pub OpenMousePos: ImVec2, // Set on OpenPopup(), copy of mouse position at the time of opening popup
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
