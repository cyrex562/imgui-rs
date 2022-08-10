use crate::Context;
use crate::types::Id32;

#[derive(Default,Debug,Clone)]
struct StackLevelInfo
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
    pub results: Vec<StackLevelInfo>,
    // bool                    CopyToClipboardOnCtrlC;
    pub copy_to_clopboard_on_ctrl_c: bool,
    // float                   CopyToClipboardLastTime;
    pub copy_to_clipboard_last_time: f32,
    // ImGuiStackTool()        { memset(this, 0, sizeof(*this)); CopyToClipboardLastTime = -FLT_MAX; }
}

#[derive(Debug,Default,Clone)]
pub struct  StackSizes
{
    // short   SizeOfIDStack;
    pub id_stack_size: usize,
    // short   size_of_color_stack;
    pub color_stack_size: usize,
    // short   size_of_style_var_stack;
    pub style_var_stack_size: usize,
    // short   size_of_font_stack;
    pub font_stack_size: usize,
    // short   size_of_focus_scope_stack;
    pub focus_scope_stack_size: usize,
    // short   size_of_group_stack;
    pub group_stack_size: usize,
    // short   size_of_item_flags_stack;
    pub item_flags_stack_size: usize,
    // short   size_ofbegin_popup_stack;
    pub begin_popup_stack_size: usize,
    // short   size_of_disabled_stack;
    pub disabled_stack_size: usize,
}

impl StackSizes {
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
//     size_of_color_stack = g.color_stack.size;
//     size_of_style_var_stack = g.style_var_stack.size;
//     size_of_font_stack = g.font_stack.size;
//     size_of_focus_scope_stack = g.FocusScopeStack.size;
//     size_of_group_stack = g.group_stack.size;
//     size_of_item_flags_stack = g.item_flags_stack.size;
//     size_ofbegin_popup_stack = g.begin_popup_stack.size;
//     size_of_disabled_stack = g.DisabledStackSize;
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
//     IM_ASSERT(size_of_group_stack      == g.group_stack.size        && "BeginGroup/EndGroup Mismatch!");
//     IM_ASSERT(size_ofbegin_popup_stack == g.begin_popup_stack.size   && "begin_popup/EndPopup or BeginMenu/EndMenu Mismatch!");
//     IM_ASSERT(size_of_disabled_stack   == g.DisabledStackSize      && "BeginDisabled/EndDisabled Mismatch!");
//     IM_ASSERT(size_of_item_flags_stack  >= g.item_flags_stack.size    && "push_item_flag/PopItemFlag Mismatch!");
//     IM_ASSERT(size_of_color_stack      >= g.color_stack.size        && "PushStyleColor/PopStyleColor Mismatch!");
//     IM_ASSERT(size_of_style_var_stack   >= g.style_var_stack.size     && "PushStyleVar/PopStyleVar Mismatch!");
//     IM_ASSERT(size_of_font_stack       >= g.font_stack.size         && "PushFont/PopFont Mismatch!");
//     IM_ASSERT(size_of_focus_scope_stack == g.FocusScopeStack.size   && "PushFocusScope/PopFocusScope Mismatch!");
// }


    //     void SetToCurrentState();
    pub fn set_to_current_state(&mut self, g: &mut Context) {
        let window = g.current_window_mut();
        self.id_stack_size = window.id_stack.len();
        self.color_stack_size = g.color_stack.len();
        self.style_var_stack_size = g.style_var_stack.len();
        self.font_stack_size = g.font_stack.len();
        self.focus_scope_stack_size = g.focus_scope_stack.len();
        self.group_stack_size = g.group_stack.len();
        self.item_flags_stack_size = g.item_flags_stack.len();
        self.begin_popup_stack_size = g.begin_popup_stack.len();
        self.disabled_stack_size = g.disabled_stack_size;
    }
    //     void CompareWithCurrentState();
    pub fn compare_with_current_state(&self) {
        todo!();
    }
}
