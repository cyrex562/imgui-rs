use libc::c_int;

// typedef int ImGuiPopupFlags;        // -> enum ImGuiPopupFlags_      // Flags: for OpenPopup*(), BeginPopupContext*(), IsPopupOpen()
pub type ImGuiPopupFlags = c_int;

// Flags for OpenPopup*(), BeginPopupContext*(), IsPopupOpen() functions.
// - To be backward compatible with older API which took an 'int mouse_button = 1' argument, we need to treat
//   small flags values as a mouse button index, so we encode the mouse button in the first few bits of the flags.
//   It is therefore guaranteed to be legal to pass a mouse button index in ImGuiPopupFlags.
// - For the same reason, we exceptionally default the ImGuiPopupFlags argument of BeginPopupContextXXX functions to 1 instead of 0.
//   IMPORTANT: because the default parameter is 1 (==ImGuiPopupFlags_MouseButtonRight), if you rely on the default parameter
//   and want to another another flag, you need to pass in the ImGuiPopupFlags_MouseButtonRight flag.
// - Multiple buttons currently cannot be combined/or-ed in those functions (we could allow it later).
// enum ImGuiPopupFlags_
// {
pub const ImGuiPopupFlags_None: ImGuiPopupFlags = 0;
pub const ImGuiPopupFlags_MouseButtonLeft: ImGuiPopupFlags = 0;
// For BeginPopupContext*(): open on Left Mouse release. Guaranteed to always be == 0 (same as ImGuiMouseButton_Left)
pub const ImGuiPopupFlags_MouseButtonRight: ImGuiPopupFlags = 1;
// For BeginPopupContext*(): open on Right Mouse release. Guaranteed to always be == 1 (same as ImGuiMouseButton_Right)
pub const ImGuiPopupFlags_MouseButtonMiddle: ImGuiPopupFlags = 2;
// For BeginPopupContext*(): open on Middle Mouse release. Guaranteed to always be == 2 (same as ImGuiMouseButton_Middle)
pub const ImGuiPopupFlags_MouseButtonMask_: ImGuiPopupFlags = 0x1F;
pub const ImGuiPopupFlags_MouseButtonDefault_: ImGuiPopupFlags = 1;
pub const ImGuiPopupFlags_NoOpenOverExistingPopup: ImGuiPopupFlags = 1 << 5;
// For OpenPopup*(); BeginPopupContext*(): don't open if there's already a popup at the same level of the popup stack
pub const ImGuiPopupFlags_NoOpenOverItems: ImGuiPopupFlags = 1 << 6;
// For BeginPopupContextWindow(): don't return true when hovering items; only when hovering empty space
pub const ImGuiPopupFlags_AnyPopupId: ImGuiPopupFlags = 1 << 7;
// For IsPopupOpen(): ignore the ImguiHandle parameter and test for any popup.
pub const ImGuiPopupFlags_AnyPopupLevel: ImGuiPopupFlags = 1 << 8;
// For IsPopupOpen(): search/test at any level of the popup stack (default test in the current level)
pub const ImGuiPopupFlags_AnyPopup: ImGuiPopupFlags =
    ImGuiPopupFlags_AnyPopupId | ImGuiPopupFlags_AnyPopupLevel;
// };
