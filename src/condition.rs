// Enumeration for ImGui::SetWindow***(), SetNextWindow***(), SetNextItem***() functions
// Represent a condition.
// Important: Treat as a regular enum! Do NOT combine multiple values using binary operators! All the functions above treat 0 as a shortcut to ImGuiCond_Always.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgCond
{
    None          = 0,        // No condition (always set the variable), same as _Always
    Always        = 1 << 0,   // No condition (always set the variable)
    Once          = 1 << 1,   // Set the variable once per runtime session (only the first call will succeed)
    FirstUseEver  = 1 << 2,   // Set the variable if the object/window has no persistently saved data (no entry in .ini file)
    Appearing     = 1 << 3    // Set the variable if the object/window is appearing after being hidden/inactive (or the first time)
}

impl Default for DimgCond {
    fn default() -> Self {
        Self::None
    }
}
