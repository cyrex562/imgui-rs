use crate::types::Id32;

#[derive(Default,Debug,Clone)]
struct ImGuiStackLevelInfo
{
    //Id32                 id;
    pub id: Id32,
    //ImS8                    QueryFrameCount;            // >= 1: Query in progress
    pub query_frame_count: i8,
    // bool                    QuerySuccess;               // Obtained result from debug_hook_id_info()
    pub query_success: bool,
    // DataType           DataType : 8;
    pub data_type: DimgDataType,
    // char                    Desc[57];                   // Arbitrarily sized buffer to hold a result (FIXME: could replace Results[] with a chunk stream?) FIXME: Now that we added CTRL+C this should be fixed.
    pub desc: String,
    // ImGuiStackLevelInfo()   { memset(this, 0, sizeof(*this)); }
}

// state for Stack tool queries
#[derive(Default,Debug,Clone)]
pub struct StackTool
{
    // int                     LastActiveFrame;
    pub last_active_frame: i32,
    // int                     StackLevel;                 // -1: query stack and resize Results, >= 0: individual stack level
    pub stack_level: i32,
    // Id32                 QueryId;                    // id to query details for
    pub query_id: Id32,
    // ImVector<ImGuiStackLevelInfo> Results;
    pub results: Vec<DimgStackLevelInfo>,
    // bool                    CopyToClipboardOnCtrlC;
    pub copy_to_clopboard_on_ctrl_c: bool,
    // float                   CopyToClipboardLastTime;
    pub copy_to_clipboard_last_time: f32,
    // ImGuiStackTool()        { memset(this, 0, sizeof(*this)); CopyToClipboardLastTime = -FLT_MAX; }
}

#[derive(Debug,Default,Clone)]
pub struct  ImGuiStackSizes
{
    // short   SizeOfIDStack;
    pub SizeofIDStack: i16,
    // short   SizeOfColorStack;
    pub SizeOfColorStack: i16,
    // short   SizeOfStyleVarStack;
    pub SizeOfStyleVarStack: i16,
    // short   SizeOfFontStack;
    pub SizeOfFontStack: i16,
    // short   SizeOfFocusScopeStack;
    pub SizeOfFocusScopeStack: i16,
    // short   SizeOfGroupStack;
    pub SizeOfGroupStack: i16,
    // short   SizeOfItemFlagsStack;
    pub SizeOfItemFlagsStack: i16,
    // short   SizeOfbegin_popupStack;
    pub SizeOfbegin_popupStack: i16,
    // short   SizeOfDisabledStack;
    pub SizeOfDisabledStack: i16,
}

impl ImGuiStackSizes {
    // ImGuiStackSizes() { memset(this, 0, sizeof(*this)); }
    pub fn new()-> Self {
        Self {
            ..Default::default()
        }
    }

    //
// Save current stack sizes for later compare
// void ImGuiStackSizes::SetToCurrentState()
// {
//     ImGuiContext& g = *GImGui;
//     Window* window = g.current_window;
//     SizeOfIDStack = window.IDStack.size;
//     SizeOfColorStack = g.color_stack.size;
//     SizeOfStyleVarStack = g.style_var_stack.size;
//     SizeOfFontStack = g.font_stack.size;
//     SizeOfFocusScopeStack = g.FocusScopeStack.size;
//     SizeOfGroupStack = g.group_stack.size;
//     SizeOfItemFlagsStack = g.item_flags_stack.size;
//     SizeOfbegin_popupStack = g.begin_popup_stack.size;
//     SizeOfDisabledStack = g.DisabledStackSize;
// }

// Compare to detect usage errors
// void ImGuiStackSizes::CompareWithCurrentState()
// {
//     ImGuiContext& g = *GImGui;
//     Window* window = g.current_window;
//     IM_UNUSED(window);
//
//     // window stacks
//     // NOT checking: dc.item_width, dc.text_wrap_pos (per window) to allow user to conveniently push once and not pop (they are cleared on Begin)
//     IM_ASSERT(SizeOfIDStack         == window.IDStack.size     && "push_id/PopID or TreeNode/TreePop Mismatch!");
//
//     // Global stacks
//     // For color, style and font stacks there is an incentive to use Push/Begin/Pop/.../End patterns, so we relax our checks a little to allow them.
//     IM_ASSERT(SizeOfGroupStack      == g.group_stack.size        && "BeginGroup/EndGroup Mismatch!");
//     IM_ASSERT(SizeOfbegin_popupStack == g.begin_popup_stack.size   && "begin_popup/EndPopup or BeginMenu/EndMenu Mismatch!");
//     IM_ASSERT(SizeOfDisabledStack   == g.DisabledStackSize      && "BeginDisabled/EndDisabled Mismatch!");
//     IM_ASSERT(SizeOfItemFlagsStack  >= g.item_flags_stack.size    && "push_item_flag/PopItemFlag Mismatch!");
//     IM_ASSERT(SizeOfColorStack      >= g.color_stack.size        && "PushStyleColor/PopStyleColor Mismatch!");
//     IM_ASSERT(SizeOfStyleVarStack   >= g.style_var_stack.size     && "PushStyleVar/PopStyleVar Mismatch!");
//     IM_ASSERT(SizeOfFontStack       >= g.font_stack.size         && "PushFont/PopFont Mismatch!");
//     IM_ASSERT(SizeOfFocusScopeStack == g.FocusScopeStack.size   && "PushFocusScope/PopFocusScope Mismatch!");
// }


    //     void SetToCurrentState();
    pub fn SetToCurrentState(&mut self) {
        todo!()
    }
    //     void CompareWithCurrentState();
    pub fn CompareWithCurrentState(&self) {

    }
}
