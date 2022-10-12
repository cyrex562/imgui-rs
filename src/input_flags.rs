#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiInputFlags;            // -> enum ImGuiInputFlags_         // Flags: for IsKeyPressedEx()
pub type ImGuiInputFlags = c_int;


// Flags for IsKeyPressedEx(). In upcoming feature this will be used more (and IsKeyPressedEx() renamed)
// Don't mistake with ImGuiInputTextFlags! (for InputText() function)
// enum ImGuiInputFlags_
// {
    // Flags for IsKeyPressedEx()
    pub const ImGuiInputFlags_None: ImGuiInputFlags = 0;
    pub const ImGuiInputFlags_Repeat: ImGuiInputFlags = 1 << 0;   // Return true on successive repeats. Default for legacy IsKeyPressed(). NOT Default for legacy IsMouseClicked(). MUST BE == 1.
    pub const ImGuiInputFlags_RepeatRateDefault: ImGuiInputFlags = 1 << 1;   // Repeat rate: Regular (default)
    pub const ImGuiInputFlags_RepeatRateNavMove: ImGuiInputFlags = 1 << 2;   // Repeat rate: Fast
    pub const ImGuiInputFlags_RepeatRateNavTweak: ImGuiInputFlags = 1 << 3;   // Repeat rate: Faster
    pub const ImGuiInputFlags_RepeatRateMask_: ImGuiInputFlags = ImGuiInputFlags_RepeatRateDefault | ImGuiInputFlags_RepeatRateNavMove | ImGuiInputFlags_RepeatRateNavTweak;
// };
