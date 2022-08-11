#[derive(Debug, Clone)]
pub enum DockNodeState {
    Unknown,
    HostWindowHiddenBecauseSingleWindow,
    HostWindowHiddenBecauseWindowsAreResizing,
    HostWindowVisible,
}

impl Default for DockNodeState {
    fn default() -> Self {
        Self::Unknown
    }
}
