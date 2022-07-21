/// flags for ImDrawList functions
/// (Legacy: bit 0 must always correspond to ImDrawFlags_Closed to be backward compatible with old API using a bool. Bits 1..3 must be unused)
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DrawFlags
{
    None                        = 0,
    Closed                     , // PathStroke(), AddPolyline(): specify that shape should be closed (Important: this is always == 1 for legacy reason)
    RoundCornersTopLeft        , // add_rect(), add_rect_filled(), PathRect(): enable rounding top-left corner only (when rounding > 0.0, we default to all corners). Was 0x01.
    RoundCornersTopRight       , // add_rect(), add_rect_filled(), PathRect(): enable rounding top-right corner only (when rounding > 0.0, we default to all corners). Was 0x02.
    RoundCornersBottomLeft     , // add_rect(), add_rect_filled(), PathRect(): enable rounding bottom-left corner only (when rounding > 0.0, we default to all corners). Was 0x04.
    RoundCornersBottomRight    , // add_rect(), add_rect_filled(), PathRect(): enable rounding bottom-right corner only (when rounding > 0.0, we default to all corners). Wax 0x08.
    RoundCornersNone           , // add_rect(), add_rect_filled(), PathRect(): disable rounding on all corners (when rounding > 0.0). This is NOT zero, NOT an implicit flag!
}
