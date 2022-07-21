// Extend
pub enum DimgSelectableFlags
{
    // NB: need to be in sync with last value of
    NoHoldingActiveID     ,
    SelectOnNav           ,  // (WIP) Auto-select when moved into. This is not exposed in public API as to handle multi-select and modifiers we will need user to explicitly control focus scope. May be replaced with a BeginSelection() API.
    SelectOnClick         ,  // Override button behavior to react on Click (default is Click+Release)
    SelectOnRelease       ,  // Override button behavior to react on Release (default is Click+Release)
    SpanAvailWidth        ,  // Span all avail width even if we declared less for layout purpose. FIXME: We may be able to remove this (added in 6251d379, 2bcafc86 for menus)
    DrawHoveredWhenHeld   ,  // Always show active when held, even is not hovered. This concept could probably be renamed/formalized somehow.
    SetNavIdOnHover       ,  // Set Nav/Focus id on mouse hover (used by MenuItem)
    NoPadWithHalfSpacing   = 1 << 27   // Disable padding each side with ItemSpacing * 0.5
}
