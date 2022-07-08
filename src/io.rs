use std::ffi::c_void;
use crate::imgui_globals::GImGui;
use crate::imgui_h::{IM_UNICODE_CODEPOINT_INVALID, ImFont, ImFontAtlas, ImGuiBackendFlags, ImGuiConfigFlags, ImGuiKey, ImGuiKeyData, ImGuiModFlags, ImGuiViewport, ImVec2};
use crate::imgui_h::ImGuiBackendFlags::ImGuiBackendFlags_None;
use crate::imgui_h::ImGuiConfigFlags::ImGuiConfigFlags_None;
use crate::imgui_h::ImGuiKey::ImGuiKey_None;
use crate::imgui_h::ImGuiModFlags::ImGuiModFlags_None;
use crate::imgui_input_event::{ImGuiInputEvent, ImGuiInputEventVal};

#[allow(non_snake_case)]
pub struct DimgIo {
    //------------------------------------------------------------------
    // Configuration                            // Default value
    //------------------------------------------------------------------

    pub ConfigFlags: ImGuiConfigFlags,
    // = 0              // See ImGuiConfigFlags_ enum. Set by user/application. Gamepad/keyboard navigation options, etc.
    pub BackendFlags: ImGuiBackendFlags,
    // = 0              // See ImGuiBackendFlags_ enum. Set by backend (imgui_impl_xxx files or custom backend) to communicate features supported by the backend.
    pub DisplaySize: ImVec2,
    // <unset>          // Main display size, in pixels (generally == GetMainViewport()->size). May change every frame.
    pub DeltaTime: f32,
    // = 1.0/60.0     // time elapsed since last frame, in seconds. May change every frame.
    pub IniSavingRate: f32,
    // = 5.0           // Minimum time between saving positions/sizes to .ini file, in seconds.
    pub IniFilename: String,
    // const char* IniFilename;                    // = "imgui.ini"    // Path to .ini file (important: default "imgui.ini" is relative to current working dir!). Set NULL to disable automatic .ini loading/saving or if you want to manually call LoadIniSettingsXXX() / SaveIniSettingsXXX() functions.
    pub LogFilename: String,
    // const char* LogFilename;                    // = "imgui_log.txt"// Path to .log file (default parameter to ImGui::LogToFile when no file is specified).
    pub MouseDoubleClickTime: f32,
    // = 0.30          // time for a double-click, in seconds.
    pub MouseDoubleClickMaxDist: f32,
    // = 6.0           // Distance threshold to stay in to validate a double-click, in pixels.
    pub MouseDragThreshold: f32,
    // = 6.0           // Distance threshold before considering we are dragging.
    pub KeyRepeatDelay: f32,
    // = 0.250         // When holding a key/button, time before it starts repeating, in seconds (for buttons in Repeat mode, etc.).
    pub KeyRepeatRate: f32,
    // = 0.050         // When holding a key/button, rate at which it repeats, in seconds.
    pub UserData: *mut c_void,
    // void*       user_data;                       // = NULL           // Store your own data for retrieval by callbacks.
    pub Fonts: *mut ImFontAtlas,
    // ImFontAtlas*Fonts;                          // <auto>           // font atlas: load, rasterize and pack one or more fonts into a single texture.
    pub FontGlobalScale: f32,
    // = 1.0           // Global scale all fonts
    pub FontAllowUserScaling: bool,
    // = false          // Allow user scaling text of individual window with CTRL+Wheel.
    pub FontDefault: *mut ImFont,
    // ImFont*     FontDefault;                    // = NULL           // font to use on NewFrame(). Use NULL to uses Fonts->Fonts[0].
    pub DisplayFramebufferScale: ImVec2,        // = (1, 1)         // For retina display or other situations where window coordinates are different from framebuffer coordinates. This generally ends up in ImDrawData::FramebufferScale.

    // Docking options (when ImGuiConfigFlags_DockingEnable is set)
    pub ConfigDockingNoSplit: bool,
    // = false          // Simplified docking mode: disable window splitting, so docking is limited to merging multiple windows together into tab-bars.
    pub ConfigDockingWithShift: bool,
    // = false          // Enable docking with holding Shift key (reduce visual noise, allows dropping in wider space)
    pub ConfigDockingAlwaysTabBar: bool,
    // = false          // [BETA] [FIXME: This currently creates regression with auto-sizing and general overhead] Make every single floating window display within a docking node.
    pub ConfigDockingTransparentPayload: bool,// = false          // [BETA] Make window or viewport transparent when docking and only display docking boxes on the target viewport. Useful if rendering of multiple viewport cannot be synced. Best used with ConfigViewportsNoAutoMerge.

    // viewport options (when ImGuiConfigFlags_ViewportsEnable is set)
    pub ConfigViewportsNoAutoMerge: bool,
    // = false;         // Set to make all floating imgui windows always create their own viewport. Otherwise, they are merged into the main host viewports when overlapping it. May also set ImGuiViewportFlags_NoAutoMerge on individual viewport.
    pub ConfigViewportsNoTaskBarIcon: bool,
    // = false          // Disable default OS task bar icon flag for secondary viewports. When a viewport doesn't want a task bar icon, ImGuiViewportFlags_NoTaskBarIcon will be set on it.
    pub ConfigViewportsNoDecoration: bool,
    // = true           // Disable default OS window decoration flag for secondary viewports. When a viewport doesn't want window decorations, ImGuiViewportFlags_NoDecoration will be set on it. Enabling decoration can create subsequent issues at OS levels (e.g. minimum window size).
    pub ConfigViewportsNoDefaultParent: bool, // = false          // Disable default OS parenting to main viewport for secondary viewports. By default, viewports are marked with parent_viewport_id = <main_viewport>, expecting the platform backend to setup a parent/child relationship between the OS windows (some backend may ignore this). Set to true if you want the default to be 0, then all viewports will be top-level OS windows.

