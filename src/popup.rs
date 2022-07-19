use std::collections::HashSet;
use crate::imgui_h::ImGuiID;
use crate::imgui_vec::Vector2D;
use crate::imgui_window::ImGuiWindow;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;

// Storage for current popup stack
#[derive(Debug,Default,Clone)]
pub struct PopupData
{
    // ImGuiID             popup_id;        // Set on OpenPopup()
    pub popup_id: Id32,
    // ImGuiWindow*        window;         // Resolved on BeginPopup() - may stay unresolved if user never calls OpenPopup()
    pub window_id: Id32,
    // ImGuiWindow*        source_window;   // Set on OpenPopup() copy of nav_window at the time of opening the popup
    pub source_window_id:Id32,
    // int                 parent_nav_layer; // Resolved on BeginPopup(). Actually a ImGuiNavLayer type (declared down below), initialized to -1 which is not part of an enum, but serves well-enough as "not any of layers" value
    pub parent_nav_layer: i32,
    // int                 open_frame_count; // Set on OpenPopup()
    pub open_frame_count: i32,
    // ImGuiID             open_parent_id;   // Set on OpenPopup(), we need this to differentiate multiple menu sets from each others (e.g. inside menu bar vs loose menu items)
    pub open_parent_id: Id32,
    // Vector2D              open_popup_pos;   // Set on OpenPopup(), preferred popup position (typically == open_mouse_pos when using mouse)
    pub open_popup_pos: Vector2D,
    // Vector2D              open_mouse_pos;   // Set on OpenPopup(), copy of mouse position at the time of opening popup
    pub open_mouse_pos: Vector2D,
}

impl PopupData {
    // ImGuiPopupData()    { memset(this, 0, sizeof(*this)); parent_nav_layer = open_frame_count = -1; }
    pub fn new() -> Self {
        Self {
            parent_nav_layer: -1,
            open_frame_count: -1,
            ..Default::default()
        }
    }
}

pub enum PopupPositionPolicy
{
    Default,
    ComboBox,
    Tooltip
}

// pub const AnyPopup: i32                = DimgPopupFlags::AnyPopupId | DimgPopupFlags::AnyPopupLevel;
pub const POPUP_FLAGS_ANY_POPUP: HashSet<PopupFlags> = HashSet::from([
    PopupFlags::AnyPopupId, PopupFlags::AnyPopupLevel
]);

// flags for OpenPopup*(), BeginPopupContext*(), IsPopupOpen() functions.
// - To be backward compatible with older API which took an 'int mouse_button = 1' argument, we need to treat
//   small flags values as a mouse button index, so we encode the mouse button in the first few bits of the flags.
//   It is therefore guaranteed to be legal to pass a mouse button index in ImGuiPopupFlags.
// - For the same reason, we exceptionally default the ImGuiPopupFlags argument of BeginPopupContextXXX functions to 1 instead of 0.
//   IMPORTANT: because the default parameter is 1 (==ImGuiPopupFlags_MouseButtonRight), if you rely on the default parameter
//   and want to another another flag, you need to pass in the ImGuiPopupFlags_MouseButtonRight flag.
// - Multiple buttons currently cannot be combined/or-ed in those functions (we could allow it later).
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum PopupFlags
{
    None                    = 0,
    // ImGuiPopupFlags_MouseButtonLeft         = 0,        // For BeginPopupContext*(): open on Left Mouse release. Guaranteed to always be == 0 (same as ImGuiMouseButton_Left)
    MouseButtonRight        = 1,        // For BeginPopupContext*(): open on Right Mouse release. Guaranteed to always be == 1 (same as ImGuiMouseButton_Right)
    MouseButtonMiddle       = 2,        // For BeginPopupContext*(): open on Middle Mouse release. Guaranteed to always be == 2 (same as ImGuiMouseButton_Middle)
    MouseButtonMask_        = 0x1F,
    // ImGuiPopupFlags_MouseButtonDefault_     = 1,
    NoOpenOverExistingPopup = 1 << 5,   // For OpenPopup*(), BeginPopupContext*(): don't open if there's already a popup at the same level of the popup stack
    NoOpenOverItems         = 1 << 6,   // For BeginPopupContextWindow(): don't return true when hovering items, only when hovering empty space
    AnyPopupId              = 1 << 7,   // For IsPopupOpen(): ignore the ImGuiID parameter and test for any popup.
    AnyPopupLevel           = 1 << 8,   // For IsPopupOpen(): search/test at any level of the popup stack (default test in the current level)

}
