#![allow(non_snake_case)]

use crate::imgui_cpp::GImGui;

impl ImGuiIO {
    // ImGuiIO::ImGuiIO()
    pub fn new() -> Self {
        // Most fields are initialized with zero
        // memset(this, 0, sizeof(*this));
        let mut out = Self { ..Default::default() };
        // IM_STATIC_ASSERT(IM_ARRAYSIZE(ImGuiIO::MouseDown) == ImGuiMouseButton_COUNT && IM_ARRAYSIZE(ImGuiIO::MouseClicked) == ImGuiMouseButton_COUNT);

        // Settings
        out.ConfigFlags = ImGuiConfigFlags_None;
        out.BackendFlags = ImGuiBackendFlags_None;
        out.DisplaySize = ImVec2(-1f32, -1f32);
        out.DeltaTime = 1f32 / 60f32;
        out.IniSavingRate = 5f32;
        out.IniFilename = "imgui.ini"; // Important: "imgui.ini" is relative to current working dir, most apps will want to lock this to an absolute path (e.g. same path as executables).
        out.LogFilename = "imgui_log.txt";
        out.MouseDoubleClickTime = 0.3f32;
        out.MouseDoubleClickMaxDist = 6f32;
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//         for (int i = 0; i < ImGuiKey_COUNT; i+ +)
        for i in 0 .. ImGuiKey_COUNT
        {
            out.KeyMap[i] = -1;
        }
// #endif
        out.KeyRepeatDelay = 0.275f32;
        out.KeyRepeatRate = 0.05f32;
        out.HoverDelayNormal = 0.3f32;
        out.HoverDelayShort = 0.1f32;
        out.UserData = None;

        out.Fonts = None;
        out.FontGlobalScale = 1f32;
        out.FontDefault = None;
        out.FontAllowUserScaling = false;
        out.DisplayFramebufferScale = ImVec2(1f32, 1f32);

        // Docking options (when ImGuiConfigFlags_DockingEnable is set)
        out.ConfigDockingNoSplit = false;
        out.ConfigDockingWithShift = false;
        out.ConfigDockingAlwaysTabBar = false;
        out.ConfigDockingTransparentPayload = false;

        // Viewport options (when ImGuiConfigFlags_ViewportsEnable is set)
        out.ConfigViewportsNoAutoMerge = false;
        out.ConfigViewportsNoTaskBarIcon = false;
        out.ConfigViewportsNoDecoration = true;
        out.ConfigViewportsNoDefaultParent = false;

        // Miscellaneous options
        out.MouseDrawCursor = false;
// #ifdef __APPLE__
        out.ConfigMacOSXBehaviors = true;  // Set Mac OS X style defaults based on __APPLE__ compile time flag
// #else
        out.ConfigMacOSXBehaviors = false;
// #endif
        out.ConfigInputTrickleEventQueue = true;
        out.ConfigInputTextCursorBlink = true;
        out.ConfigInputTextEnterKeepActive = false;
        out.ConfigDragClickToInputText = false;
        out.ConfigWindowsResizeFromEdges = true;
        out.ConfigWindowsMoveFromTitleBarOnly = false;
        out.ConfigMemoryCompactTimer = 60f32;

        // Platform Functions
        out.BackendPlatformName = BackendRendererName = None;
        out.BackendPlatformUserData = BackendRendererUserData = BackendLanguageUserData = None;
        out.GetClipboardTextFn = GetClipboardTextFn_DefaultImpl;   // Platform dependent default implementations
        out.SetClipboardTextFn = SetClipboardTextFn_DefaultImpl;
        out.ClipboardUserData = None;
        out.SetPlatformImeDataFn = SetPlatformImeDataFn_DefaultImpl;

        // Input (NB: we already have memset zero the entire structure!)
        out.MousePos = ImVec2(-f32::MAX, -f32::MAX);
        out.MousePosPrev = ImVec2(-f32::MAX, -f32::MAX);
        out.MouseDragThreshold = 6f32;
        // for (int i = 0; i < IM_ARRAYSIZE(MouseDownDuration); i+ +)
        for i in 0 .. out.MouseDownDuration.len()
        {
            out.MouseDownDuration[i] = -1f32;
            out.MouseDownDurationPrev[i] = -1f32;
        }
        // for (int i = 0; i < IM_ARRAYSIZE(KeysData); i+ +)
        for i in 0 .. out.KeysData.len()
        {
            out.KeysData[i].DownDuration = -1f32;
            out.KeysData[i].DownDurationPrev = -1f32;
        }
        out.AppAcceptingEvents = true;
        out.BackendUsingLegacyKeyArrays = (ImS8) - 1;
        out.BackendUsingLegacyNavInputArray = true; // assume using legacy array until proven wrong
        out
    }

