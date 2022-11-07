#![allow(non_snake_case)]

use std::ptr::{null, null_mut};
use libc::{c_char, c_double, c_float, c_int, c_void};
use crate::backend_flags::ImGuiBackendFlags;
use crate::core::config_flags::{ImguiConfigFlags, ImGuiConfigFlags_None};
use crate::font::ImFont;
use crate::font_atlas::ImFontAtlas;
use crate::imgui::GImGui;
use crate::a_imgui_cpp::{GImGui, ImTextCharFromUtf8};
use input_event::ImguiInputEvent;
use crate::input_ops::{GetKeyData, IsGamepadKey};
use crate::platform_ime_data::ImGuiPlatformImeData;
use crate::core::string_ops::ImTextCharFromUtf8;
use crate::core::type_defs::{ImguiHandle, ImWchar, ImWchar16};
use crate::core::vec2::ImVec2;
use crate::viewport::ImguiViewport;

pub mod input_event;
pub mod input_event_type;
pub mod input_ops;
pub mod input_source;
pub mod io_ops;
pub mod key;
mod key_data;
pub mod keyboard_ops;
pub mod mod_flags;
pub mod mouse_button;
pub mod mouse_cursor;
pub mod mouse_ops;

#[derive(Default, Debug, Clone)]
pub struct ImguiIo {
    //------------------------------------------------------------------
    // Configuration                            // Default value
    //------------------------------------------------------------------
    pub ConfigFlags: ImguiConfigFlags,
    // = 0              // See ImGuiConfigFlags_ enum. Set by user/application. Gamepad/keyboard navigation options, etc.
    pub BackendFlags: ImGuiBackendFlags,
    // = 0              // See ImGuiBackendFlags_ enum. Set by backend (imgui_impl_xxx files or custom backend) to communicate features supported by the backend.
    pub DisplaySize: ImVec2,
    // <unset>          // Main display size, in pixels (generally == GetMainViewport().Size). May change every frame.
    pub DeltaTime: c_float,
    // = 1.0f/60f32     // Time elapsed since last frame, in seconds. May change every frame.
    pub IniSavingRate: c_float,
    // = 5f32           // Minimum time between saving positions/sizes to .ini file, in seconds.
    // const char* IniFilename;                    // = "imgui.ini"    // Path to .ini file (important: default "imgui.ini" is relative to current working dir!). Set NULL to disable automatic .ini loading/saving or if you want to manually call LoadIniSettingsXXX() / SaveIniSettingsXXX() functions.
    pub IniFilename: *const c_char,
    // const char* LogFilename;                    // = "imgui_log.txt"// Path to .log file (default parameter to LogToFile when no file is specified).
    pub LogFilename: *const c_char,
    pub MouseDoubleClickTime: c_double,
    // = 0.3f32          // Time for a double-click, in seconds.
    pub MouseDoubleClickMaxDist: c_float,
    // = 6f32           // Distance threshold to stay in to validate a double-click, in pixels.
    pub MouseDragThreshold: c_float,
    // = 6f32           // Distance threshold before considering we are dragging.
    pub KeyRepeatDelay: c_float,
    // = 0.275f         // When holding a key/button, time before it starts repeating, in seconds (for buttons in Repeat mode, etc.).
    pub KeyRepeatRate: c_float,
    // = 0.05f32         // When holding a key/button, rate at which it repeats, in seconds.
    pub HoverDelayNormal: c_float,
    // = 0.30 sec       // Delay on hovering before IsItemHovered(ImGuiHoveredFlags_DelayNormal) returns true.
    pub HoverDelayShort: c_float,
    // = 0.10 sec       // Delay on hovering before IsItemHovered(ImGuiHoveredFlags_DelayShort) returns true.
    // void*       UserData;                       // = NULL           // Store your own data for retrieval by callbacks.
    pub UserData: Vec<u8>,
    pub Fonts: Option<ImFontAtlas>,
    // <auto>           // Font atlas: load, rasterize and pack one or more fonts into a single texture.
    pub FontGlobalScale: c_float,
    // = 1.0           // Global scale all fonts
    pub FontAllowUserScaling: bool,
    // = false          // Allow user scaling text of individual window with CTRL+Wheel.
    // ImFont*     FontDefault;                    // = NULL           // Font to use on NewFrame(). Use NULL to uses Fonts.Fonts[0].
    pub FontDefault: *mut ImFont,
    pub DisplayFramebufferScale: ImVec2,        // = (1, 1)         // For retina display or other situations where window coordinates are different from framebuffer coordinates. This generally ends up in ImDrawData::FramebufferScale.

