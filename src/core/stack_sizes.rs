#![allow(non_snake_case)]

use crate::GImGui;
use libc::c_short;

#[derive(Default, Debug, Clone)]
pub struct ImGuiStackSizes {
    pub SizeOfIDStack: c_short,
    pub SizeOfColorStack: c_short,
    pub SizeOfStyleVarStack: c_short,
    pub SizeOfFontStack: c_short,
    pub SizeOfFocusScopeStack: c_short,
    pub SizeOfGroupStack: c_short,
    pub SizeOfItemFlagsStack: c_short,
    pub SizeOfBeginPopupStack: c_short,
    pub SizeOfDisabledStack: c_short,
}

impl ImGuiStackSizes {
    // ImGuiStackSizes() { memset(this, 0, sizeof(*this)); }

    // c_void SetToCurrentState();
    pub unsafe fn SetToCurrentState(&mut self) {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        let mut window = g.current_window_mut().unwrap();
        self.SizeOfIDStack = window.id_stack.Size;
        self.SizeOfColorStack = g.ColorStack.Size;
        self.SizeOfStyleVarStack = g.styleVarStack.Size;
        self.SizeOfFontStack = g.FontStack.Size;
        self.SizeOfFocusScopeStack = g.FocusScopeStack.Size;
        self.SizeOfGroupStack = g.GroupStack.Size;
        self.SizeOfItemFlagsStack = g.ItemFlagsStack.Size;
        self.SizeOfBeginPopupStack = g.BeginPopupStack.len();
        self.SizeOfDisabledStack = g.DisabledStackSize;
    }

    // c_void CompareWithCurrentState();
    pub unsafe fn CompareWithCurrentState(&mut self) {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        let mut window = g.current_window_mut().unwrap();
        // IM_UNUSED(window);

        // Window stacks
        // NOT checking: DC.ItemWidth, DC.TextWrapPos (per window) to allow user to conveniently push once and not pop (they are cleared on Begin)
        // IM_ASSERT(SizeOfIDStack         == window.id_stack.Size     && "PushID/PopID or TreeNode/TreePop Mismatch!");

        // Global stacks
        // For color, style and font stacks there is an incentive to use Push/Begin/Pop/.../End patterns, so we relax our checks a little to allow them.
        // IM_ASSERT(SizeOfGroupStack      == g.GroupStack.Size        && "BeginGroup/EndGroup Mismatch!");
        // IM_ASSERT(SizeOfBeginPopupStack == g.BeginPopupStack.Size   && "BeginPopup/EndPopup or BeginMenu/EndMenu Mismatch!");
        // IM_ASSERT(SizeOfDisabledStack   == g.DisabledStackSize      && "BeginDisabled/EndDisabled Mismatch!");
        // IM_ASSERT(SizeOfItemFlagsStack  >= g.ItemFlagsStack.Size    && "PushItemFlag/PopItemFlag Mismatch!");
        // IM_ASSERT(SizeOfColorStack      >= g.ColorStack.Size        && "PushStyleColor/PopStyleColor Mismatch!");
        // IM_ASSERT(SizeOfStyleVarStack   >= g.styleVarStack.Size     && "PushStyleVar/PopStyleVar Mismatch!");
        // IM_ASSERT(SizeOfFontStack       >= g.FontStack.Size         && "PushFont/PopFont Mismatch!");
        // IM_ASSERT(SizeOfFocusScopeStack == g.FocusScopeStack.Size   && "PushFocusScope/PopFocusScope Mismatch!");
    }
}
