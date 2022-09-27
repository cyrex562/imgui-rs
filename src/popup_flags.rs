use libc::c_int;

// typedef int ImGuiPopupFlags;        // -> enum ImGuiPopupFlags_      // Flags: for OpenPopup*(), BeginPopupContext*(), IsPopupOpen()
pub type ImGuiPopupFlags = c_int;
