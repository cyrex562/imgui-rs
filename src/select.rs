// Extend
pub enum DimgSelectableFlags
{
    // NB: need to be in sync with last value of
    NoHoldingActiveID      = 1 << 20,
    SelectOnNav            = 1 << 21,  // (WIP) Auto-select when moved into. This is not exposed in public API as to handle multi-select and modifiers we will need user to explicitly control focus scope. May be replaced with a BeginSelection() API.
    SelectOnClick          = 1 << 22,  // Override button behavior to react on Click (default is Click+Release)
    SelectOnRelease        = 1 << 23,  // Override button behavior to react on Release (default is Click+Release)
    SpanAvailWidth         = 1 << 24,  // Span all avail width even if we declared less for layout purpose. FIXME: We may be able to remove this (added in 6251d379, 2bcafc86 for menus)
    DrawHoveredWhenHeld    = 1 << 25,  // Always show active when held, even is not hovered. This concept could probably be renamed/formalized somehow.
    SetNavIdOnHover        = 1 << 26,  // Set Nav/Focus id on mouse hover (used by MenuItem)
    NoPadWithHalfSpacing   = 1 << 27   // Disable padding each side with ItemSpacing * 0.5
}