    // Docking options (when ImGuiConfigFlags_DockingEnable is set)
    pub ConfigDockingNoSplit: bool,
    // = false          // Simplified docking mode: disable window splitting, so docking is limited to merging multiple windows together into tab-bars.
    pub ConfigDockingWithShift: bool,
    // = false          // Enable docking with holding Shift key (reduce visual noise, allows dropping in wider space)
    pub ConfigDockingAlwaysTabBar: bool,
    // = false          // [BETA] [FIXME: This currently creates regression with auto-sizing and general overhead] Make every single floating window display within a docking node.
    pub ConfigDockingTransparentPayload: bool,// = false          // [BETA] Make window or viewport transparent when docking and only display docking boxes on the target viewport. Useful if rendering of multiple viewport cannot be synced. Best used with ConfigViewportsNoAutoMerge.

    // Viewport options (when ImGuiConfigFlags_ViewportsEnable is set)
    pub ConfigViewportsNoAutoMerge: bool,
    // = false;         // Set to make all floating imgui windows always create their own viewport. Otherwise, they are merged into the main host viewports when overlapping it. May also set ImGuiViewportFlags_NoAutoMerge on individual viewport.
    pub ConfigViewportsNoTaskBarIcon: bool,
    // = false          // Disable default OS task bar icon flag for secondary viewports. When a viewport doesn't want a task bar icon, ImGuiViewportFlags_NoTaskBarIcon will be set on it.
    pub ConfigViewportsNoDecoration: bool,
    // = true           // Disable default OS window decoration flag for secondary viewports. When a viewport doesn't want window decorations, ImGuiViewportFlags_NoDecoration will be set on it. Enabling decoration can create subsequent issues at OS levels (e.g. minimum window size).
    pub ConfigViewportsNoDefaultParent: bool, // = false          // Disable default OS parenting to main viewport for secondary viewports. By default, viewports are marked with ParentViewportId = <main_viewport>, expecting the platform backend to setup a parent/child relationship between the OS windows (some backend may ignore this). Set to true if you want the default to be 0, then all viewports will be top-level OS windows.

    // Miscellaneous options
    pub MouseDrawCursor: bool,
    // = false          // Request ImGui to draw a mouse cursor for you (if you are on a platform without a mouse cursor). Cannot be easily renamed to 'io.ConfigXXX' because this is frequently used by backend implementations.
    pub ConfigMacOSXBehaviors: bool,
    // = defined(__APPLE__) // OS X style: Text editing cursor movement using Alt instead of Ctrl, Shortcuts using Cmd/Super instead of Ctrl, Line/Text Start and End using Cmd+Arrows instead of Home/End, Double click selects by word instead of selecting whole text, Multi-selection in lists uses Cmd/Super instead of Ctrl.
    pub ConfigInputTrickleEventQueue: bool,
    // = true           // Enable input queue trickling: some types of events submitted during the same frame (e.g. button down + up) will be spread over multiple frames, improving interactions with low framerates.
    pub ConfigInputTextCursorBlink: bool,
    // = true           // Enable blinking cursor (optional as some users consider it to be distracting).
    pub ConfigInputTextEnterKeepActive: bool,
    // = false          // [BETA] Pressing Enter will keep item active and select contents (single-line only).
    pub ConfigDragClickToInputText: bool,
    // = false          // [BETA] Enable turning DragXXX widgets into text input with a simple mouse click-release (without moving). Not desirable on devices without a keyboard.
    pub ConfigWindowsResizeFromEdges: bool,
    // = true           // Enable resizing of windows from their edges and from the lower-left corner. This requires (io.BackendFlags & IM_GUI_BACKEND_FLAGS_HAS_MOUSE_CURSORS) because it needs mouse cursor feedback. (This used to be a per-window ImGuiWindowFlags_ResizeFromAnySide flag)
    pub ConfigWindowsMoveFromTitleBarOnly: bool,
    // = false       // Enable allowing to move windows only when clicking on their title bar. Does not apply to windows without a title bar.
    pub ConfigMemoryCompactTimer: c_float,       // = 60f32          // Timer (in seconds) to free transient windows/tables memory buffers when unused. Set to -1.0 to disable.

