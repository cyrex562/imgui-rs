use std::ptr::{null, null_mut};
use crate::imgui_h::ImGuiID;
use crate::imgui_rect::ImRect;
use crate::imgui_window::{ImGuiItemFlags, ImGuiWindow};

pub enum ImGuiActivateFlags
{
    None                 = 0,
    PreferInput          = 1 << 0,       // Favor activation that requires keyboard text input (e.g. for Slider/Drag). Default if keyboard is available.
    PreferTweak          = 1 << 1,       // Favor activation for tweaking with arrows or gamepad (e.g. for Slider/Drag). Default if keyboard is not available.
    TryToPreserveState   = 1 << 2        // Request widget to preserve state if it can (e.g. InputText will try to preserve cursor/selection)
}


// Early work-in-progress API for ScrollToItem()
pub enum ImGuiScrollFlags
{
    None                   = 0,
    KeepVisibleEdgeX       = 1 << 0,       // If item is not visible: scroll as little as possible on X axis to bring item back into view [default for X axis]
    KeepVisibleEdgeY       = 1 << 1,       // If item is not visible: scroll as little as possible on Y axis to bring item back into view [default for Y axis for windows that are already visible]
    KeepVisibleCenterX     = 1 << 2,       // If item is not visible: scroll to make the item centered on X axis [rarely used]
    KeepVisibleCenterY     = 1 << 3,       // If item is not visible: scroll to make the item centered on Y axis
    AlwaysCenterX          = 1 << 4,       // Always center the result item on X axis [rarely used]
    AlwaysCenterY          = 1 << 5,       // Always center the result item on Y axis [default for Y axis for appearing window)
    NoScrollParent         = 1 << 6,       // Disable forwarding scrolling to parent window if required to keep item/rect visible (only scroll window the function was applied to).
    
}

pub const ImGuiScrollFlags_MaskX: ImGuiScrollFlags = ImGuiScrollFlags::KeepVisibleEdgeX | ImGuiScrollFlags::KeepVisibleCenterX | ImGuiScrollFlags::AlwaysCenterX;
pub const ImGuiScrollFlags_MaskY: ImGuiScrollFlags = ImGuiScrollFlags::KeepVisibleEdgeY | ImGuiScrollFlags::KeepVisibleCenterY | ImGuiScrollFlags::AlwaysCenterY;

pub enum ImGuiNavHighlightFlags
{
    INone             = 0,
    ITypeDefault      = 1 << 0,
    ITypeThin         = 1 << 1,
    IAlwaysDraw       = 1 << 2,       // Draw rectangular highlight if (g.NavId == id) _even_ when using the mouse.
    INoRounding       = 1 << 3
}

pub enum ImGuiNavDirSourceFlags
{
    None             = 0,
    RawKeyboard      = 1 << 0,   // Raw keyboard (not pulled from nav), facilitate use of some functions before we can unify nav and keys
    Keyboard         = 1 << 1,
    PadDPad          = 1 << 2,
    PadLStick        = 1 << 3
}

pub enum ImGuiNavMoveFlags
{
    None                  = 0,
    LoopX                 = 1 << 0,   // On failed request, restart from opposite side
    LoopY                 = 1 << 1,
    WrapX                 = 1 << 2,   // On failed request, request from opposite side one line down (when NavDir==right) or one line up (when NavDir==left)
    WrapY                 = 1 << 3,   // This is not super useful but provided for completeness
    AllowCurrentNavId     = 1 << 4,   // Allow scoring and considering the current NavId as a move target candidate. This is used when the move source is offset (e.g. pressing PageDown actually needs to send a Up move request, if we are pressing PageDown from the bottom-most item we need to stay in place)
    AlsoScoreVisibleSet   = 1 << 5,   // Store alternate result in NavMoveResultLocalVisible that only comprise elements that are already fully visible (used by PageUp/PageDown)
    ScrollToEdgeY         = 1 << 6,   // Force scrolling to min/max (used by Home/End) // FIXME-NAV: Aim to remove or reword, probably unnecessary
    Forwarded             = 1 << 7,
    DebugNoResult         = 1 << 8,   // Dummy scoring for debug purpose, don't apply result
    FocusApi              = 1 << 9,
    Tabbing               = 1 << 10,  // == Focus + Activate if item is Inputable + DontChangeNavHighlight
    Activate              = 1 << 11,
    DontSetNavHighlight   = 1 << 12   // Do not alter the visible state of keyboard vs mouse nav highlight
}

#[derive(Default,Debug,Clone)]
pub struct ImGuiNavItemData
{
    // ImGuiWindow*        Window;         // Init,Move    // Best candidate window (result->ItemWindow->RootWindowForNav == request->Window)
    pub Window: *mut ImGuiWindow,
    // ImGuiID             ID;             // Init,Move    // Best candidate item ID
    pub ID: ImGuiID,
    // ImGuiID             FocusScopeId;   // Init,Move    // Best candidate focus scope ID
    pub FocusScopeId: ImGuiID,
    // ImRect              RectRel;        // Init,Move    // Best candidate bounding box in window relative space
    pub RectRel: ImRect,
    // ImGuiItemFlags      InFlags;        // ????,Move    // Best candidate item flags
    pub InFlags: ImGuiItemFlags,
    // float               DistBox;        //      Move    // Best candidate box distance to current NavId
    pub DistBox: f32,
    // float               DistCenter;     //      Move    // Best candidate center distance to current NavId
    pub DistCenter: f32,
    // float               DistAxial;      //      Move    // Best candidate axial distance to current NavId
    pub DistAxial: f32,
}

impl ImGuiNavItemData {
    // ImGuiNavItemData()  { Clear(); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     void Clear()        { Window = NULL; ID = FocusScopeId = 0; InFlags = 0; DistBox = DistCenter = DistAxial = FLT_MAX; }
    pub fn Clear(&mut self) {
        self.Window = null_mut();
        self.ID = 0;
        self.FocusScopeId = 0;
        self.InFlags = ImGuiItemFlags::None;
        self.DistBox = f32::MAX;
        self.DistCenter = f32::MAX;
        self.DistAxial = f32::MAX;
    }
}