    // Miscellaneous options
    pub MouseDrawCursor: bool,
    // = false          // Request ImGui to draw a mouse cursor for you (if you are on a platform without a mouse cursor). Cannot be easily renamed to 'io.ConfigXXX' because this is frequently used by backend implementations.
    pub ConfigMacOSXBehaviors: bool,
    // = defined(__APPLE__) // OS X style: Text editing cursor movement using Alt instead of Ctrl, Shortcuts using Cmd/Super instead of Ctrl, Line/Text Start and End using Cmd+Arrows instead of Home/End, Double click selects by word instead of selecting whole text, Multi-selection in lists uses Cmd/Super instead of Ctrl.
    pub ConfigInputTrickleEventQueue: bool,
    // = true           // Enable input queue trickling: some types of events submitted during the same frame (e.g. button down + up) will be spread over multiple frames, improving interactions with low framerates.
    pub ConfigInputTextCursorBlink: bool,
    // = true           // Enable blinking cursor (optional as some users consider it to be distracting).
    pub ConfigDragClickToInputText: bool,
    // = false          // [BETA] Enable turning DragXXX widgets into text input with a simple mouse click-release (without moving). Not desirable on devices without a keyboard.
    pub ConfigWindowsResizeFromEdges: bool,
    // = true           // Enable resizing of windows from their edges and from the lower-left corner. This requires (io.BackendFlags & ImGuiBackendFlags_HasMouseCursors) because it needs mouse cursor feedback. (This used to be a per-window ImGuiWindowFlags_ResizeFromAnySide flag)
    pub ConfigWindowsMoveFromTitleBarOnly: bool,
    // = false       // Enable allowing to move windows only when clicking on their title bar. Does not apply to windows without a title bar.
    pub ConfigMemoryCompactTimer: f32,      // = 60.0          // Timer (in seconds) to free transient windows/tables memory buffers when unused. Set to -1.0 to disable.

    //------------------------------------------------------------------
    // Platform Functions
    // (the imgui_impl_xxxx backend files are setting those up for you)
    //------------------------------------------------------------------

    // Optional: Platform/Renderer backend name (informational only! will be displayed in About Window) + User data for backend/wrappers to store their own stuff.
    // const char* BackendPlatformName;            // = NULL
    pub BackendPlatformName: String,
    // const char* BackendRendererName;            // = NULL
    pub BackendRendererName: String,
    // void*       BackendPlatformUserData;        // = NULL           // User data for platform backend
    pub BackendPlatformUserData: Vec<u8>,
    // void*       BackendRendererUserData;        // = NULL           // User data for renderer backend
    pub BackendRendererUserData: Vec<u8>,
    // void*       BackendLanguageUserData;        // = NULL           // User data for non C++ programming language backend
    pub BackendLanguageUserData: Vec<u8>,

    // Optional: Access OS clipboard
    // (default to use native Win32 clipboard on windows, otherwise uses a private clipboard. Override to access OS clipboard on other architectures)
    // const char* (*GetClipboardTextFn)(void* user_data);
    // void        (*SetClipboardTextFn)(void* user_data, const char* text);
    // void*       ClipboardUserData;

    // Optional: Notify OS Input Method Editor of the screen position of your cursor for text input position (e.g. when using Japanese/Chinese IME on windows)
    // (default to use native imm32 api on windows)
    // void        (*SetPlatformImeDataFn)(ImGuiViewport* viewport, ImGuiPlatformImeData* data);
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     void*       ImeWindowHandle;                // = NULL           // [Obsolete] Set ImGuiViewport::PlatformHandleRaw instead. Set this to your HWND to get automatic IME cursor positioning.
    pub ImeWindowHandle: *mut c_void,
// #else
//     void*       _UnusedPadding;                                     // Unused field to keep data structure the same size.
// #endif

    //------------------------------------------------------------------
    // Input - Call before calling NewFrame()
    //------------------------------------------------------------------

    // // Input Functions
    //  void  AddKeyEvent(ImGuiKey key, bool down);                   // Queue a new key down/up event. Key should be "translated" (as in, generally ImGuiKey_A matches the key end-user would use to emit an 'A' character)
    //  void  AddKeyAnalogEvent(ImGuiKey key, bool down, float v);    // Queue a new key down/up event for analog values (e.g. ImGuiKey_Gamepad_ values). Dead-zones should be handled by the backend.
    //  void  AddMousePosEvent(float x, float y);                     // Queue a mouse position update. Use -FLT_MAX,-FLT_MAX to signify no mouse (e.g. app not focused and not hovered)
    //  void  AddMouseButtonEvent(int button, bool down);             // Queue a mouse button change
    //  void  AddMouseWheelEvent(float wh_x, float wh_y);             // Queue a mouse wheel update
    //  void  AddMouseViewportEvent(ImGuiID id);                      // Queue a mouse hovered viewport. Requires backend to set ImGuiBackendFlags_HasMouseHoveredViewport to call this (for multi-viewport support).
    //  void  AddFocusEvent(bool focused);                            // Queue a gain/loss of focus for the application (generally based on OS/platform focus of your window)
    //  void  AddInputCharacter(unsigned int c);                      // Queue a new character input
    //  void  AddInputCharacterUTF16(ImWchar16 c);                    // Queue a new character input from an UTF-16 character, it can be a surrogate
    //  void  AddInputCharactersUTF8(const char* str);                // Queue a new characters input from an UTF-8 string
    // 
    //  void  SetKeyEventNativeData(ImGuiKey key, int native_keycode, int native_scancode, int native_legacy_index = -1); // [Optional] Specify index for legacy <1.87 IsKeyXXX() functions with native indices + specify native keycode, scancode.
    //  void  SetAppAcceptingEvents(bool accepting_events);           // Set master flag for accepting key/mouse/text events (default to true). Useful if you have native dialog boxes that are interrupting your application loop/refresh, and you want to disable events being queued while your app is frozen.
    //  void  ClearInputCharacters();                                 // [Internal] clear the text input buffer manually
    //  void  ClearInputKeys();                                       // [Internal] Release all keys

    //------------------------------------------------------------------
    // Output - Updated by NewFrame() or EndFrame()/Render()
    // (when reading from the io.WantCaptureMouse, io.WantCaptureKeyboard flags to dispatch your inputs, it is
    //  generally easier and more correct to use their state BEFORE calling NewFrame(). See FAQ for details!)
    //------------------------------------------------------------------

