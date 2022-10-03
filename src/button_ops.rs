use crate::item_flags::ImGuiItemFlags_ButtonRepeat;

// c_void PushButtonRepeat(bool repeat)
pub fn PushButtonRepeat(repeat: bool)
{
    PushItemFlag(ImGuiItemFlags_ButtonRepeat, repeat);
}

// c_void PopButtonRepeat()
pub fn PopButtonRepeat()
{
    PopItemFlag();
}
