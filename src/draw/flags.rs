use std::collections::HashSet;

// IM_STATIC_ASSERT(DrawFlags::RoundCornersTopLeft == (1 << 4));
// static inline ImDrawFlags fix_rect_corner_flags(ImDrawFlags flags)
pub fn fix_rect_corner_flags(flags: &HashSet<DrawFlags>) -> DrawFlags {
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    // Legacy Support for hard coded ~0 (used to be a suggested equivalent to ImDrawCornerFlags_All)
    //   ~0   --> ImDrawFlags_RoundCornersAll or 0
    // if flags.is_empty()){
    //     return DrawFlags::RoundCornersAll;
    // }

    // Legacy Support for hard coded 0x01 to 0x0F (matching 15 out of 16 old flags combinations)
    //   0x01 --> ImDrawFlags_RoundCornersTopLeft (VALUE 0x01 OVERLAPS ImDrawFlags_Closed but ImDrawFlags_Closed is never valid in this path!)
    //   0x02 --> ImDrawFlags_RoundCornersTopRight
    //   0x03 --> ImDrawFlags_RoundCornersTopLeft | ImDrawFlags_RoundCornersTopRight
    //   0x04 --> ImDrawFlags_RoundCornersBotLeft
    //   0x05 --> ImDrawFlags_RoundCornersTopLeft | ImDrawFlags_RoundCornersBotLeft
    //   ...
    //   0x0F --> ImDrawFlags_RoundCornersAll or 0
    // (See all values in ImDrawCornerFlags_)
    // if (flags >= 0x01 && flags <= 0x0F)
    //     return (flags << 4);

    // We cannot support hard coded 0x00 with 'float rounding > 0.0' --> replace with ImDrawFlags_RoundCornersNone or use 'float rounding = 0.0'


    // If this triggers, please update your code replacing hardcoded values with new ImDrawFlags_RoundCorners* values.
    // Note that ImDrawFlags_Closed (== 0x01) is an invalid flag for add_rect(), add_rect_filled(), PathRect() etc...
    // IM_ASSERT((flags & 0x0F) == 0 && "Misuse of legacy hardcoded ImDrawCornerFlags values!");
    //
    // if ((flags & DrawFlags::RoundCornersMask_) == 0)
    //     flags |= DrawFlags::RoundCornersAll;
    //
    // return flags;
    todo!()
}

/// flags for ImDrawList functions
/// (Legacy: bit 0 must always correspond to ImDrawFlags_Closed to be backward compatible with old API using a bool. Bits 1..3 must be unused)
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DrawFlags {
    None = 0,
    Closed,
    // PathStroke(), add_polyline(): specify that shape should be closed (Important: this is always == 1 for legacy reason)
    RoundCornersTopLeft,
    // add_rect(), add_rect_filled(), PathRect(): enable rounding top-left corner only (when rounding > 0.0, we default to all corners). Was 0x01.
    RoundCornersTopRight,
    // add_rect(), add_rect_filled(), PathRect(): enable rounding top-right corner only (when rounding > 0.0, we default to all corners). Was 0x02.
    RoundCornersBottomLeft,
    // add_rect(), add_rect_filled(), PathRect(): enable rounding bottom-left corner only (when rounding > 0.0, we default to all corners). Was 0x04.
    RoundCornersBottomRight,
    // add_rect(), add_rect_filled(), PathRect(): enable rounding bottom-right corner only (when rounding > 0.0, we default to all corners). Wax 0x08.
    RoundCornersNone, // add_rect(), add_rect_filled(), PathRect(): disable rounding on all corners (when rounding > 0.0). This is NOT zero, NOT an implicit flag!
}
