use libc::c_float;
use crate::item_flags::ImGuiItemFlags_ButtonRepeat;
use crate::item_ops::{PopItemFlag, PushItemFlag};
use crate::window::ImGuiWindow;

pub unsafe fn PushButtonRepeat(repeat: bool)
{
    PushItemFlag(ImGuiItemFlags_ButtonRepeat, repeat);
}

pub unsafe fn PopButtonRepeat()
{
    PopItemFlag();
}

pub unsafe fn PushTextWrapPos(wrap_pos_x: c_float)
{
    let mut window: &mut ImGuiWindow =  GetCurrentWindow();
    window.DC.TextWrapPosStack.push(window.DC.TextWrapPos);
    window.DC.TextWrapPos = wrap_pos_x;
}

pub unsafe fn PopTextWrapPos()
{
    let mut window: &mut ImGuiWindow =  GetCurrentWindow();
    window.DC.TextWrapPos = window.DC.TextWrapPosStack.last().unwrap().clone();
    window.DC.TextWrapPosStack.pop_back();
}
