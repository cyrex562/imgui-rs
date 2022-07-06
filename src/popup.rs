use crate::imgui_h::ImGuiID;
use crate::imgui_vec::ImVec2;
use crate::imgui_window::ImGuiWindow;

// Storage for current popup stack
#[derive(Debug,Default,Clone)]
pub struct ImGuiPopupData
{
    // ImGuiID             PopupId;        // Set on OpenPopup()
    pub PopupId: ImGuiID,
    // ImGuiWindow*        Window;         // Resolved on BeginPopup() - may stay unresolved if user never calls OpenPopup()
    pub Window: *mut ImGuiWindow,
    // ImGuiWindow*        SourceWindow;   // Set on OpenPopup() copy of NavWindow at the time of opening the popup
    pub SourceWindow: *mut ImGuiWindow,
    // int                 ParentNavLayer; // Resolved on BeginPopup(). Actually a ImGuiNavLayer type (declared down below), initialized to -1 which is not part of an enum, but serves well-enough as "not any of layers" value
    pub ParentNavLayer: i32,
    // int                 OpenFrameCount; // Set on OpenPopup()
    pub OpenFrameCount: i32,
    // ImGuiID             OpenParentId;   // Set on OpenPopup(), we need this to differentiate multiple menu sets from each others (e.g. inside menu bar vs loose menu items)
    pub OpenParentId: ImGuiID,
    // ImVec2              OpenPopupPos;   // Set on OpenPopup(), preferred popup position (typically == OpenMousePos when using mouse)
    pub OpenPopupPos: ImVec2,
    // ImVec2              OpenMousePos;   // Set on OpenPopup(), copy of mouse position at the time of opening popup
    pub OpenMousePos: ImVec2,
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