    //------------------------------------------------------------------
    // Platform Functions
    // (the imgui_impl_xxxx backend files are setting those up for you)
    //------------------------------------------------------------------
    // Optional: Platform/Renderer backend name (informational only! will be displayed in
    // About Window) + User data for backend/wrappers to store their own stuff.
    pub BackendPlatformName: *const c_char,
    pub BackendRendererName: *const c_char,
    pub BackendPlatformUserData: *mut c_void,
    // User data for platform backend
    pub BackendRendererUserData: *mut c_void,
     // User data for renderer backend
    pub BackendLanguageUserData: *mut c_void, // User data for non C++ programming
    // language backend
    // Optional: Access OS clipboard
    // (default to use native Win32 clipboard on Windows, otherwise uses a private
    // clipboard. Override to access OS clipboard on other architectures)
    pub GetClipboardTextFn: fn(user_data: *mut c_void) -> *const c_char,
    pub SetClipboardTextFn: fn(user_dat: *mut c_void, text: &String),
    pub ClipboardUserData: *mut c_void,
    // Optional: Notify OS Input Method Editor of the screen position of your cursor for
    // text input position (e.g. when using Japanese/Chinese IME on Windows)
    // (default to use native imm32 api on Windows)
    pub SetPlatformImeDataFn: Option<fn(viewport: &mut ImguiViewport, data: &mut  ImGuiPlatformImeData)>,
    //------------------------------------------------------------------
    // Input - Call before calling NewFrame()
    //------------------------------------------------------------------
    //------------------------------------------------------------------
    // Output - Updated by NewFrame() or EndFrame()/Render()
    // (when reading from the io.WantCaptureMouse, io.WantCaptureKeyboard flags to
    // dispatch your inputs, it is
    //  generally easier and more correct to use their state BEFORE calling NewFrame().
    // See FAQ for details!)
    //------------------------------------------------------------------
    pub WantCaptureMouse: bool,
    // Set when Dear ImGui will use mouse inputs, in this case do not dispatch them to
    // your main game/application (either way, always pass on mouse inputs to imgui).
    // (e.g. unclicked mouse is hovering over an imgui window, widget is active, mouse
    // was clicked over an imgui window, etc.).
    pub WantCaptureKeyboard: bool,
    // Set when Dear ImGui will use keyboard inputs, in this case do not dispatch them to
    // your main game/application (either way, always pass keyboard inputs to imgui).
    // (e.g. InputText active, or an imgui window is focused and navigation is enabled,
    // etc.).
    pub WantTextInput: bool,
    // Mobile/console: when set, you may display an on-screen keyboard. This is set by
    // Dear ImGui when it wants textual keyboard input to happen (e.g. when a InputText
    // widget is active).
    pub WantSetMousePos: bool,
    // MousePos has been altered, backend should reposition mouse on next frame. Rarely
    // used! Set only when ImGuiConfigFlags_NavEnableSetMousePos flag is enabled.
    pub WantSaveIniSettings: bool,
    // When manual .ini load/save is active (io.IniFilename == NULL), this will be set
    // to notify your application that you can call SaveIniSettingsToMemory() and save
    // yourself. Important: clear io.WantSaveIniSettings yourself after saving!
    pub NavActive: bool,
    // Keyboard/Gamepad navigation is currently allowed (will handle ImGuiKey_NavXXX
    // events) = a window is focused and it doesn't use the ImGuiWindowFlags_NoNavInputs
    // flag.
    pub NavVisible: bool,
    // Keyboard/Gamepad navigation is visible and allowed (will handle ImGuiKey_NavXXX
    // events).
    pub Framerate: c_float,
    // Estimate of application framerate (rolling average over 60 frames, based on io
    // .DeltaTime), in frame per second. Solely for convenience. Slow applications may
    // not want to use a moving average or may want to reset underlying buffers
    // occasionally.
    pub MetricsRenderVertices: c_int,
    // Vertices output during last call to Render()
    pub MetricsRenderIndices: c_int,
    // Indices output during last call to Render() = number of triangles * 3
    pub MetricsRenderWindows: c_int,
    // Number of visible windows
    pub MetricsActiveWindows: c_int,
    // Number of active windows
    pub MetricsActiveAllocations: c_int,
    // Number of active allocations, updated by MemAlloc/MemFree based on current context.
    // May be off if you have multiple imgui contexts.
    // Mouse delta. Note that this is zero if either current or previous position are
    // invalid (-f32::MAX,-f32::MAX), so a disappearing/reappearing mouse won't have
    // a huge delta.
    pub MouseDelta: ImVec2,
    //------------------------------------------------------------------
    // [Internal] Dear ImGui will maintain those fields. Forward compatibility not guaranteed!
    //------------------------------------------------------------------
    // Main Input State
    // (this block used to be written by backend, since 1.87 it is best to NOT write to those directly, call the AddXXX functions above instead)
    // (reading from those variables is fair game, as they are extremely unlikely to be moving anywhere)
    pub MousePos: ImVec2,
    // Mouse buttons: 0=left, 1=right, 2=middle + extras (ImGuiMouseButton_COUNT == 5). Dear ImGui mostly uses left and right buttons. Others buttons allows us to track if the mouse is being used by your application + available to user as a convenience via IsMouse** API.
    pub MouseDown: [bool; 5],
    pub MouseWheel: c_float,
    // Mouse wheel Vertical: 1 unit scrolls about 5 lines text.
    pub MouseWheelH: c_float,
    // Mouse wheel Horizontal. Most users don't have a mouse with an horizontal wheel, may not be filled by all backends.
    pub MouseHoveredViewport: ImguiHandle,
    // (Optional) Modify using io.AddMouseViewportEvent(). With multi-viewports: viewport the OS mouse is hovering. If possible _IGNORING_ viewports with the ImGuiViewportFlags_NoInputs flag is much better (few backends can handle that). Set io.BackendFlags |= IM_GUI_BACKEND_FLAGS_HAS_MOUSE_HOVERED_VIEWPORT if you can provide this info. If you don't imgui will infer the value using the rectangles and last focused time of the viewports it knows about (ignoring other OS windows).
    pub KeyCtrl: bool,
    // Keyboard modifier down: Control
    pub KeyShift: bool,
    // Keyboard modifier down: Shift
    pub KeyAlt: bool,
    // Keyboard modifier down: Alt
    pub KeySuper: bool,                           // Keyboard modifier down: Cmd/Super/Windows

