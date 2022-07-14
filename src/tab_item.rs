// Extend
pub enum ImGuiTabItemFlags
{
    SectionMask_              = Leading | Trailing,
    NoCloseButton             = 1 << 20,  // Track whether p_open was set or not (we'll need this info on the next frame to recompute ContentWidth during layout)
    Button                    = 1 << 21,  // Used by TabItemButton, change the tab item behavior to mimic a button
    Unsorted                  = 1 << 22,  // [Docking] Trailing tabs with the _Unsorted flag will be sorted based on the dock_order of their window.
    Preview                   = 1 << 23   // [Docking] Display tab shape for docking preview (height is adjusted slightly to compensate for the yet missing tab bar)
}
