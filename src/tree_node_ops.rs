// inline bool     TreeNodeBehaviorIsOpen(ImGuiID id, ImGuiTreeNodeFlags flags = 0)    
pub fn TreeNodeBehaviorIsOpen(id: ImGuiID, flags: ImGuiTreeNodeFlags) -> bool {
    return TreeNodeUpdateNextOpen(id, flags);
}   // Renamed in 1.89