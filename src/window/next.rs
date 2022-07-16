use crate::condition::Condition;
use crate::rect::Rect;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;
use crate::window::ImGuiSizeCallback;
use crate::window::class::WindowClass;

// Storage for SetNexWindow** functions
#[derive(Debug, Clone, Default)]
pub struct NextWindowData {
    // ImGuiNextWindowDataFlags    flags;
    pub Flags: NextWindowDataFlags,
    // ImGuiCond                   PosCond;
    pub PosCond: Condition,
    // ImGuiCond                   SizeCond;
    pub SizeCond: Condition,
    // ImGuiCond                   CollapsedCond;
    pub CollapseCond: Condition,
    // ImGuiCond                   DockCond;
    pub DockCond: Condition,
    // Vector2D                      PosVal;
    pub PosVal: Vector2D,
    // Vector2D                      PosPivotVal;
    pub PosPivotVal: Vector2D,
    // Vector2D                      SizeVal;
    pub SizeVal: Vector2D,
    // Vector2D                      ContentSizeVal;
    pub ContentSizeVal: Vector2D,
    // Vector2D                      ScrollVal;
    pub ScrollVal: Vector2D,
    // bool                        PosUndock;
    pub PosUndock: bool,
    // bool                        CollapsedVal;
    pub CollapsedVal: bool,
    // ImRect                      SizeConstraintRect;
    pub SizeConstraintRect: Rect,
    // ImGuiSizeCallback           SizeCallback;
    pub SizeCallback: ImGuiSizeCallback,
    // void*                       SizeCallbackUserData;
    pub SizeCallbackUserData: Vec<u8>,
    // float                       BgAlphaVal;             // Override background alpha
    pub BgAlphaVal: f32,
    // ImGuiID                     viewport_id;
    pub ViewportId: Id32,
    // ImGuiID                     dock_id;
    pub DockId: Id32,
    // ImGuiWindowClass            window_class;
    pub WindowClass: WindowClass,
    // Vector2D                      MenuBarOffsetMinVal;    // (Always on) This is not exposed publicly, so we don't clear it and it doesn't have a corresponding flag (could we? for consistency?)
    pub MenuBarOffsetMinVal: Vector2D,

}

impl NextWindowData {
    // ImGuiNextWindowData()       { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     inline void ClearFlags()    { flags = ImGuiNextWindowDataFlags_None; }
    pub fn ClearFlags(&mut self) {
        self.flags = NextWindowDataFlags::None
    }
}


pub enum NextWindowDataFlags {
    None = 0,
    HasPos = 1 << 0,
    HasSize = 1 << 1,
    HasContentSize = 1 << 2,
    HasCollapsed = 1 << 3,
    HasSizeConstraint = 1 << 4,
    HasFocus = 1 << 5,
    HasBgAlpha = 1 << 6,
    HasScroll = 1 << 7,
    HasViewport = 1 << 8,
    HasDock = 1 << 9,
    HasWindowClass = 1 << 10,
}
