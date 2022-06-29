pub enum ImGuiLogType
{
    None = 0,
    TTY,
    File,
    Buffer,
    Clipboard
}

pub enum ImGuiDebugLogFlags
{
    // Event types
    None             = 0,
    EventActiveId    = 1 << 0,
    EventFocus       = 1 << 1,
    EventPopup       = 1 << 2,
    EventNav         = 1 << 3,
    EventIO          = 1 << 4,
    EventDocking     = 1 << 5,
    EventViewport    = 1 << 6,
    OutputToTTY      = 1 << 10   // Also send output to TTY
}

pub const ImGuiDebugLogFlags_EventMask: u32 = ImGuiDebugLogFlags::EventActiveId | ImGuiDebugLogFlags::EventFocus | ImGuiDebugLogFlags::EventPopup | ImGuiDebugLogFlags::EventNav | ImGuiDebugLogFlags::EventIO | ImGuiDebugLogFlags::EventDocking | ImGuiDebugLogFlags::EventViewport;
