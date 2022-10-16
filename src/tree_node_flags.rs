#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiTreeNodeFlags;     // -> enum ImGuiTreeNodeFlags_   // Flags: for TreeNode(); TreeNodeEx(); CollapsingHeader()
pub type ImGuiTreeNodeFlags = c_int;

    pub const ImGuiTreeNodeFlags_None: ImGuiTreeNodeFlags = 0;
    pub const ImGuiTreeNodeFlags_Selected: ImGuiTreeNodeFlags = 1 << 0;   // Draw as selected
    pub const ImGuiTreeNodeFlags_Framed: ImGuiTreeNodeFlags = 1 << 1;   // Draw frame with background (e.g. for CollapsingHeader)
    pub const ImGuiTreeNodeFlags_AllowItemOverlap: ImGuiTreeNodeFlags = 1 << 2;   // Hit testing to allow subsequent widgets to overlap this one
    pub const ImGuiTreeNodeFlags_NoTreePushOnOpen: ImGuiTreeNodeFlags = 1 << 3;   // Don't do a TreePush() when open (e.g. for CollapsingHeader) = no extra indent nor pushing on ID stack
    pub const ImGuiTreeNodeFlags_NoAutoOpenOnLog: ImGuiTreeNodeFlags = 1 << 4;   // Don't automatically and temporarily open node when Logging is active (by default logging will automatically open tree nodes)
    pub const ImGuiTreeNodeFlags_DefaultOpen: ImGuiTreeNodeFlags = 1 << 5;   // Default node to be open
    pub const ImGuiTreeNodeFlags_OpenOnDoubleClick: ImGuiTreeNodeFlags = 1 << 6;   // Need double-click to open node
    pub const ImGuiTreeNodeFlags_OpenOnArrow: ImGuiTreeNodeFlags = 1 << 7;   // Only open when clicking on the arrow part. If ImGuiTreeNodeFlags_OpenOnDoubleClick is also set; single-click arrow or double-click all box to open.
    pub const ImGuiTreeNodeFlags_Leaf: ImGuiTreeNodeFlags = 1 << 8;   // No collapsing; no arrow (use as a convenience for leaf nodes).
    pub const ImGuiTreeNodeFlags_Bullet: ImGuiTreeNodeFlags = 1 << 9;   // Display a bullet instead of arrow
    pub const ImGuiTreeNodeFlags_FramePadding: ImGuiTreeNodeFlags = 1 << 10;  // Use FramePadding (even for an unframed text node) to vertically align text baseline to regular widget height. Equivalent to calling AlignTextToFramePadding().
    pub const ImGuiTreeNodeFlags_SpanAvailWidth: ImGuiTreeNodeFlags = 1 << 11;  // Extend hit box to the right-most edge; even if not framed. This is not the default in order to allow adding other items on the same line. In the future we may refactor the hit system to be front-to-back; allowing natural overlaps and then this can become the default.
    pub const ImGuiTreeNodeFlags_SpanFullWidth: ImGuiTreeNodeFlags = 1 << 12;  // Extend hit box to the left-most and right-most edges (bypass the indented area).
    pub const ImGuiTreeNodeFlags_NavLeftJumpsBackHere: ImGuiTreeNodeFlags = 1 << 13;  // (WIP) Nav: left direction may move to this TreeNode() from any of its child (items submitted between TreeNode and TreePop)
    //pub const ImGuiTreeNodeFlags_NoScrollOnOpen: ImGuiTreeNodeFlags = 1 << 14;  // FIXME: TODO: Disable automatic scroll on TreePop() if node got just open and contents is not visible
    pub const ImGuiTreeNodeFlags_CollapsingHeader: ImGuiTreeNodeFlags = ImGuiTreeNodeFlags_Framed | ImGuiTreeNodeFlags_NoTreePushOnOpen | ImGuiTreeNodeFlags_NoAutoOpenOnLog;
