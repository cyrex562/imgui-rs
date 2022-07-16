use crate::config::ConfigFlags;
use crate::context::Context;
use crate::types::Id32;
use crate::direction::Direction;
use crate::dock_context::{DockContext, dock_context_clear_nodes, dock_context_rebuild_nodes};
use crate::dock_node::{DockNodeFlags, DockNodeSettings};
use crate::rect::Rect;
use crate::settings::SettingsHandler;
use crate::types::INVALID_ID;



/// List of colors that are stored at the time of Begin() into Docked windows.
/// We currently store the packed colors in a simple array window->dock_style.colors[].
/// A better solution may involve appending into a log of colors in ImGuiContext + store offsets into those arrays in ImGuiWindow,
/// but it would be more complex as we'd need to double-buffer both as e.g. drop target may refer to window from last frame.
pub enum DimgWindowDockStyleCol
{
    Text,
    Tab,
    TabHovered,
    TabActive,
    TabUnfocused,
    TabUnfocusedActive,
    LastItem
}

// void ImGui::DockContextInitialize(ImGuiContext* ctx)

// Docking context update function, called by NewFrame()
// void ImGui::DockContextNewFrameUpdateDocking(ImGuiContext* ctx)
pub fn dock_context_new_frame_update_docking(ctx: &mut Context)
{
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc  = &ctx->DockContext;
    let mut dc: &mut DockContext = &mut ctx.dock_context;
    if !(ctx.io.config_flags.contains(ConfigFlags::DockingEnable)) {
        return;
    }

    // [DEBUG] Store hovered dock node.
    // We could in theory use DockNodeTreeFindVisibleNodeByPos() on the root host dock node, but using ->dock_node is a good shortcut.
    // Note this is mostly a debug thing and isn't actually used for docking target, because docking involve more detailed filtering.
    g.HoveredDockNode = NULL;
    if (ImGuiWindow* hovered_window = g.hovered_window_under_moving_window)
    {
        if (hovered_window.DockNodeAsHost)
            g.HoveredDockNode = DockNodeTreeFindVisibleNodeByPos(hovered_window.DockNodeAsHost, g.io.mouse_pos);
        else if (hovered_window.root_window.DockNode)
            g.HoveredDockNode = hovered_window.root_window.DockNode;
    }

    // Process Docking requests
    for (int n = 0; n < dc.Requests.size; n += 1)
        if (dc.Requests[n].Type == ImGuiDockRequestType_Dock)
            DockContextProcessDock(ctx, &dc.Requests[n]);
    dc.Requests.resize(0);

    // Create windows for each automatic docking nodes
    // We can have NULL pointers when we delete nodes, but because id are recycled this should amortize nicely (and our node count will never be very high)
    for (int n = 0; n < dc.Nodes.data.size; n += 1)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc.Nodes.data[n].val_p)
            if (node.IsFloatingNode())
                DockNodeUpdate(node);
}

void ImGui::DockContextEndFrame(ImGuiContext* ctx)
{
    // Draw backgrounds of node missing their window
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc = &g.DockContext;
    for (int n = 0; n < dc.Nodes.data.size; n += 1)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc.Nodes.data[n].val_p)
            if (node.LastFrameActive == g.frame_count && node.IsVisible && node.HostWindow && node.IsLeafNode() && !node.IsBgDrawnThisFrame)
            {
                Rect bg_rect(node.pos + Vector2D::new(0.0, GetFrameHeight()), node.pos + node.size);
                ImDrawFlags bg_rounding_flags = CalcRoundingFlagsForRectInRect(bg_rect, node.HostWindow.rect(), DOCKING_SPLITTER_SIZE);
                node.HostWindow.DrawList.ChannelsSetCurrent(0);
                node.HostWindow.DrawList.AddRectFilled(bg_rect.min, bg_rect.max, node.LastBgColor, node.HostWindow.WindowRounding, bg_rounding_flags);
            }
}

static ImGuiDockNode* ImGui::DockContextFindNodeByID(ImGuiContext* ctx, ImGuiID id)
{
    return (ImGuiDockNode*)ctx.DockContext.Nodes.GetVoidPtr(id);
}

ImGuiID ImGui::DockContextGenNodeID(ImGuiContext* ctx)
{
    // Generate an id for new node (the exact id value doesn't matter as long as it is not already used)
    // FIXME-OPT FIXME-DOCK: This is suboptimal, even if the node count is small enough not to be a worry.0
    // We should poke in ctx->Nodes to find a suitable id faster. Even more so trivial that ctx->Nodes lookup is already sorted.
    ImGuiID id = 0x0001;
    while (DockContextFindNodeByID(ctx, id) != NULL)
        id += 1;
    return id;
}

static ImGuiDockNode* ImGui::DockContextAddNode(ImGuiContext* ctx, ImGuiID id)
{
    // Generate an id for the new node (the exact id value doesn't matter as long as it is not already used) and add the first window.
    ImGuiContext& g = *ctx;
    if (id == 0)
        id = DockContextGenNodeID(ctx);
    else
        IM_ASSERT(DockContextFindNodeByID(ctx, id) == NULL);

    // We don't set node->last_frame_alive on construction. Nodes are always created at all time to reflect .ini settings!
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextAddNode 0x%08X\n", id);
    ImGuiDockNode* node = IM_NEW(ImGuiDockNode)(id);
    ctx.DockContext.Nodes.SetVoidPtr(node.ID, node);
    return node;
}