    pub WantCaptureMouse: bool,
    // Set when Dear ImGui will use mouse inputs, in this case do not dispatch them to your main game/application (either way, always pass on mouse inputs to imgui). (e.g. unclicked mouse is hovering over an imgui window, widget is active, mouse was clicked over an imgui window, etc.).
    pub WantCaptureKeyboard: bool,
    // Set when Dear ImGui will use keyboard inputs, in this case do not dispatch them to your main game/application (either way, always pass keyboard inputs to imgui). (e.g. InputText active, or an imgui window is focused and navigation is enabled, etc.).
    pub WantTextInput: bool,
    // Mobile/console: when set, you may display an on-screen keyboard. This is set by Dear ImGui when it wants textual keyboard input to happen (e.g. when a InputText widget is active).
    pub WantSetMousePos: bool,
    // MousePos has been altered, backend should reposition mouse on next frame. Rarely used! Set only when ImGuiConfigFlags_NavEnableSetMousePos flag is enabled.
    pub WantSaveIniSettings: bool,
    // When manual .ini load/save is active (io.IniFilename == NULL), this will be set to notify your application that you can call SaveIniSettingsToMemory() and save yourself. Important: clear io.WantSaveIniSettings yourself after saving!
    pub NavActive: bool,
    // Keyboard/Gamepad navigation is currently allowed (will handle ImGuiKey_NavXXX events) = a window is focused and it doesn't use the ImGuiWindowFlags_NoNavInputs flag.
    pub NavVisible: bool,
    // Keyboard/Gamepad navigation is visible and allowed (will handle ImGuiKey_NavXXX events).
    pub Framerate: f32,
    // Rough estimate of application framerate, in frame per second. Solely for convenience. Rolling average estimation based on io.DeltaTime over 120 frames.
    pub MetricsRenderVertices: i32,
    // Vertices output during last call to Render()
    pub MetricsRenderIndices: i32,
    // Indices output during last call to Render() = number of triangles * 3
    pub MetricsRenderWindows: i32,
    // Number of visible windows
    pub MetricsActiveWindows: i32,
    // Number of active windows
    pub MetricsActiveAllocations: i32,
    // Number of active allocations, updated by MemAlloc/MemFree based on current context. May be off if you have multiple imgui contexts.
    pub MouseDelta: ImVec2,                         // Mouse delta. Note that this is zero if either current or previous position are invalid (-FLT_MAX,-FLT_MAX), so a disappearing/reappearing mouse won't have a huge delta.

    // Legacy: before 1.87, we required backend to fill io.KeyMap[] (imgui->native map) during initialization and io.KeysDown[] (native indices) every frame.
    // This is still temporarily supported as a legacy feature. However the new preferred scheme is for backend to call io.AddKeyEvent().
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//     int         KeyMap[ImGuiKey_COUNT];             // [LEGACY] Input: map of indices into the KeysDown[512] entries array which represent your "native" keyboard state. The first 512 are now unused and should be kept zero. Legacy backend will write into KeyMap[] using ImGuiKey_ indices which are always >512.
//     bool        KeysDown[ImGuiKey_COUNT];           // [LEGACY] Input: Keyboard keys that are pressed (ideally left in the "native" order your engine has access to keyboard keys, so you can use your own defines/enums for keys). This used to be [512] sized. It is now ImGuiKey_COUNT to allow legacy io.KeysDown[GetKeyIndex(...)] to work without an overflow.
// #endif

    //------------------------------------------------------------------
    // [Internal] Dear ImGui will maintain those fields. Forward compatibility not guaranteed!
    //------------------------------------------------------------------

    // Main Input State
    // (this block used to be written by backend, since 1.87 it is best to NOT write to those directly, call the AddXXX functions above instead)
    // (reading from those variables is fair game, as they are extremely unlikely to be moving anywhere)
    pub MousePos: ImVec2,
    // Mouse position, in pixels. Set to ImVec2(-FLT_MAX, -FLT_MAX) if mouse is unavailable (on another screen, etc.)
    pub MouseDown: [bool; 5],
    // bool        MouseDown[5];                       // Mouse buttons: 0=left, 1=right, 2=middle + extras (ImGuiMouseButton_COUNT == 5). Dear ImGui mostly uses left and right buttons. Others buttons allows us to track if the mouse is being used by your application + available to user as a convenience via IsMouse** API.
    pub MouseWheel: f32,
    // Mouse wheel Vertical: 1 unit scrolls about 5 lines text.
    pub MouseWheelH: f32,
    // Mouse wheel Horizontal. Most users don't have a mouse with an horizontal wheel, may not be filled by all backends.
    pub MouseHoveredViewport: ImGuiID,
    // (Optional) Modify using io.AddMouseViewportEvent(). With multi-viewports: viewport the OS mouse is hovering. If possible _IGNORING_ viewports with the ImGuiViewportFlags_NoInputs flag is much better (few backends can handle that). Set io.BackendFlags |= ImGuiBackendFlags_HasMouseHoveredViewport if you can provide this info. If you don't imgui will infer the value using the rectangles and last focused time of the viewports it knows about (ignoring other OS windows).
    pub KeyCtrl: bool,
    // Keyboard modifier down: Control
    pub KeyShift: bool,
    // Keyboard modifier down: Shift
    pub KeyAlt: bool,
    // Keyboard modifier down: Alt
    pub KeySuper: bool,
    // Keyboard modifier down: Cmd/Super/windows
    // float       NavInputs[ImGuiNavInput_COUNT];     // Gamepad inputs. Cleared back to zero by EndFrame(). Keyboard keys will be auto-mapped and be written here by NewFrame().
    pub NavInputs: Vec<f32>,

