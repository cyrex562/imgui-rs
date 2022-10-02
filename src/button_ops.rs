
c_void PushButtonRepeat(bool repeat)
{
    PushItemFlag(ImGuiItemFlags_ButtonRepeat, repeat);
}

c_void PopButtonRepeat()
{
    PopItemFlag();
}