static void ImGui::DockContextRemoveNode(ImGuiContext* ctx, ImGuiDockNode* node, bool merge_sibling_into_parent_node)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc  = &ctx.DockContext;

    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextRemoveNode 0x%08X\n", node.ID);
    IM_ASSERT(DockContextFindNodeByID(ctx, node.ID) == node);
    IM_ASSERT(node.ChildNodes[0] == NULL && node.ChildNodes[1] == NULL);
    IM_ASSERT(node.Windows.size == 0);

    if (node.HostWindow)
        node.HostWindow.DockNodeAsHost = NULL;

    ImGuiDockNode* parent_node = node.ParentNode;
    const bool merge = (merge_sibling_into_parent_node && parent_node != NULL);
    if (merge)
    {
        IM_ASSERT(parent_node.ChildNodes[0] == node || parent_node.ChildNodes[1] == node);
        ImGuiDockNode* sibling_node = (parent_node.ChildNodes[0] == node ? parent_node.ChildNodes[1] : parent_node.ChildNodes[0]);
        DockNodeTreeMerge(&g, parent_node, sibling_node);
    }
    else
    {
        for (int n = 0; parent_node && n < IM_ARRAYSIZE(parent_node.ChildNodes); n += 1)
            if (parent_node.ChildNodes[n] == node)
                node.ParentNode.ChildNodes[n] = NULL;
        dc.Nodes.SetVoidPtr(node.ID, NULL);
        IM_DELETE(node);
    }
}


#[derive(Debug,Clone,Default)]
pub struct DimgDockRequest
{
    // ImGuiDockRequestType    Type;
    pub request_type: DimgDockRequestType,
    // ImGuiWindow*            DockTargetWindow;   // Destination/Target window to dock into (may be a loose window or a dock_node, might be NULL in which case DockTargetNode cannot be NULL)
    pub dock_target_window: Id32,
    // ImGuiDockNode*          DockTargetNode;     // Destination/Target Node to dock into
    pub dock_target_node: Id32,
    // ImGuiWindow*            DockPayload;        // Source/Payload window to dock (may be a loose window or a dock_node), [Optional]
    pub dock_payload: Id32,
    // ImGuiDir                DockSplitDir;
    pub dock_split_dir: Direction,
    // float                   DockSplitRatio;
    pub dock_split_ratio: f32,
    // bool                    DockSplitOuter;
    pub dock_split_outer: bool,
    // ImGuiWindow*            UndockTargetWindow;
    pub undock_target_window: Id32,
    // ImGuiDockNode*          UndockTargetNode;
    pub undock_target_node: Id32,
}

impl DimgDockRequest {
    //ImGuiDockRequest()
    pub fn new() -> Self
    {
        // Type = None;
        // DockTargetWindow = DockPayload = UndockTargetWindow = NULL;
        // DockTargetNode = UndockTargetNode = NULL;
        // DockSplitDir = ImGuiDir_None;
        // DockSplitRatio = 0.5;
        // DockSplitOuter = false;
        Self {
            request_type: DimgDockRequestType::None,
            dock_target_window: INVALID_ID,
            dock_payload: INVALID_ID,
            undock_target_window: INVALID_ID,
            dock_target_node: INVALID_ID,
            undock_target_node: INVALID_ID,
            dock_split_dir: Direction::None,
            dock_split_ratio: 0.5,
            dock_split_outer: false,
        }
    }
}


#[derive(Debug,Clone)]
pub enum DimgDockRequestType
{
    None = 0,
    Dock,
    Undock,
    Split                  // split is the same as Dock but without a DockPayload
}

impl Default for DimgDockRequestType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug,Default,Clone)]
pub struct DimgDockPreviewData
{
    // ImGuiDockNode   FutureNode;
    pub future_node: DimgDockNode,
    // bool            IsDropAllowed;
    pub is_drop_allowed: bool,
    // bool            IsCenterAvailable;
    pub is_center_available: bool,
    // bool            IsSidesAvailable;           // Hold your breath, grammar freaks..
    pub is_sides_available: bool,
    // bool            IsSplitDirExplicit;         // Set when hovered the drop rect (vs. implicit SplitDir==None when hovered the window)
    pub is_split_dir_explicit: bool,
    // ImGuiDockNode*  SplitNode;
    pub split_node: Id32,
    // ImGuiDir        SplitDir;
    pub split_dir: Direction,
    // float           SplitRatio;
    pub split_ratio: f32,
    // ImRect          DropRectsDraw[ImGuiDir_COUNT + 1];  // May be slightly different from hit-testing drop rects used in DockNodeCalcDropRects()
    pub drop_rects_draw: [Rect; 5 ],
}

impl DimgDockPreviewData {

    // ImGuiDockPreviewData() : FutureNode(0) {
    pub fn new() -> Self {
    // IsDropAllowed = IsCenterAvailable = IsSidesAvailable = IsSplitDirExplicit = false; SplitNode = NULL; SplitDir = ImGuiDir_None; SplitRatio = 0.f; for (int n = 0; n < IM_ARRAYSIZE(DropRectsDraw); n += 1) DropRectsDraw[n] = ImRect(+FLT_MAX, +FLT_MAX, -FLT_MAX, -FLT_MAX);
    Self {
        future_node: (),
        is_drop_allowed: false,
        is_center_available: false,
        is_sides_available: false,
        is_split_dir_explicit: false,
        split_node: 0,
        split_dir: Direction::None,
        split_ratio: 0.0,
        drop_rects_draw: [Rect::new();5]
    }
    }
}

// Docking
// static const float DOCKING_TRANSPARENT_PAYLOAD_ALPHA        = 0.50;    // For use with io.config_docking_transparent_payload. Apply to viewport _or_ WindowBg in host viewport.
pub const DOCKING_TRANSPARENT_PAYLOAD_ALPHA: f32 = 0.50;

// static const float DOCKING_SPLITTER_SIZE                    = 2.0;
pub const DOCKING_SPLITTER_SIZE: f32 = 2.0;
