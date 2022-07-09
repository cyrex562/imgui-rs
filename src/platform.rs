use crate::vec_nd::DimgVec2D;

// (Optional) This is required when enabling multi-viewport. Represent the bounds of each connected monitor/display and their DPI.
// We use this information for multiple DPI support + clamping the position of popups and tooltips so they don't straddle multiple monitors.
#[derive(Debug,Clone,Default)]
pub struct DimgPlatformMonitor
{
    // ImVec2  MainPos, MainSize;      // Coordinates of the area displayed on this monitor (Min = upper left, Max = bottom right)
    pub MainPos: DimgVec2D,
    pub MainSize: DimgVec2D,
    // ImVec2  work_pos, work_size;      // Coordinates without task bars / side bars / menu bars. Used to avoid positioning popups/tooltips inside this region. If you don't have this info, please copy the value for MainPos/MainSize.
    pub WorkPos: DimgVec2D,
    pub WorkSize: DimgVec2D,
    pub DpiScale: f32,              // 1.0 = 96 DPI
    // ImGuiPlatformMonitor()          { MainPos = MainSize = work_pos = work_size = ImVec2(0, 0); dpi_scale = 1.0; }
}

impl DimgPlatformMonitor {
    pub fn new() -> Self {
        Self {
            MainPos: Default::default(),
            MainSize: Default::default(),
            WorkPos: Default::default(),
            WorkSize: Default::default(),
            DpiScale: 1.0
        }
    }
}

// (Optional) Support for IME (Input Method Editor) via the io.SetPlatformImeDataFn() function.
#[derive(Debug,Default,Clone)]
pub struct DimgPlatformImeData
{
    pub WantVisible: bool,        // A widget wants the IME to be visible
    pub InputPos: DimgVec2D,           // Position of the input cursor
    pub InputLineHeight: f32,   // Line height

    // ImGuiPlatformImeData() { memset(this, 0, sizeof(*this)); }
}

impl DimgPlatformImeData {
    pub fn new(initial_input_pos: DimgVec2D) -> Self {
        Self {
            WantVisible: false,
            InputPos: initial_input_pos,
            InputLineHeight: 0.0
        }
    }
}
