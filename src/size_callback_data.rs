use crate::vec2::ImVec2;
use libc::c_void;

// Resizing callback data to apply custom constraint. As enabled by SetNextWindowSizeConstraints(). Callback is called during the next Begin().
// NB: For basic min/max size constraint on each axis you don't need to use the callback! The SetNextWindowSizeConstraints() parameters are enough.
#[derive(Default, Debug, Copy, Clone)]
pub struct ImGuiSizeCallbackData {
    // *mut c_void   UserData;       // Read-only.   What user passed to SetNextWindowSizeConstraints(). Generally store an integer or float in here (need reinterpret_cast<>).
    pub UserData: Vec<u8>,
    // ImVec2  Pos;            // Read-only.   Window position, for reference.
    pub Pos: ImVec2,
    // ImVec2  CurrentSize;    // Read-only.   Current window size.
    pub CurrentSize: ImVec2,
    // ImVec2  DesiredSize;    // Read-write.  Desired size, based on user's mouse position. Write to this field to restrain resizing.
    pub DesiredSize: ImVec2,
}
