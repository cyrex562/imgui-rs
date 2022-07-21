use std::collections::HashSet;
use crate::Context;
use crate::item::{ItemFlags, pop_item_flag, push_item_flag};

pub enum ButtonFlags
{
    PressedOnClick         = 1 << 4,   // return true on click (mouse down event)
    PressedOnClickRelease  = 1 << 5,   // [Default] return true on click + release on same item <-- this is what the majority of Button are using
    PressedOnClickReleaseAnywhere = 1 << 6, // return true on click + release even if the release event is not done while hovering the item
    PressedOnRelease       = 1 << 7,   // return true on release (default requires click+release)
    PressedOnDoubleClick   = 1 << 8,   // return true on double-click (default requires click+release)
    PressedOnDragDropHold  = 1 << 9,   // return true when held into while we are drag and dropping another item (used by e.g. tree nodes, collapsing headers)
    Repeat                 = 1 << 10,  // hold to repeat
    FlattenChildren        = 1 << 11,  // allow interactions even if a child window is overlapping
    AllowItemOverlap       = 1 << 12,  // require previous frame hovered_id to either match id or be null before being usable, use along with SetItemAllowOverlap()
    DontClosePopups        = 1 << 13,  // disable automatically closing parent popup on press // [UNUSED]
    //Disabled             = 1 << 14,  // disable interactions -> use BeginDisabled() or ImGuiItemFlags_Disabled
    AlignTextBaseLine      = 1 << 15,  // vertically align button to match text baseline - ButtonEx() only // FIXME: Should be removed and handled by SmallButton(), not possible currently because of dc.CursorPosPrevLine
    NoKeyModifiers         = 1 << 16,  // disable mouse interaction if a key modifier is held
    NoHoldingActiveId      = 1 << 17,  // don't set active_id while holding the mouse (PressedOnClick only)
    NoNavFocus             = 1 << 18,  // don't override navigation focus when activated
    NoHoveredOnFocus       = 1 << 19,  // don't report as hovered when nav focus is on this item
None                   = 0,
    MouseButtonLeft        = 1 << 0,   // React on left mouse button (default)
    MouseButtonRight       = 1 << 1,   // React on right mouse button
    MouseButtonMiddle      = 1 << 2,   // React on center mouse button
}



// pub const MouseButtonMask_: i32       = ButtonFlags::MouseButtonLeft | ButtonFlags::MouseButtonRight | ButtonFlags::MouseButtonMiddle;
pub const MOUSE_BTN_MASK: HashSet<ButtonFlags> = HashSet::from([
   ButtonFlags::MouseButtonLeft, ButtonFlags::MouseButtonRight, ButtonFlags::MouseButtonMiddle
]);

pub const MOUSE_BTN_DFLT: ButtonFlags = ButtonFlags::MouseButtonLeft;

// void ImGui::PushButtonRepeat(bool repeat)
pub fn push_button_repeat(g: &mut Context, repeat: bool)
{
    push_item_flag(g, &ItemFlags::ButtonRepeat, repeat);
}

// void ImGui::PopButtonRepeat()
pub fn pop_button_repeat(g: &mut Context)
{
    pop_item_flag(g);
}
