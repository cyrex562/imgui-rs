#![allow(non_snake_case)]

#[derive(Default,Debug,Clone)]
pub union ImGuiInputEventVal {
    // ImGuiInputEventMousePos     mouse_pos;       // if Type == ImGuiInputEventType_MousePos
    pub MousePos: ImGuiInputEventMousePos,
    //     ImGuiInputEventMouseWheel   mouse_wheel;     // if Type == ImGuiInputEventType_MouseWheel
    pub MouseWheel: ImGuiInputEventMouseWheel,
    //     ImGuiInputEventMouseButton  MouseButton;    // if Type == ImGuiInputEventType_MouseButton
    pub MouseButton: ImGuiInputEventMouseButton,
    //     ImGuiInputEventMouseViewport mouse_viewport; // if Type == ImGuiInputEventType_MouseViewport
    pub MouseViewport: ImGuiInputEventMouseViewport,
    //     ImGuiInputEventKey          Key;            // if Type == ImGuiInputEventType_Key
    pub Key: ImGuiInputEventKey,
    //     ImGuiInputEventText         Text;           // if Type == ImGuiInputEventType_Text
    pub Text: ImGuiInputEventText,
    //     ImGuiInputEventAppFocused   AppFocused;     // if Type == ImGuiInputEventType_Focus
    pub AppFocused: ImGuiInputEventAppFocused,
}

impl ImGuiInputEventVal {
    pub fn new() -> Self {
        Self {
            ..Default()
        }
    }
}

#[derive(Default,Debug,Clone)]
pub struct DimgInputEvent
{
    // ImGuiInputEventType             Type;
    pub input_event_type: ImGuiInputEventType,
    // ImGuiInputSource                Source;
    pub source: ImGuiInputSource,
    pub val: ImGuiInputEventVal,
    // bool                            AddedByTestEngine;
    pub added_byt_test_engine: bool,

    // ImGuiInputEvent() { memset(this, 0, sizeof(*this)); }
}

impl DimgInputEvent {
    pub fn new() -> Self {
        Self {
            ..Default()
        }
    }
}
