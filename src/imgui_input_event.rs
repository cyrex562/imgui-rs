#![allow(non_snake_case)]

#[derive(Default, Debug, Clone)]
pub struct ImGuiInputEvent {
    pub Type: ImGuiInputEventType,
    pub Source: ImGuiInputSource,
    pub MousePos: ImGuiInputEventMousePos,
    // if Type == ImGuiInputEventType_MousePos
    pub MouseWheel: ImGuiInputEventMouseWheel,
    // if Type == ImGuiInputEventType_MouseWheel
    pub MouseButton: ImGuiInputEventMouseButton,
    // if Type == ImGuiInputEventType_MouseButton
    pub MouseViewport: ImGuiInputEventMouseViewport,
    // if Type == ImGuiInputEventType_MouseViewport
    pub Key: ImGuiInputEventKey,
    // if Type == ImGuiInputEventType_Key
    pub Text: ImGuiInputEventText,
    // if Type == ImGuiInputEventType_Text
    pub AppFocused: ImGuiInputEventAppFocused,
    // if Type == ImGuiInputEventType_Focus
    pub IgnoredAsSame: bool,
    pub AddedByTestEngine: bool,

// ImGuiInputEvent() { memset(this, 0, sizeof(*this)); }
}