    // Other state maintained from data above + io function calls
    pub KeyMods: ImGuiModFlags,
    // Key mods flags (same as io.KeyCtrl/KeyShift/KeyAlt/KeySuper but merged into flags), updated by NewFrame()
    pub KeysData: Vec<ImGuiKeyData>,
    // Key state for all known keys. Use IsKeyXXX() functions to access this.
    pub WantCaptureMouseUnlessPopupClose: bool,
    // Alternative to WantCaptureMouse: (WantCaptureMouse == true && WantCaptureMouseUnlessPopupClose == false) when a click over void is expected to close a popup.
    pub MousePosPrev: ImVec2,
    // Previous mouse position (note that MouseDelta is not necessary == MousePos-MousePosPrev, in case either position is invalid)
    pub MouseClickedPos: [ImVec2; 5],
    // Position at time of clicking
    pub MouseClickedTime: [f64; 5],
    // time of last click (used to figure out double-click)
    pub MouseClicked: [bool; 5],
    // Mouse button went from !down to down (same as MouseClickedCount[x] != 0)
    pub MouseDoubleClicked: [bool; 5],
    // Has mouse button been double-clicked? (same as MouseClickedCount[x] == 2)
    pub MouseClickedCount: [u16; 5],
    // == 0 (not clicked), == 1 (same as MouseClicked[]), == 2 (double-clicked), == 3 (triple-clicked) etc. when going from !down to down
    pub MouseClickedLastCount: [u16; 5],
    // Count successive number of clicks. Stays valid after mouse release. Reset after another click is done.
    pub MouseReleased: [bool; 5],
    // Mouse button went from down to !down
    pub MouseDownOwned: [bool; 5],
    // Track if button was clicked inside a dear imgui window or over void blocked by a popup. We don't request mouse capture from the application if click started outside ImGui bounds.
    pub MouseDownOwnedUnlessPopupClose: [bool; 5],
    // Track if button was clicked inside a dear imgui window.
    pub MouseDownDuration: [f32; 5],
    // Duration the mouse button has been down (0.0 == just clicked)
    pub MouseDownDurationPrev: [f32; 5],
    // Previous time the mouse button has been down
    pub MouseDragMaxDistanceAbs: [ImVec2; 5],
    // Maximum distance, absolute, on each axis, of how much mouse has traveled from the clicking point
    pub MouseDragMaxDistanceSqr: [f32; 5],
    // Squared maximum distance of how much mouse has traveled from the clicking point (used for moving thresholds)
    pub NavInputsDownDuration: Vec<f32>,
    pub NavInputsDownDurationPrev: Vec<f32>,
    pub PenPressure: f32,
    // Touch/Pen pressure (0.0 to 1.0, should be >0.0 only when MouseDown[0] == true). Helper storage currently unused by Dear ImGui.
    pub AppFocusLost: bool,
    // Only modify via AddFocusEvent()
    pub AppAcceptingEvents: bool,
    // Only modify via SetAppAcceptingEvents()
    pub BackendUsingLegacyKeyArrays: i8,
    // -1: unknown, 0: using AddKeyEvent(), 1: using legacy io.KeysDown[]
    pub BackendUsingLegacyNavInputArray: bool,
    // 0: using AddKeyAnalogEvent(), 1: writing to legacy io.NavInputs[] directly
    pub InputQueueSurrogate: Vec<u8>,
    // For AddInputCharacterUTF16()
    pub InputQueueCharacters: Vec<u8>,         // Queue of _characters_ input (obtained by platform backend). Fill using AddInputCharacter() helper.

    //    ImGuiIO();
}

impl DimgIo {
    pub fn new() -> Self {
        let mut out = Self { ..Default() };

        // Most fields are initialized with zero
        // memset(this, 0, sizeof(*this));
        // IM_STATIC_ASSERT(IM_ARRAYSIZE(ImGuiIO::MouseDown) == ImGuiMouseButton_COUNT && IM_ARRAYSIZE(ImGuiIO::MouseClicked) == ImGuiMouseButton_COUNT);

        // Settings
        out.ConfigFlags = ImGuiConfigFlags_None;
        out.BackendFlags = ImGuiBackendFlags_None;
        out.DisplaySize = ImVec2(-1.0, -1.0);
        out.DeltaTime = 1.0 / 60.0;
        out.IniSavingRate = 5.0;
        out.IniFilename = "imgui.ini".to_string(); // Important: "imgui.ini" is relative to current working dir, most apps will want to lock this to an absolute path (e.g. same path as executables).
        out.LogFilename = "imgui_log.txt".to_string();
        out.MouseDoubleClickTime = 0.30;
        out.MouseDoubleClickMaxDist = 6.0;
        // # ifndef
        // IMGUI_DISABLE_OBSOLETE_KEYIO
        // for (int i = 0; i < ImGuiKey_COUNT; i+ +)
        // KeyMap[i] = -1;
        // # endif
        out.KeyRepeatDelay = 0.275;
        out.KeyRepeatRate = 0.050;
        out.UserData = NULL;

        out.Fonts = NULL;
        out.FontGlobalScale = 1.0;
        out.FontDefault = NULL;
        out.FontAllowUserScaling = false;
        out.DisplayFramebufferScale = ImVec2(1.0, 1.0);

        // Docking options (when ImGuiConfigFlags_DockingEnable is set)
        out.ConfigDockingNoSplit = false;
        out.ConfigDockingWithShift = false;
        out.ConfigDockingAlwaysTabBar = false;
        out.ConfigDockingTransparentPayload = false;

        // viewport options (when ImGuiConfigFlags_ViewportsEnable is set)
        out.ConfigViewportsNoAutoMerge = false;
        out.ConfigViewportsNoTaskBarIcon = false;
        out.ConfigViewportsNoDecoration = true;
        out.ConfigViewportsNoDefaultParent = false;

        // Miscellaneous options
        out.MouseDrawCursor = false;
        // # ifdef
        // __APPLE__
        // ConfigMacOSXBehaviors = true;  // Set Mac OS X style defaults based on __APPLE__ compile time flag # else
        // ConfigMacOSXBehaviors = false;
        // # endif
        out.ConfigInputTrickleEventQueue = true;
        out.ConfigInputTextCursorBlink = true;
        out.ConfigWindowsResizeFromEdges = true;
        out.ConfigWindowsMoveFromTitleBarOnly = false;
        out.ConfigMemoryCompactTimer = 60.0;

        // Platform Functions
        // out.BackendPlatformName = BackendRendererName = NULL;
        out.BackendPlatformName = "".to_string();
        out.BackendRendererName = "".to_string();
        // out.BackendPlatformUserData = BackendRendererUserData = BackendLanguageUserData = NULL;
        out.BackendPlatformUserData = Vec::new();
        out.GetClipboardTextFn = GetClipboardTextFn_DefaultImpl;   // Platform dependent default implementations
        out.SetClipboardTextFn = SetClipboardTextFn_DefaultImpl;
        out.ClipboardUserData = Vec::new();
        out.SetPlatformImeDataFn = SetPlatformImeDataFn_DefaultImpl;

        // Input (NB: we already have memset zero the entire structure!)
        out.MousePos = ImVec2(-FLT_MAX, -FLT_MAX);
        out.MousePosPrev = ImVec2(-FLT_MAX, -FLT_MAX);
        out.MouseDragThreshold = 6.0;
        // for (int i = 0; i < IM_ARRAYSIZE(MouseDownDuration); i+ +) MouseDownDuration[i] = MouseDownDurationPrev[i] = -1.0;
        out.MouseDownDurationPrev = [-1.0; 5];
        out.MouseDownDuration = [-1.0; 5];
        // TODO
        // for (int i = 0; i < IM_ARRAYSIZE(KeysData); i+ +) { KeysData[i].down_duration = KeysData[i].down_duration_prev = -1.0; }
        //out.KeysData =

        // for (int i = 0; i < IM_ARRAYSIZE(NavInputsDownDuration); i+ +) NavInputsDownDuration[i] = -1.0;
        // out.NavInputsDownDuration
        out.AppAcceptingEvents = true;
        out.BackendUsingLegacyKeyArrays = -1;
        out.BackendUsingLegacyNavInputArray = true; // assume using legacy array until proven wrong
        out
    }


