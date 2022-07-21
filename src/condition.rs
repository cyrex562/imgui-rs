/// Enumeration for ImGui::SetWindow***(), SetNextWindow***(), SetNextItem***() functions
/// Represent a condition.
/// Important: Treat as a regular enum! Do NOT combine multiple values using binary operators! All the functions above treat 0 as a shortcut to Cond::Always.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Condition {
    None,
    // No condition (always set the variable), same as _Always
    Always,
    // No condition (always set the variable)
    Once,
    // Set the variable once per runtime session (only the first call will succeed)
    FirstUseEver,
    // Set the variable if the object/window has no persistently saved data (no entry in .ini file)
    Appearing,         // Set the variable if the object/window is appearing after being hidden/inactive (or the first time)
}

impl Default for Condition {
    fn default() -> Self {
        Self::None
    }
}