    // Other state maintained from data above + IO function calls
    pub KeyMods: ImGuiModFlags,
    // Key mods flags (same as io.KeyCtrl/KeyShift/KeyAlt/KeySuper but merged into flags), updated by NewFrame()
    // ImGuiKeyData KeysData[ImGuiKey_KeysData_SIZE];  // Key state for all known keys. Use IsKeyXXX() functions to access this.
    pub KeysData: [ImguiKeyData; ImGuiKey_KeysData_SIZE as usize],
    pub WantCaptureMouseUnlessPopupClose: bool,
    // Alternative to WantCaptureMouse: (WantCaptureMouse == true && WantCaptureMouseUnlessPopupClose == false) when a click over void is expected to close a popup.
    pub MousePosPrev: ImVec2,
    // Previous mouse position (note that MouseDelta is not necessary == MousePos-MousePosPrev, in case either position is invalid)
    // ImVec2      MouseClickedPos[5];                 // Position at time of clicking
    pub MouseClickedPos: [ImVec2; 5],
    // double      MouseClickedTime[5];                // Time of last click (used to figure out double-click)
    pub MouseClickedTime: [c_double; 5],
    // bool        MouseClicked[5];                    // Mouse button went from !Down to Down (same as MouseClickedCount[x] != 0)
    pub MouseClicked: [bool; 5],
    // bool        MouseDoubleClicked[5];              // Has mouse button been double-clicked? (same as MouseClickedCount[x] == 2)
    pub MouseDoubleClicked: [bool; 5],
    // ImU16       MouseClickedCount[5];               // == 0 (not clicked), == 1 (same as MouseClicked[]), == 2 (double-clicked), == 3 (triple-clicked) etc. when going from !Down to Down
    pub MouseClickedCount: [usize; 5],
    // ImU16       MouseClickedLastCount[5];           // Count successive number of clicks. Stays valid after mouse release. Reset after another click is done.
    pub MouseClickedLastCount: [u16; 5],
    // bool        MouseReleased[5];                   // Mouse button went from Down to !Down
    pub MouseReleased: [bool; 5],
    // bool        MouseDownOwned[5];                  // Track if button was clicked inside a dear imgui window or over void blocked by a popup. We don't request mouse capture from the application if click started outside ImGui bounds.
    pub MouseDownOwned: [bool; 5],
    // bool        MouseDownOwnedUnlessPopupClose[5];  // Track if button was clicked inside a dear imgui window.
    pub MosueDownOwnedUnlessPopupClose: [bool; 5],
    // c_float       MouseDownDuration[5];               // Duration the mouse button has been down (0.0 == just clicked)
    pub MouseDownDuration: [t: c_float; 5],
    // c_float       MouseDownDurationPrev[5];           // Previous time the mouse button has been down
    pub MouseDownDurationPrev: [c_float; 5],
    // ImVec2      MouseDragMaxDistanceAbs[5];         // Maximum distance, absolute, on each axis, of how much mouse has traveled from the clicking point
    pub MouseDragMaxDistanceAbs: [ImVec2; 5],
    // c_float       MouseDragMaxDistanceSqr[5];         // Squared maximum distance of how much mouse has traveled from the clicking point (used for moving thresholds)
    pub MouseDragMaxDistanceSqr: [c_float; 5],
    pub PenPressure: c_float,
    // Touch/Pen pressure (0.0 to 1.0, should be >0.0 only when MouseDown[0] == true). Helper storage currently unused by Dear ImGui.
    pub AppFocusLost: bool,
    // Only modify via AddFocusEvent()
    pub AppAcceptingEvents: bool,
    // Only modify via SetAppAcceptingEvents()
    pub BackendUsingLegacyKeyArrays: i8,
    // -1: unknown, 0: using AddKeyEvent(), 1: using legacy io.KeysDown[]
    pub BackendUsingLegacyNavInputArray: bool,
    // 0: using AddKeyAnalogEvent(), 1: writing to legacy io.NavInputs[] directly
    pub InputQueueSurrogate: ImWchar16,
    // For AddInputCharacterUTF16()
    // ImVector<ImWchar> InputQueueCharacters;         // Queue of _characters_ input (obtained by platform backend). Fill using AddInputCharacter() helper.
    pub InputQueueCharacters: Vec<char>,
}