    // Pass in translated ASCII characters for text input.
    // - with glfw you can get those from the callback set in glfwSetCharCallback()
    // - on Windows you can get those using ToAscii+keyboard state, or via the WM_CHAR message
    // FIXME: Should in theory be called "AddCharacterEvent()" to be consistent with new API
    // void ImGuiIO::AddInputCharacter(unsigned int c)
    pub fn AddInputCharacter(&mut self, c: u32)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.IO == this && "Can only add events to current context.");
        if c == 0 || !self.AppAcceptingEvents {
            return;
        }

        // ImGuiInputEvent e;
        let mut e: ImGuiInputEvent = ImGuiInputEvent::new();
        e.Type = ImGuiInputEventType_Text;
        e.Source = ImGuiInputSource_Keyboard;
        e.Text.Char = c;
        g.InputEventsQueue.push(e);
    }

    // UTF16 strings use surrogate pairs to encode codepoints >= 0x10000, so
    // we should save the high surrogate.
    // void ImGuiIO::AddInputCharacterUTF16(ImWchar16 c)
    pub fn AddInputCharacterUTF16(&mut self, c: ImWchar16)
    {
        if (c == 0 && self.InputQueueSurrogate == 0) || !self.AppAcceptingEvents {
            return;
        }

        if (c & 0xFC00) == 0xD800 // High surrogate, must save
        {
            if self.InputQueueSurrogate != 0 {
                self.AddInputCharacter(IM_UNICODE_CODEPOINT_INVALID);
            }
            self.InputQueueSurrogate = c;
            return;
        }

        // ImWchar cp = c;
        let cp: ImWchar = c;
        if self.InputQueueSurrogate != 0
        {
            if (c & 0xFC00) != 0xDC00 // Invalid low surrogate
            {
                self.AddInputCharacter(IM_UNICODE_CODEPOINT_INVALID);
            }
            else
            {
    // #if IM_UNICODE_CODEPOINT_MAX == 0xFFFF
                cp = IM_UNICODE_CODEPOINT_INVALID; // Codepoint will not fit in ImWchar
    // #else
                cp = (((self.InputQueueSurrogate - 0xD800) << 10) + (c - 0xDC00) + 0x10000);
    // #endif
            }

            self.InputQueueSurrogate = 0;
        }
        self.AddInputCharacter(cp);
    }

    // void ImGuiIO::AddInputCharactersUTF8(const char* utf8_chars)
    pub unsafe fn AddInputCharactersUTF8(&mut self, mut utf8_chars: *char)
    {
        if !self.AppAcceptingEvents {
            return;
        }
        while *utf8_chars != 0
        {
            // unsigned int c = 0;
            let mut c: u32 = 0;
            utf8_chars += ImTextCharFromUtf8(&c, utf8_chars, NULL);
            if c != 0 {
                self.AddInputCharacter(c);
            }
        }
    }

    // void ImGuiIO::ClearInputCharacters()
    pub fn ClearInputCharacters(&mut self)
    {
        // InputQueueCharacters.resize(0);
        self.InputQueueCharacters.clear();
    }

    // void ImGuiIO::ClearInputKeys()
    pub fn ClearInputKeys(&mut self)
    {
    // #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    //     memset(KeysDown, 0, sizeof(KeysDown));
        self.KeysDown.clear();
    // #endif
    //     for (int n = 0; n < IM_ARRAYSIZE(KeysData); n++)
       for n in 0 .. self.KeysData.len()
        {

            self.KeysData[n].Down             = false;
            self.KeysData[n].DownDuration     = -1f32;
            self.KeysData[n].DownDurationPrev = -1f32;
        }
        self.KeyCtrl = false;
        self.KeyShift = false;
        self.KeyAlt = false;
        self.KeySuper = false;
        self.KeyMods = ImGuiModFlags_None;
    }



    // Queue a new key down/up event.
    // - ImGuiKey key:       Translated key (as in, generally ImGuiKey_A matches the key end-user would use to emit an 'A' character)
    // - bool down:          Is the key down? use false to signify a key release.
    // - float analog_value: 0.0f32..1.0f
    void ImGuiIO::AddKeyAnalogEvent(ImGuiKey key, bool down, float analog_value)
    {
        //if (e->Down) { IMGUI_DEBUG_LOG_IO("AddKeyEvent() Key='%s' %d, NativeKeycode = %d, NativeScancode = %d\n", ImGui::GetKeyName(e->Key), e->Down, e->NativeKeycode, e->NativeScancode); }
        if (key == ImGuiKey_None || !AppAcceptingEvents)
            return;
        let g = GImGui; // ImGuiContext& g = *GImGui;
        IM_ASSERT(&g.IO == this && "Can only add events to current context.");
        IM_ASSERT(ImGui::IsNamedKey(key)); // Backend needs to pass a valid ImGuiKey_ constant. 0..511 values are legacy native key codes which are not accepted by this API.
        IM_ASSERT(!ImGui::IsAliasKey(key)); // Backend cannot submit ImGuiKey_MouseXXX values they are automatically inferred from AddMouseXXX() events.

        // Verify that backend isn't mixing up using new io.AddKeyEvent() api and old io.KeysDown[] + io.KeyMap[] data.
    // #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
        IM_ASSERT((BackendUsingLegacyKeyArrays == -1 || BackendUsingLegacyKeyArrays == 0) && "Backend needs to either only use io.AddKeyEvent(), either only fill legacy io.KeysDown[] + io.KeyMap[]. Not both!");
        if (BackendUsingLegacyKeyArrays == -1)
            for (int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n++)
                IM_ASSERT(KeyMap[n] == -1 && "Backend needs to either only use io.AddKeyEvent(), either only fill legacy io.KeysDown[] + io.KeyMap[]. Not both!");
        BackendUsingLegacyKeyArrays = 0;
    // #endif
        if (ImGui::IsGamepadKey(key))
            BackendUsingLegacyNavInputArray = false;

        // Partial filter of duplicates (not strictly needed, but makes data neater in particular for key mods and gamepad values which are most commonly spmamed)
        ImGuiKeyData* key_data = ImGui::GetKeyData(key);
        if (key_data->Down == down && key_data->AnalogValue == analog_value)
        {
            bool found = false;
            for (int n = g.InputEventsQueue.Size - 1; n >= 0 && !found; n--)
                if (g.InputEventsQueue[n].Type == ImGuiInputEventType_Key && g.InputEventsQueue[n].Key.Key == key)
                    found = true;
            if (!found)
                return;
        }

        // Add event
        ImGuiInputEvent e;
        e.Type = ImGuiInputEventType_Key;
        e.Source = ImGui::IsGamepadKey(key) ? ImGuiInputSource_Gamepad : ImGuiInputSource_Keyboard;
        e.Key.Key = key;
        e.Key.Down = down;
        e.Key.AnalogValue = analog_value;
        g.InputEventsQueue.push(e);
    }

    void ImGuiIO::AddKeyEvent(ImGuiKey key, bool down)
    {
        if (!AppAcceptingEvents)
            return;
        AddKeyAnalogEvent(key, down, down ? 1f32 : 0.0f32);
    }

    // [Optional] Call after AddKeyEvent().
    // Specify native keycode, scancode + Specify index for legacy <1.87 IsKeyXXX() functions with native indices.
    // If you are writing a backend in 2022 or don't use IsKeyXXX() with native values that are not ImGuiKey values, you can avoid calling this.
    void ImGuiIO::SetKeyEventNativeData(ImGuiKey key, int native_keycode, int native_scancode, int native_legacy_index)
    {
        if (key == ImGuiKey_None)
            return;
        IM_ASSERT(ImGui::IsNamedKey(key)); // >= 512
        IM_ASSERT(native_legacy_index == -1 || ImGui::IsLegacyKey(native_legacy_index)); // >= 0 && <= 511
        IM_UNUSED(native_keycode);  // Yet unused
        IM_UNUSED(native_scancode); // Yet unused

        // Build native->imgui map so old user code can still call key functions with native 0..511 values.
    // #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
        const int legacy_key = (native_legacy_index != -1) ? native_legacy_index : native_keycode;
        if (!ImGui::IsLegacyKey(legacy_key))
            return;
        KeyMap[legacy_key] = key;
        KeyMap[key] = legacy_key;
    // #else
        IM_UNUSED(key);
        IM_UNUSED(native_legacy_index);
    // #endif
    }

    // Set master flag for accepting key/mouse/text events (default to true). Useful if you have native dialog boxes that are interrupting your application loop/refresh, and you want to disable events being queued while your app is frozen.
    void ImGuiIO::SetAppAcceptingEvents(bool accepting_events)
    {
        AppAcceptingEvents = accepting_events;
    }

    // Queue a mouse move event
    void ImGuiIO::AddMousePosEvent(float x, float y)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        IM_ASSERT(&g.IO == this && "Can only add events to current context.");
        if (!AppAcceptingEvents)
            return;

        ImGuiInputEvent e;
        e.Type = ImGuiInputEventType_MousePos;
        e.Source = ImGuiInputSource_Mouse;
        e.MousePos.PosX = x;
        e.MousePos.PosY = y;
        g.InputEventsQueue.push(e);
    }

    void ImGuiIO::AddMouseButtonEvent(int mouse_button, bool down)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        IM_ASSERT(&g.IO == this && "Can only add events to current context.");
        IM_ASSERT(mouse_button >= 0 && mouse_button < ImGuiMouseButton_COUNT);
        if (!AppAcceptingEvents)
            return;

        ImGuiInputEvent e;
        e.Type = ImGuiInputEventType_MouseButton;
        e.Source = ImGuiInputSource_Mouse;
        e.MouseButton.Button = mouse_button;
        e.MouseButton.Down = down;
        g.InputEventsQueue.push(e);
    }

    // Queue a mouse wheel event (most mouse/API will only have a Y component)
    void ImGuiIO::AddMouseWheelEvent(float wheel_x, float wheel_y)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        IM_ASSERT(&g.IO == this && "Can only add events to current context.");
        if ((wheel_x == 0.0f32 && wheel_y == 0.0f32) || !AppAcceptingEvents)
            return;

        ImGuiInputEvent e;
        e.Type = ImGuiInputEventType_MouseWheel;
        e.Source = ImGuiInputSource_Mouse;
        e.MouseWheel.WheelX = wheel_x;
        e.MouseWheel.WheelY = wheel_y;
        g.InputEventsQueue.push(e);
    }

    void ImGuiIO::AddMouseViewportEvent(ImGuiID viewport_id)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        IM_ASSERT(&g.IO == this && "Can only add events to current context.");
        IM_ASSERT(g.IO.BackendFlags & ImGuiBackendFlags_HasMouseHoveredViewport);

        ImGuiInputEvent e;
        e.Type = ImGuiInputEventType_MouseViewport;
        e.Source = ImGuiInputSource_Mouse;
        e.MouseViewport.HoveredViewportID = viewport_id;
        g.InputEventsQueue.push(e);
    }

    void ImGuiIO::AddFocusEvent(bool focused)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        IM_ASSERT(&g.IO == this && "Can only add events to current context.");

        ImGuiInputEvent e;
        e.Type = ImGuiInputEventType_Focus;
        e.AppFocused.Focused = focused;
        g.InputEventsQueue.push(e);
    }
}
