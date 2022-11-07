use crate::item_flags::ImGuiItemFlags_ButtonRepeat;
use crate::item_ops::{PopItemFlag, PushItemFlag};
use crate::window::ImguiWindow;
use libc::c_float;

pub unsafe fn PushButtonRepeat(repeat: bool) {
    PushItemFlag(ImGuiItemFlags_ButtonRepeat, repeat);
}

pub unsafe fn PopButtonRepeat() {
    PopItemFlag();
}

pub unsafe fn PushTextWrapPos(wrap_pos_x: c_float) {
    let mut window: &mut ImguiWindow = GetCurrentWindow();
    window.dc.TextWrapPosStack.push(window.dc.TextWrapPos);
    window.dc.TextWrapPos = wrap_pos_x;
}

pub unsafe fn PopTextWrapPos() {
    let mut window: &mut ImguiWindow = GetCurrentWindow();
    window.dc.TextWrapPos = window.dc.TextWrapPosStack.last().unwrap().clone();
    window.dc.TextWrapPosStack.pop_back();
}