impl ImguiIo {
    // ImGuiIO::ImGuiIO()
    pub fn new() -> Self {
        // Most fields are initialized with zero
        // memset(this, 0, sizeof(*this));
        let mut out = Self { ..Default::default() };
        // IM_STATIC_ASSERT(IM_ARRAYSIZE(ImGuiIO::MouseDown) == ImGuiMouseButton_COUNT && IM_ARRAYSIZE(ImGuiIO::MouseClicked) == ImGuiMouseButton_COUNT);

        // Settings
        out.ConfigFlags = ImGuiConfigFlags_None;
        out.BackendFlags = ImGuiBackendFlags_None;
        out.DisplaySize = ImVec2::from_floats(-1.0, -1.0);
        out.DeltaTime = 1.0 / 60f32;
        out.IniSavingRate = 5f32;
        out.IniFilename = String::from("imgui.ini").as_ptr().into(); // Important: "imgui.ini" is relative to current working dir, most apps will want to lock this to an absolute path (e.g. same path as executables).
        out.LogFilename = String::from("imgui_log.txt").into();
        out.MouseDoubleClickTime = 0.3f32;
        out.MouseDoubleClickMaxDist = 6f32;
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//         for (i: c_int = 0; i < ImGuiKey_COUNT; i+ +)
        for i in 0 .. ImGuiKey_COUNT
        {
            out.KeyMap[i] = -1;
        }
// #endif
        out.KeyRepeatDelay = 0.275f32;
        out.KeyRepeatRate = 0.05f32;
        out.HoverDelayNormal = 0.3f32;
        out.HoverDelayShort = 0.1.0;
        out.UserData = None;

        out.Fonts = None;
        out.FontGlobalScale = 1.0;
        out.FontDefault = None;
        out.FontAllowUserScaling = false;
        out.DisplayFramebufferScale = ImVec2::from_floats(1.0, 1.0);

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
        out.BackendPlatformName = None;
        out.BackendRendererName = None;
        out.BackendPlatformUserData = None;
        out.BackendRendererUserData = None;
        out.BackendLanguageUserData = None;
        out.GetClipboardTextFn = GetClipboardTextFn_DefaultImpl;   // Platform dependent default implementations
        out.SetClipboardTextFn = SetClipboardTextFn_DefaultImpl;
        out.ClipboardUserData = None;
        out.SetPlatformImeDataFn = SetPlatformImeDataFn_DefaultImpl;

        // Input (NB: we already have memset zero the entire structure!)
        out.MousePos = ImVec2::from_floats(-f32::MAX, -f32::MAX);
        out.MousePosPrev = ImVec2::from_floats(-f32::MAX, -f32::MAX);
        out.MouseDragThreshold = 6f32;
        // for (i: c_int = 0; i < IM_ARRAYSIZE(MouseDownDuration); i+ +)
        for i in 0 .. out.MouseDownDuration.len()
        {
            out.MouseDownDuration[i] = -1.0;
            out.MouseDownDurationPrev[i] = -1.0;
        }
        // for (i: c_int = 0; i < IM_ARRAYSIZE(KeysData); i+ +)
        for i in 0 .. out.KeysData.len()
        {
            out.KeysData[i].DownDuration = -1.0;
            out.KeysData[i].DownDurationPrev = -1.0;
        }
        out.AppAcceptingEvents = true;
        out.BackendUsingLegacyKeyArrays =  - 1;
        out.BackendUsingLegacyNavInputArray = true; // assume using legacy array until proven wrong
        out
    }