    // Input Functions
    //  void  AddKeyEvent(ImGuiKey key, bool down);                   // Queue a new key down/up event. Key should be "translated" (as in, generally ImGuiKey_A matches the key end-user would use to emit an 'A' character)
    pub fn AddKeyEvent(&mut self, key: &ImGuiKey, down: bool) {
        if !AppAcceptingEvents {
            return;
        }
        self.AddKeyAnalogEvent(key, down, if down { 1.0 } else { 0.0 });
    }
    //  void  AddKeyAnalogEvent(ImGuiKey key, bool down, float v);    // Queue a new key down/up event for analog values (e.g. ImGuiKey_Gamepad_ values). Dead-zones should be handled by the backend.
    // Queue a new key down/up event.
// - ImGuiKey key:       Translated key (as in, generally ImGuiKey_A matches the key end-user would use to emit an 'A' character)
// - bool down:          Is the key down? use false to signify a key release.
// - float analog_value: 0.0..1.0
    pub fn AddKeyAnalogEvent(&mut self, key: &ImGuiKey, down: bool, v: f32) {
        //if (e->down) { IMGUI_DEBUG_LOG_IO("AddKeyEvent() Key='%s' %d, NativeKeycode = %d, NativeScancode = %d\n", ImGui::GetKeyName(e->Key), e->down, e->NativeKeycode, e->NativeScancode); }
        if key == ImGuiKey_None || !self.AppAcceptingEvents {
            return;
        }
        //ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.");
        // IM_ASSERT(ImGui::IsNamedKey(key)); // Backend needs to pass a valid ImGuiKey_ constant. 0..511 values are legacy native key codes which are not accepted by this API.

        // Verify that backend isn't mixing up using new io.AddKeyEvent() api and old io.KeysDown[] + io.KeyMap[] data.
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//     IM_ASSERT((BackendUsingLegacyKeyArrays == -1 || BackendUsingLegacyKeyArrays == 0) && "Backend needs to either only use io.AddKeyEvent(), either only fill legacy io.KeysDown[] + io.KeyMap[]. Not both!");
//     if (BackendUsingLegacyKeyArrays == -1)
//         for (int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n++)
//             IM_ASSERT(KeyMap[n] == -1 && "Backend needs to either only use io.AddKeyEvent(), either only fill legacy io.KeysDown[] + io.KeyMap[]. Not both!");
//     BackendUsingLegacyKeyArrays = 0;
// #endif
        if IsGamepadKey(key) {
            self.BackendUsingLegacyNavInputArray = false;
        }

        // Partial filter of duplicates (not strictly needed, but makes data neater in particular for key mods and gamepad values which are most commonly spmamed)
        let key_data = GetKeyData(key);
        if key_data.Down == down && key_data.AnalogValue == analog_value {
            let mut found = false;
            // for (int n = g.input_events_queue.size - 1; n >= 0 && !found; n--){
            let mut n = GImGui.InputEventsQueue.Size - 1;
            while n >= 0 && !found {
                if GImGui.InputEventsQueue[n].Type == ImGuiInputEventType_Key && GImGui.InputEventsQueue[n].Key.Key == key {
                    found = true;
                }
            }
            if !found {
                return;
            }
        }

        // Add event
        let mut e: ImGuiInputEvent = ImGuiInputEvent::new();
        e.Type = ImGuiInputEventType_Key;
        e.Source = if IsGamepadKey(key) {
            ImGuiInputSource_Gamepad
        } else { ImGuiInputSource_Keyboard };
        e.Key.Key = key;
        e.Key.Down = down;
        e.Key.AnalogValue = analog_value;
        GImGui.InputEventsQueue.push_back(e);
    }
    //  void  AddMousePosEvent(float x, float y);                     // Queue a mouse position update. Use -FLT_MAX,-FLT_MAX to signify no mouse (e.g. app not focused and not hovered)
    pub fn AddMousePosEvent(&mut self, x: f32, y: f32) {
        // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.");
        if !AppAcceptingEvents {
            return;
        }

        let mut e = ImGuiInputEvent::new();
        e.Type = ImGuiInputEventType_MousePos;
        e.Source = ImGuiInputSource_Mouse;
        e.MousePos.PosX = x;
        e.MousePos.PosY = y;
        GImGui.InputEventsQueue.push_back(e);
    }
    //  void  AddMouseButtonEvent(int button, bool down);             // Queue a mouse button change
    pub fn AddMouseButtonEvent(&mut self, button: i32, down: bool) {
        // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.");
        // IM_ASSERT(mouse_button >= 0 && mouse_button < ImGuiMouseButton_COUNT);
        if !self.AppAcceptingEvents {
            return;
        }

        let mut e = ImGuiInputEvent::new();
        e.Type = ImGuiInputEventType_MouseButton;
        e.Source = ImGuiInputSource_Mouse;
        e.MouseButton.Button = mouse_button;
        e.MouseButton.Down = down;
        GImGui.InputEventsQueue.push_back(e);
    }
    //  void  AddMouseWheelEvent(float wh_x, float wh_y);             // Queue a mouse wheel update
    pub fn AddMouseWheelEvent(&mut self, wh_x: f32, wh_y: f32) {
        // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.");
        if (wheel_x == 0.0 && wheel_y == 0.0) || !AppAcceptingEvents {
            return;
        }

        //ImGuiInputEvent e;
        let mut e = ImGuiInputEvent::new();
        e.Type = ImGuiInputEventType_MouseWheel;
        e.Source = ImGuiInputSource_Mouse;
        e.MouseWheel.WheelX = wheel_x;
        e.MouseWheel.WheelY = wheel_y;
        GImGui.InputEventsQueue.push_back(e);
    }
    //  void  AddMouseViewportEvent(ImGuiID id);                      // Queue a mouse hovered viewport. Requires backend to set ImGuiBackendFlags_HasMouseHoveredViewport to call this (for multi-viewport support).
    pub fn AddMouseViewportEvent(&mut self, id: ImGuiID) {
        // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.");
        // IM_ASSERT(g.io.BackendFlags & ImGuiBackendFlags_HasMouseHoveredViewport);

        // ImGuiInputEvent e;
        let mut e = ImGuiInputEvent::new();
        e.Type = ImGuiInputEventType_MouseViewport;
        e.Source = ImGuiInputSource_Mouse;
        e.MouseViewport.HoveredViewportID = viewport_id;
        GImGui.InputEventsQueue.push_back(e);
    }
    //  void  AddFocusEvent(bool focused);                            // Queue a gain/loss of focus for the application (generally based on OS/platform focus of your window)
    pub fn AddFocusEvent(&mut self, focused: bool) {
        // ImGuiContext& g = *GImGui;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.");

        // ImGuiInputEvent e;
        let mut e = ImGuiInputEvent::new();
        e.Type = ImGuiInputEventType_Focus;
        e.AppFocused.Focused = focused;
        GImGui.InputEventsQueue.push_back(e);
    }
    //  void  AddInputCharacter(unsigned int c);                      // Queue a new character input

