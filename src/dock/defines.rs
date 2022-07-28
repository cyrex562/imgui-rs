/// List of colors that are stored at the time of Begin() into Docked windows.
/// We currently store the packed colors in a simple array window->dock_style.colors[].
/// A better solution may involve appending into a log of colors in ImGuiContext + store offsets into those arrays in ImGuiWindow,
/// but it would be more complex as we'd need to double-buffer both as e.g. drop target may refer to window from last frame.
#[derive(Debug, Clone)]
pub enum WindowDockStyleColor {
    None,
    Text,
    Tab,
    TabHovered,
    TabActive,
    TabUnfocused,
    TabUnfocusedActive,
    LastItem,
}

impl Default for WindowDockStyleColor {
    fn default() -> Self {
        Self::None
    }
}

// Docking
// static let DOCKING_TRANSPARENT_PAYLOAD_ALPHA        = 0.50;    // For use with io.config_docking_transparent_payload. Apply to viewport _or_ WindowBg in host viewport.
pub const DOCKING_TRANSPARENT_PAYLOAD_ALPHA: f32 = 0.50;

// static let DOCKING_SPLITTER_SIZE                    = 2.0;
pub const DOCKING_SPLITTER_SIZE: f32 = 2.0;
