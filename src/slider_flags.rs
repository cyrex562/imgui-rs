use libc::c_int;

// typedef int ImGuiSliderFlags;       // -> enum ImGuiSliderFlags_     // Flags: for DragFloat(), DragInt(), SliderFloat(), SliderInt() etc.
pub type ImGuiSliderFlags = c_int;