    // Pass in translated ASCII characters for text input.
// - with glfw you can get those from the callback set in glfwSetCharCallback()
// - on windows you can get those using ToAscii+keyboard state, or via the WM_CHAR message
// FIXME: Should in theory be called "AddCharacterEvent()" to be consistent with new API
    pub fn AddInputCharacter(c: u32) {
        // ImGuiContext & g = *;
        // IM_ASSERT(&g.io == this && "Can only add events to current context.".to_string());
        if c == 0 || !AppAcceptingEvents {
            return;
        }
        let e: ImGuiInputEvent = ImGuiInputEvent {
            Type: ImGuiInputEventType_Text,
            Source: ImGuiInputSource_Keyboard,
            val: ImGuiInputEventVal::new(),
            AddedByTestEngine: false,
        };
        GImGui.InputEventsQueue.push_back(e);
    }
    //  void  AddInputCharacterUTF16(ImWchar16 c);                    // Queue a new character input from an UTF-16 character, it can be a surrogate
    // UTF16 strings use surrogate pairs to encode codepoints >= 0x10000, so
// we should save the high surrogate.
    pub fn AddInputCharacterUTF16(&mut self, c: ImWchar16) {
        if (c == 0 && InputQueueSurrogate == 0) || !AppAcceptingEvents {
            return;
        }

        if (c & 0xFC00) == 0xD800 { // High surrogate, must save {
            if InputQueueSurrogate != 0 {
                AddInputCharacter(IM_UNICODE_CODEPOINT_INVALID);
            }
            InputQueueSurrogate = c;
            return;
        }

        //ImWchar cp = c;
        let cp: ImWchar = c;
        if InputQueueSurrogate != 0 {
            if (c & 0xFC00) != 0xDC00 { // Invalid low surrogate {
                AddInputCharacter(IM_UNICODE_CODEPOINT_INVALID);
            } else {
// #if IM_UNICODE_CODEPOINT_MAX == 0xFFFF
//             cp = IM_UNICODE_CODEPOINT_INVALID; // Codepoint will not fit in ImWchar
// #else
//             cp = (ImWchar)(((InputQueueSurrogate - 0xD800) << 10) + (c - 0xDC00) + 0x10000);
// #endif
            }

            InputQueueSurrogate = 0;
        }
        self.AddInputCharacter(cp);
    }

    //  void  AddInputCharactersUTF8(const char* str);                // Queue a new characters input from an UTF-8 string
    pub fn AddInputCharactersUTF8(&mut self, in_str: &str) {
        if !self.AppAcceptingEvents {
            return;
        }
        while *utf8_chars != 0 {
            let c: u32 = 0;
            utf8_chars += ImTextCharFromUtf8(&c, utf8_chars, NULL);
            if c != 0 {
                self.AddInputCharacter(c);
            }
        }
    }