    // Pass in translated ASCII characters for text input.
    // - with glfw you can get those from the callback set in glfwSetCharCallback()
    // - on Windows you can get those using ToAscii+keyboard state, or via the WM_CHAR message
    // FIXME: Should in theory be called "AddCharacterEvent()" to be consistent with new API
    // void ImGuiIO::AddInputCharacter(unsigned c: c_int)
    pub fn AddInputCharacter(&mut self, c: u32)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.IO == this && "Can only add events to current context.");
        if c == 0 || !self.AppAcceptingEvents {
            return;
        }

        // ImGuiInputEvent e;
        let mut e: ImguiInputEvent = ImguiInputEvent::new();
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

        // let cp: ImWchar = c;
        let mut cp: ImWchar = c as ImWchar;
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
                cp = (((self.InputQueueSurrogate - 0xD800) << 10) + (c - 0xDC00) + 0x10000) as ImWchar;
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
            // unsigned c: c_int = 0;
            let mut c: u32 = 0;
            utf8_chars += ImTextCharFromUtf8(&mut c, utf8_chars as *const c_char, null_mut());
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
    //     for (n: c_int = 0; n < IM_ARRAYSIZE(KeysData); n++)
       for n in 0 .. self.KeysData.len()
        {

            self.KeysData[n].Down             = false;
            self.KeysData[n].DownDuration     = -1.0;
            self.KeysData[n].DownDurationPrev = -1.0;
        }
        self.KeyCtrl = false;
        self.KeyShift = false;
        self.KeyAlt = false;
        self.KeySuper = false;
        self.KeyMods = ImGuiModFlags_None;
    }



    // Queue a new key down/up event.
    // - ImGuiKey key:       Translated key (as in, generally ImGuiKey_A matches the key end-user would use to emit an 'A' character)
    // - down: bool:          Is the key down? use false to signify a key release.
    // - c_float analog_value: 0.0..1.0f
    // void ImGuiIO::AddKeyAnalogEvent(ImGuiKey key, down: bool, c_float analog_value)
    pub fn AddKeyAnalogEvent(&mut self, key: ImGuiKey, down: bool, analog_value: c_float)
    {
        //if (e->Down) { IMGUI_DEBUG_LOG_IO("AddKeyEvent() Key='{}' {}, NativeKeycode = {}, NativeScancode = {}\n", GetKeyName(e->Key), e->Down, e->NativeKeycode, e->NativeScancode); }
        if key == ImGuiKey_None || !self.AppAcceptingEvents {
            return;
        }
        let g = GImGui; // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.IO == this && "Can only add events to current context.");
        // IM_ASSERT(IsNamedKey(key)); // Backend needs to pass a valid ImGuiKey_ constant. 0..511 values are legacy native key codes which are not accepted by this API.
        // IM_ASSERT(!IsAliasKey(key)); // Backend cannot submit ImGuiKey_MouseXXX values they are automatically inferred from AddMouseXXX() events.

        // Verify that backend isn't mixing up using new io.AddKeyEvent() api and old io.KeysDown[] + io.KeyMap[] data.
    // #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    //     IM_ASSERT((BackendUsingLegacyKeyArrays == -1 || BackendUsingLegacyKeyArrays == 0) && "Backend needs to either only use io.AddKeyEvent(), either only fill legacy io.KeysDown[] + io.KeyMap[]. Not both!");
    //     if (BackendUsingLegacyKeyArrays == -1) {
    //         for (n: c_int = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n+ +){
    //             IM_ASSERT(KeyMap[n] == -1 && "Backend needs to either only use io.AddKeyEvent(), either only fill legacy io.KeysDown[] + io.KeyMap[]. Not both!");
    //         }
    //     }
        self.BackendUsingLegacyKeyArrays = 0;
    // #endif
        if IsGamepadKey(key) {
            self.BackendUsingLegacyNavInputArray = false;
        }

        // Partial filter of duplicates (not strictly needed, but makes data neater in particular for key mods and gamepad values which are most commonly spmamed)
        let key_data = GetKeyData(key);
        if key_data.Down == down && key_data.AnalogValue == analog_value
        {
            let mut found = false;
            // for (n: c_int = g.InputEventsQueue.Size - 1; n >= 0 && !found; n--)
            for n in g.InputEventsQueue.len() - 1 .. 0
            {
                if g.InputEventsQueue[n].Type == ImGuiInputEventType_Key && g.InputEventsQueue[n].Key.Key == key {
                    found = true;
                }
            }
            if !found {
                return;
            }
        }

        // Add event
        let mut e = ImguiInputEvent::new();
        e.Type = ImGuiInputEventType_Key;
        e.Source = if IsGamepadKey(key) { ImGuiInputSource_Gamepad } else { ImGuiInputSource_Keyboard };
        e.Key.Key = key;
        e.Key.Down = down;
        e.Key.AnalogValue = analog_value;
        g.InputEventsQueue.push(e);
    }

    // void ImGuiIO::AddKeyEvent(ImGuiKey key, down: bool)
    pub fn AddKeyEvent(&mut self, key: ImGuiKey, down: bool)
    {
        if !self.AppAcceptingEvents {
            return;
        }
        self.AddKeyAnalogEvent(key, down, if down { 1.0 } else { 0.0 });
    }

    // [Optional] Call after AddKeyEvent().
    // Specify native keycode, scancode + Specify index for legacy <1.87 IsKeyXXX() functions with native indices.
    // If you are writing a backend in 2022 or don't use IsKeyXXX() with native values that are not ImGuiKey values, you can avoid calling this.
    // void ImGuiIO::SetKeyEventNativeData(ImGuiKey key, native_keycode: c_int, native_scancode: c_int, native_legacy_index: c_int)
    pub fn SetKeyEventNativeData(&mut self, key:ImGuiKey, native_keycode: c_int, native_scancode: c_int, native_legacy_index: c_int)
    {
        if (key == ImGuiKey_None) {
            return;
        }
        // IM_ASSERT(IsNamedKey(key)); // >= 512
        // IM_ASSERT(native_legacy_index == -1 || IsLegacyKey(native_legacy_index)); // >= 0 && <= 511
        // IM_UNUSED(native_keycode);  // Yet unused
        // IM_UNUSED(native_scancode); // Yet unused

        // Build native->imgui map so old user code can still call key functions with native 0..511 values.
    // #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    //     let legacy_key = if (native_legacy_index != -1) { native_legacy_index } else { native_keycode };
    //     if !IsLegacyKey(legacy_key) {
    //         return;
    //     }
    //     self.KeyMap[legacy_key] = key;
    //     self.KeyMap[key] = legacy_key;
    // #else
    //     IM_UNUSED(key);
    //     IM_UNUSED(native_legacy_index);
    // #endif
    }

    // Set master flag for accepting key/mouse/text events (default to true). Useful if you have native dialog boxes that are interrupting your application loop/refresh, and you want to disable events being queued while your app is frozen.
    // void ImGuiIO::SetAppAcceptingEvents(accepting_events: bool)
    pub fn SetAppAcceptingEvents(&mut self, accepting_events: bool)
    {
        self.AppAcceptingEvents = accepting_events;
    }

    // Queue a mouse move event
    // void ImGuiIO::AddMousePosEvent(c_float x, c_float y)
    pub fn AddMousePosEvent(&mut self, x: c_float, y: c_float)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.IO == this && "Can only add events to current context.");
        if (!self.AppAcceptingEvents) {
            return;
        }

        // ImGuiInputEvent e;
        let mut e = ImguiInputEvent::new();
        e.Type = ImGuiInputEventType_MousePos;
        e.Source = ImGuiInputSource_Mouse;
        e.MousePos.PosX = x;
        e.MousePos.PosY = y;
        g.InputEventsQueue.push(e);
    }

    // void ImGuiIO::AddMouseButtonEvent(mouse_button: c_int, down: bool)
    pub fn AddMouseButtonEvent(&mut self, mouse_button: c_int, down: bool)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.IO == this && "Can only add events to current context.");
        // IM_ASSERT(mouse_button >= 0 && mouse_button < ImGuiMouseButton_COUNT);
        if (!self.AppAcceptingEvents) {
            return;
        }

        // ImGuiInputEvent e;
        let mut e = ImguiInputEvent::new();
        e.Type = ImGuiInputEventType_MouseButton;
        e.Source = ImGuiInputSource_Mouse;
        e.MouseButton.Button = mouse_button;
        e.MouseButton.Down = down;
        g.InputEventsQueue.push(e);
    }

    // Queue a mouse wheel event (most mouse/API will only have a Y component)
    // void ImGuiIO::AddMouseWheelEvent(c_float wheel_x, c_float wheel_y)
    pub fn AddMouseWheelEvent(&mut self, wheel_x: c_float, wheel_y: c_float)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.IO == this && "Can only add events to current context.");
        if (wheel_x == 0.0 && wheel_y == 0.0) || !self.ppAcceptingEvents {
            return;
        }

        // ImGuiInputEvent e;
        let mut e = ImguiInputEvent::new();
        e.Type = ImGuiInputEventType_MouseWheel;
        e.Source = ImGuiInputSource_Mouse;
        e.MouseWheel.WheelX = wheel_x;
        e.MouseWheel.WheelY = wheel_y;
        g.InputEventsQueue.push(e);
    }

    // void ImGuiIO::AddMouseViewportEvent(ImguiHandle viewport_id)
    pub fn AddMouseViewportEvent(&mut self, viewport_id: ImguiHandle)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.IO == this && "Can only add events to current context.");
        // IM_ASSERT(g.IO.BackendFlags & IM_GUI_BACKEND_FLAGS_HAS_MOUSE_HOVERED_VIEWPORT);

        // ImGuiInputEvent e;
        let mut e = ImguiInputEvent::new();
        e.Type = ImGuiInputEventType_MouseViewport;
        e.Source = ImGuiInputSource_Mouse;
        e.MouseViewport.HoveredViewportID = viewport_id;
        g.InputEventsQueue.push(e);
    }

    // void ImGuiIO::AddFocusEvent(focused: bool)
    pub fn AddFocusEvent(&mut self, focused: bool)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.IO == this && "Can only add events to current context.");

        // ImGuiInputEvent e;
        let mut e = ImguiInputEvent::new();
        e.Type = ImGuiInputEventType_Focus;
        e.AppFocused.Focused = focused;
        g.InputEventsQueue.push(e);
    }

    // Input Functions
    // void  AddKeyEvent(ImGuiKey key, down: bool);                   // Queue a new key down/up event. Key should be "translated" (as in, generally ImGuiKey_A matches the key end-user would use to emit an 'A' character)

    // void  AddKeyAnalogEvent(ImGuiKey key, down: bool, c_float v);    // Queue a new key down/up event for analog values (e.g. ImGuiKey_Gamepad_ values). Dead-zones should be handled by the backend.

    // void  AddMousePosEvent(c_float x, c_float y);                     // Queue a mouse position update. Use -f32::MAX,-f32::MAX to signify no mouse (e.g. app not focused and not hovered)

    // void  AddMouseButtonEvent(button: c_int, down: bool);             // Queue a mouse button change

    // void  AddMouseWheelEvent(c_float wh_x, c_float wh_y);             // Queue a mouse wheel update

    // void  AddMouseViewportEvent(ImguiHandle id);                      // Queue a mouse hovered viewport. Requires backend to set

    // IM_GUI_BACKEND_FLAGS_HAS_MOUSE_HOVERED_VIEWPORT to call this (for multi-viewport support).

    // void  AddFocusEvent(focused: bool);                            // Queue a gain/loss of focus for the application (generally based on OS/platform focus of your window)

    // void  AddInputCharacter(unsigned c: c_int);                      // Queue a new character input

    // void  AddInputCharacterUTF16(ImWchar16 c);                    // Queue a new character input from an UTF-16 character, 
    // it can be a surrogate

    // void  AddInputCharactersUTF8(const char* str);                // Queue a new characters input from an UTF-8 string

    // void  SetKeyEventNativeData(ImGuiKey key, native_keycode: c_int, native_scancode: c_int, native_legacy_index: c_int = -1); // [Optional] Specify index for legacy <1.87 IsKeyXXX() functions with native indices + specify native keycode, scancode.

    // void  SetAppAcceptingEvents(accepting_events: bool);           // Set master flag for accepting key/mouse/text events (default to true). Useful if you have native dialog boxes that are interrupting your application loop/refresh, and you want to disable events being queued while your app is frozen.

    // void  ClearInputCharacters();                                 // [Internal] Clear the text input buffer manually

    // void  ClearInputKeys();                                       // [Internal] Release all keys
}
