#![allow(non_snake_case)]

use libc::{c_float, c_int, c_uint};
use crate::input_event_type::{ImGuiInputEventType};
use crate::input_source::ImGuiInputSource;
use crate::key::ImGuiKey;
use crate::type_defs::ImGuiID;

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


// FIXME: Structures in the union below need to be declared as anonymous unions appears to be an extension?
// Using ImVec2::new() would fail on Clang 'union member 'MousePos' has a non-trivial default constructor'
#[derive(Default,Debug,Clone)]
pub struct ImGuiInputEventMousePos      {
    pub PosX: c_float,
    pub PosY: c_float }

#[derive(Default,Debug,Clone)]
pub struct ImGuiInputEventMouseWheel    {
    // WheelX: c_float, WheelY;
    pub WheelX: c_float,
    pub WheelY: c_float
}

#[derive(Default,Debug,Clone)]
pub struct ImGuiInputEventMouseButton   {
    pub Button: c_int,
    pub Down: bool }

#[derive(Default,Debug,Clone)]
pub struct ImGuiInputEventMouseViewport {
    // ImGuiID pub(crate) HoveredViewportID;
    pub HoveredViewportID: ImGuiID,
}

#[derive(Default,Debug,Clone)]
struct ImGuiInputEventKey           {
    // ImGuiKey pub(crate) Key;
    pub Key: ImGuiKey,
    // bool pub(crate) Down;
    pub Down: Down,
    // let mut AnalogValue: c_float = 0f32;
}

#[derive(Default,Debug,Clone)]
struct ImGuiInputEventText          {
    // c_uint Char;
    pub Char: c_uint,
}

#[derive(Default,Debug,Clone)]
struct ImGuiInputEventAppFocused    {
    // bool Focused;
    pub Focused: bool,
};
