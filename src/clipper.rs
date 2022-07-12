use crate::imgui_list_clipper::ImGuiListClipper;

#[derive(Debug,Clone,Default)]
pub struct ImGuiListClipperRange
{
    // int     min;
    pub Min: i32,
    // int     max;
    pub Max: i32,
    // bool    PosToIndexConvert;      // Begin/End are absolute position (will be converted to indices later)
    pub PosToIndexConvert: bool,
    // ImS8    PosToIndexOffsetMin;    // Add to min after converting to indices
    pub PosToIndexOffsetMin: i8,
    // ImS8    PosToIndexOffsetMax;    // Add to min after converting to indices
    pub PosToIndexOffsetMax: i8,
}

impl ImGuiListClipperRange {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    // static ImGuiListClipperRange    FromIndices(int min, int max)                               { ImGuiListClipperRange r = { min, max, false, 0, 0 }; return r; }
    pub fn FromIndices(min: i32, max: i32) -> Self {
        Self {
            Min: min,
            Max: max,
            PosToIndexConvert: false,
            PosToIndexOffsetMin: 0,
            PosToIndexOffsetMax: 0
        }
    }
    //     static ImGuiListClipperRange    FromPositions(float y1, float y2, int off_min, int off_max) { ImGuiListClipperRange r = { y1, y2, true, (ImS8)off_min, (ImS8)off_max }; return r; }
    pub fn FromPositions(y1: f32, y2: f32, off_min: i32, off_max: i32) -> Self {
        Self {
            Min: y1 as i32,
            Max: y2 as i32,
            PosToIndexConvert: true,
            PosToIndexOffsetMin: off_min as i8,
            PosToIndexOffsetMax: off_max as i8,
        }
    }
}

// Temporary clipper data, buffers shared/reused between instances
#[derive(Debug,Clone,Default)]
pub struct ListClipperData
{
    // ImGuiListClipper*               ListClipper;
    pub ListClipper: *mut ImGuiListClipper,
    // float                           LossynessOffset;
    pub LossynessOffset: f32,
    // int                             StepNo;
    pub StepNo: i32,
    // int                             ItemsFrozen;
    pub ItemsFrozen: i32,
    // ImVector<ImGuiListClipperRange> Ranges;
    pub Ranges: Vec<ImGuiListClipperRange>,
}

impl ListClipperData {
    // ImGuiListClipperData()          { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     void                            Reset(ImGuiListClipper* clipper) { ListClipper = clipper; StepNo = ItemsFrozen = 0; Ranges.resize(0); }
    pub fn reset(&mut self, clipper: *mut ImGuiListClipper) {
        self.ListClipper = clipper;
        self.StepNo = 0;
        self.ItemsFrozen = 0;
        self.Ranges.clear()
    }
}
