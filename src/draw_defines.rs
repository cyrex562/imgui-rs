/// flags for ImDrawList functions
/// (Legacy: bit 0 must always correspond to ImDrawFlags_Closed to be backward compatible with old API using a bool. Bits 1..3 must be unused)
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DrawFlags
{
    None                        = 0,
    Closed                      = 1 << 0, // PathStroke(), AddPolyline(): specify that shape should be closed (Important: this is always == 1 for legacy reason)
    RoundCornersTopLeft         = 1 << 4, // add_rect(), add_rect_filled(), PathRect(): enable rounding top-left corner only (when rounding > 0.0, we default to all corners). Was 0x01.
    RoundCornersTopRight        = 1 << 5, // add_rect(), add_rect_filled(), PathRect(): enable rounding top-right corner only (when rounding > 0.0, we default to all corners). Was 0x02.
    RoundCornersBottomLeft      = 1 << 6, // add_rect(), add_rect_filled(), PathRect(): enable rounding bottom-left corner only (when rounding > 0.0, we default to all corners). Was 0x04.
    RoundCornersBottomRight     = 1 << 7, // add_rect(), add_rect_filled(), PathRect(): enable rounding bottom-right corner only (when rounding > 0.0, we default to all corners). Wax 0x08.
    RoundCornersNone            = 1 << 8, // add_rect(), add_rect_filled(), PathRect(): disable rounding on all corners (when rounding > 0.0). This is NOT zero, NOT an implicit flag!
}