    //  void  SetKeyEventNativeData(ImGuiKey key, int native_keycode, int native_scancode, int native_legacy_index = -1); // [Optional] Specify index for legacy <1.87 IsKeyXXX() functions with native indices + specify native keycode, scancode.
    // [Optional] Call after AddKeyEvent().
// Specify native keycode, scancode + Specify index for legacy <1.87 IsKeyXXX() functions with native indices.
// If you are writing a backend in 2022 or don't use IsKeyXXX() with native values that are not ImGuiKey values, you can avoid calling this.
    pub fn SetKeyEventNativeData(key: ImGuiKey, native_keycode: i32, native_scancode: i32, native_legacy_index: i32) {
        if key == ImGuiKey_None {
            return;
        }
        // IM_ASSERT(ImGui::IsNamedKey(key)); // >= 512
        // IM_ASSERT(native_legacy_index == -1 || ImGui::IsLegacyKey(native_legacy_index)); // >= 0 && <= 511
        // IM_UNUSED(native_keycode);  // Yet unused
        // IM_UNUSED(native_scancode); // Yet unused

        // Build native->imgui map so old user code can still call key functions with native 0..511 values.
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//     const int legacy_key = (native_legacy_index != -1) ? native_legacy_index : native_keycode;
//     if (!ImGui::IsLegacyKey(legacy_key))
//         return;
//     KeyMap[legacy_key] = key;
//     KeyMap[key] = legacy_key;
// #else
//     IM_UNUSED(key);
//     IM_UNUSED(native_legacy_index);
// #endif
    }
    //  void  SetAppAcceptingEvents(bool accepting_events);           // Set master flag for accepting key/mouse/text events (default to true). Useful if you have native dialog boxes that are interrupting your application loop/refresh, and you want to disable events being queued while your app is frozen.
    // Set master flag for accepting key/mouse/text events (default to true). Useful if you have native dialog boxes that are interrupting your application loop/refresh, and you want to disable events being queued while your app is frozen.
    pub fn SetAppAcceptingEvents(&mut self, accepting_events: bool) {
        self.AppAcceptingEvents = accepting_events;
    }
    //  void  ClearInputCharacters();                                 // [Internal] clear the text input buffer manually
    pub fn ClearInputCharacters(&mut self) {
        self.InputQueueCharacters.resize(0, 0);
    }
    //  void  ClearInputKeys();                                       // [Internal] Release all keys
    pub fn ClearInputKeys(&mut self) {
        for n in 0..self.KeysData.len() {
            self.KeysData[n].Down = false;
            self.KeysData[n].DownDuration = -1.0;
            self.KeysData[n].DownDurationPrev = -1.0;
        }
        self.KeyCtrl = false;
        self.KeyShift = false;
        self.KeyAlt = false;
        self.KeySuper = false;
        self.KeyMods = ImGuiModFlags_None;
        // for (int n = 0; n < IM_ARRAYSIZE(NavInputsDownDuration); n++)
        for n in 0..self.NavInputsDownDuration.len() {
            self.NavInputsDownDuration[n] = -1.0;
            self.NavInputsDownDurationPrev[n] = -1.0;
        }
    }
}


// (Optional) Access via ImGui::GetPlatformIO()
#[derive(Debug,Clone,Default)]
pub struct DimgPlatformIo
{
    //------------------------------------------------------------------
    // Input - Backend interface/functions + Monitor List
    //------------------------------------------------------------------

    // (Optional) Platform functions (e.g. Win32, GLFW, SDL2)
    // For reference, the second column shows which function are generally calling the Platform Functions:
    //   N = ImGui::NewFrame()                        ~ beginning of the dear imgui frame: read info from platform/OS windows (latest size/position)
    //   F = ImGui::Begin(), ImGui::EndFrame()        ~ during the dear imgui frame
    //   U = ImGui::UpdatePlatformWindows()           ~ after the dear imgui frame: create and update all platform/OS windows
    //   R = ImGui::RenderPlatformWindowsDefault()    ~ render
    //   D = ImGui::DestroyPlatformWindows()          ~ shutdown
    // The general idea is that NewFrame() we will read the current Platform/OS state, and UpdatePlatformWindows() will write to it.
    //
    // The functions are designed so we can mix and match 2 imgui_impl_xxxx files, one for the Platform (~window/input handling), one for Renderer.
    // Custom engine backends will often provide both Platform and Renderer interfaces and so may not need to use all functions.
    // Platform functions are typically called before their Renderer counterpart, apart from Destroy which are called the other way.

    // Platform function --------------------------------------------------- Called by -----


    // (Optional) Monitor list
    // - Updated by: app/backend. Update every frame to dynamically support changing monitor or DPI configuration.
    // - Used by: dear imgui to query DPI info, clamp popups/tooltips within same monitor and not have them straddle monitors.
    // ImVector<ImGuiPlatformMonitor>  Monitors;
    pub Monitors: Vec<ImGuiPlatformMonitor>,

    //------------------------------------------------------------------
    // Output - List of viewports to render into platform windows
    //------------------------------------------------------------------

    // viewports list (the list is updated by calling ImGui::EndFrame or ImGui::Render)
    // (in the future we will attempt to organize this feature to remove the need for a "main viewport")
    // ImVector<ImGuiViewport*>        viewports;                              // Main viewports, followed by all secondary viewports.
    pub Viewports: Vec<ImGuiViewport>,
    // ImGuiPlatformIO()               { memset(this, 0, sizeof(*this)); }     // Zero clear
}

impl DimgPlatformIo {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    // void    (*Platform_CreateWindow)(ImGuiViewport* vp);                    // . . U . .  // Create a new platform window for the given viewport
    pub fn Platform_CreateWindow(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Platform_DestroyWindow)(ImGuiViewport* vp);                   // N . U . D  //
    pub fn Platform_DestroyWindow(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Platform_ShowWindow)(ImGuiViewport* vp);                      // . . U . .  // Newly created windows are initially hidden so SetWindowPos/size/Title can be called on them before showing the window
    pub fn Platform_ShowWindow(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Platform_SetWindowPos)(ImGuiViewport* vp, ImVec2 pos);        // . . U . .  // Set platform window position (given the upper-left corner of client area)
    pub fn Platform_SetWindowPos(&mut self, vp: &mut ImGuiViewport, pos: ImVec2) {
        todo!()
    }
    //     ImVec2  (*Platform_GetWindowPos)(ImGuiViewport* vp);                    // N . . . .  //
    pub fn Platform_GetWindowPos(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Platform_SetWindowSize)(ImGuiViewport* vp, ImVec2 size);      // . . U . .  // Set platform window client area size (ignoring OS decorations such as OS title bar etc.)
    pub fn Platform_SetWindowSize(&mut self, vp: &mut ImGuiViewport, size: &ImVec2) {
        todo!()
    }
    //     ImVec2  (*Platform_GetWindowSize)(ImGuiViewport* vp);                   // N . . . .  // Get platform window client area size
    pub fn Platform_GetWindowSize(&mut self, vp: &mut ImGuiViewport) -> ImVec2 {
        todo!()
    }
    //     void    (*Platform_SetWindowFocus)(ImGuiViewport* vp);                  // N . . . .  // Move window to front and set input focus
    pub fn Platform_SetWindowFocus(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }

