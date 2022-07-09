use crate::condition::DimgCond;
use crate::context::DimgContext;
use crate::window::DimgHoveredFlags;
use crate::rect::DimgRect;
use crate::types::DimgId;
use crate::window::DimgItemFlags;

impl DimgNextItemData {
    // ImGuiNextItemData()         { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     inline void ClearFlags()    { flags = ImGuiNextItemDataFlags_None; } // Also cleared manually by ItemAdd()!
    pub fn ClearFlags(&mut self) {
        self.Flags = ImGuiNextItemDataFlags::None
    }
}

#[derive(Debug,Clone,Default)]
pub struct DimgNextItemData
{
    // ImGuiNextItemDataFlags      flags;
    pub Flags: ImGuiNextItemDataFlags,
    // float                       width;          // Set by SetNextItemWidth()
    pub Width: f32,
    // ImGuiID                     FocusScopeId;   // Set by SetNextItemMultiSelectData() (!= 0 signify value has been set, so it's an alternate version of HasSelectionData, we don't use flags for this because they are cleared too early. This is mostly used for debugging)
    pub FocusScopeId: DimgId,
    // ImGuiCond                   OpenCond;
    pub OpenCond: DimgCond,
    // bool                        OpenVal;        // Set by SetNextItemOpen()
    pub OpenVal: bool,
}

/// Status storage for the last submitted item
#[derive(Debug,Clone,Default)]
pub struct DimgLastItemData
{
    // ImGuiID                 id;
    pub ID: DimgId,
    // ImGuiItemFlags          InFlags;            // See ImGuiItemFlags_
    pub InFlags: DimgItemFlags,
    // ImGuiItemStatusFlags    StatusFlags;        // See ImGuiItemStatusFlags_
    pub StatusFlags: DimgItemStatusFlags,
    // ImRect                  rect;               // Full rectangle
    pub Rect: DimgRect,
    // ImRect                  NavRect;            // Navigation scoring rectangle (not displayed)
    pub NavRect: DimgRect,
    // ImRect                  DisplayRect;        // Display rectangle (only if ImGuiItemStatusFlags_HasDisplayRect is set)
    pub DisplayRect: DimgRect,
    // ImGuiLastItemData()     { memset(this, 0, sizeof(*this)); }
}

impl DimgLastItemData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

pub enum DimgItemStatusFlags
{
    None               = 0,
    HoveredRect        = 1 << 0,   // Mouse position is within item rectangle (does NOT mean that the window is in correct z-order and can be hovered!, this is only one part of the most-common IsItemHovered test)
    HasDisplayRect     = 1 << 1,   // g.last_item_data.DisplayRect is valid
    Edited             = 1 << 2,   // Value exposed by item was edited in the current frame (should match the bool return value of most widgets)
    ToggledSelection   = 1 << 3,   // Set when Selectable(), TreeNode() reports toggling a selection. We can't report "Selected", only state changes, in order to easily handle clipping with less issues.
    ToggledOpen        = 1 << 4,   // Set when TreeNode() reports toggling their open state.
    HasDeactivated     = 1 << 5,   // Set if the widget/group is able to provide data for the Deactivated flag.
    Deactivated        = 1 << 6,   // Only valid if HasDeactivated is set.
    HoveredWindow      = 1 << 7,   // Override the hovered_window test to allow cross-window hover testing.
    FocusedByTabbing   = 1 << 8,    // Set when the Focusable item just got focused by Tabbing (FIXME: to be removed soon)
// #ifdef IMGUI_ENABLE_TEST_ENGINE
     // [imgui_tests only]
    Openable           = 1 << 20,  // Item is an openable (e.g. TreeNode)
    Opened             = 1 << 21,  //
    Checkable          = 1 << 22,  // Item is a checkable (e.g. CheckBox, MenuItem)
    Checked            = 1 << 23   //
// #endif
}

pub enum ImGuiNextItemDataFlags
{
    None     = 0,
    HasWidth = 1 << 0,
    HasOpen  = 1 << 1
}

