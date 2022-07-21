use std::collections::HashSet;
use crate::defines;

// Extend ImGuiTreeNodeFlags_
pub enum DimgTreeNodeFlags
{
    ClipLabelForTrailingButton = 1 << 20
}


// pub const ImGuiTreeNodeFlags_CollapsingHeader: i32     = ImGuiTreeNodeFlags::ImGuiTreeNodeFlags_Framed | ImGuiTreeNodeFlags::ImGuiTreeNodeFlags_NoTreePushOnOpen | ImGuiTreeNodeFlags::ImGuiTreeNodeFlags_NoAutoOpenOnLog;
pub const TREE_NODE_FLAGS_COLLAPSING_HDR: HashSet<DimgTreeNodeFlags> = HashSet::from([
    DimgTreeNodeFlags::Framed, DimgTreeNodeFlags::NoTreePushOnOpen, DimgTreeNodeFlags::NoAutoOpenOnLog
]);

// flags for ImGui::TreeNodeEx(), ImGui::CollapsingHeader*()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgTreeNodeFlags
{
    None                 = 0,
    Selected            ,   // Draw as selected
    Framed              ,   // Draw frame with background (e.g. for CollapsingHeader)
    AllowItemOverlap    ,   // Hit testing to allow subsequent widgets to overlap this one
    NoTreePushOnOpen    ,   // Don't do a TreePush() when open (e.g. for CollapsingHeader) = no extra indent nor pushing on id stack
    NoAutoOpenOnLog     ,   // Don't automatically and temporarily open node when Logging is active (by default logging will automatically open tree nodes)
    DefaultOpen         ,   // Default node to be open
    OpenOnDoubleClick   ,   // Need double-click to open node
    OpenOnArrow         ,   // Only open when clicking on the arrow part. If ImGuiTreeNodeFlags_OpenOnDoubleClick is also set, single-click arrow or double-click all box to open.
    Leaf                ,   // No collapsing, no arrow (use as a convenience for leaf nodes).
    Bullet              ,   // Display a bullet instead of arrow
    frame_padding        ,  // Use FramePadding (even for an unframed text node) to vertically align text baseline to regular widget height. Equivalent to calling AlignTextToFramePadding().
    SpanAvailWidth      ,  // Extend hit box to the right-most edge, even if not framed. This is not the default in order to allow adding other items on the same line. In the future we may refactor the hit system to be front-to-back, allowing natural overlaps and then this can become the default.
    SpanFullWidth       ,  // Extend hit box to the left-most and right-most edges (bypass the indented area).
    NavLeftJumpsBackHere,  // (WIP) Nav: left direction may move to this TreeNode() from any of its child (items submitted between TreeNode and TreePop)
    //ImGuiTreeNodeFlags_NoScrollOnOpen     = 1 << 14,  // FIXME: TODO: Disable automatic scroll on TreePop() if node got just open and contents is not visible
    // ImGuiTreeNodeFlags_CollapsingHeader     = ImGuiTreeNodeFlags_Framed | ImGuiTreeNodeFlags_NoTreePushOnOpen | ImGuiTreeNodeFlags_NoAutoOpenOnLog
}