    //     bool    (*Platform_GetWindowFocus)(ImGuiViewport* vp);                  // . . U . .  //
    pub fn Platform_GetWindowFocus(&mut self, vp: &mut ImGuiViewport) -> bool {
        todo!()
    }
    //     bool    (*Platform_GetWindowMinimized)(ImGuiViewport* vp);              // N . . . .  // Get platform window minimized state. When minimized, we generally won't attempt to get/set size and contents will be culled more easily
    pub fn Platform_GetWindowMinimized(&mut self, vp: &mut ImGuiViewport) -> bool {
        todo!()
    }
    //     void    (*Platform_SetWindowTitle)(ImGuiViewport* vp, const char* str); // . . U . .  // Set platform window title (given an UTF-8 string)
    pub fn Platform_SetWindowTitle(&mut self, vp: &mut ImGuiViewport, in_str: &String) {
        todo!()
    }
    //     void    (*Platform_SetWindowAlpha)(ImGuiViewport* vp, float alpha);     // . . U . .  // (Optional) Setup global transparency (not per-pixel transparency)
    pub fn Platform_SetWindowAlpha(&mut self, vp: &mut ImGuiViewport, alpha: f32) {
        todo!()
    }
    //     void    (*Platform_UpdateWindow)(ImGuiViewport* vp);                    // . . U . .  // (Optional) Called by UpdatePlatformWindows(). Optional hook to allow the platform backend from doing general book-keeping every frame.
    pub fn Platform_UpdateWindow(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Platform_RenderWindow)(ImGuiViewport* vp, void* render_arg);  // . . . R .  // (Optional) Main rendering (platform side! This is often unused, or just setting a "current" context for OpenGL bindings). 'render_arg' is the value passed to RenderPlatformWindowsDefault().
    pub fn Platform_RenderWindow(&mut self, vp: &mut ImGuiViewport, render_arg: *mut c_void) {
        todo!()
    }
    //     void    (*Platform_SwapBuffers)(ImGuiViewport* vp, void* render_arg);   // . . . R .  // (Optional) Call Present/SwapBuffers (platform side! This is often unused!). 'render_arg' is the value passed to RenderPlatformWindowsDefault().
    pub fn Platform_SwapBuffers(&mut self, vp: &mut ImGuiViewport, render_arg: *mut c_void) {
        todo!()
    }
    //     float   (*Platform_GetWindowDpiScale)(ImGuiViewport* vp);               // N . . . .  // (Optional) [BETA] FIXME-DPI: DPI handling: Return DPI scale for this viewport. 1.0 = 96 DPI.
    pub fn Platform_GetWindowDpiScale(&mut self, vp: &mut ImGuiViewport) -> f32 {
        todo!()
    }

    //     void    (*Platform_OnChangedViewport)(ImGuiViewport* vp);               // . F . . .  // (Optional) [BETA] FIXME-DPI: DPI handling: Called during Begin() every time the viewport we are outputting into changes, so backend has a chance to swap fonts to adjust style.
    pub fn Platform_OnChangedViewport(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     int     (*Platform_CreateVkSurface)(ImGuiViewport* vp, ImU64 vk_inst, const void* vk_allocators, ImU64* out_vk_surface); // (Optional) For a Vulkan Renderer to call into Platform code (since the surface creation needs to tie them both).
    pub fn Platform_CreateVkSurface(&mut self, vp: &mut ImGuiViewport, vk_inst: u64, vk_allocators: *const c_void, out_vk_surface: &mut u64) -> i32 {
        todo!()
    }

    //
    //     // (Optional) Renderer functions (e.g. DirectX, OpenGL, Vulkan)
    //     void    (*Renderer_CreateWindow)(ImGuiViewport* vp);                    // . . U . .  // Create swap chain, frame buffers etc. (called after Platform_CreateWindow)
    pub fn Platform_CreateWindow2(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Renderer_DestroyWindow)(ImGuiViewport* vp);                   // N . U . D  // Destroy swap chain, frame buffers etc. (called before Platform_DestroyWindow)
    pub fn Platform_DestroyWindow2(&mut self, vp: &mut ImGuiViewport) {
        todo!()
    }
    //     void    (*Renderer_SetWindowSize)(ImGuiViewport* vp, ImVec2 size);      // . . U . .  // Resize swap chain, frame buffers etc. (called after Platform_SetWindowSize)
    pub fn Renderer_SetWindowSize(&mut self, vp: &mut ImGuiViewport, size: ImVec2) {
        todo!()
    }
    //     void    (*Renderer_RenderWindow)(ImGuiViewport* vp, void* render_arg);  // . . . R .  // (Optional) clear framebuffer, setup render target, then render the viewport->DrawData. 'render_arg' is the value passed to RenderPlatformWindowsDefault().
    pub fn Renderer_RenderWindow(&mut self, vp: &mut ImGuiViewport, render_arg: *mut c_void) {
        todo!()
    }
    //     void    (*Renderer_SwapBuffers)(ImGuiViewport* vp, void* render_arg);   // . . . R .  // (Optional) Call Present/SwapBuffers. 'render_arg' is the value passed to RenderPlatformWindowsDefault().
    pub fn Renderer_SwapBuffers(&mut self, vp: &mut ImGuiViewport, render_arg: &mut c_void) {
        todo!()
    }
}
