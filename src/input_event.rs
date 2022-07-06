#![allow(non_snake_case)]

#[derive(Default,Debug,Clone)]
pub union ImGuiInputEventVal {
    // ImGuiInputEventMousePos     MousePos;       // if Type == ImGuiInputEventType_MousePos
    pub MousePos: ImGuiInputEventMousePos,
    //     ImGuiInputEventMouseWheel   MouseWheel;     // if Type == ImGuiInputEventType_MouseWheel
    pub MouseWheel: ImGuiInputEventMouseWheel,
    //     ImGuiInputEventMouseButton  MouseButton;    // if Type == ImGuiInputEventType_MouseButton
    pub MouseButton: ImGuiInputEventMouseButton,
    //     ImGuiInputEventMouseViewport MouseViewport; // if Type == ImGuiInputEventType_MouseViewport
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
pub struct ImGuiInputEvent
{
    // ImGuiInputEventType             Type;
    pub Type: ImGuiInputEventType,
    // ImGuiInputSource                Source;
    pub Source: ImGuiInputSource,
    pub val: ImGuiInputEventVal,
    // bool                            AddedByTestEngine;
    pub AddedByTestEngine: bool,

    // ImGuiInputEvent() { memset(this, 0, sizeof(*this)); }
}

impl ImGuiInputEvent {
    pub fn new() -> Self {
        Self {
            ..Default()
        }
    }
}
