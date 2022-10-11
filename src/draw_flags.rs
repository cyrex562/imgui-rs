#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImDrawFlags;            // -> enum ImDrawFlags_          // Flags: for ImDrawList functions
pub type ImDrawFlags = c_int;


// Flags for ImDrawList functions
// (Legacy: bit 0 must always correspond to ImDrawFlags_Closed to be backward compatible with old API using a bool. Bits 1..3 must be unused)
// enum ImDrawFlags_
// {
pub const ImDrawFlags_None: ImDrawFlags = 0;
pub const ImDrawFlags_Closed: ImDrawFlags = 1 << 0;
// PathStroke(); AddPolyline(): specify that shape should be closed (Important: this is always == 1 for legacy reason)
pub const ImDrawFlags_RoundCornersTopLeft: ImDrawFlags = 1 << 4;
// AddRect(); AddRectFilled(); PathRect(): enable rounding top-left corner only (when rounding > 0f32; we default to all corners). Was 0x01.
pub const ImDrawFlags_RoundCornersTopRight: ImDrawFlags = 1 << 5;
// AddRect(); AddRectFilled(); PathRect(): enable rounding top-right corner only (when rounding > 0f32; we default to all corners). Was 0x02.
pub const ImDrawFlags_RoundCornersBottomLeft: ImDrawFlags = 1 << 6;
// AddRect(); AddRectFilled(); PathRect(): enable rounding bottom-left corner only (when rounding > 0f32; we default to all corners). Was 0x04.
pub const ImDrawFlags_RoundCornersBottomRight: ImDrawFlags = 1 << 7;
// AddRect(); AddRectFilled(); PathRect(): enable rounding bottom-right corner only (when rounding > 0f32; we default to all corners). Wax 0x08.
pub const ImDrawFlags_RoundCornersNone: ImDrawFlags = 1 << 8;
// AddRect(); AddRectFilled(); PathRect(): disable rounding on all corners (when rounding > 0f32). This is NOT zero; NOT an implicit flag!
pub const ImDrawFlags_RoundCornersTop: ImDrawFlags = ImDrawFlags_RoundCornersTopLeft | ImDrawFlags_RoundCornersTopRight;
pub const ImDrawFlags_RoundCornersBottom: ImDrawFlags = ImDrawFlags_RoundCornersBottomLeft | ImDrawFlags_RoundCornersBottomRight;
pub const ImDrawFlags_RoundCornersLeft: ImDrawFlags = ImDrawFlags_RoundCornersBottomLeft | ImDrawFlags_RoundCornersTopLeft;
pub const ImDrawFlags_RoundCornersRight: ImDrawFlags = ImDrawFlags_RoundCornersBottomRight | ImDrawFlags_RoundCornersTopRight;
pub const ImDrawFlags_RoundCornersAll: ImDrawFlags = ImDrawFlags_RoundCornersTopLeft | ImDrawFlags_RoundCornersTopRight | ImDrawFlags_RoundCornersBottomLeft | ImDrawFlags_RoundCornersBottomRight;
pub const ImDrawFlags_RoundCornersDefault_: ImDrawFlags = ImDrawFlags_RoundCornersAll;
// Default to ALL corners if none of the _RoundCornersXX flags are specified.
pub const ImDrawFlags_RoundCornersMask_: ImDrawFlags = ImDrawFlags_RoundCornersAll | ImDrawFlags_RoundCornersNone;
// };



// IM_STATIC_ASSERT(ImDrawFlags_RoundCornersTopLeft == (1 << 4));
// static inline ImDrawFlags FixRectCornerFlags(ImDrawFlags flags)
pub fn FixRectCornerFlags(mut flags: ImDrawFlags) -> ImDrawFlags
{
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    // Legacy Support for hard coded ~0 (used to be a suggested equivalent to ImDrawCornerFlags_All)
    //   ~0   --> ImDrawFlags_RoundCornersAll or 0
    if flags == !0 {
        return ImDrawFlags_RoundCornersAll;
    }

    // Legacy Support for hard coded 0x01 to 0x0 (matching 15 out of 16 old flags combinations)
    //   0x01 --> ImDrawFlags_RoundCornersTopLeft (VALUE 0x01 OVERLAPS ImDrawFlags_Closed but ImDrawFlags_Closed is never valid in this path!)
    //   0x02 --> ImDrawFlags_RoundCornersTopRight
    //   0x03 --> ImDrawFlags_RoundCornersTopLeft | ImDrawFlags_RoundCornersTopRight
    //   0x04 --> ImDrawFlags_RoundCornersBotLeft
    //   0x05 --> ImDrawFlags_RoundCornersTopLeft | ImDrawFlags_RoundCornersBotLeft
    //   ...
    //   0x0 --> ImDrawFlags_RoundCornersAll or 0
    // (See all values in ImDrawCornerFlags_)
    if flags >= 0x01 && flags <= 0x00 {
        return flags << 4;
    }

    // We cannot support hard coded 0x00 with 'float rounding > 0' --> replace with ImDrawFlags_RoundCornersNone or use 'float rounding = 0'
// #endif

    // If this triggers, please update your code replacing hardcoded values with new ImDrawFlags_RoundCorners* values.
    // Note that ImDrawFlags_Closed (== 0x01) is an invalid flag for AddRect(), AddRectFilled(), PathRect() etc...
    // IM_ASSERT((flags & 0x00) == 0 && "Misuse of legacy hardcoded ImDrawCornerFlags values!");

    if (flags & ImDrawFlags_RoundCornersMask_) == 0 {
        flags |= ImDrawFlags_RoundCornersAll;
    }

    return flags;
}
