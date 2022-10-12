use libc::c_int;

// typedef int ImGuiTreeNodeFlags;     // -> enum ImGuiTreeNodeFlags_   // Flags: for TreeNode(), TreeNodeEx(), CollapsingHeader()
pub type ImGuiTreeNodeFlags = c_int;
