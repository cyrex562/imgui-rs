use crate::item_flags::ImGuiItemFlags_ButtonRepeat;

// c_void PushButtonRepeat(repeat: bool)
pub fn PushButtonRepeat(repeat: bool)
{
    PushItemFlag(ImGuiItemFlags_ButtonRepeat, repeat);
}

// c_void PopButtonRepeat()
pub fn PopButtonRepeat()
{
    PopItemFlag();
}
